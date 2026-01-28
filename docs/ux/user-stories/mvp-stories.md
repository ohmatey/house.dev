# MVP User Stories - house.dev

**Document Status:** Living Document
**Last Updated:** 2026-01-27
**Project:** house.dev - Developer disk space cleanup tool
**Version:** 1.0 MVP

---

## TLDR

- **5 core user stories** covering the complete MVP workflow
- Focus: Select folder → Scan → Identify large folders → Delete safely
- All stories align with single persona: Multi-Project Developer
- Success criteria emphasize speed, clarity, and safety
- MVP scope intentionally minimal to validate core value proposition

---

## Story Format

Each story follows this structure:
- **As a** [persona]
- **I want to** [action]
- **So that** [outcome/benefit]
- **Acceptance Criteria** (testable, specific)
- **Priority** (P0 = MVP critical, P1 = MVP nice-to-have)

---

## User Story 1: Select Projects Folder

**As a** developer with multiple projects,
**I want to** select my projects directory once and have it remembered,
**So that** I never have to reconfigure the tool or manually navigate folders again.

### Acceptance Criteria

**Given** I launch house.dev for the first time
**When** I see the initial empty state
**Then** I see a clear call-to-action to "Select Folder"

**Given** I click "Select Folder"
**When** the native folder picker opens
**Then** I can browse my filesystem and select any directory

**Given** I select a valid directory (e.g., ~/projects)
**When** the selection is confirmed
**Then** the selected path is displayed in the UI
**And** the path is persisted to local storage
**And** an automatic scan begins immediately

**Given** I close and reopen the app
**When** the app launches
**Then** my previously selected folder is automatically loaded
**And** the app begins scanning without requiring reselection

**Given** I want to change the projects folder
**When** I click "Select Folder" again
**Then** I can choose a different directory
**And** the new path replaces the old one
**And** a new scan begins automatically

### Priority
**P0 - Critical**

### Related JTBD
Primary job: "Set up once, use forever without friction"

---

## User Story 2: Scan for Large Folders

**As a** developer,
**I want to** quickly scan my projects directory and see all large deletable folders,
**So that** I can identify what's consuming disk space without manual searching.

### Acceptance Criteria

**Given** I have selected a valid projects directory
**When** the scan starts
**Then** I see a loading indicator with text "Scanning folders..."
**And** the UI is clearly in a loading state (buttons disabled)

**Given** the scan is running
**When** large folders are discovered
**Then** the scan completes in < 30 seconds for up to 50 projects
**And** only folders > 1MB are included in results

**Given** the scan completes successfully
**When** results are displayed
**Then** I see a list of folders sorted by size (largest first)
**And** each folder shows: name, path, size (human-readable), file count
**And** I see summary statistics: total size, folder count

**Given** the scan finds no large folders
**When** results are displayed
**Then** I see an empty state message: "No large folders found ✨"
**And** the message is positive/reassuring (not an error)

**Given** the scan encounters an error (permissions, invalid path)
**When** the error occurs
**Then** I see a clear error message explaining the problem
**And** I have the option to select a different folder

**Given** I want to refresh results
**When** I click the "Scan" button
**Then** the scan reruns and updates results
**And** the button is disabled during scanning

### Priority
**P0 - Critical**

### Related JTBD
Primary job: "Quickly identify which projects have large deletable folders"

### Technical Notes
- Scans recursively for these folder types: node_modules, .git, target, build, dist, .next, .nuxt, vendor, __pycache__, .venv, venv, .cargo, Pods, DerivedData, .gradle, obj, bin
- Minimum size threshold: 1MB (configurable in backend)
- Results sorted by size descending

---

## User Story 3: Understand Folder Impact

**As a** developer deciding what to delete,
**I want to** see visual size comparisons and detailed folder information,
**So that** I can prioritize which folders to delete first and understand the impact.

### Acceptance Criteria

**Given** scan results are displayed
**When** I view the folder list
**Then** each folder row shows:
- Folder name and parent path (on hover/tooltip)
- Size in human-readable format (MB, GB)
- File count (formatted with commas)
- Visual size bar (relative to largest folder)
- Folder icon
- Delete button

**Given** folders vary in size
**When** I view the size bars
**Then** the largest folder has a 100% width bar
**And** smaller folders have proportionally sized bars
**And** bars use color coding:
  - Red/orange for > 500MB (large)
  - Yellow for 100MB-500MB (medium)
  - Default for < 100MB

**Given** I see the summary statistics bar
**When** results are loaded
**Then** I see:
- Total size of all deletable folders
- Total count of folders found
**And** statistics are prominently displayed above the folder list

**Given** I hover over a folder row
**When** my cursor is over the folder name
**Then** I see the full absolute path in a tooltip
**And** the row highlights slightly for clarity

### Priority
**P0 - Critical**

### Related JTBD
Functional job: "See visual size comparisons to prioritize deletion"

### Design Notes
- Size bars provide instant visual hierarchy
- Color coding draws attention to largest offenders
- Tooltip prevents path truncation issues

---

## User Story 4: Safely Delete Folders

**As a** developer,
**I want to** delete large folders with confidence they'll be moved to trash (not permanently deleted),
**So that** I can reclaim space without fear of losing important data.

### Acceptance Criteria

**Given** I want to delete a folder
**When** I click the delete button (🗑) next to a folder
**Then** a confirmation dialog appears
**And** the dialog shows:
  - Title: "Delete folder?"
  - Full path of folder to be deleted
  - Warning: "This will permanently delete: [path]"
  - "Cancel" and "Delete" buttons

**Given** the confirmation dialog is open
**When** I click "Cancel" or click outside the dialog
**Then** the dialog closes
**And** no deletion occurs
**And** the folder remains in the list

**Given** the confirmation dialog is open
**When** I click "Delete"
**Then** the folder is moved to system trash (not permanently deleted)
**And** the dialog closes
**And** a new scan is triggered automatically
**And** the deleted folder no longer appears in results

**Given** a deletion fails (permissions error, folder in use)
**When** the error occurs
**Then** I see an error message explaining the failure
**And** the folder remains in the list
**And** I can try again or skip it

**Given** I delete multiple folders in succession
**When** each deletion completes
**Then** the scan updates automatically after each deletion
**And** the folder list refreshes without requiring manual "Scan" button clicks

### Priority
**P0 - Critical**

### Related JTBD
Emotional job: "Feel confident I'm deleting safely, not losing important code"

### Safety Implementation
- **Move to trash** (not `rm -rf`) using system trash APIs
- **Confirmation required** - no accidental single-click deletion
- **Full path displayed** - user sees exactly what will be deleted
- **Recoverable** - user can restore from trash if mistake made

### Technical Notes
- Backend uses system trash/recycle bin APIs (macOS/Linux/Windows)
- Permissions errors handled gracefully
- Auto-rescan after deletion keeps UI in sync

---

## User Story 5: Persistent Workflow State

**As a** developer who uses house.dev regularly,
**I want to** have my folder selection and preferences remembered across app restarts,
**So that** I can open the app and immediately see current results without reconfiguration.

### Acceptance Criteria

**Given** I have previously selected a projects folder
**When** I close and reopen the app
**Then** my folder selection is automatically loaded
**And** the selected path is displayed in the UI
**And** an automatic scan begins on launch

**Given** the app launches with a saved folder path
**When** the automatic scan completes
**Then** I see up-to-date results immediately
**And** I don't need to click "Scan" manually

**Given** I delete some folders and close the app
**When** I reopen the app later
**Then** the scan runs fresh (not showing old deleted folders)
**And** results reflect the current state of my projects directory

**Given** my saved folder path no longer exists (renamed, moved, deleted)
**When** the app launches
**Then** I see an error message explaining the folder is unavailable
**And** I have the option to select a new folder
**And** the invalid path is cleared from storage

**Given** I want to clear my saved path
**When** I select a new folder
**Then** the new path fully replaces the old one
**And** there's no stale configuration

### Priority
**P0 - Critical**

### Related JTBD
Functional job: "Set once, forget configuration forever"

### Persistence Implementation
- Uses Tauri's plugin-store for local storage
- Key: `watched_path`
- Validates path exists before auto-scanning
- Graceful error handling for stale paths

---

## Out of Scope for MVP

The following features are explicitly **NOT** included in MVP to maintain focus on core value:

**Multi-Folder Selection**
- Rationale: Adds complexity; single projects directory covers 90% of use cases
- Future: Could add in v2 if user research shows need

**Custom Folder Type Configuration**
- Rationale: Predefined list of heavy folders covers all major ecosystems
- Future: Allow users to add custom folder names in settings

**Folder Age Filtering**
- Rationale: Users can manually decide based on path/project recognition
- Future: "Last modified" date could help identify stale projects

**Batch Selection & Deletion**
- Rationale: MVP focuses on careful, deliberate deletion
- Future: Checkbox selection + "Delete selected" button

**Statistics & History**
- Rationale: Not critical for core job (reclaim space now)
- Future: Track "space saved over time" or deletion history

**Dark/Light Mode Toggle**
- Rationale: Developers overwhelmingly prefer dark mode
- Future: Add light mode if user feedback requests it

**Scheduled Scans**
- Rationale: Not needed for reactive tool (used when disk full)
- Future: Background monitoring or scheduled cleanup notifications

---

## Success Metrics

### User Experience Metrics
- **Time to first scan:** < 2 minutes from first launch (including folder selection)
- **Scan performance:** < 30 seconds for 50+ projects
- **Deletion speed:** < 5 seconds per folder (including confirmation)
- **Error rate:** < 5% of scan/delete operations fail

### Qualitative Success Criteria
- Users report feeling "in control" of disk space
- Users trust the tool with deletion (trash safety)
- Users don't need documentation to complete primary workflow
- Tool feels "fast" and "minimal" (not bloated)

### Business/Product Metrics (Future)
- GitHub stars as proxy for developer interest
- Weekly active users (if telemetry added)
- User retention (return usage after initial cleanup)

---

## Testing Strategy

**Manual Testing:**
- Test on macOS, Linux, Windows
- Test with various project types (Node, Rust, Python, Go)
- Test with empty directories, single project, 50+ projects
- Test error cases: invalid paths, permission errors, disk full
- Test folder deletion: verify trash functionality, test recovery

**Automated Testing (Future):**
- Unit tests for size calculations and formatting
- Integration tests for scan and delete operations
- E2E tests for complete user workflows

---

## References

**Persona:** `/docs/ux/personas.md` - Multi-Project Developer
**Related Workflows:** Core workflow is linear: Select → Scan → Review → Delete
**Technical Implementation:** See `/src/main.ts` for current implementation

---

## Changelog

**2026-01-27:**
- Initial MVP user stories created
- 5 stories covering complete core workflow
- Acceptance criteria focused on speed, safety, clarity
- Explicitly defined out-of-scope features to prevent scope creep
