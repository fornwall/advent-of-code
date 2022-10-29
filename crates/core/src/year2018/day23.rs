use crate::input::Input;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    const fn with_value(v: i32) -> Self {
        Self::new(v, v, v)
    }

    fn min(&self, p: Self) -> Self {
        Self {
            x: std::cmp::min(self.x, p.x),
            y: std::cmp::min(self.y, p.y),
            z: std::cmp::min(self.z, p.z),
        }
    }

    fn max(&self, p: Self) -> Self {
        Self {
            x: std::cmp::max(self.x, p.x),
            y: std::cmp::max(self.y, p.y),
            z: std::cmp::max(self.z, p.z),
        }
    }

    pub const fn distance_between(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl std::ops::Index<usize> for Position {
    type Output = i32;

    fn index(&self, i: usize) -> &i32 {
        #![allow(clippy::panic)]
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl std::ops::Add<(i32, i32, i32)> for Position {
    type Output = Self;

    fn add(self, other: (i32, i32, i32)) -> Self {
        Self::new(self.x + other.0, self.y + other.1, self.z + other.2)
    }
}

impl std::ops::Add<Self> for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub<Self> for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::IndexMut<usize> for Position {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        #![allow(clippy::panic)]
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
    }
}

struct Nanobot {
    pos: Position,
    radius: i32,
}

impl Nanobot {
    fn parse(input_string: &str) -> Result<Vec<Self>, String> {
        input_string
            .lines()
            .enumerate()
            .map(|(line_index, line)| {
                let line_number = line_index + 1;
                let parts: Vec<&str> = line
                    .split(|c| c == '<' || c == '>' || c == ',' || c == '=')
                    .collect();
                let error_message = || format!("Invalid input on line {}", line_number);
                if parts.len() != 8 {
                    return Err(error_message());
                }
                let error_mapper = |_| error_message();
                let x = parts[2].parse::<i32>().map_err(error_mapper)?;
                let y = parts[3].parse::<i32>().map_err(error_mapper)?;
                let z = parts[4].parse::<i32>().map_err(error_mapper)?;
                let pos = Position::new(x, y, z);
                let radius = parts[7].parse::<i32>().map_err(error_mapper)?;
                Ok(Self { pos, radius })
            })
            .collect::<Result<Vec<Self>, String>>()
    }

    const fn is_bot_within_range(&self, other: &Self) -> bool {
        self.pos.distance_between(other.pos) <= self.radius
    }
}

/// An axis-aligned bounding box (AABB).
///
/// The min and max values are both inclusive.
#[derive(Copy, Clone)]
#[allow(clippy::upper_case_acronyms)]
struct AABB {
    min: Position,
    max: Position,
}

impl AABB {
    const fn new() -> Self {
        let min = Position::with_value(std::i32::MAX);
        let max = Position::with_value(std::i32::MIN);
        Self { min, max }
    }

    const fn with_corners(min: Position, max: Position) -> Self {
        Self { min, max }
    }

    fn add_point(&mut self, pos: Position) {
        self.min = self.min.min(pos);
        self.max = self.max.max(pos);
    }

    fn add_sphere(&mut self, center: Position, radius: i32) {
        self.add_point(center - Position::with_value(radius));
        self.add_point(center + Position::with_value(radius));
    }

    fn overlaps(&self, bot: &Nanobot) -> bool {
        let dist = self.distance_from(bot.pos);
        dist <= bot.radius
    }

    fn num_overlaps(&self, bots: &[Nanobot]) -> usize {
        bots.iter().filter(|bot| self.overlaps(bot)).count()
    }

    fn distance_from(&self, point: Position) -> i32 {
        #![allow(clippy::manual_clamp)]
        let mut closest: Position = point;

        for i in 0..3 {
            if closest[i] > self.max[i] {
                closest[i] = self.max[i];
            } else if closest[i] < self.min[i] {
                closest[i] = self.min[i];
            }
        }

        // Manhattan distance
        let diff = point - closest;
        diff.x.abs() + diff.y.abs() + diff.z.abs()
    }

    const fn width(&self) -> i32 {
        self.max.x - self.min.x + 1
    }

    const fn height(&self) -> i32 {
        self.max.y - self.min.y + 1
    }

    const fn depth(&self) -> i32 {
        self.max.z - self.min.z + 1
    }

    const fn volume(&self) -> usize {
        let w = self.width() as usize;
        let h = self.height() as usize;
        let d = self.depth() as usize;

        w.saturating_mul(h).saturating_mul(d)
    }
}

struct OctreeNode {
    level: u8,
    max_possible: usize,
    bounds: AABB,
    children: Vec<Rc<RefCell<OctreeNode>>>,
}

impl OctreeNode {
    fn subdivide(&mut self, tree: &mut Octree, bots: &[Nanobot]) {
        let min = self.bounds.min;

        let w = self.bounds.width() - 1;
        let h = self.bounds.height() - 1;
        let d = self.bounds.depth() - 1;

        let hw = w / 2;
        let hh = w / 2;
        let hd = w / 2;

        let new_bounds = [
            AABB::with_corners(min + (0, 0, 0), min + (hw, hh, hd)),
            AABB::with_corners(min + (0, 0, hd + 1), min + (hw, hh, d)),
            AABB::with_corners(min + (hw + 1, 0, 0), min + (w, hh, hd)),
            AABB::with_corners(min + (hw + 1, 0, hd + 1), min + (w, hh, d)),
            AABB::with_corners(min + (0, hh + 1, 0), min + (hw, h, hd)),
            AABB::with_corners(min + (0, hh + 1, hd + 1), min + (hw, h, d)),
            AABB::with_corners(min + (hw + 1, hh + 1, 0), min + (w, h, hd)),
            AABB::with_corners(min + (hw + 1, hh + 1, hd + 1), min + (w, h, d)),
        ];

        for bounds in new_bounds.iter() {
            let new_node = Rc::new(RefCell::new(Self {
                level: self.level + 1,
                max_possible: bounds.num_overlaps(bots),
                bounds: *bounds,
                children: Vec::new(),
            }));

            self.children.push(new_node.clone());
            tree.leaves.push(new_node);
        }
    }
}

/// An octree is a tree data structure in which each internal node has exactly eight children.
/// - https://en.wikipedia.org/wiki/Octree
///
/// Three-dimensional space is partitioned by recursively subdividing it into eight octants.
struct Octree {
    _root: Rc<RefCell<OctreeNode>>,
    leaves: Vec<Rc<RefCell<OctreeNode>>>,
}

impl Octree {
    /// Create a new octree containing all the bots.
    fn new(bots: &[Nanobot]) -> Self {
        let mut bounds = AABB::new();
        for bot in bots {
            bounds.add_sphere(bot.pos, bot.radius);
        }

        let root = Rc::new(RefCell::new(OctreeNode {
            level: 0,
            max_possible: bots.len(),
            bounds,
            children: Vec::new(),
        }));

        Self {
            _root: root.clone(),
            leaves: vec![root],
        }
    }
}

// https://www.forrestthewoods.com/blog/solving-advent-of-code-in-under-a-second/
pub fn solve(input: &mut Input) -> Result<i32, String> {
    let bots = Nanobot::parse(input.text)?;

    if input.is_part_one() {
        let strongest_bot = bots
            .iter()
            .max_by(|x, y| x.radius.cmp(&y.radius))
            .ok_or("No robot specified")?;
        return Ok(bots
            .iter()
            .filter(|&bot| strongest_bot.is_bot_within_range(bot))
            .count() as i32);
    }

    let mut octree = Octree::new(&bots);
    let origin = Position::new(0, 0, 0);
    let mut best_leaf: Option<Rc<RefCell<OctreeNode>>> = None;

    while let Some(leaf) = octree.leaves.pop() {
        let mut inner_leaf = leaf.borrow_mut();
        assert!(inner_leaf.children.is_empty());

        match best_leaf.clone() {
            Some(old_best) => {
                let old_best = old_best.borrow();
                if inner_leaf.max_possible < old_best.max_possible {
                    continue;
                }

                if inner_leaf.bounds.min == inner_leaf.bounds.max {
                    if inner_leaf.max_possible > old_best.max_possible
                        || inner_leaf.bounds.distance_from(origin)
                            < old_best.bounds.distance_from(origin)
                    {
                        // New best!
                        best_leaf = Some(leaf.clone());
                    }
                    continue;
                }
            }
            None => {
                // Found our first candidate!
                if inner_leaf.bounds.min == inner_leaf.bounds.max {
                    best_leaf = Some(leaf.clone());
                    continue;
                }
            }
        };

        if inner_leaf.max_possible > 1 {
            assert_ne!(inner_leaf.bounds.min, inner_leaf.bounds.max);
            inner_leaf.subdivide(&mut octree, &bots);

            // This could be faster
            octree.leaves.sort_by(|a, b| {
                let a = a.borrow();
                let b = b.borrow();

                // Sort by leaf with max possible overlaps
                if a.max_possible == b.max_possible {
                    let av = a.bounds.volume();
                    let bv = b.bounds.volume();

                    if av == bv {
                        // Put volumes closer to the origin at the end
                        let dist_a = a.bounds.distance_from(origin);
                        let dist_b = b.bounds.distance_from(origin);
                        dist_b.cmp(&dist_a)
                    } else {
                        // Put larger volumes at the end
                        av.cmp(&bv)
                    }
                } else {
                    // Put leaves with larger max_possible values at the end
                    a.max_possible.cmp(&b.max_possible)
                }
            });
        }
    }

    let temp = best_leaf.ok_or("No solution found")?;
    let best_leaf = temp.borrow();
    assert_eq!(best_leaf.bounds.min, best_leaf.bounds.max); // down to a single point

    let pt = best_leaf.bounds.min;
    Ok(pt.x.abs() + pt.y.abs() + pt.z.abs())
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!(
            "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1" => 7
    );

    test_part_two!(
            "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5" => 36
    );

    let input = include_str!("day23_input.txt");
    test_part_one!(input => 270);
    test_part_two!(input => 106_323_091);
}
