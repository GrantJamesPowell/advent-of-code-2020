#![allow(dead_code)]

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    multi::{many0, many1},
    sequence::tuple,
    IResult,
};
mod common;

fn main() {
    let f = std::fs::read_to_string("./src/inputs/day16.txt").expect("file exists");
    let bits: Vec<u8> = f
        .lines()
        .flat_map(|line| line.chars().flat_map(|c| hex_to_bits(c)))
        .collect();
    let packets = packets(&bits);

    // Part 1
    println!("Day 16 pt. 1 Answer {:?}", packets[0].version_sum());

    // Part 2
    println!("Day 16 pt. 2 Answer {:?}", packets[0].eval());
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: usize,
    inner: PacketInner,
}

impl Packet {
    fn version_sum(&self) -> usize {
        let inner_sum = match &self.inner {
            Literal(_) => 0,
            Op { sub_packets, .. } => sub_packets.iter().map(|x| x.version_sum()).sum(),
        };

        inner_sum + self.version
    }

    fn eval(&self) -> usize {
        use Operation::*;

        match &self.inner {
            Literal(val) => *val,
            Op { sub_packets, op } => {
                let mut evaled_sub_packets = sub_packets.iter().map(|packet| packet.eval());
                match op {
                    Sum => evaled_sub_packets.sum(),
                    Product => evaled_sub_packets.product(),
                    Minimum => evaled_sub_packets.min().expect("at least one subpacket"),
                    Maximum => evaled_sub_packets.max().expect("at least one subpacket"),
                    LessThan => {
                        let first = evaled_sub_packets.next().expect("always two");
                        let second = evaled_sub_packets.next().expect("always two");
                        if first < second {
                            1
                        } else {
                            0
                        }
                    }
                    GreaterThan => {
                        let first = evaled_sub_packets.next().expect("always two");
                        let second = evaled_sub_packets.next().expect("always two");
                        if first > second {
                            1
                        } else {
                            0
                        }
                    }
                    EqualTo => {
                        let first = evaled_sub_packets.next().expect("always two");
                        let second = evaled_sub_packets.next().expect("always two");
                        if first == second {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum PacketInner {
    Literal(usize),
    Op {
        op: Operation,
        sub_packets: Vec<Packet>,
    },
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

use PacketInner::*;

fn packets(bits: &[u8]) -> Vec<Packet> {
    if let Ok((rest, packets)) = many0(packet)(bits) {
        println!("rest {:?}", rest);
        return packets;
    }

    panic!("failed to find packets")
}

fn packet(bits: &[u8]) -> IResult<&[u8], Packet> {
    let (rest, version) = version(bits)?;
    let (rest, inner) = alt((literal, op))(rest)?;

    Ok((rest, Packet { version, inner }))
}

fn op(bits: &[u8]) -> IResult<&[u8], PacketInner> {
    let (remaining, (type_id, length_type_id)) = tuple((take(3usize), take(1usize)))(bits)?;

    let op = match common::bits_as_u8s_to_bytes(type_id) {
        0 => Operation::Sum,
        1 => Operation::Product,
        2 => Operation::Minimum,
        3 => Operation::Maximum,
        5 => Operation::GreaterThan,
        6 => Operation::LessThan,
        7 => Operation::EqualTo,
        _ => panic!("invalid op code"),
    };

    let (remaining, sub_packets) = match length_type_id {
        [1] => {
            let (mut remaining, number_of_subpackets) = take(11usize)(remaining)?;
            let number_of_subpackets = common::bits_as_u8s_to_bytes(number_of_subpackets);

            let mut sub_packets = Vec::new();
            for _ in 0..number_of_subpackets {
                let (new_remaining, packet) = packet(remaining)?;
                sub_packets.push(packet);
                remaining = new_remaining;
            }

            (remaining, sub_packets)
        }
        [0] => {
            let (remaining, length_of_subpackets) = take(15usize)(remaining)?;
            let length_of_subpackets = common::bits_as_u8s_to_bytes(length_of_subpackets);
            let (remaining, packets) = take(length_of_subpackets)(remaining)?;
            let (extra, packets) = many1(packet)(packets)?;
            debug_assert!(extra.is_empty());
            (remaining, packets)
        }
        _ => panic!("invalid length type id"),
    };

    Ok((remaining, Op { op, sub_packets }))
}

fn literal(bits: &[u8]) -> IResult<&[u8], PacketInner> {
    let inner_num = tuple((tag(&[1]), take(4usize)));
    let final_num = tuple((tag(&[0]), take(4usize)));

    let (remaining, (_tag, leading_nums, final_num)) =
        tuple((tag(&[1, 0, 0]), many0(inner_num), final_num))(bits)?;

    let mut num_bits = Vec::with_capacity(leading_nums.len() + 1);

    for (tag, bits) in leading_nums {
        debug_assert_eq!(&tag, &[1]);
        num_bits.extend(bits)
    }

    let (tag, bits) = final_num;
    debug_assert_eq!(tag, &[0]);
    num_bits.extend(bits);

    Ok((remaining, Literal(common::bits_as_u8s_to_bytes(&num_bits))))
}

fn version(input: &[u8]) -> IResult<&[u8], usize> {
    (take(3usize))(input)
        .map(|(remaining, version)| (remaining, common::bits_as_u8s_to_bytes(version)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let input = [1, 1, 0];

        assert_eq!(version(&input), Ok(([].as_slice(), 6)))
    }

    #[test]
    fn test_op_with_sub_packet_num() {
        let input = [
            1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ];

        assert_eq!(
            packet(&input),
            Ok((
                [0, 0, 0, 0, 0].as_slice(),
                Packet {
                    version: 7,
                    inner: Op {
                        op: Operation::Maximum,
                        sub_packets: vec![
                            Packet {
                                version: 2,
                                inner: Literal(1)
                            },
                            Packet {
                                version: 4,
                                inner: Literal(2)
                            },
                            Packet {
                                version: 1,
                                inner: Literal(3)
                            }
                        ]
                    }
                }
            ))
        )
    }

    #[test]
    fn test_op_with_sub_packet_len() {
        let input = [
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];

        assert_eq!(
            packet(&input),
            Ok((
                [0, 0, 0, 0, 0, 0, 0].as_slice(),
                Packet {
                    version: 1,
                    inner: Op {
                        op: Operation::LessThan,
                        sub_packets: vec![
                            Packet {
                                version: 6,
                                inner: Literal(10)
                            },
                            Packet {
                                version: 2,
                                inner: Literal(20)
                            }
                        ]
                    }
                }
            ))
        );
    }

    #[test]
    fn test_literal_packets() {
        let input = [
            1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,
        ];
        assert_eq!(
            packet(&input),
            Ok((
                [0u8, 0, 0].as_slice(),
                Packet {
                    version: 6,
                    inner: Literal(2021)
                }
            ))
        )
    }
}

fn hex_to_bits(c: char) -> [u8; 4] {
    match c {
        '0' => [0, 0, 0, 0],
        '1' => [0, 0, 0, 1],
        '2' => [0, 0, 1, 0],
        '3' => [0, 0, 1, 1],
        '4' => [0, 1, 0, 0],
        '5' => [0, 1, 0, 1],
        '6' => [0, 1, 1, 0],
        '7' => [0, 1, 1, 1],
        '8' => [1, 0, 0, 0],
        '9' => [1, 0, 0, 1],
        'A' => [1, 0, 1, 0],
        'B' => [1, 0, 1, 1],
        'C' => [1, 1, 0, 0],
        'D' => [1, 1, 0, 1],
        'E' => [1, 1, 1, 0],
        'F' => [1, 1, 1, 1],
        other => panic!("Unexpected hex digit {:?}", other),
    }
}
