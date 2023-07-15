@binding(0) @group(0) var<storage, read> current: array<u32>;
@binding(1) @group(0) var<storage, read_write> next: array<u32>;
@binding(2) @group(0) var<uniform> size: u32;
@binding(3) @group(0) var<storage, read_write> moved: atomic<u32>;
@binding(4) @group(0) var<uniform> rule_bits: u32;

const ELF_NONE = 0u;
const ELF_STILL = 1u;
const ELF_NORTH = 2u;
const ELF_EAST = 3u;
const ELF_SOUTH = 4u;
const ELF_WEST = 5u;

fn modulo_euclidean(a: i32, b: i32) -> i32 {
    let m = a % b;
    return m + select(0, b, m < 0);
}

fn getIndex(x: i32, y: i32) -> u32 {
    let w = i32(size);
    return u32(modulo_euclidean(y, w) * w + modulo_euclidean(x, w));
}

fn getDirection(rule_bits: u32) -> u32 {
    switch rule_bits {
        case 7u: { return ELF_NORTH; }
        case 224u: { return ELF_SOUTH; }
        case 41u: { return ELF_WEST; }
        default: { return ELF_EAST; }
    }
}


fn getCell(x: i32, y: i32) -> u32 {
    return current[getIndex(x, y)];
}

fn elfAt(x: i32, y: i32, val: u32) -> u32 {
    return select(0u, val, getCell(x, y) > 0u);
}

// "If there is no elf in the n, ne, or nw adjacent positions, the elf proposes moving north one step"
// (0b0000_0111, (0, -1)),
// "if there is no elf in the s, se, or sw adjacent positions, the elf proposes moving south one step"
// (0b1110_0000, (0, 1)),
// "if there is no elf in the w, nw, or sw adjacent positions, the elf proposes moving west one step"
// (0b0010_1001, (-1, 0)),
// "if there is no elf in the e, ne, or se adjacent positions, the elf proposes moving east one step"
// (0b1001_0100, (1, 0)),
@compute @workgroup_size(8, 8)
fn propose_movement(@builtin(global_invocation_id) grid: vec3<u32>) {
    let x = i32(grid.x);
    let y = i32(grid.y);
    let bitmask = elfAt(x - 1, y - 1, 1u) + elfAt(x, y - 1, 2u) + elfAt(x + 1, y - 1, 4u) + elfAt(x - 1, y, 8u) + elfAt(x + 1, y, 16u) + elfAt(x - 1, y + 1, 32u) + elfAt(x, y + 1, 64u) + elfAt(x + 1, y + 1, 128u);
    var proposal: u32;
    if getCell(x, y) == 0u {
        proposal = ELF_NONE;
    } else if bitmask == 0u {
        proposal = ELF_STILL;
    } else if (bitmask & (rule_bits & 255u)) == 0u {
        proposal = getDirection(rule_bits & 255u);
    } else if (bitmask & ((rule_bits >> 8u) & 255u)) == 0u {
        proposal = getDirection((rule_bits >> 8u) & 255u);
    } else if (bitmask & ((rule_bits >> 16u) & 255u)) == 0u {
        proposal = getDirection((rule_bits >> 16u) & 255u);
    } else if (bitmask & ((rule_bits >> 24u) & 255u)) == 0u {
        proposal = getDirection((rule_bits >> 24u) & 255u);
    } else {
        proposal = ELF_STILL;
    }
    next[getIndex(x, y)] = proposal;
} 

@compute @workgroup_size(8, 8)
fn apply_movement(@builtin(global_invocation_id) grid: vec3<u32>) {
    let x = i32(grid.x);
    let y = i32(grid.y);
    let this_cell_elf = getCell(x, y);
    var value: u32;
    switch this_cell_elf {
        case 0u: { // ELF_NONE - see if someone moved into here.
            let north = getCell(x, y - 1);
            let east = getCell(x + 1, y);
            let south = getCell(x, y + 1);
            let west = getCell(x - 1, y);
            value = u32((north == ELF_SOUTH && south != ELF_NORTH) || (east == ELF_WEST && west != ELF_EAST) || (south == ELF_NORTH && north != ELF_SOUTH) || (west == ELF_EAST && east != ELF_WEST));
            if value != 0u {
                atomicStore(&moved, 1u);
            }
        }
        case 1u: { // ELF_STILL
            value = 1u;
        }
        default: { // Proposed moving out - ELF_{NORTH,EAST,SOUTH,WEST}. Remain here only on collision:
            value = u32((this_cell_elf == ELF_NORTH && getCell(x, y - 2) == ELF_SOUTH) || (this_cell_elf == ELF_EAST && getCell(x + 2, y) == ELF_WEST) || (this_cell_elf == ELF_SOUTH && getCell(x, y + 2) == ELF_NORTH) || (this_cell_elf == ELF_WEST && getCell(x - 2, y) == ELF_EAST));
        }
    }
    next[getIndex(x, y)] = value;
} 
