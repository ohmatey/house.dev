# Developer Persona - house.dev

**Document Status:** Living Document
**Last Updated:** 2026-01-27
**Project:** house.dev - Developer disk space cleanup tool

---

## TLDR

- **Primary Persona:** Active Developer maintaining 5-15+ projects simultaneously
- **Core Pain Point:** Disk space consumed by build artifacts and dependencies (node_modules, .git, target, etc.)
- **Key Behavior:** Works on multiple projects but rarely cleans up after switching contexts
- **Primary Goal:** Reclaim disk space quickly without manually hunting through project directories
- **Success Metric:** Ability to identify and delete 10GB+ of unused folders in under 2 minutes

---

## Persona: The Multi-Project Developer

### Demographics
- **Name:** Alex (Composite persona)
- **Role:** Software Developer, Full-Stack Engineer, or Engineering Manager
- **Experience:** 2-15+ years in software development
- **Tech Stack:** Multi-language (JavaScript/TypeScript, Rust, Python, Go, etc.)
- **Work Style:** Juggles 5-15 active projects, frequent context switching
- **Platform:** macOS or Linux primary, Windows secondary

### Psychographics
- **Values:** Efficiency, minimal friction, tools that "just work"
- **Preferences:** CLI-first mindset but appreciates good desktop UIs for visual tasks
- **Workflow:** Fast-paced, impatient with slow tools, allergic to feature bloat
- **Aesthetic:** Prefers dark mode, monospace fonts, developer-focused design

### Technical Context
- **Development Environment:**
  - Uses multiple package managers (npm, pnpm, yarn, cargo, pip, go mod, composer)
  - Builds projects across different frameworks (React, Next.js, Rust, Python, etc.)
  - Manages client projects, side projects, open source contributions, and experimental repos
- **Disk Space Reality:**
  - Each Node.js project: 200MB-2GB in node_modules
  - Git repos: 50MB-500MB+ in .git history
  - Rust projects: 500MB-5GB in target directories
  - Total accumulation: 20GB-100GB+ of deletable artifacts across all projects

### Pain Points & Frustrations

#### Primary Pain Point: Invisible Disk Space Bloat
- **Severity:** High
- **Frequency:** Weekly to monthly
- **Description:** Alex doesn't realize how much space old projects consume until receiving "Disk Almost Full" warnings. At that point, it's urgent and stressful.

**Research Evidence:**
- Developers report cleaning up [over 50GB of npm dependencies](https://itnext.io/how-i-cleaned-up-my-hard-drive-from-over-50-gbs-of-npm-dependencies-5d2d7d2ad476) from accumulated projects
- [Manual cleanup is tedious](https://dev.to/buildwebcrumbs/running-out-of-disk-space-delete-nodemodules-36l6) - finding folders scattered across directories takes time
- Context switching between projects means [abandoned repositories accumulate](https://khides.com/en/blog/developer-disk-cleanup/) without cleanup

#### Secondary Pain Points

**Lack of Visibility**
- Can't easily see which projects are consuming the most space
- Doesn't know whether a project is "safe" to clean (active vs. archived)
- No visual comparison of folder sizes across projects

**Manual Process is Tedious**
- Opening Finder/File Explorer and navigating through nested directories
- Running terminal commands like `du -sh */node_modules` is slow and unergonomic
- No batch operations - must delete folders one at a time
- Fear of permanently deleting wrong folders

**Existing Tools Have Friction**
- CLI tools like [npkill](https://github.com/voidcosmos/npkill) require remembering commands
- Desktop disk analyzers (DaisyDisk, WinDirStat) are generic, not developer-focused
- No focus on specific heavy folders (node_modules, target, .git)

### Jobs-to-be-Done (JTBD)

**Primary Job:**
*When I receive a low disk space warning, I want to quickly identify and delete large project artifacts, so I can reclaim space without disrupting my current work.*

**Functional Jobs:**
- Discover which projects have the largest deletable folders
- Safely delete folders (move to trash, not permanent deletion)
- Scan a specific projects directory rather than entire system
- See visual size comparisons to prioritize what to delete first

**Emotional Jobs:**
- Feel in control of my disk space situation
- Avoid the stress of urgent cleanup under disk pressure
- Feel confident I'm deleting the right things, not important code

**Social Jobs:**
- (Minimal - this is a personal productivity tool)

### Current Workarounds

**Manual Terminal Commands:**
```bash
# Find all node_modules folders
find ~/projects -name "node_modules" -type d

# Calculate sizes (slow)
du -sh ~/projects/*/node_modules
```

**Problems:** Slow, requires terminal expertise, no visual feedback, no batch deletion

**Generic Disk Analyzers:**
- Tools: DaisyDisk, ncdu, WinDirStat, Disk Inventory X

**Problems:** Too generic, show every file type, no focus on developer artifacts, cluttered UI

**Existing Developer Tools:**
- npkill (CLI for node_modules)
- node-cleaner (GUI for node_modules only)

**Problems:** Limited to JavaScript ecosystem, CLI-only or feature-poor GUIs

### Goals & Success Criteria

**Primary Goal:**
Reclaim 10GB+ of disk space in under 2 minutes without fear of deleting important files.

**Success Metrics:**
- Time to scan projects directory: < 30 seconds for 50+ projects
- Time to identify top 10 largest folders: Instant (visual scan of UI)
- Time to delete 5-10 folders: < 60 seconds total
- Confidence in deletion safety: 100% (trash, not permanent delete)

**Secondary Goals:**
- Set projects directory once, never configure again
- Understand what was cleaned up (size/file counts)
- Minimal cognitive load - no decisions about "what is safe to delete"

### Feature Preferences & Anti-Patterns

**What Alex Wants:**
- Minimal, fast, focused UI showing only what matters
- Dark mode (mandatory for credibility with developers)
- Monospace fonts (visual identity as developer tool)
- Instant scan results
- One-click deletion with safety (trash)
- Persistent settings across sessions
- Native desktop app performance (not Electron bloat)

**What Alex Doesn't Want:**
- Feature bloat (settings panels, customization options, themes)
- Onboarding wizards or tutorials
- Asking permission for every action (too many confirmation dialogs)
- Subscription pricing or ads
- Cloud sync or accounts
- Generic file management features

### Technology Adoption Profile

**Early Adopter Characteristics:**
- Comfortable installing from GitHub releases or package managers
- Reads README before installing
- Values open source transparency
- Willing to report bugs or contribute
- Shares useful tools with team in Slack/Discord

**Decision Criteria:**
- "Does it solve my specific problem?"
- "Is it fast and minimal?"
- "Can I trust it? (Open source, active repo)"
- "Will it bloat my system?"

**Adoption Blockers:**
- Looks like malware (unsigned binary)
- Requires admin permissions without explanation
- Too many features = "not for me"
- Slow performance = instant uninstall

---

## Design Implications

**UI Principles:**
1. **Speed First** - Prioritize fast scans and instant visual feedback
2. **Developer Aesthetic** - JetBrains Mono, dark mode, minimal chrome
3. **Clarity Over Cleverness** - Show folder names, sizes, file counts explicitly
4. **Safe by Default** - Trash (not delete), confirmation dialogs for destructive actions
5. **Zero Configuration** - Select folder once, works forever

**Interaction Patterns:**
- Single selection of projects directory (persisted)
- Automatic scan on launch (if directory already set)
- Manual refresh when needed
- One-click delete with confirmation
- Visual size bars for quick comparison

**Copy & Tone:**
- Technical but friendly ("node_modules", not "dependencies folder")
- Minimal text, maximum information density
- No hand-holding or tutorials
- Straightforward error messages

---

## Research Sources

- [Developer disk cleanup guide - node_modules, Docker, WSL2 caches](https://khides.com/en/blog/developer-disk-cleanup/)
- [Cleaning up 50GB+ of npm dependencies](https://itnext.io/how-i-cleaned-up-my-hard-drive-from-over-50-gbs-of-npm-dependencies-5d2d7d2ad476)
- [Developer productivity pain points research](https://jellyfish.co/library/developer-productivity/pain-points/)
- [Running out of disk space? Delete node_modules](https://dev.to/buildwebcrumbs/running-out-of-disk-space-delete-nodemodules-36l6)
- [node-modules-cleanup tool research](https://github.com/sebastianekstrom/node-modules-cleanup)
- [Free up disk space - build artifacts and dependencies](https://medium.com/@ashu.tripathi91/free-up-disk-space-clean-up-build-artifacts-and-dependencies-7a2cf5f1195d)

---

## Validation & Iteration

**Assumptions to Test:**
1. Do developers prefer desktop GUI over CLI for this task?
2. Is "move to trash" sufficient safety, or do users want preview mode?
3. Do users want to scan entire system or just one projects directory?
4. Is automatic scan on launch helpful or annoying?

**Future Research:**
- User interviews with 5-10 developers about cleanup habits
- Analytics on most frequently deleted folder types
- Usability testing on folder selection and deletion flow
