#[cfg(feature = "visualization")]
use svgplot::{
    SvgCircle, SvgColor, SvgGroup, SvgImage, SvgPath, SvgScript, SvgShape, SvgStyle, SvgTransform,
    SvgUse,
};

use crate::input::Input;

pub fn solve(input: &Input) -> Result<i32, String> {
    const MAX_STEPS: usize = 10_000;
    let mut remaining_trips = input.part_values(1, 3);

    let mut valley = parse(input.text)?;
    let mut reachable = vec![0; valley.width];
    #[cfg(feature = "visualization")]
    let mut reachable_per_step = vec![(reachable.clone(), true)];
    #[cfg(feature = "visualization")]
    let mut svg = SvgImage::new().style("background:black");
    #[cfg(feature = "visualization")]
    {
        let blizzard_def_id = svg.define(
            SvgPath::default()
                .shape(
                    SvgShape::new()
                        .move_to_absolute(0.5, 0.25)
                        .line_to_relative(0.25, 0.5)
                        .line_to_relative(-0.5, 0.0)
                        .close(),
                )
                .fill(SvgColor::Rgb(0x00, 0xB1, 0xD2)),
        );

        for (blizzard, a, d, dy, dir) in [
            (&valley.blizzards_up, 1., 1., 0., "up"),
            (&valley.blizzards_down, 1., -1., 1., "down"),
        ] {
            for y in 0..valley.height {
                let mut group = SvgGroup::new().class(format!("blizzard blizzard-{}", dir));
                for (x, col) in blizzard.iter().enumerate() {
                    if (col & (1 << y)) == 0 {
                        group.add(
                            SvgUse::new(blizzard_def_id).transform(SvgTransform::Matrix {
                                a,
                                b: 0.,
                                c: 0.,
                                d,
                                dx: x as f64,
                                dy,
                            }),
                        );
                    }
                }
                svg.add(group);
            }
        }
        for (blizzard, b, c, dx, dir) in [
            (&valley.blizzards_right, -1., -1., 1., "right"),
            (&valley.blizzards_left, 1., 1., 0., "left"),
        ] {
            for (_x, col) in blizzard.iter().enumerate() {
                let mut group = SvgGroup::new().class(format!("blizzard blizzard-{}", dir));
                for y in 0..valley.height {
                    if (col & (1 << y)) == 0 {
                        group.add(
                            SvgUse::new(blizzard_def_id).transform(SvgTransform::Matrix {
                                a: 0.,
                                b,
                                c,
                                d: 0.,
                                dx,
                                dy: y as f64 + dx,
                            }),
                        );
                    }
                }
                svg.add(group);
            }
        }
    }

    let top_row_bitmask = 1;
    let bottom_row_bitmask = 1 << (valley.height - 1);

    for minute in 0..MAX_STEPS {
        valley
            .blizzards_up
            .iter_mut()
            .for_each(|m| *m = (*m >> 1) | ((*m & top_row_bitmask) << (valley.height - 1)));
        valley
            .blizzards_down
            .iter_mut()
            .for_each(|m| *m = (*m << 1) | ((*m & bottom_row_bitmask) >> (valley.height - 1)));
        valley.blizzards_right.rotate_right(1);
        valley.blizzards_left.rotate_left(1);

        let heading_down = remaining_trips % 2 == 1;

        let one_trip_completed = if heading_down {
            reachable[valley.width - 1] & bottom_row_bitmask != 0
        } else {
            reachable[0] & 1 != 0
        };

        if one_trip_completed {
            reachable.fill(0);
            remaining_trips -= 1;
            #[cfg(feature = "visualization")]
            {
                reachable_per_step.push((reachable.clone(), !heading_down));
            }
            if remaining_trips == 0 {
                #[cfg(feature = "visualization")]
                {
                    let step_duration = 1000;
                    let animation_duration = step_duration - 200;

                    svg = svg
                        .data_attribute("steps".to_string(), format!("{}", minute + 1))
                        .data_attribute("step-duration".to_string(), format!("{}", step_duration))
                        .view_box((-1, -1, valley.width as i64 + 2, valley.height as i64 + 2));

                    svg.add(SvgStyle::new(format!(".blizzard {{ transition: transform {}ms; }} .elf {{ transition: fill-opacity {}ms ease-in-out; }}", animation_duration, animation_duration)));

                    let mut reachable_array = Vec::new();
                    for (reachable, heading_down) in reachable_per_step.iter() {
                        let mut this_reachable = Vec::new();
                        this_reachable.push(if *heading_down {
                            (0_i32, -1_i32)
                        } else {
                            ((valley.width - 1) as i32, valley.height as i32)
                        });
                        for (x, col) in reachable.iter().enumerate() {
                            for y in 0..valley.height {
                                if col & (1 << y) > 0 {
                                    this_reachable.push((x as i32, y as i32));
                                }
                            }
                        }
                        reachable_array.push(this_reachable);
                    }

                    let mut reachable_array_js = String::from("const reachablePerStep = [");
                    for (arr_idx, arr) in reachable_array.iter().enumerate() {
                        if arr_idx > 0 {
                            reachable_array_js.push(',');
                        }
                        reachable_array_js.push('[');
                        for (idx, (x, y)) in arr.iter().enumerate() {
                            if idx > 0 {
                                reachable_array_js.push(',');
                            }
                            reachable_array_js.push_str(&format!("[{},{}]", x, y));
                        }
                        reachable_array_js.push(']');
                    }
                    reachable_array_js.push(']');
                    let reachable_circle_id = svg.define(SvgCircle {
                        cx: 0.5,
                        cy: 0.5,
                        r: 0.25,
                        fill: Some(SvgColor::Rgb(0xfd, 0xdb, 0x27)),
                    });

                    let reachable_even_path_id = svg.add_with_id(SvgGroup::new().class("elf"));
                    let reachable_odd_path_id = svg.add_with_id(SvgGroup::new().class("elf"));
                    svg.add(SvgScript::new(format!(
                        "{};\n\
                        const leftBlizzards = document.querySelectorAll('.blizzard-left');\n\
                        const rightBlizzards = document.querySelectorAll('.blizzard-right');\n\
                        const upBlizzards = document.querySelectorAll('.blizzard-up');\n\
                        const downBlizzards = document.querySelectorAll('.blizzard-down');\n\
                        const evenPath = document.getElementById('{}');\n\
                        const oddPath = document.getElementById('{}');\n\
                        const width = {};\n\
                        const height = {};\n\
                        const mod = (n, m) => (n % m + m) % m;\n\
                        window.onNewStep = (step) => {{\n\
                            const newCircles = reachablePerStep[step].map(a => {{\n\
                                const c = document.createElementNS('http://www.w3.org/2000/svg', 'use');\n\
                                c.setAttribute('href', '#{}');\n\
                                c.setAttribute('x', a[0]);\n\
                                c.setAttribute('y', a[1]);\n\
                                return c;\n
                            }});\n\
                            const [oldPath, newPath] = (step % 2 == 0) ? [oddPath, evenPath] : [evenPath, oddPath];\n\
                            oldPath.setAttribute('fill-opacity', 0);\n\
                            newPath.setAttribute('fill-opacity', 1);\n\
                            newPath.replaceChildren(...newCircles);\n\
                            for (let [idx, el] of leftBlizzards.entries()) {{\n\
                                let amount = mod((idx - step), width);\n\
                                if (amount === width -1) {{ el.style.transition = 'none'; }} else {{ el.style.transition = ''; }}\n\
                                el.style.transform = `translate(${{amount}}px,0px)`;\n\
                            }}\n\
                            for (let [idx, el] of rightBlizzards.entries()) {{\n\
                                let amount = mod((idx + step), width);\n\
                                if (amount === 0) {{ el.style.transition = 'none'; }} else {{ el.style.transition = ''; }}\n\
                                el.style.transform = `translate(${{amount}}px,0px)`;\n\
                            }}\n\
                            for (let [idx, el] of upBlizzards.entries()) {{\n\
                                let amount = mod((idx - step), height);\n\
                                if (amount === height - 1) {{ el.style.transition = 'none'; }} else {{ el.style.transition = ''; }}\n\
                                el.style.transform = `translate(0px,${{amount}}px)`;\n\
                            }}\n\
                            for (let [idx, el] of downBlizzards.entries()) {{\n\
                                let amount = mod((idx + step), height);\n\
                                if (amount === 0) {{ el.style.transition = 'none'; }} else {{ el.style.transition = ''; }}\n\
                                el.style.transform = `translate(0px,${{amount}}px)`;\n\
                            }}\n\
                        }};\n",
                        reachable_array_js, reachable_even_path_id, reachable_odd_path_id, valley.width, valley.height, reachable_circle_id
                    )));
                    svg.add(
                        SvgPath::default()
                            .fill(SvgColor::Rgb(0xff, 0xff, 0xff))
                            .shape(
                                SvgShape::at(-1, -1)
                                    .line_to_relative(1, 0)
                                    .line_to_relative(0, valley.height as i32 + 2)
                                    .line_to_relative(-1, 0)
                                    .close()
                                    .move_to_absolute(-1, valley.height as i32)
                                    .line_to_relative(valley.width as i32, 0)
                                    .line_to_relative(0, 1)
                                    .line_to_relative(-(valley.width as i32), 0)
                                    .close()
                                    .move_to_absolute(1, -1)
                                    .line_to_relative(valley.width as i32, 0)
                                    .line_to_relative(0, 1)
                                    .line_to_relative(-(valley.width as i32), 0)
                                    .close()
                                    .move_to_absolute(valley.width as i32, -1)
                                    .line_to_relative(1, 0)
                                    .line_to_relative(0, valley.height as i32 + 2)
                                    .line_to_relative(-1, 0)
                                    .close(),
                            ),
                    );
                    input.rendered_svg.replace(svg.to_svg_string());
                }
                return Ok(minute as i32 + 1);
            }
            continue;
        }

        let mut prev = if heading_down { top_row_bitmask } else { 0 };
        let last = if heading_down { 0 } else { bottom_row_bitmask };
        for x in 0..valley.width {
            let prev = std::mem::replace(&mut prev, reachable[x]);
            let next = reachable.get(x + 1).copied().unwrap_or(last);

            // Expand reachable up, down, left and right:
            reachable[x] |= (reachable[x] >> 1) | (reachable[x] << 1) | prev | next;
            // Positions where there are blizzards are not reachable:
            reachable[x] &= valley.blizzards_up[x]
                & valley.blizzards_down[x]
                & valley.blizzards_right[x]
                & valley.blizzards_left[x];
        }

        #[cfg(feature = "visualization")]
        reachable_per_step.push((reachable.clone(), heading_down));
    }

    Err(format!("No solution found in {MAX_STEPS} minutes"))
}

struct Valley {
    width: usize,
    height: usize,
    blizzards_up: Vec<u64>,
    blizzards_down: Vec<u64>,
    blizzards_right: Vec<u64>,
    blizzards_left: Vec<u64>,
}

fn parse(input: &str) -> Result<Valley, String> {
    let width = input
        .find('\n')
        .ok_or("Invalid input - not multiple lines")?
        - 2;
    let height = input.lines().count() - 2;
    if height > 64 {
        return Err("Too big height for input - must be less than 64".to_string());
    }

    let mut blizzards_up = vec![(1 << height) - 1; width];
    let mut blizzards_down = vec![(1 << height) - 1; width];
    let mut blizzards_right = vec![(1 << height) - 1; width];
    let mut blizzards_left = vec![(1 << height) - 1; width];

    for (y, line) in input.lines().skip(1).take(height).enumerate() {
        if line.len() != width + 2 {
            return Err("Not all lines have equal length".to_string());
        }
        for (x, c) in line.bytes().skip(1).take(width).enumerate() {
            match c {
                b'^' => blizzards_up[x] &= !(1 << y),
                b'v' => blizzards_down[x] &= !(1 << y),
                b'>' => blizzards_right[x] &= !(1 << y),
                b'<' => blizzards_left[x] &= !(1 << y),
                _ => {}
            }
        }
    }

    Ok(Valley {
        width,
        height,
        blizzards_up,
        blizzards_down,
        blizzards_right,
        blizzards_left,
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
    test_part_one!(test_input => 18);
    test_part_two!(test_input => 54);

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 242);
    test_part_two!(real_input => 720);
}
