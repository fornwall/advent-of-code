use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Nanobot {
    pos: Pos,
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
                let pos = Pos::new(x, y, z);
                let radius = parts[7].parse::<i32>().map_err(error_mapper)?;
                Ok(Self { pos, radius })
            })
            .collect::<Result<Vec<Self>, String>>()
    }

    const fn is_bot_within_range(&self, other: &Self) -> bool {
        self.pos.distance_between(other.pos) <= self.radius
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

impl Pos {
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
}

impl std::ops::Index<usize> for Pos {
    type Output = i32;

    fn index(&self, i: usize) -> &i32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid index"),
        }
    }
}

impl std::ops::Add<(i32, i32, i32)> for Pos {
    type Output = Self;

    fn add(self, other: (i32, i32, i32)) -> Self {
        Self::new(self.x + other.0, self.y + other.1, self.z + other.2)
    }
}

impl std::ops::Add<Pos> for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl std::ops::Sub<Pos> for Pos {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl std::ops::IndexMut<usize> for Pos {
    fn index_mut(&mut self, i: usize) -> &mut i32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid index"),
        }
    }
}

// min/max are both INCLUSIVE
#[derive(Copy, Clone, Debug)]
struct AABB {
    min: Pos,
    max: Pos,
}

impl AABB {
    const fn new() -> Self {
        let min = Pos::with_value(std::i32::MAX);
        let max = Pos::with_value(std::i32::MIN);
        Self { min, max }
    }

    const fn with_corners(min: Pos, max: Pos) -> Self {
        Self { min, max }
    }

    fn add_point(&mut self, pos: Pos) {
        self.min = self.min.min(pos);
        self.max = self.max.max(pos);
    }

    fn add_sphere(&mut self, center: Pos, radius: i32) {
        self.add_point(center - Pos::with_value(radius));
        self.add_point(center + Pos::with_value(radius));
    }

    fn overlaps(&self, bot: &Nanobot) -> bool {
        let dist = self.distance_from(bot.pos);
        dist <= bot.radius
    }

    fn num_overlaps(&self, bots: &[Nanobot]) -> usize {
        bots.iter().filter(|bot| self.overlaps(bot)).count()
    }

    fn distance_from(&self, point: Pos) -> i32 {
        let mut closest: Pos = point;

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

    #[allow(dead_code)]
    const fn contains(&self, pt: Pos) -> bool {
        pt.x >= self.min.x
            && pt.x <= self.max.x
            && pt.y >= self.min.y
            && pt.y <= self.max.y
            && pt.z >= self.min.z
            && pt.z <= self.max.z
    }
}

impl Pos {
    pub const fn distance_between(&self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

#[derive(Debug)]
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

struct Octree {
    _root: Rc<RefCell<OctreeNode>>,
    leaves: Vec<Rc<RefCell<OctreeNode>>>,
}

impl Octree {
    fn new(bots: &[Nanobot]) -> Self {
        let bounds = bots.iter().fold(AABB::new(), |aabb, bot| {
            let mut result = aabb;
            result.add_sphere(bot.pos, bot.radius);
            result
        });

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

pub fn part1(input_string: &str) -> Result<usize, String> {
    let bots = Nanobot::parse(input_string)?;
    let strongest_bot = bots
        .iter()
        .max_by(|x, y| x.radius.cmp(&y.radius))
        .ok_or("No robot specified")?;
    Ok(bots
        .iter()
        .filter(|&bot| strongest_bot.is_bot_within_range(bot))
        .count())
}

// https://www.forrestthewoods.com/blog/solving-advent-of-code-in-under-a-second/
pub fn part2(input_string: &str) -> Result<i32, String> {
    let bots = Nanobot::parse(input_string)?;
    let mut octree = Octree::new(&bots);
    let origin = Pos::new(0, 0, 0);
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
fn tests_part1() {
    assert_eq!(
        Ok(7),
        part1(
            "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1"
        )
    );

    assert_eq!(Ok(270), part1(include_str!("day23_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok(36),
        part2(
            "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5"
        )
    );

    assert_eq!(Ok(106_323_091), part2(include_str!("day23_input.txt")));
}
