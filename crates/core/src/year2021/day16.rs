use crate::input::Input;

/// Constructs a u16 with the lowest n bits set.
///
/// Does not work for n == 0.
const fn set_lowest_bits(n: u8) -> u8 {
    u8::MAX >> (u8::BITS as u8 - n)
}

fn bit_value(character: u8) -> u8 {
    if character.is_ascii_digit() {
        character - b'0'
    } else {
        if !(b'A'..=b'F').contains(&character) {
            return 0;
        }
        0b1010 + (character - b'A')
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Transmission<'a> {
    bit_offset: usize,
    hex_bytes: &'a [u8],
}

impl<'a> Transmission<'a> {
    const fn new(hex_bytes: &'a [u8]) -> Self {
        Self {
            bit_offset: 0,
            hex_bytes,
        }
    }

    fn next_bits(&mut self, num_bits: usize) -> Option<u16> {
        let mut remaining_bits = num_bits;
        let mut result = 0_u16;
        while remaining_bits > 0 {
            let hex_byte_offset = self.bit_offset / 4;
            let bits_left_in_byte = 4 - (self.bit_offset % 4);
            let bits_to_read = std::cmp::min(remaining_bits, bits_left_in_byte);
            let hex_byte_value = bit_value(*self.hex_bytes.get(hex_byte_offset)?);
            remaining_bits -= bits_to_read;
            result |=
                // First shift to have interesting bits as lowest bits
                u16::from((hex_byte_value >> (bits_left_in_byte - bits_to_read))
                    // Mask out the relevant bits:
                    & set_lowest_bits(bits_to_read as u8))
                    // Now shift them to relevant position
                    << (remaining_bits as u16);
            self.bit_offset += bits_to_read;
        }
        Some(result)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum LengthOrValue {
    Value(u64),
    TotalBitLength(u16),
    NumPackets(u16),
}

impl LengthOrValue {
    const fn expects_more(self) -> bool {
        match self {
            Self::TotalBitLength(v) | Self::NumPackets(v) => v > 0,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    contents: LengthOrValue,
    length: u8,
}

impl Packet {
    fn parse(transmission: &mut Transmission) -> Option<Self> {
        let initial_bit_offset = transmission.bit_offset;

        let version = transmission.next_bits(3)?;
        let type_id = transmission.next_bits(3)?;
        let length_or_value = if type_id == 4 {
            // Literal value packet - a single binary number.
            let mut value = 0_u64;
            let mut bit_offset = 0_usize;
            loop {
                let next_five_bits = transmission.next_bits(5)?;
                value |= u64::from(next_five_bits & 0b1111) << (bit_offset as u64);
                bit_offset += 4;
                if bit_offset > 60 {
                    return None;
                }
                if next_five_bits & 0b10000 == 0 {
                    let mut new_value = 0;
                    while bit_offset != 0 {
                        new_value |= (value & 0b1111) << (bit_offset - 4);
                        value >>= 4;
                        bit_offset -= 4;
                    }
                    break LengthOrValue::Value(new_value);
                }
            }
        } else {
            // Operator packet that performs some calculation on one or more sub-packets contained within.
            let next_bit = transmission.next_bits(1)?;
            if next_bit == 0 {
                // "If the length type ID is 0, then the next 15 bits are a number that represents
                // the total length in bits of the sub-packets contained by this packet."
                LengthOrValue::TotalBitLength(transmission.next_bits(15)?)
            } else {
                // "If the length type ID is 1, then the next 11 bits are a number that represents
                // the number of sub-packets immediately contained by this packet."
                LengthOrValue::NumPackets(transmission.next_bits(11)?)
            }
        };
        Some(Self {
            version: version as u8,
            type_id: type_id as u8,
            contents: length_or_value,
            length: (transmission.bit_offset - initial_bit_offset) as u8,
        })
    }

    fn parse_and_eval(transmission: &mut Transmission) -> Option<u64> {
        let current_package = Self::parse(transmission)?;

        let mut remaining_length = match current_package.contents {
            LengthOrValue::Value(value) => {
                return Some(value);
            }
            _ => current_package.contents,
        };

        let mut current_value = if current_package.type_id == 1 {
            1
        } else if current_package.type_id == 2 {
            u64::MAX
        } else {
            0
        };

        while remaining_length.expects_more() {
            let bit_offset_at_start_of_sub_packets = transmission.bit_offset;
            let next_package_value = Self::parse_and_eval(transmission)?;
            match current_package.type_id {
                0 => {
                    current_value = current_value.checked_add(next_package_value)?;
                }
                1 => {
                    current_value = current_value.checked_mul(next_package_value)?;
                }
                2 => {
                    current_value = std::cmp::min(next_package_value, current_value);
                }
                3 => {
                    current_value = std::cmp::max(next_package_value, current_value);
                }
                5 => {
                    let second_package_value = Self::parse_and_eval(transmission)?;
                    return Some(u64::from(next_package_value > second_package_value));
                }
                6 => {
                    let second_package_value = Self::parse_and_eval(transmission)?;
                    return Some(u64::from(next_package_value < second_package_value));
                }
                7 => {
                    let second_package_value = Self::parse_and_eval(transmission)?;
                    return Some(u64::from(next_package_value == second_package_value));
                }
                _ => {
                    return None;
                }
            }

            remaining_length = match remaining_length {
                LengthOrValue::NumPackets(value) => LengthOrValue::NumPackets(value - 1),
                LengthOrValue::TotalBitLength(value) => {
                    let consumed_bit_length =
                        transmission.bit_offset - bit_offset_at_start_of_sub_packets;
                    if consumed_bit_length > usize::from(u16::MAX)
                        || consumed_bit_length as u16 > value
                    {
                        return None;
                    }
                    LengthOrValue::TotalBitLength(value - consumed_bit_length as u16)
                }
                _ => {
                    return None;
                }
            };
        }
        Some(current_value)
    }
}

pub fn solve(input: &Input) -> Result<u64, String> {
    let hex_bytes = input.text.as_bytes();
    let mut transmission = Transmission::new(hex_bytes);
    if input.is_part_one() {
        let mut version_sum = 0_u64;
        while let Some(packet) = Packet::parse(&mut transmission) {
            version_sum += u64::from(packet.version);
        }
        Ok(version_sum)
    } else {
        Packet::parse_and_eval(&mut transmission)
            .ok_or_else(|| "Unable to parse outermost package - check transmission".to_string())
    }
}

#[test]
pub fn test_packet_parsing() {
    // First example:
    let hex_bytes = b"D2FE28";
    let mut transmission = Transmission::new(hex_bytes);
    assert_eq!(
        Some(Packet {
            version: 6,
            type_id: 4,
            contents: LengthOrValue::Value(2021),
            length: 21
        }),
        Packet::parse(&mut transmission)
    );
    assert!(Packet::parse(&mut transmission).is_none());

    // Second example:
    let hex_bytes = b"38006F45291200";
    let mut transmission = Transmission::new(hex_bytes);
    assert_eq!(
        Some(Packet {
            version: 1,
            type_id: 6,
            contents: LengthOrValue::TotalBitLength(27),
            length: 22
        }),
        Packet::parse(&mut transmission)
    );
    assert_eq!(
        Some(Packet {
            version: 0b110,
            type_id: 0b100,
            contents: LengthOrValue::Value(10),
            length: 11,
        }),
        Packet::parse(&mut transmission)
    );
    assert_eq!(
        Some(Packet {
            version: 0b010,
            type_id: 0b100,
            contents: LengthOrValue::Value(20),
            length: 16,
        }),
        Packet::parse(&mut transmission)
    );
    assert!(Packet::parse(&mut transmission).is_none());

    // Third example:
    let hex_bytes = b"EE00D40C823060";
    let mut transmission = Transmission::new(hex_bytes);
    assert_eq!(
        Some(Packet {
            version: 7,
            type_id: 3,
            contents: LengthOrValue::NumPackets(3),
            length: 18,
        }),
        Packet::parse(&mut transmission)
    );
    assert_eq!(
        Some(Packet {
            version: 0b010,
            type_id: 0b100,
            contents: LengthOrValue::Value(1),
            length: 11,
        }),
        Packet::parse(&mut transmission)
    );
    assert_eq!(
        Some(Packet {
            version: 0b100,
            type_id: 0b100,
            contents: LengthOrValue::Value(2),
            length: 11,
        }),
        Packet::parse(&mut transmission)
    );
    assert_eq!(
        Some(Packet {
            version: 0b001,
            type_id: 0b100,
            contents: LengthOrValue::Value(3),
            length: 11,
        }),
        Packet::parse(&mut transmission)
    );
    assert!(Packet::parse(&mut transmission).is_none());
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two, test_part_two_error};

    let real_input = include_str!("day16_input.txt");

    test_part_one!("8A004A801A8002F478" => 16);
    test_part_one!("620080001611562C8802118E34" => 12);
    test_part_one!("C0015000016115A2E0802F182340" => 23);
    test_part_one!("A0016C880162017C3686B18A3D4780" => 31);
    test_part_one!(real_input => 960);

    test_part_two!("C200B40A82" => 3);
    test_part_two!("04005AC33890" => 54);
    test_part_two!("880086C3E88112" => 7);
    test_part_two!("CE00C43D881120" => 9);
    test_part_two!("D8005AC2A8F0" => 1);
    test_part_two!("F600BC2D8F" => 0);
    test_part_two!("9C005AC2F8F0" => 0);
    test_part_two!("9C0141080250320F1802104A08" => 1);
    test_part_two!(real_input => 12_301_926_782_560);

    #[cfg(feature = "count-allocations")]
    {
        let allocations = allocation_counter::measure(|| {
            test_part_one!(real_input => 960);
            test_part_two!(real_input => 12_301_926_782_560);
        });
        assert_eq!(allocations.count_total, 0);
    }

    test_part_two_error!("b1/       5105 	   |" => "Unable to parse outermost package - check transmission");
}
