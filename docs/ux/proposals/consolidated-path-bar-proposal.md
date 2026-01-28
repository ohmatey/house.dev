# Consolidated Path Bar Design Proposal

**Date:** 2026-01-28
**Feature:** Unified path display + folder selection + scan action
**Status:** Proposal
**Related Documents:** [personas.md](../personas.md), [mvp-stories.md](../user-stories/mvp-stories.md)

---

## TLDR

- **Problem:** Current UI has 3 separate elements (header buttons, path display, actions) creating visual fragmentation
- **Goal:** Consolidate path display, folder selection, and scanning into a single cohesive "command bar" area
- **Constraints:** Dark mode only, monospace font (Source Code Pro), minimal aesthetic, must feel alive
- **Proposals:** 3 creative approaches balancing terminal aesthetic with playful interaction

---

## Context

### Current Implementation

The current UI structure:

```
[Header]
  house.dev         [📁 Select Folder] [↻ Scan]

[Path Display Box]
  /Users/alex/projects

[Stats Bar]
  Total Size: 12.4 GB    Folders: 23

[Folder List]
  ...
```

**Issues:**
- Visual hierarchy unclear - which element to interact with first?
- Path display is passive/boring - just gray text in a box
- Actions are distant from the path they operate on
- No sense of "aliveness" or feedback before scanning

### User Context (Persona)

**Primary Persona:** Multi-Project Developer (Alex)
- Values: Speed, minimal friction, developer aesthetic
- Workflow: Fast-paced, impatient with slow tools
- Aesthetic preferences: Dark mode, monospace fonts, terminal vibes
- Emotional job: Feel in control and confident while cleaning up disk space

**Design Principles:**
1. Speed First - Instant feedback, clear actions
2. Developer Aesthetic - Terminal inspiration, coding font
3. Clarity Over Cleverness - Obvious what to do next
4. Safe by Default - No fear of destructive actions

---

## Proposed Solutions

I'm presenting 3 creative approaches that consolidate the path bar while staying minimal and fun. Each balances terminal aesthetic with a touch of playfulness.

---

## Option 1: "Terminal Prompt Bar"

### Concept
Style the path bar like a terminal prompt (`$`), where the path acts as the current working directory and actions are "commands" you can trigger.

### Visual Layout

```
┌─────────────────────────────────────────────────────────────────┐
│ $ /Users/alex/projects                          [select] [scan] │
└─────────────────────────────────────────────────────────────────┘
```

**Detailed Design:**

```css
┌─────────────────────────────────────────────────────────────────┐
│ ┃ $ ~/projects/web-apps                       select │ scan ↻   │
│   └─ Blinking cursor when hovering                              │
└─────────────────────────────────────────────────────────────────┘

States:
1. Empty:     $ ~                                    select │ scan
              └─ "No folder selected" faded hint
2. Selected:  $ ~/projects                           select │ scan
              └─ Path truncates from left: ...cts/web-apps
3. Scanning:  $ ~/projects [████████░░] 34/50
              └─ Progress bar inline, cancel button right
```

### Interaction Patterns

**Hover States:**
- **Over path**: Cursor becomes text cursor, subtle blue glow around path
- **Over `select`**: Button background fades to `--bg-tertiary`, text color shifts to `--accent`
- **Over `scan`**: Same as select, arrow rotates slightly (playful micro-animation)

**Click States:**
- **Click path**: Opens folder selection dialog (same as clicking "select")
- **Click `select`**: Opens folder selection dialog
- **Click `scan`**: Triggers scan with smooth transition to progress state

**Empty State:**
```
$ ~                                               select │ scan
  └─ "Select a projects folder to begin" in faded text
```

**Path Selected:**
```
$ ~/projects/my-apps                              select │ scan ↻
  └─ Full path shown in tooltip on hover
  └─ Path truncates intelligently: ~/...ts/my-apps
```

**Scanning State:**
```
$ ~/projects [████████░░░░] 67/120               cancel
  └─ Animated progress bar with folder count
  └─ Spinner icon replaces $ symbol
  └─ "cancel" button replaces scan
```

### Micro-Animations

1. **Path appear**: Fade + slide in from left when folder selected
2. **Scan trigger**:
   - `scan` button briefly pulses blue
   - `$` symbol morphs into spinner
   - Progress bar grows from left to right
3. **Scan complete**:
   - Progress bar fills completely
   - Brief green flash
   - Smoothly transitions to showing results below
4. **Blinking cursor**: When hovering over path, add blinking text cursor after path (like terminal)

### Color Usage

- **`$` prompt symbol**: `--accent` (#3b82f6) - makes it feel like a command you're about to run
- **Path text**: `--text-primary` (#e5e5e5) when selected, `--text-muted` (#555555) when empty
- **Progress bar fill**: `--accent` with subtle gradient
- **Buttons**: `--text-secondary` → `--accent` on hover

### Why This Works

**Pros:**
- Instantly recognizable to developers (terminal prompt metaphor)
- Path is clickable, reducing friction (don't need to find "select folder" button)
- Scanning feels like "executing a command" - familiar mental model
- Progress bar inline feels integrated, not a separate loading state

**Cons:**
- Clicking the path to select folder may not be obvious without discovery
- `$` symbol might confuse non-Unix developers (Windows users)

**Tradeoffs:**
- Sacrifices explicit "Select Folder" label for cleaner design
- Relies on muscle memory after first use

---

## Option 2: "File Explorer Bar" (macOS Finder Inspired)

### Concept
Style the path bar like a macOS Finder window title bar - showing the folder icon, path, and action icons on the right. Clean, familiar, polished.

### Visual Layout

```
┌─────────────────────────────────────────────────────────────────┐
│ 📁 /Users/alex/projects                              📂  ↻      │
└─────────────────────────────────────────────────────────────────┘
```

**Detailed Design:**

```css
┌─────────────────────────────────────────────────────────────────┐
│ 📁 ~/projects/web-apps                               📂  ↻      │
│    └─ Breadcrumb-style path with hover states                  │
└─────────────────────────────────────────────────────────────────┘

States:
1. Empty:     📂 No folder selected                       📂  ↻
              └─ Folder icon gray, path text faded
2. Selected:  📁 ~/projects/web-apps                      📂  ↻
              └─ Folder icon colorized, path segments clickable
3. Scanning:  📁 ~/projects    ⏳ Scanning...            [cancel]
              └─ Spinner icon, scanning text, cancel button
```

### Interaction Patterns

**Hover States:**
- **Over folder icon**: Icon bounces slightly, background circle appears
- **Over path segments**: Each segment (e.g., `~`, `projects`, `web-apps`) becomes individually clickable with underline
- **Over action icons**: Background circle appears (`--bg-tertiary`), icon color shifts to `--accent`

**Click States:**
- **Click folder icon or path**: Opens folder selection dialog
- **Click path segment**: (Future enhancement) Navigate to parent folder
- **Click 📂 icon**: Opens folder selection dialog
- **Click ↻ icon**: Triggers scan

**Empty State:**
```
📂 No folder selected                                    📂  ↻
   └─ Folder icon is hollow/outline style
   └─ Gray muted text
```

**Path Selected:**
```
📁 ~/projects/my-apps                                    📂  ↻
   └─ Folder icon solid blue
   └─ Path segments separated by / with hover states
   └─ Tooltip shows full path
```

**Scanning State:**
```
📁 ~/projects    [⏳ Scanning folders... 42/150]        ✕
   └─ Animated spinner
   └─ Progress count inline
   └─ Cancel icon (✕) replaces scan icon
```

### Micro-Animations

1. **Folder icon animation on select**: Icon bounces and color shifts from gray → blue
2. **Path breadcrumbs appear**: Each segment fades in sequentially (~ → projects → web-apps)
3. **Scan icon rotation**: ↻ rotates 360° on click before morphing to progress state
4. **Progress pulse**: Entire bar has subtle breathing glow during scan
5. **Completion**: Check mark (✓) briefly appears before returning to normal state

### Color Usage

- **Folder icon**: Gray (`--text-muted`) when empty, blue (`--accent`) when selected
- **Path text**: `--text-primary` with segments slightly brighter on hover
- **Action icons**: `--text-secondary` → `--accent` on hover
- **Progress state**: Spinner in `--accent`, background shifts to `--bg-tertiary`

### Why This Works

**Pros:**
- Familiar metaphor (file explorer) - immediately understandable
- Path segments can be made interactive (navigate to parent folders)
- Icons are self-documenting (📁 = folder, 📂 = change folder, ↻ = refresh)
- Clean, polished, professional feel

**Cons:**
- Less "terminal vibes" - might feel too generic/corporate
- Icons may be too playful for some developers

**Tradeoffs:**
- More visual weight (icons take space)
- Slightly less minimal than terminal prompt approach

---

## Option 3: "Command Center Bar" (Action-First Design)

### Concept
Path bar doubles as an action center - large clickable regions for "select folder" and "scan", with the path integrated as context in between. The entire bar feels like a control panel.

### Visual Layout

```
┌─────────────────────────────────────────────────────────────────┐
│  SELECT FOLDER          ~/projects              SCAN ↻          │
│  [=============]        (hoverable)          [=========]         │
└─────────────────────────────────────────────────────────────────┘
```

**Detailed Design:**

```css
┌─────────────────────────────────────────────────────────────────┐
│ ┏━━━━━━━━━━━━┓                                ┏━━━━━━━━━┓       │
│ ┃  SELECT     ┃    ~/projects/web-apps        ┃  SCAN ↻ ┃       │
│ ┗━━━━━━━━━━━━┛                                ┗━━━━━━━━━┛       │
└─────────────────────────────────────────────────────────────────┘

States:
1. Empty:     [SELECT FOLDER]  No folder selected      [SCAN ↻]
              └─ "Select" button emphasized, "Scan" disabled/dim
2. Selected:  [CHANGE]  ~/projects/web-apps            [SCAN ↻]
              └─ Button label changes to "Change", path prominent
3. Scanning:  [CANCEL]  Scanning ~/projects... [█░░░░]  42/150
              └─ Progress bar integrated, folder count on right
```

### Interaction Patterns

**Hover States:**
- **Over SELECT/CHANGE button**: Button glows blue, path dims slightly to emphasize action
- **Over path**: Path brightens, subtle blue underline appears, cursor changes to pointer
- **Over SCAN button**: Button glows blue, ↻ icon rotates 15° clockwise (primes the action)

**Click States:**
- **Click SELECT/CHANGE**: Opens folder selection dialog
- **Click path**: Opens folder selection dialog (redundant but convenient)
- **Click SCAN**: Entire bar transforms into scanning state smoothly

**Empty State:**
```
┏━━━━━━━━━━━━┓                                ┏━━━━━━━━━┓
┃  SELECT     ┃    No folder selected          ┃ SCAN ↻  ┃ (dimmed)
┗━━━━━━━━━━━━┛                                ┗━━━━━━━━━┛
└─ Select button pulsing gently (call to action)
```

**Path Selected:**
```
┏━━━━━━━━┓                                    ┏━━━━━━━━━┓
┃ CHANGE ┃      ~/projects/web-apps            ┃  SCAN ↻ ┃
┗━━━━━━━━┛                                    ┗━━━━━━━━━┛
└─ Both buttons active, path is prominent center text
```

**Scanning State:**
```
┏━━━━━━━━┓                                    ┏━━━━━━━━━┓
┃ CANCEL ┃  Scanning... [████░░░░] 42/150     ┃         ┃
┗━━━━━━━━┛                                    ┗━━━━━━━━━┛
└─ Left button becomes cancel, right side shows progress
└─ Progress bar and count animate smoothly
```

### Micro-Animations

1. **Button label morph**: "SELECT" → "CHANGE" text morphs/fades when path selected
2. **Path appearance**: Fades in center with slight scale animation (0.95 → 1.0)
3. **Scan trigger**:
   - Entire bar briefly flashes blue
   - SCAN button morphs into empty space
   - Progress bar slides in from right
4. **Progress animation**:
   - Progress bar fills left to right
   - Folder count increments with slight bounce
5. **Completion**:
   - Green flash across entire bar
   - "Done!" text briefly appears before returning to normal

### Color Usage

- **Buttons**: Background `--bg-tertiary`, border `--border`
- **Button hover**: Background `--accent` with 0.1 opacity overlay, border `--accent`
- **Path text**: `--text-primary` (large, prominent)
- **Progress bar**: `--accent` fill, `--bg-tertiary` background
- **Empty state**: Path text `--text-muted`

### Why This Works

**Pros:**
- Zero ambiguity - large buttons make actions obvious
- Great for first-time users (discoverability)
- Path is visually prominent and clickable
- Scanning state feels integrated (entire bar is the progress indicator)
- Bold, confident design - feels powerful

**Cons:**
- Takes more vertical space (large buttons)
- Less minimal/elegant than other options
- Button labels may feel too "loud" for minimalist aesthetic

**Tradeoffs:**
- Sacrifices minimalism for clarity and discoverability
- Better for new users, potentially redundant for power users

---

## Comparison Matrix

| Criteria | Option 1: Terminal Prompt | Option 2: File Explorer | Option 3: Command Center |
|----------|--------------------------|------------------------|--------------------------|
| **Minimalism** | ⭐⭐⭐⭐⭐ Sleek, compact | ⭐⭐⭐⭐ Clean, polished | ⭐⭐⭐ Bold, larger |
| **Developer Aesthetic** | ⭐⭐⭐⭐⭐ Pure terminal vibes | ⭐⭐⭐ Familiar but generic | ⭐⭐⭐⭐ Confident, modern |
| **Discoverability** | ⭐⭐⭐ Requires learning | ⭐⭐⭐⭐ Icons help | ⭐⭐⭐⭐⭐ Obvious buttons |
| **Fun Factor** | ⭐⭐⭐⭐ Blinking cursor, command metaphor | ⭐⭐⭐⭐ Breadcrumbs, bouncing icon | ⭐⭐⭐⭐⭐ Button morphing, bar flash |
| **Flexibility** | ⭐⭐⭐ Compact, fixed height | ⭐⭐⭐⭐ Breadcrumbs expandable | ⭐⭐⭐ Large buttons limit flexibility |
| **Persona Fit (Alex)** | ⭐⭐⭐⭐⭐ Perfect for terminal fans | ⭐⭐⭐⭐ Familiar to macOS users | ⭐⭐⭐⭐ Power user vibes |

---

## Recommendation

**Recommended: Option 1 - "Terminal Prompt Bar"**

### Rationale

**Best Persona Alignment:**
- Alex (Multi-Project Developer) values speed, minimal friction, and developer aesthetic
- Terminal prompt metaphor resonates deeply with target audience
- Feels like a tool built *by* developers *for* developers

**Design Principle Fit:**
1. **Speed First** ✓ - Compact, no visual clutter, instant actions
2. **Developer Aesthetic** ✓✓✓ - Terminal prompt is peak developer UX
3. **Clarity Over Cleverness** ✓ - Once learned, extremely clear
4. **Safe by Default** ✓ - Actions are explicit (select, scan, cancel)

**Minimal Yet Playful:**
- Blinking cursor on hover adds subtle life
- Progress bar inline feels integrated
- `$` → spinner morph during scan is delightful but not over-the-top
- Maintains focus on content (folder list below)

**Tradeoffs Accepted:**
- Slightly lower discoverability for first-time users
- Mitigation: Empty state hint ("Select a projects folder to begin")
- Power users will learn instantly; new users discover quickly

### Alternative Consideration

**Option 2 (File Explorer)** is a strong second choice if:
- User testing shows terminal metaphor is confusing
- We want broader appeal beyond hardcore developer audience
- Breadcrumb navigation to parent folders is desired feature

**Option 3 (Command Center)** is best if:
- Discoverability is top priority (e.g., less technical users)
- We want bold, confident visual identity
- Willing to sacrifice minimalism for clarity

---

## Implementation Notes

### Technical Considerations

**HTML Structure (Option 1 - Terminal Prompt):**
```html
<div class="path-bar">
  <span class="prompt-symbol">$</span>
  <div class="path-content" id="path-display">
    <span class="path-text">~/projects/web-apps</span>
    <span class="path-cursor" id="cursor"></span>
  </div>
  <div class="path-actions">
    <button class="path-btn" id="select-btn">select</button>
    <span class="action-separator">│</span>
    <button class="path-btn" id="scan-btn">scan ↻</button>
  </div>
</div>
```

**CSS Variables Needed:**
```css
.path-bar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 16px;
  background: var(--bg-secondary);
  border: 1px solid var(--border);
  border-radius: var(--radius);
  font-family: 'Source Code Pro', monospace;
}

.prompt-symbol {
  color: var(--accent);
  font-weight: 700;
  font-size: 16px;
}

.path-content {
  flex: 1;
  display: flex;
  align-items: center;
  cursor: pointer;
  position: relative;
}

.path-text {
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.path-text.empty {
  color: var(--text-muted);
  font-style: italic;
}

.path-cursor {
  display: none;
  width: 2px;
  height: 16px;
  background: var(--accent);
  margin-left: 4px;
  animation: blink 1s step-end infinite;
}

.path-content:hover .path-cursor {
  display: block;
}

@keyframes blink {
  0%, 50% { opacity: 1; }
  51%, 100% { opacity: 0; }
}

.path-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.path-btn {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  font-family: inherit;
  font-size: 12px;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  transition: all 0.15s ease;
}

.path-btn:hover {
  background: var(--bg-tertiary);
  color: var(--accent);
}

.path-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
}

.action-separator {
  color: var(--border);
  user-select: none;
}
```

**JavaScript State Management:**
```javascript
const pathBarStates = {
  EMPTY: 'empty',
  SELECTED: 'selected',
  SCANNING: 'scanning'
};

function updatePathBar(state, data = {}) {
  const pathBar = document.querySelector('.path-bar');
  const promptSymbol = document.querySelector('.prompt-symbol');
  const pathText = document.querySelector('.path-text');
  const selectBtn = document.querySelector('#select-btn');
  const scanBtn = document.querySelector('#scan-btn');

  switch(state) {
    case pathBarStates.EMPTY:
      promptSymbol.textContent = '$';
      pathText.textContent = '~';
      pathText.classList.add('empty');
      scanBtn.disabled = true;
      break;

    case pathBarStates.SELECTED:
      promptSymbol.textContent = '$';
      pathText.textContent = data.path || '';
      pathText.classList.remove('empty');
      scanBtn.disabled = false;
      break;

    case pathBarStates.SCANNING:
      promptSymbol.innerHTML = '<span class="spinner-small"></span>';
      selectBtn.textContent = 'cancel';
      scanBtn.style.display = 'none';
      // Show progress bar inline
      break;
  }
}
```

### Accessibility

**Keyboard Navigation:**
- Tab order: path-content → select button → scan button
- Enter/Space on path-content opens folder selection
- Enter/Space on buttons triggers actions

**Screen Reader Support:**
```html
<div class="path-bar" role="region" aria-label="Folder selection and scanning controls">
  <span class="prompt-symbol" aria-hidden="true">$</span>
  <div class="path-content" role="button" aria-label="Selected folder: home projects web-apps. Click to change folder" tabindex="0">
    ...
  </div>
  <button id="select-btn" aria-label="Select folder">select</button>
  <button id="scan-btn" aria-label="Scan selected folder">scan ↻</button>
</div>
```

**Focus States:**
```css
.path-content:focus,
.path-btn:focus {
  outline: 2px solid var(--accent);
  outline-offset: 2px;
}
```

### Animation Performance

**Use CSS transforms (GPU-accelerated):**
- Spinner rotation: `transform: rotate()`
- Progress bar: `transform: scaleX()`
- Micro-animations: `transform: scale()` and `opacity`

**Avoid:**
- Animating `width` or `height` (causes reflow)
- Animating `color` directly (use `opacity` on overlay instead)

---

## Success Metrics

**User Experience Metrics:**
- Time to first scan (from app launch): Target < 5 seconds
- Time to understand controls (first-time user): Target < 10 seconds via observation
- Number of clicks to scan: 2 (select folder → scan) - unchanged, but clearer

**Qualitative Metrics:**
- User feedback: "Feels like a terminal" / "Love the minimal design"
- Reduced confusion about where to click
- Perceived speed improvement (visual feedback during actions)

**Technical Metrics:**
- Animation frame rate: Target 60fps for all micro-animations
- Path bar render time: Target < 16ms (one frame)

---

## Next Steps

**If Option 1 (Terminal Prompt) is approved:**

1. **Design Refinement**
   - Create high-fidelity mockups in Figma or code prototype
   - Test blinking cursor animation timing (1s blink feels right?)
   - Finalize color values and hover states

2. **Implementation**
   - Refactor `render()` function in main.ts to extract path bar
   - Update CSS in style.css with new `.path-bar` classes
   - Implement state machine for empty/selected/scanning states
   - Add micro-animations (cursor blink, spinner morph)

3. **Testing**
   - Manual testing: empty state → select folder → scan → cancel → select new folder
   - Accessibility testing: keyboard nav, screen reader support
   - Performance testing: animation frame rates, hover smoothness

4. **Iteration**
   - Gather user feedback (ideally 5-10 developers)
   - Adjust animation timings if too fast/slow
   - Consider adding tooltip for full path on hover

**If user testing is needed first:**
- Create interactive prototype (CodePen or Figma Prototype)
- Test with 5 developers (mix of terminal-first and GUI-first users)
- Validate assumptions about terminal prompt metaphor clarity

---

## Open Questions

1. **Path truncation strategy**: Truncate from left (`...ts/web-apps`) or middle (`~/proj.../web-apps`)?
   - Recommendation: From left - most important part is folder name at end

2. **Click target size**: Should entire path bar be clickable for folder selection?
   - Recommendation: Yes - entire path-content div is clickable, reduces friction

3. **Empty state CTA**: Should "select" button pulse/glow to draw attention?
   - Recommendation: Subtle pulse (2s interval) on empty state only, stops once folder selected

4. **Progress bar style**: Inline progress bar or just text count (`42/150`)?
   - Recommendation: Both - small inline bar (4px height) + text count for precision

5. **Cancel button placement**: Replace "scan" button or add as third button?
   - Recommendation: Replace "scan" button during scanning to avoid layout shift

---

## Appendix: Alternative Micro-Interactions

**If we want to go even more playful:**

**Typing Animation on Path Change:**
- When folder selected, path "types in" character by character (fast, 20ms per char)
- Adds delight but may slow perceived speed

**Scan Button "Charge Up":**
- Hold scan button for 300ms to trigger (like charging a weapon)
- Visual: Button glows brighter as you hold
- Prevents accidental clicks but adds friction

**Path Segments Clickable (Breadcrumbs):**
- Each path segment (`~`, `projects`, `web-apps`) is separately clickable
- Click parent folder to navigate up directory tree
- Adds power user feature but increases complexity

**Sound Effects:**
- Subtle "ping" on scan complete
- Click sound on button press
- May be annoying for some users - needs preference toggle

---

## References

**Design Inspiration:**
- [Hyper Terminal](https://hyper.is/) - Modern terminal aesthetics
- [Fig](https://fig.io/) - Developer tool UI patterns
- [GitHub CLI](https://cli.github.com/) - Minimal command-line UX
- [Warp Terminal](https://www.warp.dev/) - Command bar + prompt integration

**Interaction Pattern Research:**
- [macOS Finder Title Bar](https://support.apple.com/guide/mac-help/mchlp2470/mac) - Path display patterns
- [VS Code Command Palette](https://code.visualstudio.com/docs/getstarted/userinterface#_command-palette) - Action-first design
- [Terminal Prompt Customization](https://www.gnu.org/software/bash/manual/html_node/Controlling-the-Prompt.html) - Developer familiarity

**Related UX Documents:**
- [personas.md](../personas.md) - Target user: Multi-Project Developer
- [mvp-stories.md](../user-stories/mvp-stories.md) - User story: Select Projects Folder + Scan
- [CLAUDE.md](/Users/ohmatey/Documents/projects/house.dev/CLAUDE.md) - Design constraints: dark mode, minimal, JetBrains Mono
