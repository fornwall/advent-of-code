# ASCII Art Pattern Library

## Quick Pattern Reference

### Horizontal Dividers

```
Simple line:
==================================================

Fancy line:
══════════════════════════════════════════════════

Dashed line:
--------------------------------------------------

Dots:
..................................................

Equals with spacing:
= = = = = = = = = = = = = = = = = = = = = = = = =
```

### Corners & Brackets

```
Simple box:
+--------+
|        |
+--------+

Rounded corners (approximated):
.--------.
|        |
'--------'

Heavy box:
╔════════╗
║        ║
╚════════╝

Light box:
┌────────┐
│        │
└────────┘
```

### Arrows & Pointers

```
Down: ▼  v  ↓  ⬇
Up:   ▲  ^  ↑  ⬆
Left: ◀  <  ←  ⬅
Right:►  >  →  ➡

Block arrows:
█████████▼
█████████◀█████████▶
█████████▲

Pointed:
    /\
   /  \
  /    \
 /      \

    \/
   /  \
  /    \
```

### Borders & Frames

```
Simple frame:
+---+---+---+
| A | B | C |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+

Fancy frame:
┏━━━┳━━━┳━━━┓
┃ A ┃ B ┃ C ┃
┣━━━╋━━━╋━━━┫
┃ 1 ┃ 2 ┃ 3 ┃
┗━━━┻━━━┻━━━┛

Shadowed box:
╭─────────╮
│ Content │
╰─────────╯ ▄
        ▄▄▄
```

### Flowchart Shapes

```
Diamond (decision):
    ◇
   ╱ ╲
  │   │
   ╲ ╱
    ◇

Or textual:
      |
    / \
   /   \
  | YES |
   \   /
    \ /
     |

Circle (terminator):
    ◯
  ╱   ╲
 │     │
  ╲   ╱
    ◯

Rectangle (process):
  ┌────────┐
  │ PROCESS│
  └────────┘
```

### Progress Indicators

```
Bar (filled):
[████████░░░░░░░░░░] 40%

Spinner:
┌─ ─┐  ┌─ ─┐  ┌─ ─┐
│ ↗ │→ │ ↙ │→ │ ↖ │
└─ ─┘  └─ ─┘  └─ ─┘

Dots:
Processing ● ○ ○
Processing ○ ● ○
Processing ○ ○ ●

Stars:
⭐ Task 1
⭐ Task 2
☆ Task 3
```

### Decorative Elements

```
Stars:
✦ ✧ ✨ ★ ☆ ⋆ ✪

Bullets:
• ◦ ▪ ▫ ▸ ▹ ◆ ◇

Separators:
· • ✦ ◆ ◉ ●●●●●

Brackets:
❰ Text ❱
❲ Text ❳
⟨ Text ⟩
⦃ Text ⦄
```

### Tree & Hierarchy

```
Simple tree:
.
├── folder1/
│   ├── file1
│   └── file2
├── folder2/
│   └── file3
└── folder3/

With links:
parent
├─→ child1
│   ├─→ grandchild1
│   └─→ grandchild2
└─→ child2
```

### Text Styles

```
Bold (doubled characters):
TTHHIISS IISS BBOOLLDD

Outline:
.---.
|BOX|
'---'

Shadow:
╔═══╗
║BOX║
╚═╧═╝
 ▀▀▀

Hollow:
▄▄▄▄▄▄▄▄▄▄
█       █
█  TEXT  █
█       █
▀▀▀▀▀▀▀▀▀▀

With color escape codes (for terminals):
\033[1;31mRED\033[0m
\033[1;32mGREEN\033[0m
```

### Common Dividers for Sections

```
Chapter style:
═══════════════════════════════════════════════════
  SECTION TITLE
═══════════════════════════════════════════════════

Header style:
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃ SECTION TITLE                                  ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

Minimal style:
─ SECTION TITLE ─────────────────────────────────
```

## Monospace Font Recommendations

When creating ASCII art, test in these monospace fonts:
- **Courier New** (Windows/Mac standard)
- **Monaco** (Mac)
- **Consolas** (Windows)
- **JetBrains Mono** (Cross-platform code editor favorite)
- **FiraCode** (Modern with ligatures)

Always verify alignment using a monospace editor or terminal.
