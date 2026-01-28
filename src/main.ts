import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open } from "@tauri-apps/plugin-dialog";
import { load, Store } from "@tauri-apps/plugin-store";

interface FolderInfo {
  path: string;
  name: string;
  size: number;
  file_count: number;
}

interface ScanResult {
  folders: FolderInfo[];
  total_size: number;
  scan_path: string;
}

interface PathValidation {
  valid: boolean;
  reason: string | null;
}

interface ComplexityCheck {
  estimated_directories: number;
  is_large: boolean;
  is_blocked: boolean;
  recommendation: string;
}

let store: Store;
let currentPath: string | null = null;
let isScanning = false;
let folders: FolderInfo[] = [];
let showRecoveryBanner = false;

const SCAN_TIMEOUT_MS = 60000; // 60 second timeout

async function init() {
  store = await load("settings.json");
  currentPath = (await store.get<string>("watched_path")) ?? null;

  // Validate saved path on startup before auto-scanning
  if (currentPath) {
    const validation = await invoke<PathValidation>("validate_path", {
      path: currentPath,
    });

    if (!validation.valid) {
      console.warn("Saved path is invalid:", validation.reason);
      showRecoveryBanner = true;
      currentPath = null;
    }
  }

  render();

  if (currentPath) {
    await safeScan();
  }

  // Hide to tray on window close instead of quitting
  const appWindow = getCurrentWindow();
  appWindow.onCloseRequested(async (event) => {
    event.preventDefault();
    await appWindow.hide();
  });

  // Listen for scan trigger from tray menu
  listen("trigger-scan", () => {
    safeScan();
  });

  // Emergency reset keyboard shortcut: Cmd+Shift+R (Mac) or Ctrl+Shift+R (Windows/Linux)
  document.addEventListener("keydown", async (e) => {
    if ((e.metaKey || e.ctrlKey) && e.shiftKey && e.key.toLowerCase() === "r") {
      e.preventDefault();
      const confirmed = await showWarningDialog(
        "Reset Application",
        "This will clear all saved settings and stop any running scans. Continue?",
        "Reset"
      );
      if (confirmed) {
        await clearSavedPath();
        window.location.reload();
      }
    }
  });
}

async function selectFolder() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: "Select projects folder to monitor",
  });

  if (selected && typeof selected === "string") {
    // Layer 1: Validate path safety
    const validation = await invoke<PathValidation>("validate_path", {
      path: selected,
    });

    if (!validation.valid) {
      await showWarningDialog(
        "Invalid Folder",
        validation.reason || "This folder cannot be scanned.",
        "OK"
      );
      return;
    }

    // Layer 2: Check folder complexity
    try {
      const complexity = await invoke<ComplexityCheck>("check_folder_complexity", {
        path: selected,
      });

      // Hard block for extremely large folders
      if (complexity.is_blocked) {
        await showWarningDialog(
          "Folder Too Large",
          `${complexity.recommendation}\n\nEstimated directories: ~${complexity.estimated_directories.toLocaleString()}`,
          "OK"
        );
        return;
      }

      if (complexity.is_large) {
        const proceed = await showWarningDialog(
          "Large Folder Warning",
          `${complexity.recommendation}\n\nEstimated directories: ~${complexity.estimated_directories.toLocaleString()}\n\nContinue anyway?`,
          "Continue"
        );

        if (!proceed) {
          return;
        }
      }
    } catch (err) {
      console.error("Complexity check failed:", err);
      // Continue anyway if complexity check fails
    }

    // Path validated, save and scan
    currentPath = selected;
    showRecoveryBanner = false;
    await store.set("watched_path", currentPath);
    render();
    await safeScan();
  }
}

async function clearSavedPath(): Promise<void> {
  currentPath = null;
  folders = [];
  isScanning = false;
  showRecoveryBanner = false;
  await store.delete("watched_path");
  render();
}

async function cancelScan(): Promise<void> {
  try {
    await invoke("cancel_scan");
  } catch (err) {
    console.error("Failed to cancel scan:", err);
  }
  isScanning = false;
  render();
}

async function safeScan(): Promise<void> {
  if (!currentPath || isScanning) return;

  isScanning = true;
  render();

  try {
    // Create a timeout promise
    const timeoutPromise = new Promise<never>((_, reject) => {
      setTimeout(() => reject(new Error("Scan timed out")), SCAN_TIMEOUT_MS);
    });

    // Race between scan and timeout
    const result = await Promise.race([
      invoke<ScanResult>("scan_folders", { path: currentPath }),
      timeoutPromise,
    ]);

    folders = result.folders;
  } catch (err) {
    console.error("Scan failed:", err);
    folders = [];

    if (err instanceof Error && err.message === "Scan timed out") {
      showRecoveryBanner = true;
      const reset = await showWarningDialog(
        "Scan Timed Out",
        "The scan took too long. This folder may be too large.\n\nWould you like to clear the saved folder and select a different one?",
        "Clear & Reset"
      );
      if (reset) {
        await clearSavedPath();
        return;
      }
    }
  }

  isScanning = false;
  render();
}

async function deleteFolder(path: string) {
  const confirmed = await showConfirmDialog(
    "Delete folder?",
    `This will move to trash:\n${path}`
  );

  if (confirmed) {
    try {
      await invoke("delete_folder", { path });
      await safeScan();
    } catch (err) {
      console.error("Delete failed:", err);
    }
  }
}

async function openInFinder(path: string) {
  try {
    await invoke("open_in_finder", { path });
  } catch (err) {
    console.error("Failed to open in Finder:", err);
  }
}

function getFolderType(folderPath: string): string {
  const parts = folderPath.split("/");
  return parts[parts.length - 1] || "";
}

function getRelativePath(fullPath: string, basePath: string | null): string {
  if (!basePath) return fullPath;
  // Remove the base path and the folder name to get the path in between
  const relativePath = fullPath.startsWith(basePath)
    ? fullPath.slice(basePath.length)
    : fullPath;
  // Remove leading slash and the folder name at the end
  const parts = relativePath.replace(/^\//, "").split("/");
  // Remove the last part (the folder itself) to show path TO it
  parts.pop();
  return parts.join("/") || ".";
}

function showConfirmDialog(title: string, message: string): Promise<boolean> {
  return new Promise((resolve) => {
    const overlay = document.createElement("div");
    overlay.className = "dialog-overlay";
    overlay.innerHTML = `
      <div class="dialog">
        <h3>${title}</h3>
        <p>${message.replace(/\n/g, "<br>")}</p>
        <div class="dialog-actions">
          <button class="btn btn-secondary" id="dialog-cancel">Cancel</button>
          <button class="btn btn-danger" id="dialog-confirm">Delete</button>
        </div>
      </div>
    `;

    document.body.appendChild(overlay);

    overlay.querySelector("#dialog-cancel")?.addEventListener("click", () => {
      overlay.remove();
      resolve(false);
    });

    overlay.querySelector("#dialog-confirm")?.addEventListener("click", () => {
      overlay.remove();
      resolve(true);
    });

    overlay.addEventListener("click", (e) => {
      if (e.target === overlay) {
        overlay.remove();
        resolve(false);
      }
    });
  });
}

function showWarningDialog(
  title: string,
  message: string,
  confirmText: string
): Promise<boolean> {
  return new Promise((resolve) => {
    const overlay = document.createElement("div");
    overlay.className = "dialog-overlay";
    overlay.innerHTML = `
      <div class="dialog warning-dialog">
        <h3>⚠️ ${title}</h3>
        <p>${message.replace(/\n/g, "<br>")}</p>
        <div class="dialog-actions">
          <button class="btn btn-secondary" id="dialog-cancel">Cancel</button>
          <button class="btn btn-warning" id="dialog-confirm">${confirmText}</button>
        </div>
      </div>
    `;

    document.body.appendChild(overlay);

    overlay.querySelector("#dialog-cancel")?.addEventListener("click", () => {
      overlay.remove();
      resolve(false);
    });

    overlay.querySelector("#dialog-confirm")?.addEventListener("click", () => {
      overlay.remove();
      resolve(true);
    });

    overlay.addEventListener("click", (e) => {
      if (e.target === overlay) {
        overlay.remove();
        resolve(false);
      }
    });
  });
}

function formatSize(bytes: number): string {
  if (bytes === 0) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
}

function getSizeClass(bytes: number): string {
  if (bytes > 500 * 1024 * 1024) return "large"; // > 500MB
  if (bytes > 100 * 1024 * 1024) return "medium"; // > 100MB
  return "";
}

function render() {
  const app = document.getElementById("app");
  if (!app) return;

  const totalSize = folders.reduce((sum, f) => sum + f.size, 0);
  const maxSize = Math.max(...folders.map((f) => f.size), 1);

  app.innerHTML = `
    ${
      showRecoveryBanner
        ? `
      <div class="recovery-banner">
        <span>⚠️ There was an issue with the saved folder path.</span>
        <button class="btn btn-small btn-secondary" id="clear-path">Clear & Reset</button>
        <span class="recovery-hint">Press Cmd+Shift+R for emergency reset</span>
      </div>
    `
        : ""
    }
    <header class="header">
      <h1>house.dev</h1>
      <div class="header-actions">
        <button class="btn btn-secondary" id="select-folder">
          📁 Select Folder
        </button>
        <button class="btn btn-primary" id="refresh" ${!currentPath || isScanning ? "disabled" : ""}>
          ↻ Scan
        </button>
      </div>
    </header>

    <div class="path-display">
      <span class="path ${!currentPath ? "empty" : ""}">
        ${currentPath || "No folder selected"}
      </span>
    </div>

    ${
      currentPath && !isScanning && folders.length > 0
        ? `
      <div class="stats-bar">
        <div class="stat">
          <span class="stat-label">Total Size</span>
          <span class="stat-value">${formatSize(totalSize)}</span>
        </div>
        <div class="stat">
          <span class="stat-label">Folders</span>
          <span class="stat-value">${folders.length}</span>
        </div>
      </div>
    `
        : ""
    }

    <div class="folder-list">
      ${
        isScanning
          ? `
        <div class="loading">
          <div class="spinner"></div>
          <span class="loading-text">Scanning folders...</span>
          <button class="btn btn-secondary btn-cancel" id="cancel-scan">Cancel</button>
        </div>
      `
          : !currentPath
            ? `
        <div class="empty-state">
          <div class="empty-state-icon">📂</div>
          <div class="empty-state-text">Select a projects folder to start</div>
        </div>
      `
            : folders.length === 0
              ? `
        <div class="empty-state">
          <div class="empty-state-icon">✨</div>
          <div class="empty-state-text">No large folders found</div>
        </div>
      `
              : `
        <div class="folder-list-header">
          <span>Folder</span>
          <span style="text-align: right">Size</span>
          <span style="text-align: right">Files</span>
          <span></span>
        </div>
        ${folders
          .map(
            (folder) => `
          <div class="folder-item">
            <div class="folder-name">
              <span class="folder-icon">📁</span>
              <div class="folder-name-text">
                <span class="folder-type" title="${folder.path}">${getFolderType(folder.path)}</span>
                <span class="folder-path">${getRelativePath(folder.path, currentPath)}</span>
              </div>
            </div>
            <div class="folder-size ${getSizeClass(folder.size)}">
              ${formatSize(folder.size)}
              <div class="size-bar">
                <div class="size-bar-fill ${getSizeClass(folder.size)}" style="width: ${(folder.size / maxSize) * 100}%"></div>
              </div>
            </div>
            <div class="folder-files">${folder.file_count.toLocaleString()}</div>
            <div class="folder-actions">
              <button class="btn btn-ghost btn-open" data-path="${folder.path}" title="Open in Finder">📂</button>
              <button class="btn btn-ghost btn-delete" data-path="${folder.path}" title="Delete">🗑</button>
            </div>
          </div>
        `
          )
          .join("")}
      `
      }
    </div>
  `;

  // Event listeners
  document.getElementById("select-folder")?.addEventListener("click", selectFolder);
  document.getElementById("refresh")?.addEventListener("click", safeScan);
  document.getElementById("clear-path")?.addEventListener("click", clearSavedPath);
  document.getElementById("cancel-scan")?.addEventListener("click", cancelScan);

  document.querySelectorAll(".btn-delete").forEach((btn) => {
    btn.addEventListener("click", () => {
      const path = btn.getAttribute("data-path");
      if (path) deleteFolder(path);
    });
  });

  document.querySelectorAll(".btn-open").forEach((btn) => {
    btn.addEventListener("click", () => {
      const path = btn.getAttribute("data-path");
      if (path) openInFinder(path);
    });
  });
}

init();
