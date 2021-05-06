use crate::input::Input;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Edge {
    /// Bitmask where '#' is set bit, '.' is unset. Only 10 first bits used.
    bitmask: u16,
    /// The tile that matches on the other end.
    matching: Option<TileId>,
}

impl Edge {
    const fn flipped(self) -> Self {
        Self {
            // Only the first 10 bits of the edge bitmask is used:
            bitmask: self.bitmask.reverse_bits() >> 6,
            matching: self.matching,
        }
    }
}

type TileId = u16;

#[derive(Copy, Clone, Debug)]
struct Tile {
    id: TileId,
    /// Indexed by 0,1,3,4 = Top,Right,Bottom,Left.
    edges: [Edge; 4],
    /// Indexed by row. Lowest bit to the right.
    /// Example: The row "#..#...." is stored as 0b10010000.
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
            }; 4];
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
                        for (i, &b) in bytes.iter().enumerate().take(10) {
                            if b == b'#' {
                                this_edges[0].bitmask |= 1 << (9 - i);
                            }
                        }
                    } else if line_idx == 10 {
                        // Bottom edge:
                        for (i, &b) in bytes.iter().enumerate().take(10) {
                            if b == b'#' {
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
            tiles.push(Self {
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
                edge_to_tile_idx[edge.flipped().bitmask as usize].push(tile.id);
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

    fn is_corner(&self) -> bool {
        self.edges
            .iter()
            .filter(|edge| edge.matching.is_none())
            .count()
            == 2
    }

    fn transform_to_match(
        &self,
        x: u8,
        y: u8,
        composed_image: &HashMap<(u8, u8), Self>,
        composed_image_width: u8,
    ) -> Result<Self, String> {
        let mut current = *self;

        for _flip in 0..=1 {
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
                        return Ok(current);
                    }
                }
                current = current.rotate_clockwise();
            }
            current = current.flip_horizontal();
        }
        Err("transform_to_match not found".to_string())
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

/// Key properties from the problem description:
///
/// - Each tile is a 8x8 grid.
/// - Each tile edge is 10 bits (so max 1024 distinct values).
/// - The composed image is square.
/// - The outermost edges tile edges won't line up with any other tiles.
pub fn solve(input: &mut Input) -> Result<u64, String> {
    let tiles = Tile::parse(input.text)?;

    if input.is_part_one() {
        return Ok(tiles
            .iter()
            .filter_map(|tile| {
                if tile.is_corner() {
                    Some(u64::from(tile.id))
                } else {
                    None
                }
            })
            .product());
    }

    let composed_image_tile_width = (tiles.len() as f64).sqrt() as u8;
    let composed_image_pixel_width = composed_image_tile_width * 8;

    let a_corner = *tiles
        .iter()
        .find(|tile| tile.is_corner())
        .ok_or_else(|| "No corner found".to_string())?;

    let starting_coordinates = match (
        a_corner.edges[0].matching.is_none(),
        a_corner.edges[1].matching.is_none(),
        a_corner.edges[2].matching.is_none(),
        a_corner.edges[3].matching.is_none(),
    ) {
        (true, false, false, true) => (0, 0),
        (true, true, false, false) => (composed_image_tile_width - 1, 0),
        (false, false, true, true) => (0, composed_image_tile_width - 1),
        (false, true, true, false) => {
            (composed_image_tile_width - 1, composed_image_tile_width - 1)
        }
        _ => {
            return Err("Invalid input - tile with two matches is no".to_string());
        }
    };

    let mut composed_image: HashMap<(u8, u8), Tile> = HashMap::new();
    composed_image.insert(starting_coordinates, a_corner);

    let mut stack = vec![(starting_coordinates.0, starting_coordinates.1, a_corner)];

    while let Some((x, y, popped_tile)) = stack.pop() {
        for (edge_idx, &edge) in popped_tile.edges.iter().enumerate() {
            if let Some(matched_tile) = edge.matching {
                let tile_with_matching_edge = tiles
                    .iter()
                    .find(|t| t.id == matched_tile)
                    .ok_or_else(|| "Internal error".to_string())?;

                let (new_x, new_y) = match edge_idx {
                    0 if y > 0 => (x, y - 1),
                    1 => (x + 1, y),
                    2 => (x, y + 1),
                    3 if x > 0 => (x - 1, y),
                    _ => {
                        continue;
                    }
                };
                if new_x >= composed_image_tile_width
                    || new_y >= composed_image_tile_width
                    || composed_image.contains_key(&(new_x, new_y))
                {
                    continue;
                }

                let transformed_tile = tile_with_matching_edge.transform_to_match(
                    new_x,
                    new_y,
                    &composed_image,
                    composed_image_tile_width,
                )?;

                composed_image.insert((new_x, new_y), transformed_tile);

                stack.push((new_x, new_y, transformed_tile));
            }
        }
    }

    let is_hash_at = |direction: u8, sideway: u8, offset: u8| {
        let (pixel_x, pixel_y) = match direction {
            0 => (sideway, offset),
            1 => (offset, sideway),
            2 => (sideway, composed_image_pixel_width - 1 - offset),
            _ => (composed_image_pixel_width - 1 - offset, sideway),
        };

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

    // Search for the main body center streak "#    ##    ##    ###"
    // of length 20, and then look at the sides for the full shape:
    //
    // "                  # "
    // "#    ##    ##    ###"
    // " #  #  #  #  #  #   "
    let monster_body_len = 20;
    for &direction in &[0_u8, 1, 2, 3] {
        for &flip in &[1_i8, -1] {
            let mut monster_count = 0;
            for offset in 0..(composed_image_pixel_width - monster_body_len + 1) {
                for sideway in 1..(composed_image_pixel_width - 1) {
                    if is_hash_at(direction, sideway, offset)
                        && is_hash_at(direction, sideway, offset + 5)
                        && is_hash_at(direction, sideway, offset + 6)
                        && is_hash_at(direction, sideway, offset + 11)
                        && is_hash_at(direction, sideway, offset + 12)
                        && is_hash_at(direction, sideway, offset + 17)
                        && is_hash_at(direction, sideway, offset + 18)
                        && is_hash_at(direction, sideway, offset + 19)
                        && is_hash_at(direction, (sideway as i8 - flip) as u8, offset + 18)
                        && is_hash_at(direction, (sideway as i8 + flip) as u8, offset + 1)
                        && is_hash_at(direction, (sideway as i8 + flip) as u8, offset + 4)
                        && is_hash_at(direction, (sideway as i8 + flip) as u8, offset + 7)
                        && is_hash_at(direction, (sideway as i8 + flip) as u8, offset + 10)
                        && is_hash_at(direction, (sideway as i8 + flip) as u8, offset + 13)
                        && is_hash_at(direction, (sideway as i8 + flip) as u8, offset + 16)
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

#[cfg(test)]
const fn edge(bitmask: u16) -> Edge {
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
    assert_eq!(
        rotated_tile
            .edges
            .iter()
            .map(|e| e.bitmask)
            .collect::<Vec<_>>(),
        [4, 1, 2, 3]
    );
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
        horizontally_flipped
            .edges
            .iter()
            .map(|e| e.bitmask)
            .collect::<Vec<_>>(),
        [0b10_0000_0000, 0b100, 0b11_0000_0000, 0b10]
    );
    assert_eq!(
        horizontally_flipped.body,
        [0b0000_0101, 0b0000_1010, 0, 0, 0, 0, 0, 0]
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
