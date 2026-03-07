#!/bin/bash
# ASCII Art Generator Utility
# Generates various types of ASCII art output

set -e

# Function to print usage
usage() {
    cat << EOF
Usage: $0 [command] [options]

Commands:
  banner <text>           Generate large ASCII text banner
  box <text>              Create a simple box around text
  divide [width]          Generate a horizontal divider (default: 50)
  fancy-box <text>        Create fancy Unicode box
  frame <text>            Create ASCII frame
  stars <text>            Add decorative stars around text
  
Examples:
  $0 banner "Hello World"
  $0 box "Important Message"
  $0 divide 60
  $0 fancy-box "My Title"
  $0 frame "Content"
  $0 stars "Featured"

EOF
    exit 1
}

# Banner function
banner_func() {
    local text="$1"
    if command -v figlet &> /dev/null; then
        figlet "$text"
    else
        # Fallback if figlet not installed
        echo "=== $text ===" | sed 's/./=/g'
        echo "$text"
        echo "=== $text ===" | sed 's/./=/g'
    fi
}

# Simple box function
box_func() {
    local text="$1"
    local len=$((${#text} + 4))
    printf '+%*s+\n' "$((len - 2))" | tr ' ' '-'
    echo "| $text |"
    printf '+%*s+\n' "$((len - 2))" | tr ' ' '-'
}

# Divider function
divide_func() {
    local width=${1:-50}
    printf '%*s\n' $width | tr ' ' '═'
}

# Fancy Unicode box
fancy_box_func() {
    local text="$1"
    local len=$((${#text} + 2))
    echo "╔$(printf '═%.0s' $(seq 1 $len))╗"
    echo "║ $text ║"
    echo "╚$(printf '═%.0s' $(seq 1 $len))╝"
}

# ASCII frame
frame_func() {
    local text="$1"
    echo "┌─────────────────────┐"
    printf "│ %-18s │\n" "$text"
    echo "└─────────────────────┘"
}

# Stars decorative
stars_func() {
    local text="$1"
    echo "✦═✦ $text ✦═✦"
}

# Main script
if [[ $# -lt 1 ]]; then
    usage
fi

command="$1"
shift

case "$command" in
    banner)
        [[ $# -lt 1 ]] && echo "Error: banner requires text argument" && exit 1
        banner_func "$@"
        ;;
    box)
        [[ $# -lt 1 ]] && echo "Error: box requires text argument" && exit 1
        box_func "$@"
        ;;
    divide)
        divide_func "$@"
        ;;
    fancy-box)
        [[ $# -lt 1 ]] && echo "Error: fancy-box requires text argument" && exit 1
        fancy_box_func "$@"
        ;;
    frame)
        [[ $# -lt 1 ]] && echo "Error: frame requires text argument" && exit 1
        frame_func "$@"
        ;;
    stars)
        [[ $# -lt 1 ]] && echo "Error: stars requires text argument" && exit 1
        stars_func "$@"
        ;;
    *)
        echo "Unknown command: $command"
        usage
        ;;
esac
