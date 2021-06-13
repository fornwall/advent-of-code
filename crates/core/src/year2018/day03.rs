use crate::input::Input;

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

const SQUARE_WIDTH: u32 = 1000;

impl Fabric {
    fn from_claims(claims: &[Claim]) -> Self {
        let mut result = Self {
            num_claims: vec![0; SQUARE_WIDTH as usize * SQUARE_WIDTH as usize],
        };
        claims.iter().for_each(|claim| result.add_claim(claim));
        result
    }

    fn add_claim(&mut self, claim: &Claim) {
        for y in claim.y..claim.y + claim.height {
            let row_offset = y * 1000;
            for x in claim.x..claim.x + claim.width {
                self.num_claims[(row_offset + x) as usize] += 1_u32;
            }
        }
    }

    fn inches_claimed_multiple(&self) -> u32 {
        self.num_claims.iter().filter(|&&c| c > 1).count() as u32
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
            let error_message = || format!("Invalid input on line {}", line_index + 1);
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
            let claim = Claim {
                id: parts[0],
                x: parts[1],
                y: parts[2],
                width: parts[3],
                height: parts[4],
            };
            if claim.x + claim.width > SQUARE_WIDTH || claim.y + claim.height > SQUARE_WIDTH {
                return Err(format!(
                    "Claim outside {} by {} square",
                    SQUARE_WIDTH, SQUARE_WIDTH
                ));
            }
            Ok(claim)
        })
        .collect()
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let claims = parse_input(input.text)?;
    let fabric = Fabric::from_claims(&claims);

    if input.is_part_one() {
        Ok(fabric.inches_claimed_multiple())
    } else {
        let claim_without_overlap = claims
            .iter()
            .find(|claim| fabric.is_claimed_once(claim))
            .ok_or("No claim without overlap found")?;
        Ok(claim_without_overlap.id)
    }
}

#[test]
fn tests() {
    use crate::{test_part_one, test_part_two};
    test_part_one!(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2" => 4
    );

    test_part_two!(
            "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2"
        => 3
    );

    let input = include_str!("day03_input.txt");
    test_part_one!(input => 104_126);
    test_part_two!(input => 695);
}
