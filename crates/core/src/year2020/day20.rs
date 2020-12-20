use crate::input::Input;

type EdgeBitmask = u16;
type TileId = u16;

fn flip_edge(number: EdgeBitmask) -> EdgeBitmask {
    // Only the first 10 bits of the edge bitmask is used.
    number.reverse_bits() >> 6
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut tiles: Vec<(TileId, [EdgeBitmask; 4])> = Vec::new();

    for tile_str in input.text.split("\n\n") {
        let mut tile_id = 0;
        let mut this_edges = [0 as EdgeBitmask; 4]; // Top, Right, Bottom, Left
        for (line_idx, line) in tile_str.lines().enumerate() {
            if line_idx == 0 {
                if !(line.len() == 10 && line.starts_with("Tile ") && line.ends_with(':')) {
                    return Err("Invalid tile header".to_string());
                }
                tile_id = line[5..9]
                    .parse::<u16>()
                    .map_err(|_| "Invalid tile header - cannot parse tile id")?;
            } else {
                let bytes = line.as_bytes();
                if !(bytes.len() == 10 && bytes.iter().all(|c| matches!(c, b'#' | b'.'))) {
                    return Err(
                        "Invalid tile line (not 10 in length and only '.' and '#'".to_string()
                    );
                }
                if line_idx == 1 {
                    for i in 0..10 {
                        if bytes[i] == b'#' {
                            this_edges[0] |= 1 << i;
                        }
                    }
                } else if line_idx == 10 {
                    for i in 0..10 {
                        if bytes[i] == b'#' {
                            this_edges[2] |= 1 << i;
                        }
                    }
                }
                if bytes[0] == b'#' {
                    this_edges[3] |= 1 << (line_idx - 1);
                }
                if bytes[9] == b'#' {
                    this_edges[1] |= 1 << (line_idx - 1);
                }
            }
        }
        tiles.push((tile_id, this_edges));
    }

    let mut result = 1;
    'outer: for &(tile_id, edges) in tiles.iter() {
        let mut matching_edges = 0_u64;
        for &(other_tile_id, other_edges) in tiles.iter() {
            if tile_id != other_tile_id {
                for &this_edge in edges.iter() {
                    for &other_edge in other_edges.iter() {
                        if this_edge == other_edge || this_edge == flip_edge(other_edge) {
                            matching_edges += 1;
                            if matching_edges > 2 {
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        }
        if matching_edges == 2 {
            result *= tile_id as u64;
        }
    }
    Ok(result)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";
    test_part_one!(example=> 20_899_048_083_289);
    //test_part_two!(example => 273);

    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 21599955909991);
    // test_part_two!(real_input => 0);
}
