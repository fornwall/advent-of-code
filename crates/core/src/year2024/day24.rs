use crate::common::array_stack::ArrayStack;
use crate::common::id_assigner::IdAssigner;
use crate::input::Input;

type Wire = u16;

pub fn solve(input: &Input) -> Result<String, String> {
    let mut device = Device::parse(input.text);
    if input.is_part_one() {
        Ok(device.compute_z().to_string())
    } else {
        // Make assumptions about how gates are wired to create the adder.
        // https://www.reddit.com/r/adventofcode/comments/1hnit3b/comment/m430qxz/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
        // Mine assumes that the gates are supposed to be set up in the standard five-gates-per-bit
        // form of a standard binary adder, and just finds gates that don't conform to that pattern.
        // Specifically, it repeats the following for each x input beyond x00:
        // Find the XOR gate and the AND gate that the input is connected to. (This is a hard
        //   assumption - it's gate outputs that are mixed up, not gate inputs, so this should always be true.)
        // The XOR gate should fork its output to two other gates. If it doesn't, it's one of the miswired ones.
        // The AND gate should send its output to one other gate. If it doesn't, it's one of the miswired ones.
        // (1) One of the gates below those two is another XOR gate. It should output to a Z wire. If it doesn't,
        //     it's one of the miswired ones.
        // Another of the gates below those is another AND gate. It should output to a single OR gate. If it doesn't,
        // it's one of the miswired ones.
        // The final one of the three gates should be an OR gate (though we might not find it if one of the earlier
        // gates was miswired). If we do find it, it should output to an XOR gate and an AND gate. If it doesn't, it's one of the miswired ones.
        //
        // https://www.electronics-tutorials.ws/combination/comb_7.html

        //                OR

        // FROM https://www.electronics-tutorials.ws/combination/comb_7.html
        // Check against https://www.reddit.com/r/adventofcode/comments/1hqj35q/2024_day_24_part_2_c_very_proud_of_my_solution/
        //
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

        // Note that it's always the OUTPUT wires that are miswired, never the INPUT wires.
        // TODO: Problem statement: "Your system of gates and wires has four pairs of gates which need their output wires swapped"
        // BUT: How to identify if input or output wires are swapped?
        // NOTE: The distinction if output or input is incorrect is only relevant
        //       if checking connections to other gates. If just checking the current gate it's NOT relevant.
        let mut incorrect_wires = ArrayStack::<8, Wire>::new();
        for (wire_id, &gate) in device.wire_values.iter().enumerate() {
            if let Gate::Xor(w1, w2) = gate {
                match (device.wire_types[w1 as usize], device.wire_types[w2 as usize], device.wire_types[wire_id]) {
                    // If XOR takes x or y as input, output needs to be intermediate gate. If not an error:
                    (WireType::X | WireType::Y, _, WireType::X | WireType::Y | WireType::Z) |
                    (_, WireType::X | WireType::Y, WireType::X | WireType::Y | WireType::Z) |
                    // .. so if first input is x or y and second input is not x or y, it's an error:
                    (WireType::X | WireType::Y, WireType::Z | WireType::Intermediate, _) |
                    // .. so if second input is x or y and first input is not x or y, it's an error:
                    (WireType::Z | WireType::Intermediate, WireType::X | WireType::Y, _) |
                    // Also, z should never be input:
                    (WireType::Z, _, _) | (_, WireType::Z, _) => {
                        /*
                        TODO: y00 XOR x00 -> z00 is ok at initial gate, and that seems to be the only one found
                        println!(
                            "Found incorrect XOR gate:. {} XOR {} -> {}",
                            device.id_assigner.original_value_of(w1),
                            device.id_assigner.original_value_of(w2),
                            device.id_assigner.original_value_of(wire_id as Wire),
                        );
                        incorrect_wires.push(wire_id as Wire)?;
                        */
                    }
                    _ => {}
                }
            } else if let Gate::Or(w1, w2) = gate {
                // Input AND Output must be intermediate:
                match (
                    device.wire_types[w1 as usize],
                    device.wire_types[w2 as usize],
                    device.wire_types[wire_id],
                ) {
                    (WireType::X | WireType::Y | WireType::Z, _, _)
                    | (_, WireType::X | WireType::Y | WireType::Z, _)
                    | (_, _, WireType::X | WireType::Y | WireType::Z) => {
                        println!(
                            "Found incorrect OR gate:. {} OR {} -> {}",
                            device.id_assigner.original_value_of(w1),
                            device.id_assigner.original_value_of(w2),
                            device.id_assigner.original_value_of(wire_id as Wire),
                        );
                        incorrect_wires.push(wire_id as Wire)?;
                    }
                    _ => {
                        // Inputs to OR must be AND gates:
                        if !matches!(device.wire_values[w1 as usize], Gate::And(_, _)) {
                            println!(
                                "Found OR gate with non-AND input. {} OR {}",
                                device.id_assigner.original_value_of(w1),
                                device.id_assigner.original_value_of(wire_id as Wire),
                            );
                            incorrect_wires.push(w1)?;
                        }
                        if !matches!(device.wire_values[w2 as usize], Gate::And(_, _)) {
                            println!(
                                "Found OR gate with non-AND input. From {} to {}",
                                device.id_assigner.original_value_of(w2),
                                device.id_assigner.original_value_of(wire_id as Wire),
                            );
                            incorrect_wires.push(w2)?;
                        }
                    }
                }
            } else if let Gate::And(w1, w2) = gate {
                match (device.wire_types[w1 as usize], device.wire_types[w2 as usize], device.wire_types[wire_id]) {
                    // If one input is x, other needs to be y and vice versa:
                    (WireType::X, WireType::X | WireType::Z | WireType::Intermediate, _) |
                    (WireType::Y, WireType::Y | WireType::Z | WireType::Intermediate, _) |
                    (WireType::Z | WireType::Intermediate, WireType::X, _) |
                    (WireType::Z | WireType::Intermediate, WireType::Y, _) |
                    // And must always output to intermediate:
                    (_, _, WireType::X | WireType::Y | WireType::Z) => {
                        println!(
                            "Found incorrect AND gate: {} AND {} -> {}",
                            device.id_assigner.original_value_of(w1),
                            device.id_assigner.original_value_of(w2),
                            device.id_assigner.original_value_of(wire_id as Wire),
                        );
                        incorrect_wires.push(wire_id as Wire)?;
                    }
                    _ => {
                        // AND must output to OR:
                        //   TODO: Note that wire_values[wire_id] is THIS gate, not the output gate
                        //if !matches!(device.wire_values[wire_id], Gate::Or(_, _)) {
                            //println!("Found incorrect AND gate (output not OR: {:?})", device.wire_values[wire_id]);
                        //}
                    }
                }
            }
        }
        let mut incorrect_names = Vec::<&str>::new();
        for &wire_id in incorrect_wires.slice() {
            let original_value = device.id_assigner.original_value_of(wire_id);
            incorrect_names.push(original_value);
        }
        incorrect_names.sort();
        return Ok(incorrect_names.join(","));
    }
}

#[derive(Clone, Copy, Debug)]
enum Gate {
    And(Wire, Wire),
    Or(Wire, Wire),
    Xor(Wire, Wire),
    ComputedValue(bool),
}

#[derive(Clone, Copy, Debug)]
enum WireType {
    X,
    Y,
    Z,
    Intermediate,
}

impl WireType {
    fn from_wire_name(wire_name: &str) -> Self {
        match wire_name.as_bytes()[0] {
            b'x' => WireType::X,
            b'y' => WireType::Y,
            b'z' => WireType::Z,
            _ => WireType::Intermediate,
        }
    }
}

struct Device<'a> {
    wire_values: [Gate; 1000],
    usage_counts: [u8; 1000],
    wire_types: [WireType; 1000],
    z_values: [Wire; 64],
    num_z_values: usize,
    id_assigner: IdAssigner<'a, 1024, str>,
}

impl<'a> Device<'a> {
    fn parse(input: &'a str) -> Self {
        let mut wire_values = [Gate::ComputedValue(false); 1000];
        let mut usage_counts = [0u8; 1000];
        let mut id_assigner = IdAssigner::<'a, 1024, str>::new("");
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

        for line in input.lines() {
            if let Some((wire_name, initial_value)) = line.split_once(": ") {
                let wire_id = id_assigner.id_of(wire_name).unwrap();
                let initial_value = initial_value.eq("1");
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
                /*
                println!("Parsed gate 1: {} {op_type} {} -> {}", p1_wire_name, p2_wire_name, to);
                println!("Parsed gate 2: {} {op_type} {} -> {}",
                id_assigner.original_value_of(p1_wire_id),
                id_assigner.original_value_of(p2_wire_id),
                id_assigner.original_value_of(to_wire_id));
                */
                //println!("Parsed gate: {:?} {op_type} {:?} -> {:?}", wire_types[p1_wire_id as usize], wire_types[p2_wire_id as usize], wire_types[to_wire_id as usize]);
                add_z_if(to, to_wire_id);
                wire_values[to_wire_id as usize] = match op_type {
                    "AND" => Gate::And(p1_wire_id, p2_wire_id),
                    "OR" => Gate::Or(p1_wire_id, p2_wire_id),
                    "XOR" => Gate::Xor(p1_wire_id, p2_wire_id),
                    _ => unreachable!("Strange op: '{op_type}'"),
                };

                usage_counts[p1_wire_id as usize] += 1;
                usage_counts[p2_wire_id as usize] += 1;
            }
        }

        Self {
            wire_values,
            wire_types,
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
    use crate::input::test_part_one_no_allocations;

    let test_input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
    test_part_one_no_allocations!(test_input => "4".to_string());
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
    test_part_one_no_allocations!(test_input => "2024".to_string());
    //test_part_two_no_allocations!(test_input => 0);

    let real_input = include_str!("day24_input.txt");
    test_part_one_no_allocations!(real_input => "65740327379952".to_string());
    test_part_two_no_allocations!(real_input => "TODO".to_string());
}

#[test]
pub fn tests_part2() {
    let real_input = include_str!("day24_input.txt");
    test_part_two_no_allocations!(real_input => "TODO".to_string());
}
