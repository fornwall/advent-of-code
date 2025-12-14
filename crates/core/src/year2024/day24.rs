use crate::common::array_stack::ArrayStack;
use crate::common::id_assigner::IdAssigner;
use crate::input::Input;

type Wire = u16;

pub fn solve(input: &Input) -> Result<String, String> {
    let mut device = Device::parse(input.text);
    if input.is_part_one() {
        Ok(device.compute_z().to_string())
    } else {
        // Assume that the input is a "Ripple-carry adder", as described here:
        // The output bit Zᵢ is computed from inputs Xᵢ, Yᵢ, and carry-in Cᵢ as follows:
        //
        // [Cᵢ] --------------------->┌----─┐
        // [Xᵢ] ------>┌----─┐        │ XOR │------>[Zᵢ]
        //             │ XOR │---┬--->│     │
        // [Yᵢ] ------>└----─┘   │    └----─┘
        //                       ↓
        //                    ┌----─┐
        // [Cᵢ] ------------->│ AND |--->┌----─┐
        //                    └----─┘    │ OR  │--->[Cᵢ₊₁]
        // [Xᵢ] --->┌---─-┐              │     │
        //          │ AND │------------->└----─┘
        // [Yᵢ] --->└---─-┘
        //
        // Note that it's always the OUTPUT wires that are miswired, never the INPUT wires.
        let mut incorrect_wires = ArrayStack::<8, Wire>::new();
        for (wire_id, &gate) in device.wire_values.iter().enumerate() {
            if device.wire_types[wire_id] == WireType::Z {
                // If the output of a gate is z, then the operation has to be XOR:
                if !matches!(gate, Gate::Xor(_, _))
                // .. unless it's the last z bit (not going through adder):
                && device.z_values[device.num_z_values - 1] != wire_id as Wire
                {
                    incorrect_wires.push(wire_id as Wire)?;
                }
            } else if let Gate::Xor(w1, w2) = gate {
                    if !matches!(
                        (
                            device.wire_types[w1 as usize],
                            device.wire_types[w2 as usize]
                        ),
                        (WireType::X, WireType::Y) | (WireType::Y, WireType::X)
                    ) {
                        // XOR gate must have x/y inputs.
                        incorrect_wires.push(wire_id as Wire)?;
                    } else
                        // XOR gate with x/y inputs.
                        // There must be another XOR gate with this as input.
                        if !device.wire_values.iter().any(|&gate|
                                matches!(gate, Gate::Xor(wa, wb) if wa == wire_id as Wire || wb == wire_id as Wire)) {
                                incorrect_wires.push(wire_id as Wire)?;
                            }
                } else if let Gate::And(a_input, b_input) = gate
                    // Non-initial AND gate must feed into an OR gate.
                    // The initial AND gate with (x00, y00) as input feds z00 directly, so is exempt:
                    && (a_input, b_input) != (device.x00_wire, device.y00_wire)
                    && (a_input, b_input) != (device.y00_wire, device.x00_wire)
                    && !device.wire_values.iter().any(|&gate|
                            matches!(gate, Gate::Or(wa, wb) if wa == wire_id as Wire || wb == wire_id as Wire)) {
                            incorrect_wires.push(wire_id as Wire)?;
                        }
        }
        let mut incorrect_names = Vec::<&str>::new();
        for &wire_id in incorrect_wires.slice() {
            let original_value = device
                .id_assigner
                .original_value_of(wire_id)
                .ok_or_else(|| format!("Wire name lookup failed: {wire_id}"))?;
            incorrect_names.push(original_value);
        }
        incorrect_names.sort_unstable();
        Ok(incorrect_names.join(","))
    }
}

#[derive(Clone, Copy, Debug)]
enum Gate {
    And(Wire, Wire),
    Or(Wire, Wire),
    Xor(Wire, Wire),
    ComputedValue(bool),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum WireType {
    X,
    Y,
    Z,
    Intermediate,
}

impl WireType {
    fn from_wire_name(wire_name: &str) -> Self {
        match wire_name.as_bytes()[0] {
            b'x' => Self::X,
            b'y' => Self::Y,
            b'z' => Self::Z,
            _ => Self::Intermediate,
        }
    }
}

struct Device<'a> {
    wire_values: [Gate; 1000],
    wire_types: [WireType; 1000],
    x00_wire: Wire,
    y00_wire: Wire,
    z_values: [Wire; 64],
    num_z_values: usize,
    id_assigner: IdAssigner<'a, 1024, str>,
}

impl<'a> Device<'a> {
    fn parse(input: &'a str) -> Self {
        let mut wire_values = [Gate::ComputedValue(false); 1000];
        let mut id_assigner = IdAssigner::<'a, 1024, str>::new("");
        let mut x00_wire = 0;
        let mut y00_wire = 0;

        let mut z_values = [0; 64];
        let mut wire_types = [WireType::Intermediate; 1000];
        let mut num_z_values = 0;

        let mut add_z_if = |wire_name: &str, wire_id: Wire| {
            if let Some(num_part) = wire_name.strip_prefix('z') {
                let num = num_part.parse::<u8>().unwrap();
                z_values[num as usize] = wire_id;
                num_z_values += 1;
            }
        };
        let mut add_xy0_if = |wire_name: &str, wire_id: Wire| {
            if wire_name == "x00" {
                x00_wire = wire_id;
            } else if wire_name == "y00" {
                y00_wire = wire_id;
            }
        };

        for line in input.lines() {
            if let Some((wire_name, initial_value)) = line.split_once(": ") {
                let wire_id = id_assigner.id_of(wire_name).unwrap();
                let initial_value = initial_value.eq("1");
                add_xy0_if(wire_name, wire_id);
                wire_values[wire_id as usize] = Gate::ComputedValue(initial_value);
            } else if let Some((from, to)) = line.split_once(" -> ") {
                let mut from_parts = from.split(' ');
                let p1_wire_name = from_parts.next().unwrap();
                let p1_wire_id = id_assigner.id_of(p1_wire_name).unwrap();
                wire_types[p1_wire_id as usize] = WireType::from_wire_name(p1_wire_name);
                let op_type = from_parts.next().unwrap();
                let p2_wire_name = from_parts.next().unwrap();
                let p2_wire_id = id_assigner.id_of(p2_wire_name).unwrap();
                wire_types[p2_wire_id as usize] = WireType::from_wire_name(p2_wire_name);
                let to_wire_id = id_assigner.id_of(to).unwrap();
                wire_types[to_wire_id as usize] = WireType::from_wire_name(to);
                add_z_if(to, to_wire_id);
                wire_values[to_wire_id as usize] = match op_type {
                    "AND" => Gate::And(p1_wire_id, p2_wire_id),
                    "OR" => Gate::Or(p1_wire_id, p2_wire_id),
                    "XOR" => Gate::Xor(p1_wire_id, p2_wire_id),
                    _ => unreachable!("Strange op: '{op_type}'"),
                };
            }
        }

        Self {
            wire_values,
            wire_types,
            x00_wire,
            y00_wire,
            z_values,
            num_z_values,
            id_assigner,
        }
    }

    fn compute_z(&mut self) -> u64 {
        (0..self.num_z_values).fold(0, |acc, i| {
            acc | (u64::from(self.compute_wire(self.z_values[i])) * (1 << i))
        })
    }

    fn compute_wire(&mut self, wire_id: Wire) -> bool {
        match self.wire_values[wire_id as usize] {
            Gate::And(w1, w2) => self.compute_wire(w1) & self.compute_wire(w2),
            Gate::Or(w1, w2) => self.compute_wire(w1) | self.compute_wire(w2),
            Gate::Xor(w1, w2) => self.compute_wire(w1) ^ self.compute_wire(w2),
            Gate::ComputedValue(value) => value,
        }
    }
}

#[test]
pub fn tests() {
    let test_input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    test_part_one!(test_input => "4".to_string());
    let test_input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
    test_part_one!(test_input => "2024".to_string());

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => "65740327379952".to_string());
    test_part_two!(real_input => "bgs,pqc,rjm,swt,wsv,z07,z13,z31".to_string());
}
