use crate::input::Input;

// Parse input in the format "A=<211,-141,-45>".
fn parse_vector(input: &str) -> Option<(i32, i32, i32)> {
    if input.len() > 3 {
        if let Some(stripped) = input[2..]
            .strip_prefix('<')
            .and_then(|s| s.strip_suffix('>'))
        {
            let mut number_parts = stripped.split(',');
            let x = number_parts.next()?.parse::<i16>().ok()?;
            let y = number_parts.next()?.parse::<i16>().ok()?;
            let z = number_parts.next()?.parse::<i16>().ok()?;
            return Some((i32::from(x), i32::from(y), i32::from(z)));
        }
    }
    None
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut particles = Vec::new();

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid format", line_idx + 1);

        let mut parts = line.split(", ");
        let position_part =
            parse_vector(parts.next().ok_or_else(on_error)?).ok_or_else(on_error)?;
        let speed_part = parse_vector(parts.next().ok_or_else(on_error)?).ok_or_else(on_error)?;
        let acceleration_part =
            parse_vector(parts.next().ok_or_else(on_error)?).ok_or_else(on_error)?;

        particles.push((position_part, speed_part, acceleration_part));
    }

    for _ in 0..500 {
        particles = particles
            .iter()
            .filter_map(|particle| {
                if input.is_part_two()
                    && particles
                        .iter()
                        .filter(|other_particle| other_particle.0 == particle.0)
                        .count()
                        > 1
                {
                    None
                } else {
                    let new_speed = (
                        particle.1 .0 + particle.2 .0,
                        particle.1 .1 + particle.2 .1,
                        particle.1 .2 + particle.2 .2,
                    );
                    let new_position = (
                        particle.0 .0 + new_speed.0,
                        particle.0 .1 + new_speed.1,
                        particle.0 .2 + new_speed.2,
                    );
                    let new_acceleration = particle.2;
                    Some((new_position, new_speed, new_acceleration))
                }
            })
            .collect();
    }

    if input.is_part_one() {
        Ok(particles
            .iter()
            .enumerate()
            .fold((0, i32::MAX), |acc, (idx, particle)| {
                let dist = particle.0 .0.abs() + particle.0 .1.abs() + particle.0 .2.abs();
                if dist < acc.1 {
                    (idx, dist)
                } else {
                    acc
                }
            })
            .0 as u32)
    } else {
        Ok(particles.len() as u32)
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day20_input.txt");
    test_part_one!(real_input => 91);
    test_part_two!(real_input => 567);
}
