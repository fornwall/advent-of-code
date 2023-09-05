use crate::common::id_assigner::IdAssigner;
use crate::input::Input;

type ChemicalId = usize;
type ChemicalAmount = i64;

struct Reactions {
    needed: Vec<ChemicalAmount>,
    // Indexed by chemical id that is produced, contains amount produced and required.
    produced_by: Vec<(ChemicalAmount, Vec<ChemicalAmount>)>,
    fuel_id: ChemicalId,
    ore_id: ChemicalId,
}

impl Reactions {
    fn parse(input_string: &str) -> Result<Self, String> {
        let mut id_assigner = IdAssigner::<100>::new();

        // Indexed by chemical id that is produced, to amount produced and required.
        let mut reactions: Vec<(ChemicalAmount, Vec<ChemicalAmount>)> = Vec::new();

        for (line_index, line) in input_string.lines().enumerate() {
            let error = || format!("Invalid input line {}", line_index + 1);

            // Example: "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ".
            let (from, to) = line.split_once("=>").ok_or_else(error)?;

            let mut required_chemicals = Vec::new();
            for amount_and_name in from.split(',') {
                let (amount, name) = amount_and_name.trim().split_once(' ').ok_or_else(error)?;

                let required_amount = amount.parse::<ChemicalAmount>().map_err(|_| error())?;
                let required_id = id_assigner.id_of(name.trim())? as usize;
                if required_chemicals.len() <= required_id {
                    required_chemicals.resize(required_id + 1, 0);
                }
                required_chemicals[required_id] = required_amount;
            }

            let (amount_str, name) = to.trim().split_once(' ').ok_or_else(error)?;
            let produced_chemical_amount =
                amount_str.parse::<ChemicalAmount>().map_err(|_| error())?;
            let produced_chemical_name = name.trim();
            let produced_chemical_id = id_assigner.id_of(produced_chemical_name)? as usize;

            if reactions.len() <= produced_chemical_id {
                reactions.resize_with(produced_chemical_id + 1, || (0, Vec::new()));
            }
            reactions[produced_chemical_id] = (produced_chemical_amount, required_chemicals);
        }

        let fuel_id = id_assigner.get_id("FUEL").ok_or("No FUEL encountered")? as usize;
        let ore_id = id_assigner.get_id("ORE").ok_or("No ORE encountered")? as usize;

        Ok(Self {
            needed: vec![0; id_assigner.len()],
            produced_by: reactions,
            fuel_id,
            ore_id,
        })
    }
}

fn required_ore(reactions: &mut Reactions, fuel_to_produce: ChemicalAmount) -> ChemicalAmount {
    reactions.needed.fill(0);

    reactions.needed[reactions.fuel_id] = fuel_to_produce;

    while let Some((needed_id, &needed_amount)) = reactions
        .needed
        .iter()
        .enumerate()
        .find(|&(chemical_id, &amount)| amount > 0 && chemical_id != reactions.ore_id)
    {
        let (produced_amount, required) = &reactions.produced_by[needed_id];

        let reaction_executions =
            needed_amount / *produced_amount + i64::from(needed_amount % *produced_amount != 0);

        reactions.needed[needed_id] -= reaction_executions * *produced_amount;

        for (required_id, &required_amount) in required.iter().enumerate() {
            reactions.needed[required_id] += reaction_executions * required_amount;
        }
    }

    reactions.needed[reactions.ore_id]
}

pub fn solve(input: &Input) -> Result<ChemicalAmount, String> {
    const AVAILABLE_ORE: i64 = 1_000_000_000_000;
    let mut reactions = Reactions::parse(input.text)?;

    if input.is_part_one() {
        Ok(required_ore(&mut reactions, 1))
    } else {
        let mut min_produced_fuel = 1;
        let mut max_produced_fuel = AVAILABLE_ORE;
        loop {
            let fuel_to_produce = (max_produced_fuel + min_produced_fuel) / 2;
            if fuel_to_produce == min_produced_fuel {
                return Ok(min_produced_fuel);
            }

            let ore_amount = required_ore(&mut reactions, fuel_to_produce);

            if ore_amount > AVAILABLE_ORE {
                // Uses too much ore, try less ambitious fuel production.
                max_produced_fuel = fuel_to_produce;
            } else {
                // Within our ore budget, try a higher fuel production.
                min_produced_fuel = fuel_to_produce;
            }
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL" => 165);

    test_part_one!("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT" => 13312);

    test_part_two!("157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT" => 82_892_753);

    let input = include_str!("day14_input.txt");
    test_part_one!(input => 1_590_844);
    test_part_two!(input => 1_184_209);
}
