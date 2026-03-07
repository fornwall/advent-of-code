---
name: ascii-art
description: "Create ASCII art including text banners, diagrams, flowcharts, and console designs. Use when: generating decorative ASCII text, building terminal-friendly diagrams, creating box drawings, or styling console output."
argument-hint: "Describe the ASCII art you want to create"
---

# ASCII Art Creation Skill

Generate beautiful ASCII art for documentation, terminal applications, and console output.

## When to Use

- Creating decorative text banners and headers
- Designing ASCII flowcharts, diagrams, or tree structures
- Building box-drawn layouts for terminal UIs
- Adding visual elements to README files or documentation
- Styling console output with ASCII borders and decorations

## Procedure: Text Banners

1. Decide on the text and desired style (simple, bold, decorative, shadow)
2. Use figlet or toilet tools for quick generation:
   ```bash
   figlet "Your Text Here"
   toilet -f font "Your Text Here"
   ```
3. Or manually create ASCII art using these patterns:
   ```
   ######  #######  #####
   #     # #       #     #
   #     # #       #
   ######  #####   #
   #   #   #       #
   #    #  #       #     #
   #     # #        #####
   ```
4. Adjust character spacing and alignment for proportions
5. Test rendering in target environment (terminal, markdown, code)

## Procedure: Box Drawings & Frames

1. Choose box style:
   - **Simple**: `+`, `-`, `|` characters
   - **Fancy**: Unicode box-drawing characters (─, │, ┌, ┐, └, ┘, etc.)
2. Build basic frame:
   ```
   ┌─────────────────┐
   │ Content Here    │
   └─────────────────┘
   ```
3. Add padding and internal structure as needed
4. For nested boxes, adjust corner and line characters appropriately

## Procedure: Diagrams & Flowcharts

1. Plan the layout (horizontal, vertical, or tree-based)
2. Use ASCII connectors:
   ```
   ▼ for down arrow
   ► for right arrow
   ◀ for left arrow
   ▲ for up arrow
   ```
   Or simpler: `v`, `>`, `<`, `^`
3. Build connections with lines:
   ```
       ┌──────┐
       │ Start│
       └──┬───┘
          │
       ┌──▼───┐
       │Action│
       └──┬───┘
          │
       ┌──▼──┐
       │ End │
       └─────┘
   ```
4. Ensure alignment for readability

## Procedure: Tree Structures

1. Use tree-drawing characters: `├─`, `└─`, `│`
2. Basic structure:
   ```
   root
   ├─ child1
   │  ├─ grandchild1
   │  └─ grandchild2
   └─ child2
   ```
3. Align vertical lines consistently
4. Use `└─` for last items in a branch

## Tools & Resources

### Included Scripts
- [ASCII Art Generator](./scripts/generate-ascii.sh): Bash utility for quick ASCII art generation (banner, box, divider, fancy-box, frame, stars)
- [Pattern Library](./PATTERN-LIBRARY.md): Reference guide with reusable ASCII patterns

### Command-line Tools
- `figlet`: Large ASCII text with various fonts
- `toilet`: Similar to figlet with extra style options
- `asciify`: Convert images to ASCII
- `asciiart`: Online ASCII art generator

### Online Generators
- [ASCII Art Archive](http://www.ascii-art.eu/)
- [Figlet Online](http://www.figlet.org/)
- [ASCII Flow](https://asciiflow.com/)
- [Monodraw](https://monodraw.hellix.io/) (diagram tool with ASCII export)

### Unicode Box-Drawing Characters
| Character | Name |
|-----------|------|
| ─ | Horizontal |
| │ | Vertical |
| ┌ | Top-left corner |
| ┐ | Top-right corner |
| └ | Bottom-left corner |
| ┘ | Bottom-right corner |
| ├ | Left T-junction |
| ┤ | Right T-junction |
| ┬ | Top T-junction |
| ┴ | Bottom T-junction |
| ┼ | Cross |

## Tips for Clean ASCII Art

- **Consistency**: Use the same characters for similar elements
- **Spacing**: Test in fixed-width fonts (monospace)
- **Alignment**: Use tabs or spaces consistently for monospace rendering
- **Readability**: Prioritize clarity over complexity, especially in diagrams
- **Proportions**: Keep text banners proportional for terminal widths

## Example: Complete Styled Box

```
╔════════════════════════════════════╗
║                                    ║
║  🎯  ADVENT OF CODE 2025  🎯       ║
║                                    ║
╚════════════════════════════════════╝
```
