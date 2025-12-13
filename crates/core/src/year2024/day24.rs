use crate::common::id_assigner::IdAssigner;
use crate::input::Input;

type Wire = u16;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut device = Device::parse(input.text);
    Ok(device.compute_z())
}

#[derive(Clone, Copy, Debug)]
enum Gate {
    And(Wire, Wire),
    Or(Wire, Wire),
    Xor(Wire, Wire),
    ComputedValue(bool),
}

struct Device {
    wire_values: [Gate; 1000],
    z_values: [Wire; 64],
    num_z_values: usize,
}

impl Device {
    fn parse(input: &str) -> Self {
        let mut id_assigner = IdAssigner::<1024, str>::new("");
        let mut z_values = [0; 64];
        let mut wire_values = [Gate::ComputedValue(false); 1000];
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
                let op_type = from_parts.next().unwrap();
                let p2_wire_name = from_parts.next().unwrap();
                let p2_wire_id = id_assigner.id_of(p2_wire_name).unwrap();
                let to_wire_id = id_assigner.id_of(to).unwrap();
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
            z_values,
            num_z_values,
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
    test_part_one_no_allocations!(test_input => 4);
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
    test_part_one_no_allocations!(test_input => 2024);
    //test_part_two_no_allocations!(test_input => 0);

    let real_input = include_str!("day24_input.txt");
    test_part_one_no_allocations!(real_input => 65_740_327_379_952);
    //test_part_two_no_allocations!(real_input => 0);
}
