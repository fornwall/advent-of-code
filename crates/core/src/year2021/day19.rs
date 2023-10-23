use crate::input::Input;
use std::collections::HashMap;
use std::ops::{Add, Mul, Sub};

// Based on the following nice solution:
// - https://github.com/Mesoptier/advent-of-code-2021/blob/master/src/days/day19.rs
// - https://www.reddit.com/r/adventofcode/comments/rjpf7f/comment/hp8btm1/?utm_source=share&utm_medium=web2x&context=3
pub fn solve(input: &Input) -> Result<u32, String> {
    let mut unmatched_scans = Scan::parse(input.text)?;
    let mut matched_scans: Vec<Scan> = vec![unmatched_scans.remove(0)];
    let mut matched_scanner_positions = vec![Point { x: 0, y: 0, z: 0 }];

    while !unmatched_scans.is_empty() {
        if let Some((matched_scanner_position, transformed_matched_scan, matched_scan_idx)) =
            unmatched_scans
                .iter()
                .enumerate()
                .find_map(|(scan_idx, scan)| {
                    find_match(&matched_scans, scan).map(|(p, s)| (p, s, scan_idx))
                })
        {
            matched_scans.push(transformed_matched_scan);
            matched_scanner_positions.push(matched_scanner_position);
            unmatched_scans.swap_remove(matched_scan_idx);
        } else {
            return Err("Unable to match beacons".to_string());
        }
    }

    Ok(if input.is_part_one() {
        let mut known_beacons = matched_scans
            .iter()
            .flat_map(|scan| scan.beacons.iter())
            .collect::<Vec<_>>();
        known_beacons.sort_unstable();
        known_beacons.dedup();
        known_beacons.len() as u32
    } else {
        all_pairs(&matched_scanner_positions)
            .map(|(s1, s2)| (*s1 - *s2).norm_l1())
            .max()
            .unwrap_or_default() as u32
    })
}

fn all_pairs<T>(elements: &[T]) -> impl Iterator<Item = (&T, &T)> {
    (0..elements.len())
        .flat_map(move |i| (i + 1..elements.len()).map(move |j| (&elements[i], &elements[j])))
}

struct RotationMatrix {
    elements: [i8; 9],
}

impl RotationMatrix {
    #![allow(clippy::too_many_arguments)]
    const fn new(e1: i8, e2: i8, e3: i8, e4: i8, e5: i8, e6: i8, e7: i8, e8: i8, e9: i8) -> Self {
        Self {
            elements: [e1, e2, e3, e4, e5, e6, e7, e8, e9],
        }
    }
}

impl Mul<Point> for &RotationMatrix {
    type Output = Point;

    fn mul(self, p: Point) -> Point {
        Point {
            x: i16::from(self.elements[0]) * p.x
                + i16::from(self.elements[1]) * p.y
                + i16::from(self.elements[2]) * p.z,
            y: i16::from(self.elements[3]) * p.x
                + i16::from(self.elements[4]) * p.y
                + i16::from(self.elements[5]) * p.z,
            z: i16::from(self.elements[6]) * p.x
                + i16::from(self.elements[7]) * p.y
                + i16::from(self.elements[8]) * p.z,
        }
    }
}

static ROTATION_MATRICES: [RotationMatrix; 24] = [
    RotationMatrix::new(1, 0, 0, 0, 1, 0, 0, 0, 1),
    RotationMatrix::new(1, 0, 0, 0, 0, 1, 0, -1, 0),
    RotationMatrix::new(1, 0, 0, 0, -1, 0, 0, 0, -1),
    RotationMatrix::new(1, 0, 0, 0, 0, -1, 0, 1, 0),
    RotationMatrix::new(0, 1, 0, 0, 0, 1, 1, 0, 0),
    RotationMatrix::new(0, 1, 0, 1, 0, 0, 0, 0, -1),
    RotationMatrix::new(0, 1, 0, 0, 0, -1, -1, 0, 0),
    RotationMatrix::new(0, 1, 0, -1, 0, 0, 0, 0, 1),
    RotationMatrix::new(0, 0, 1, 1, 0, 0, 0, 1, 0),
    RotationMatrix::new(0, 0, 1, 0, 1, 0, -1, 0, 0),
    RotationMatrix::new(0, 0, 1, -1, 0, 0, 0, -1, 0),
    RotationMatrix::new(0, 0, 1, 0, -1, 0, 1, 0, 0),
    RotationMatrix::new(-1, 0, 0, 0, -1, 0, 0, 0, 1),
    RotationMatrix::new(-1, 0, 0, 0, 0, 1, 0, 1, 0),
    RotationMatrix::new(-1, 0, 0, 0, 1, 0, 0, 0, -1),
    RotationMatrix::new(-1, 0, 0, 0, 0, -1, 0, -1, 0),
    RotationMatrix::new(0, -1, 0, 0, 0, -1, 1, 0, 0),
    RotationMatrix::new(0, -1, 0, 1, 0, 0, 0, 0, 1),
    RotationMatrix::new(0, -1, 0, 0, 0, 1, -1, 0, 0),
    RotationMatrix::new(0, -1, 0, -1, 0, 0, 0, 0, -1),
    RotationMatrix::new(0, 0, -1, -1, 0, 0, 0, 1, 0),
    RotationMatrix::new(0, 0, -1, 0, 1, 0, 1, 0, 0),
    RotationMatrix::new(0, 0, -1, 1, 0, 0, 0, -1, 0),
    RotationMatrix::new(0, 0, -1, 0, -1, 0, -1, 0, 0),
];

#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Point {
    x: i16,
    y: i16,
    z: i16,
}

impl Point {
    const fn norm_l1(self) -> i16 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn norm_lmax(self) -> i16 {
        self.x.abs().max(self.y.abs()).max(self.z.abs())
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

struct Scan {
    beacons: Vec<Point>,
    fingerprints: HashMap<Fingerprint, Vec<(Point, Point)>>,
}

impl Scan {
    fn parse(text: &str) -> Result<Vec<Self>, String> {
        let mut scans = Vec::new();
        let mut beacons = Vec::new();
        for line in text.lines() {
            if line.starts_with("---") || line.is_empty() {
                if !beacons.is_empty() {
                    scans.push(Self::new(beacons));
                    beacons = Vec::new();
                }
            } else {
                let coordinates = line
                    .split(',')
                    .map(|s| s.parse::<i16>().map_err(|_| "Invalid point".to_string()))
                    .collect::<Result<Vec<_>, String>>()?;
                if coordinates.len() == 3 {
                    beacons.push(Point {
                        x: coordinates[0],
                        y: coordinates[1],
                        z: coordinates[2],
                    });
                } else {
                    return Err(format!(
                        "Invalid number of coordinates: {}",
                        coordinates.len()
                    ));
                }
            }
        }
        if !beacons.is_empty() {
            scans.push(Self::new(beacons));
        }
        if scans.is_empty() {
            return Err("No scans".to_string());
        }
        Ok(scans)
    }

    fn new(beacons: Vec<Point>) -> Self {
        let mut fingerprints = HashMap::<Fingerprint, Vec<(Point, Point)>>::new();
        for (&p1, &p2) in all_pairs(&beacons) {
            let f = fingerprint(p1, p2);
            fingerprints.entry(f).or_default().push((p1, p2));
        }
        Self {
            beacons,
            fingerprints,
        }
    }
}

type Fingerprint = (i16, i16);

fn fingerprint(p1: Point, p2: Point) -> Fingerprint {
    let d = p1 - p2;
    (d.norm_l1(), d.norm_lmax())
}

fn find_match(matched_scans: &[Scan], scan_to_join: &Scan) -> Option<(Point, Scan)> {
    for matched_scan in matched_scans {
        let mut num_pairs_matching_fingerprint = 0;
        let matching_fingerprints = scan_to_join
            .fingerprints
            .iter()
            .filter(|(to_join_fingerprint, to_join_pairs)| {
                matched_scan
                    .fingerprints
                    .get(to_join_fingerprint)
                    .map_or(false, |matched_pairs| {
                        num_pairs_matching_fingerprint += to_join_pairs.len() * matched_pairs.len();
                        true
                    })
            })
            .collect::<Vec<_>>();

        if num_pairs_matching_fingerprint < 66 {
            // At least (12 choose 2) fingerprints needs to match.
            continue;
        }

        for (fingerprint, fingerprinted_beacons) in
            matching_fingerprints
                .iter()
                .flat_map(|(fingerprint, multiple_fingerprints)| {
                    multiple_fingerprints
                        .iter()
                        .map(move |fingerprinted_pair| (fingerprint, fingerprinted_pair))
                })
        {
            for matched_pair in matched_scan.fingerprints.get(fingerprint)? {
                for (first, second) in [
                    (matched_pair, fingerprinted_beacons),
                    (fingerprinted_beacons, matched_pair),
                ] {
                    let (first_1, first_2) = first;
                    let (second_1, second_2) = second;

                    if let Some(supported_rotation) = ROTATION_MATRICES.iter().find(|&rotation| {
                        *first_1 - rotation * *second_1 == *first_2 - rotation * *second_2
                    }) {
                        let translation = *first_1 - supported_rotation * *second_1;

                        let transformed_beacons = scan_to_join
                            .beacons
                            .iter()
                            .map(|p| supported_rotation * *p + translation)
                            .collect::<Vec<_>>();

                        let mut num_matches = 0;
                        for transformed_beacon in &transformed_beacons {
                            if matched_scan.beacons.contains(transformed_beacon) {
                                num_matches += 1;
                                if num_matches == 12 {
                                    return Some((translation, Scan::new(transformed_beacons)));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
    test_part_one!(example => 79);
    test_part_two!(example => 3621);

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => 378);
    test_part_two!(real_input => 13_148);
}
