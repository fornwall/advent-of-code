use crate::year2022::day24::Valley;
use svgplot::{
    SvgCircle, SvgColor, SvgGroup, SvgImage, SvgPath, SvgScript, SvgShape, SvgStyle, SvgTransform,
    SvgUse,
};

pub struct Renderer {
    pub reachable_per_step: Vec<(Vec<u64>, /*going down: */ bool)>,
    pub svg: SvgImage,
}

impl Renderer {
    pub fn new(reachable: &Vec<u64>, valley: &Valley) -> Self {
        let mut svg = SvgImage::new().style("background:black");
        let reachable_per_step = vec![(reachable.clone(), true)];

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

        Self {
            svg,
            reachable_per_step,
        }
    }

    pub fn final_svg(mut self, valley: &Valley, minute: usize) -> String {
        let step_duration = 1000;
        let animation_duration = step_duration - 200;

        self.svg = self
            .svg
            .data_attribute("steps".to_string(), format!("{}", minute + 1))
            .data_attribute("step-duration".to_string(), format!("{}", step_duration))
            .view_box((-1, -1, valley.width as i64 + 2, valley.height as i64 + 2));

        self.svg.add(SvgStyle::new(format!(".blizzard {{ transition: transform {}ms; }} .elf {{ transition: fill-opacity {}ms ease-in-out; }}", animation_duration, animation_duration)));

        let mut reachable_array = Vec::new();
        for (reachable, heading_down) in self.reachable_per_step.iter() {
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
        let reachable_circle_id = self.svg.define(SvgCircle {
            cx: 0.5,
            cy: 0.5,
            r: 0.25,
            fill: Some(SvgColor::Rgb(0xfd, 0xdb, 0x27)),
        });

        let reachable_even_path_id = self.svg.add_with_id(SvgGroup::new().class("elf"));
        let reachable_odd_path_id = self.svg.add_with_id(SvgGroup::new().class("elf"));
        self.svg.add(SvgScript::new(format!(
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
        self.svg.add(
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
        self.svg.to_svg_string()
    }
}
