struct Fabric {
    num_claims: Vec<u32>,
}

struct Claim {
    id: u32,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Fabric {
    fn new() -> Self {
        Self {
            num_claims: vec![0; 1_000_000],
        }
    }

    fn add_claim(&mut self, claim: &Claim) {
        for y in claim.y..claim.y + claim.height {
            let row_offset = y * 1000;
            for x in claim.x..claim.x + claim.width {
                self.num_claims[(row_offset + x) as usize] += 1_u32;
            }
        }
    }

    fn inches_claimed_multiple(&self) -> usize {
        self.num_claims.iter().filter(|&&c| c > 1).count()
    }

    fn is_claimed_once(&self, claim: &Claim) -> bool {
        let mut result = true;
        for y in claim.y..claim.y + claim.height {
            let row_offset = y * 1000;
            for x in claim.x..claim.x + claim.width {
                if self.num_claims[(row_offset + x) as usize] > 1 {
                    result = false;
                }
            }
        }
        result
    }
}

fn parse_input(input_string: &str) -> Result<Vec<Claim>, String> {
    input_string
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            let error_message = || format!("Invalid input on line {}: {}", line_index + 1, line);
            let parts: Vec<u32> = line
                .replace("#", "")
                .replace("@", "")
                .replace(",", " ")
                .replace(":", "")
                .replace("x", " ")
                .split_whitespace()
                .map(|s| s.parse::<u32>().map_err(|_| error_message()))
                .collect::<Result<_, _>>()?;
            if parts.len() != 5 {
                return Err(error_message());
            }
            Ok(Claim {
                id: parts[0],
                x: parts[1],
                y: parts[2],
                width: parts[3],
                height: parts[4],
            })
        })
        .collect()
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    let mut fabric = Fabric::new();

    let input = parse_input(input_string)?;
    input.iter().for_each(|claim| fabric.add_claim(claim));

    Ok(fabric.inches_claimed_multiple())
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    let mut fabric = Fabric::new();

    let claims = parse_input(input_string)?;

    claims.iter().for_each(|claim| fabric.add_claim(claim));

    let claim_without_overlap = claims
        .iter()
        .find(|claim| fabric.is_claimed_once(claim))
        .ok_or("No claim without overlap found")?;
    Ok(claim_without_overlap.id)
}

#[test]
fn tests_part1() {
    assert_eq!(
        Ok(4),
        part1(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
        )
    );

    assert_eq!(Ok(104_126), part1(include_str!("day03_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok(3),
        part2(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
        )
    );

    assert_eq!(Ok(695), part2(include_str!("day03_input.txt")));
}
