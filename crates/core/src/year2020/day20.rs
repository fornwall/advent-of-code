use crate::input::Input;
use std::collections::HashMap;

type EdgeBitmask = u16;
type TileId = u16;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Edge {
    bitmask: EdgeBitmask,
    matching: Option<TileId>,
}

impl Edge {
    fn flipped(self) -> Self {
        Self {
            bitmask: flip_edge(self.bitmask),
            matching: self.matching,
        }
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Tile {
    id: TileId,
    /// Indexed by 0,1,3,4 = Top,Right,Bottom,Left.
    edges: [Edge; 4],
    /// Indexed by row. Lowest bit to the right.
    /// Example: "#..#...." is stored as 0b10010000.
    body: [u8; 8],
}

impl Tile {
    fn parse(input: &str) -> Result<Vec<Self>, String> {
        let mut tiles = Vec::new();
        for tile_str in input.split("\n\n") {
            let mut tile_id = 0;
            let mut this_edges = [Edge {
                bitmask: 0,
                matching: None,
            }; 4]; // Top, Right, Bottom, Left
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
                                this_edges[0].bitmask |= 1 << (9 - i);
                            }
                        }
                    } else if line_idx == 10 {
                        // Bottom edge:
                        for i in 0..10 {
                            if bytes[i] == b'#' {
                                this_edges[2].bitmask |= 1 << (9 - i);
                            }
                        }
                    }

                    if bytes[0] == b'#' {
                        // Left edge:
                        this_edges[3].bitmask |= 1 << (10 - line_idx);
                    }

                    if bytes[9] == b'#' {
                        // Right edge.
                        this_edges[1].bitmask |= 1 << (10 - line_idx);
                    }
                }
            }
            tiles.push(Tile {
                id: tile_id,
                edges: this_edges,
                body,
            });
        }

        // Mapped from edge bitmask to list of tile_id:s.
        let mut edge_to_tile_idx = vec![Vec::new(); 1024];
        for tile in tiles.iter() {
            for &edge in tile.edges.iter() {
                edge_to_tile_idx[edge.bitmask as usize].push(tile.id);
                edge_to_tile_idx[flip_edge(edge.bitmask) as usize].push(tile.id);
            }
        }
        for tile in tiles.iter_mut() {
            let tile_id = tile.id;
            for edge in tile.edges.iter_mut() {
                if let Some(&other_tile_id) = edge_to_tile_idx[edge.bitmask as usize]
                    .iter()
                    .find(|&&other_tile_id| other_tile_id != tile_id)
                {
                    edge.matching = Some(other_tile_id);
                }
            }
        }

        Ok(tiles)
    }

    fn transform_to_match(
        &self,
        x: u8,
        y: u8,
        composed_image: &HashMap<(u8, u8), Self>,
        composed_image_width: u8,
    ) -> Self {
        let mut current = *self;
        for flip in 0..=2 {
            for _rotation in 0..=3 {
                if current.edges[0].matching.is_none() == (y == 0)
                    && current.edges[1].matching.is_none() == (x + 1 == composed_image_width)
                    && current.edges[2].matching.is_none() == (y + 1 == composed_image_width)
                    && current.edges[3].matching.is_none() == (x == 0)
                {
                    let mut possible = true;
                    if x != 0 {
                        if let Some(tile_to_left) = composed_image.get(&(x - 1, y)) {
                            if Some(tile_to_left.id) != current.edges[3].matching {
                                possible = false;
                            }
                        }
                    }
                    if y != 0 {
                        if let Some(tile_above) = composed_image.get(&(x, y - 1)) {
                            if Some(tile_above.id) != current.edges[0].matching {
                                possible = false;
                            }
                        }
                    }
                    if let Some(tile_to_right) = composed_image.get(&(x + 1, y)) {
                        if Some(tile_to_right.id) != current.edges[1].matching {
                            possible = false;
                        }
                    }
                    if let Some(tile_below) = composed_image.get(&(x, y + 1)) {
                        if Some(tile_below.id) != current.edges[2].matching {
                            possible = false;
                        }
                    }
                    if possible {
                        return current;
                    }
                }
                current = current.rotate_clockwise();
            }
            if flip == 1 {
                current = current.flip_vertical();
            } else {
                current = current.flip_horizontal();
            }
        }
        panic!("transform_to_match not found");
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

    fn flip_vertical(&self) -> Self {
        let mut flipped_body = self.body;
        flipped_body.reverse();
        Self {
            id: self.id,
            edges: [
                self.edges[2],
                self.edges[1].flipped(),
                self.edges[0],
                self.edges[3].flipped(),
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
                self.edges[0].flipped(),
                self.edges[3],
                self.edges[2].flipped(),
                self.edges[1],
            ],
            body: flipped_body,
        }
    }
}

const fn flip_edge(number: EdgeBitmask) -> EdgeBitmask {
    // Only the first 10 bits of the edge bitmask is used.
    number.reverse_bits() >> 6
}

/// Key properties from the problem description:
///
/// - Each tile is a 8x8 grid.
/// - Each tile edge is 10 bits (so max 1024 distinct values).
/// - The composed image is square.
/// - The outermost edges tile edges won't line up with any other tiles.
pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut tiles = Tile::parse(input.text)?;
    let mut bottom_left_corner = None;

    let composed_square_width = (tiles.len() as f64).sqrt() as u8;
    for &mut tile in tiles.iter_mut() {
        let mut matching_edges_bitmask = 0_u64;
        for (edge_idx, &edge) in tile.edges.iter().enumerate() {
            if edge.matching.is_some() {
                matching_edges_bitmask |= 1 << edge_idx;
            }
        }

        if matching_edges_bitmask == 0b0110 {
            bottom_left_corner = Some(tile);
        }
    }

    if input.is_part_one() {
        return Ok(tiles
            .iter()
            .filter_map(|tile| {
                if tile
                    .edges
                    .iter()
                    .filter(|edge| edge.matching.is_none())
                    .count()
                    == 2
                {
                    Some(tile.id as u64)
                } else {
                    None
                }
            })
            .product());
    }

    // From (x,y) to tile at position.
    let mut composed_image: HashMap<(u8, u8), Tile> = HashMap::new();

    let top_left_corner = bottom_left_corner.unwrap();
    composed_image.insert((0, 0), top_left_corner);

    let mut stack = Vec::new();
    stack.push((0, 0, top_left_corner));
    while let Some((x, y, popped_tile)) = stack.pop() {
        for (edge_idx, &edge) in popped_tile.edges.iter().enumerate() {
            if let Some(matched_tile) = edge.matching {
                let tile_with_matching_edge = tiles.iter().find(|t| t.id == matched_tile).unwrap();
                let (new_x, new_y) = match edge_idx {
                    0 if y > 0 => (x, y - 1),
                    1 => (x + 1, y),
                    2 => (x, y + 1),
                    3 if x > 0 => (x - 1, y),
                    _ => {
                        continue;
                    }
                };
                if new_x >= composed_square_width
                    || new_y >= composed_square_width
                    || composed_image.contains_key(&(new_x, new_y))
                {
                    continue;
                }

                let transformed_tile = tile_with_matching_edge.transform_to_match(
                    new_x,
                    new_y,
                    &composed_image,
                    composed_square_width,
                );

                composed_image.insert((new_x, new_y), transformed_tile);

                stack.push((new_x, new_y, transformed_tile));
            }
        }
    }

    let composed_image_width_pixels = composed_square_width * 8;

    let is_black_at = |direction: u8, pixel_x: u8, monster_direction: u8| {
        let (pixel_x, pixel_y) = match direction {
            1 => (monster_direction, pixel_x),
            3 => (composed_image_width_pixels - 1 - monster_direction, pixel_x),
            0 => (pixel_x, monster_direction),
            2 => (pixel_x, composed_image_width_pixels - 1 - monster_direction),
            _ => {
                panic!("Invalid direction");
            }
        };
        //println!("Checking {}, {}", pixel_x, pixel_y);
        let tile_x = pixel_x / 8;
        let tile_y = pixel_y / 8;
        let bit = pixel_x % 8;
        let row = pixel_y % 8;
        composed_image
            .get(&(tile_x as u8, tile_y as u8))
            .unwrap()
            .body[row as usize]
            & (1 << (7 - bit))
            != 0
    };

    // Search for the main body "#    ##    ##    ###",
    // of length 20, in the sea monster pattern:
    // "                  # "
    // "#    ##    ##    ###"
    // " #  #  #  #  #  #   "
    let monster_body_len = 20;
    for &direction in &[0_u8, 1, 2, 3] {
        for &flip in &[1_i8, -1] {
            let mut monster_count = 0;
            for x in 1..(composed_image_width_pixels - 1) {
                for y in 0..(composed_image_width_pixels - monster_body_len + 1) {
                    if is_black_at(direction, x, y)
                        && is_black_at(direction, x, y + 5)
                        && is_black_at(direction, x, y + 6)
                        && is_black_at(direction, x, y + 11)
                        && is_black_at(direction, x, y + 12)
                        && is_black_at(direction, x, y + 17)
                        && is_black_at(direction, x, y + 18)
                        && is_black_at(direction, x, y + 19)
                        && is_black_at(direction, (x as i8 - flip) as u8, y + 18)
                        && is_black_at(direction, (x as i8 + flip) as u8, y + 1)
                        && is_black_at(direction, (x as i8 + flip) as u8, y + 4)
                        && is_black_at(direction, (x as i8 + flip) as u8, y + 7)
                        && is_black_at(direction, (x as i8 + flip) as u8, y + 10)
                        && is_black_at(direction, (x as i8 + flip) as u8, y + 13)
                        && is_black_at(direction, (x as i8 + flip) as u8, y + 16)
                    {
                        monster_count += 1;
                    }
                }
            }

            if monster_count != 0 {
                return Ok(tiles
                    .iter()
                    .map(|t| {
                        t.body
                            .iter()
                            .map(|row| u64::from(row.count_ones()))
                            .sum::<u64>()
                    })
                    .sum::<u64>()
                    - monster_count * 15);
            }
        }
    }

    Err("No sea monster found".to_string())
}

fn edge(bitmask: EdgeBitmask) -> Edge {
    Edge {
        bitmask,
        matching: None,
    }
}

#[test]
pub fn test_rotate() {
    let tile = Tile {
        id: 0,
        edges: [edge(1), edge(2), edge(3), edge(4)],
        body: [0b1010_0000, 0b0101_0000, 0, 0, 0, 0, 0, 0],
    };

    let rotated_tile = tile.rotate_clockwise();
    assert_eq!(rotated_tile.id, tile.id);
    assert_eq!(rotated_tile.edges, [edge(4), edge(1), edge(2), edge(3)]);
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
}

#[test]
pub fn test_flip() {
    let tile = Tile {
        id: 17,
        edges: [edge(0b1), edge(0b10), edge(0b11), edge(0b100)],
        body: [0b1010_0000, 0b0101_0000, 0, 0, 0, 0, 0, 0],
    };

    let horizontally_flipped = tile.flip_horizontal();
    assert_eq!(17, horizontally_flipped.id);
    assert_eq!(
        horizontally_flipped.edges,
        [
            edge(0b10_0000_0000),
            edge(0b100),
            edge(0b11_0000_0000),
            edge(0b10)
        ]
    );
    assert_eq!(
        horizontally_flipped.body,
        [0b0000_0101, 0b0000_1010, 0, 0, 0, 0, 0, 0]
    );

    let vertically_flipped = tile.flip_vertical();
    assert_eq!(17, vertically_flipped.id);
    assert_eq!(
        vertically_flipped.edges,
        [
            edge(0b11),
            edge(0b01_0000_0000),
            edge(0b1),
            edge(0b00_1000_0000)
        ]
    );
    assert_eq!(
        vertically_flipped.body,
        [0, 0, 0, 0, 0, 0, 0b0101_0000, 0b1010_0000],
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
    test_part_two!(example => 273);

    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 21_599_955_909_991);
    test_part_two!(real_input => 2495);
}
