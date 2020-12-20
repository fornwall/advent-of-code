use crate::input::Input;
use std::collections::HashMap;

type EdgeBitmask = u16;
type TileId = u16;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Tile {
    id: TileId,
    /// Indexed by 0,1,3,4 = Top,Right,Bottom,Left.
    edges: [EdgeBitmask; 4],
    /// Indexed by row. Lowest bit to the right.
    /// Example: "#..#...." is stored as 0b10010000.
    body: [u8; 8],
}

impl Tile {
    fn debug_print(&self) {
        for i in 0..8 {
            let formatted_string = format!("{:0>8b}", self.body[i])
                .replace('1', "#")
                .replace('0', ".");
            println!("{}", formatted_string);
        }
    }

    fn debug_edge(&self, edge_idx: u8) {
        println!(
            "  id={}    {:0>10b}",
            self.id, self.edges[edge_idx as usize]
        );
    }

    fn rotate_clockwise(&self) -> Self {
        let rotated_edges = [self.edges[3], self.edges[0], self.edges[1], self.edges[2]];
        let mut rotated_body = [0_u8; 8];
        // abcdefgh
        // ABCDEFGH
        // =>
        // ......Aa
        // ......Bb
        // [..]
        for i in 0..8 {
            for j in 0..8 {
                rotated_body[7 - i] |= if self.body[j] & (1 << i) > 0 {
                    1 << j
                } else {
                    0
                };
            }
        }
        Self {
            id: self.id,
            edges: rotated_edges,
            body: rotated_body,
        }
    }

    fn rotate_clockwise_multiple(&self, steps: u8) -> Self {
        let mut result = *self;
        for _ in 0..steps {
            result = result.rotate_clockwise();
        }
        result
    }

    fn flip_vertical(&self) -> Self {
        let mut flipped_body = self.body;
        flipped_body.reverse();
        Self {
            id: self.id,
            edges: [
                self.edges[2],
                flip_edge(self.edges[1]),
                self.edges[0],
                flip_edge(self.edges[3]),
            ],
            body: flipped_body,
        }
    }

    fn flip_horizontal(&self) -> Self {
        let mut flipped_body = self.body;
        for b in flipped_body.iter_mut() {
            *b = b.reverse_bits();
        }
        Self {
            id: self.id,
            edges: [
                flip_edge(self.edges[0]),
                self.edges[3],
                flip_edge(self.edges[2]),
                self.edges[1],
            ],
            body: flipped_body,
        }
    }
}

fn flip_edge(number: EdgeBitmask) -> EdgeBitmask {
    // Only the first 10 bits of the edge bitmask is used.
    number.reverse_bits() >> 6
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut tiles: Vec<Tile> = Vec::new();
    // Mapped from edge bitmask to list of (tile_id, edge_direction) pairs,
    // where edge_direction is 0,1,3,4 = Top,Right,Bottom,Left.
    let mut edge_to_tile_idx = vec![Vec::new(); 1024];

    for tile_str in input.text.split("\n\n") {
        let mut tile_id = 0;
        let mut this_edges = [0 as EdgeBitmask; 4]; // Top, Right, Bottom, Left
        let mut body = [0_u8; 8];
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

                if line_idx > 1 && line_idx < 10 {
                    for i in 0..8 {
                        if bytes[i + 1] == b'#' {
                            body[line_idx - 2] |= 1 << (7 - i);
                        }
                    }
                }

                if line_idx == 1 {
                    // Top edge:
                    for i in 0..10 {
                        if bytes[i] == b'#' {
                            this_edges[0] |= 1 << (9 - i);
                        }
                    }
                } else if line_idx == 10 {
                    // Bottom edge:
                    for i in 0..10 {
                        if bytes[i] == b'#' {
                            this_edges[2] |= 1 << (9 - i);
                        }
                    }
                }
                if bytes[9] == b'#' {
                    // Right edge.
                    this_edges[1] |= 1 << (10 - line_idx);
                }
                if bytes[0] == b'#' {
                    // Left edge:
                    this_edges[3] |= 1 << (10 - line_idx);
                }
            }
        }

        /*
        println!("### Tile {}", tile_id);
        println!("  Top:    {:0>10b}", this_edges[0]);
        println!("  Right:  {:0>10b}", this_edges[1]);
        println!("  Bottom: {:0>10b}", this_edges[2]);
        println!("  Left:   {:0>10b}", this_edges[3]);
        for i in 0..8 {
            println!("  Body[{}]:   {:0>8b}", i, body[i]);
        }
        println!();
         */

        edge_to_tile_idx[this_edges[0] as usize].push(tile_id);
        edge_to_tile_idx[this_edges[1] as usize].push(tile_id);
        edge_to_tile_idx[this_edges[2] as usize].push(tile_id);
        edge_to_tile_idx[this_edges[3] as usize].push(tile_id);
        edge_to_tile_idx[flip_edge(this_edges[0]) as usize].push(tile_id);
        edge_to_tile_idx[flip_edge(this_edges[1]) as usize].push(tile_id);
        edge_to_tile_idx[flip_edge(this_edges[2]) as usize].push(tile_id);
        edge_to_tile_idx[flip_edge(this_edges[3]) as usize].push(tile_id);

        tiles.push(Tile {
            id: tile_id,
            edges: this_edges,
            body,
            //matching_edges_bitmask: 0,
        });
    }

    // The composed image is square:
    let composed_image_width = (tiles.len() as f64).sqrt() as u8;
    println!("Composed image width: {}", composed_image_width);

    let mut top_left_corner = None;
    let mut corners = Vec::new();

    for &this_tile in tiles.iter() {
        let mut matching_edges_bitmask = 0_u64;
        //for other_tile in tiles.iter() {
        //if this_tile.id != other_tile.id {
        for (this_edge_idx, &this_edge) in this_tile.edges.iter().enumerate() {
            let edge_match = &edge_to_tile_idx[this_edge as usize];
            //let flipped_edge_match = &edge_to_tile_idx[flip_edge(this_edge) as usize];
            let normal_match =
                edge_match.len() > 1 || (edge_match.len() == 1 && edge_match[0] != this_tile.id);
            //let flipped_match = flipped_edge_match.len() > 1
            //|| (flipped_edge_match.len() == 1 && flipped_edge_match[0] != this_tile.id);
            if normal_match {
                //|| flipped_match {
                //for &other_edge in other_tile.edges.iter() {
                //if this_edge == other_edge || this_edge == flip_edge(other_edge) {
                matching_edges_bitmask |= 1 << this_edge_idx;
            }
            //}
        }
        //}
        //}
        if matching_edges_bitmask.count_ones() == 2 {
            corners.push(this_tile);
            if matching_edges_bitmask == 0b0110 {
                top_left_corner = Some(this_tile);
            }
        }
    }

    if input.is_part_one() {
        return Ok(corners.iter().map(|tile| tile.id as u64).product());
    }

    // From (x,y) to tile at position.
    let mut composed_image: HashMap<(u8, u8), Tile> = HashMap::new();

    let top_left_corner = top_left_corner.unwrap();
    for &edge in top_left_corner.edges.iter() {
        edge_to_tile_idx[edge as usize].retain(|&e| e != top_left_corner.id);
    }
    composed_image.insert((0, 0), top_left_corner);

    let mut remaining_tiles = tiles
        .iter()
        .filter(|tile| tile.id != top_left_corner.id)
        .map(|tile| (tile.id, *tile))
        .collect::<HashMap<TileId, Tile>>();

    println!("Others: {}", remaining_tiles.len());
    println!("Initial top left corner (id={}):", top_left_corner.id);
    top_left_corner.debug_print();

    let mut stack = Vec::new();
    stack.push((0, 0, top_left_corner));
    while let Some((x, y, popped_tile)) = stack.pop() {
        // Remove popped tile from edge_to_tile_idx
        for (edge_idx, &edge) in popped_tile.edges.iter().enumerate() {
            let tiles_with_matching_edge = &edge_to_tile_idx[edge as usize];
            if tiles_with_matching_edge.len() == 1 {
                let tile_with_matching_edge = tiles
                    .iter()
                    .find(|t| t.id == tiles_with_matching_edge[0])
                    .unwrap();
                let (new_x, new_y) = match edge_idx {
                    0 if y > 0 => (x, y - 1),
                    1 => (x + 1, y),
                    2 => (x, y + 1),
                    3 if x > 0 => (x - 1, y),
                    _ => {
                        continue;
                    }
                };
                if composed_image.contains_key(&(new_x, new_y)) {
                    continue;
                }
                println!(
                    "Found matching tile {} to place at x={}, y={}",
                    tile_with_matching_edge.id, new_x, new_y
                );
                let which_edge_idx = tile_with_matching_edge
                    .edges
                    .iter()
                    .enumerate()
                    .find_map(|(idx, &e)| if e == edge { Some(idx) } else { None })
                    .unwrap();
                println!(
                    "Which edge idx: {}, while coming from {}",
                    which_edge_idx, edge_idx
                );

                for &edge in tile_with_matching_edge.edges.iter() {
                    edge_to_tile_idx[edge as usize].retain(|&e| e != tile_with_matching_edge.id);
                }
                //println!("edge to tile id: {:?}", edge_to_tile_idx);
                composed_image.insert((new_x, new_y), *tile_with_matching_edge);

                stack.push((new_x, new_y, *tile_with_matching_edge));
            }
        }
    }

    println!("Placed tiles: {}", composed_image.len());
    Ok(0)
}
fn try_find(
    x: u8,
    y: u8,
    composed_image: &mut HashMap<(u8, u8), Tile>,
    remaining_tiles: &mut HashMap<TileId, Tile>,
) -> bool {
    println!("About to search for (x,y)=({},{})", x, y);
    let desired_left_edge = if x == 0 {
        None
    } else {
        println!("Desired left edge:");
        composed_image.get(&(x - 1, y)).unwrap().debug_edge(1);
        Some(composed_image.get(&(x - 1, y)).unwrap().edges[1])
    };
    let desired_top_edge = if y == 0 {
        None
    } else {
        println!("Desired top edge:");
        composed_image.get(&(x, y - 1)).unwrap().debug_edge(2);
        Some(composed_image.get(&(x, y - 1)).unwrap().edges[2])
    };

    if let Some(found_tile) = find_tile(desired_left_edge, desired_top_edge, &remaining_tiles) {
        println!("Found (x,y)=({},{}) tile: {}", x, y, found_tile.id);
        composed_image.insert((x, y), found_tile);
        remaining_tiles.remove(&found_tile.id);
        true
    } else {
        println!("Unable to find at (x,y) = ({}, {})", x, y);
        false
    }
}

fn find_tile(
    left_edge: Option<EdgeBitmask>,
    top_edge: Option<EdgeBitmask>,
    remaining_tiles: &HashMap<TileId, Tile>,
) -> Option<Tile> {
    for remaining_tile in remaining_tiles.values() {
        let mut trying_tile = *remaining_tile;
        for flip in 0..=2 {
            for _rotate in 0..=4 {
                let mut possible = true;
                if let Some(top_edge) = top_edge {
                    if top_edge != trying_tile.edges[0] {
                        possible = false;
                    }
                }
                if let Some(left_edge) = left_edge {
                    if left_edge != trying_tile.edges[3] {
                        possible = false;
                    }
                }

                if possible {
                    return Some(trying_tile);
                }

                trying_tile = trying_tile.rotate_clockwise();
            }
            if flip == 0 {
                trying_tile = trying_tile.flip_horizontal();
            } else {
                trying_tile = trying_tile.flip_vertical();
            }
        }
    }
    None
}

#[test]
pub fn test_rotate() {
    let tile = Tile {
        id: 0,
        edges: [1, 2, 3, 4],
        body: [0b10100000, 0b01010000, 0, 0, 0, 0, 0, 0],
    };

    let rotated_tile = tile.rotate_clockwise();
    assert_eq!(rotated_tile.id, tile.id);
    assert_eq!(rotated_tile.edges, [4, 1, 2, 3]);
    // #.#.....
    // .#.#....
    // [6 empty rows]
    //
    // =>
    //
    // .......#
    // ......#.
    // .......#
    // ......#.
    // [4 empty rows]
    assert_eq!(rotated_tile.body, [0b1, 0b10, 0b1, 0b10, 0, 0, 0, 0]);

    assert_eq!(
        rotated_tile.rotate_clockwise(),
        tile.rotate_clockwise_multiple(2)
    );

    assert_eq!(tile, tile.rotate_clockwise_multiple(0));
    assert_eq!(tile, tile.rotate_clockwise_multiple(4));
}

#[test]
pub fn test_flip() {
    let tile = Tile {
        id: 17,
        edges: [0b1, 0b10, 0b11, 0b100],
        body: [0b10100000, 0b01010000, 0, 0, 0, 0, 0, 0],
    };

    let horizontally_flipped = tile.flip_horizontal();
    assert_eq!(17, horizontally_flipped.id);
    assert_eq!(
        horizontally_flipped.edges,
        [0b1000000000, 0b100, 0b1100000000, 0b10]
    );
    assert_eq!(
        horizontally_flipped.body,
        [0b00000101, 0b00001010, 0, 0, 0, 0, 0, 0]
    );

    let vertically_flipped = tile.flip_vertical();
    assert_eq!(17, vertically_flipped.id);
    assert_eq!(
        vertically_flipped.edges,
        [0b11, 0b0100000000, 0b1, 0b0010000000]
    );
    assert_eq!(
        vertically_flipped.body,
        [0, 0, 0, 0, 0, 0, 0b01010000, 0b10100000],
    );
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
    test_part_one!(real_input => 21_599_955_909_991);
    // test_part_two!(real_input => 0);
}
