# UX Documentation - house.dev

**Last Updated:** 2026-01-28
**Project:** house.dev - Developer disk space cleanup tool

---

## Overview

This directory contains all user experience research, design documentation, personas, and user stories for house.dev.

house.dev is a minimal desktop app helping developers reclaim disk space by identifying and safely deleting large folders (node_modules, .git, target, build artifacts, etc.) in their projects directory.

---

## Directory Structure

```
/docs/ux/
├── README.md                          # This file - index and navigation
├── personas.md                        # Developer persona (living document)
├── user-stories/
│   └── mvp-stories.md                 # MVP user stories with acceptance criteria
└── proposals/
    └── consolidated-path-bar-proposal.md  # Path bar redesign (2026-01-28)
```

---

## Living Documents

These documents are continuously updated as we learn more about users:

### [personas.md](./personas.md)
**Primary Persona:** Multi-Project Developer
- Demographics, psychographics, technical context
- Pain points and current workarounds
- Jobs-to-be-Done framework
- Design implications

**Key Insights:**
- Developers manage 5-15+ active projects simultaneously
- node_modules alone can consume 50GB+ across projects
- Manual cleanup is tedious and lacks visibility
- Users value speed, safety (trash not delete), and zero configuration

---

## User Stories

### [user-stories/mvp-stories.md](./user-stories/mvp-stories.md)
**5 Core User Stories for MVP:**
1. **Select Projects Folder** - One-time setup with persistence
2. **Scan for Large Folders** - Fast recursive scan with visual results
3. **Understand Folder Impact** - Size bars, file counts, visual comparison
4. **Safely Delete Folders** - Move to trash with confirmation
5. **Persistent Workflow State** - Auto-load saved folder on launch

**All stories include:**
- User need and outcome
- Detailed acceptance criteria (Given/When/Then)
- Priority level (P0 = MVP critical)
- JTBD alignment
- Technical implementation notes

---

## Design Proposals

### [proposals/consolidated-path-bar-proposal.md](./proposals/consolidated-path-bar-proposal.md)
**Date:** 2026-01-28
**Feature:** Unified path display + folder selection + scan action

**Problem:** Current UI has 3 separate elements (header buttons, path display, stats) creating visual fragmentation.

**Goal:** Consolidate path display, folder selection, and scanning into a single cohesive "command bar" area that feels alive and minimal.

**3 Proposed Options:**
1. **Terminal Prompt Bar** (Recommended) - `$ ~/projects [select] [scan]` - Pure developer aesthetic
2. **File Explorer Bar** - Finder-inspired with folder icon and breadcrumbs
3. **Command Center Bar** - Action-first with large clickable regions

**Recommendation:** Option 1 (Terminal Prompt) for best persona fit - minimal, fast, terminal vibes with playful micro-animations (blinking cursor, inline progress).

---

## Design Principles

Based on persona research, house.dev follows these UX principles:

1. **Speed First** - Prioritize fast scans (< 30 seconds) and instant feedback
2. **Developer Aesthetic** - JetBrains Mono, dark mode, minimal chrome
3. **Clarity Over Cleverness** - Show explicit information: folder name, size, count
4. **Safe by Default** - Move to trash (not delete), require confirmation
5. **Zero Configuration** - Select folder once, persist forever, auto-scan on launch

---

## Current State

**MVP Status:** Complete and implemented

**Implemented Features:**
- ✅ Folder selection with persistence (Tauri store)
- ✅ Fast recursive scan (< 30s for 50+ projects)
- ✅ Visual size bars and color coding
- ✅ Safe deletion (move to trash)
- ✅ Confirmation dialogs
- ✅ Auto-scan on launch
- ✅ Dark mode UI with JetBrains Mono

**Tested Folder Types:**
node_modules, .git, target, build, dist, .next, .nuxt, vendor, __pycache__, .venv, venv, .cargo, Pods, DerivedData, .gradle, obj, bin

---

## Research Sources

All persona and user story work is grounded in developer research:

**Developer Disk Space Pain Points:**
- [Developer disk cleanup guide](https://khides.com/en/blog/developer-disk-cleanup/) - node_modules, Docker, WSL2 cache management
- [Cleaning up 50GB+ of npm dependencies](https://itnext.io/how-i-cleaned-up-my-hard-drive-from-over-50-gbs-of-npm-dependencies-5d2d7d2ad476) - Real-world cleanup experience
- [Running out of disk space? Delete node_modules](https://dev.to/buildwebcrumbs/running-out-of-disk-space-delete-nodemodules-36l6) - Common developer workflow

**Developer Productivity Research:**
- [Developer productivity pain points](https://jellyfish.co/library/developer-productivity/pain-points/) - Technical debt and workflow friction
- [Free up disk space - build artifacts](https://medium.com/@ashu.tripathi91/free-up-disk-space-clean-up-build-artifacts-and-dependencies-7a2cf5f1195d) - Multi-language cleanup patterns

**Existing Tool Analysis:**
- [node-modules-cleanup](https://github.com/sebastianekstrom/node-modules-cleanup) - CLI tool for node_modules
- [node-cleaner](https://github.com/omridevk/node-cleaner) - GUI app (node_modules only)
- npkill - CLI with interactive selection

---

## Future Research Opportunities

**Validation Needed:**
1. Do developers prefer desktop GUI over CLI for this task?
   - Hypothesis: Visual feedback and size bars add value beyond CLI
2. Is "move to trash" sufficient, or do users want preview/dry-run mode?
   - Hypothesis: Trash is sufficient safety; undo is easy
3. Should we support multiple project directories vs. single root?
   - Hypothesis: Single root covers 90% of cases
4. Is auto-scan on launch helpful or intrusive?
   - Hypothesis: Helpful for "open and immediately see state" workflow

**Proposed Research Methods:**
- User interviews with 5-10 developers about cleanup habits
- Analytics on folder types deleted most frequently
- Usability testing on folder selection and deletion flow
- Beta testing with open source community (GitHub early adopters)

---

## Collaboration

**Cross-Functional Links:**
- **Product:** See `/docs/product/` for roadmap and feature prioritization (if exists)
- **Engineering:** See `/docs/engineering/` for technical architecture and implementation (if exists)
- **Testing:** See `/docs/testing/` for QA strategy and test coverage (if exists)

**Open Source Context:**
This is an open source project. UX decisions prioritize:
- Community feedback over internal stakeholder opinions
- Real developer needs over theoretical "nice to have" features
- Simplicity and focus over feature completeness

---

## Contributing

If you're contributing UX research or design work:

1. **Read existing docs first** - personas.md and mvp-stories.md are foundations
2. **Ground work in research** - include sources and evidence
3. **Keep it actionable** - focus on "what should we build" not abstract theory
4. **Maintain focus** - house.dev is intentionally minimal; avoid scope creep
5. **Update this README** - add new documents to the index

---

## Quick Reference

**Primary Persona:** Multi-Project Developer juggling 5-15+ projects
**Core Pain Point:** 20GB-100GB consumed by build artifacts, invisible until urgent
**Primary JTBD:** Reclaim disk space quickly without fear of deleting wrong things
**Success Metric:** Delete 10GB+ in under 2 minutes
**Design Philosophy:** Fast, safe, minimal - developer tool aesthetic
