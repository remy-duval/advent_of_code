use commons::{err, Result, WrapErr};

pub const TITLE: &str = "Day 16: Packet Decoder";

pub fn run(raw: String) -> Result<()> {
    let data = parse(&raw)?;
    println!("1. Version sum: {}", first_part(data.clone())?);
    println!("2. Total value: {}", second_part(data)?);
    Ok(())
}

/// The stream of bits to decode (with an underlying byte array)
#[derive(Clone)]
struct BitStream {
    /// The next bytes to process
    bytes: std::vec::IntoIter<u8>,
    /// The current byte being processed
    current: u8,
    /// The last processed bit of the current byte, if 0 we need to pull the next byte
    pos: u8,
}

/// The header of a packet in the stream
struct Header {
    version: u8,
    packet_type: PacketType,
}

/// The kind of packet it is, if not literal this is a packet of packets
#[derive(Copy, Clone)]
enum PacketType {
    Sum,
    Product,
    Minimum,
    Maximum,
    Literal,
    GreaterThan,
    LesserThan,
    Equals,
}

impl PacketType {
    /// Use this packet type as an operator on two values
    fn reduce(self, a: u64, b: u64) -> u64 {
        match self {
            PacketType::Sum => a + b,
            PacketType::Product => a * b,
            PacketType::Minimum => a.min(b),
            PacketType::Maximum => a.max(b),
            PacketType::GreaterThan => (a > b) as u64,
            PacketType::LesserThan => (a < b) as u64,
            PacketType::Equals => (a == b) as u64,
            PacketType::Literal => 0, // Should never happen
        }
    }
}

/// A trait to make processing the stream generic
trait FoldStream {
    /// Called when a literal packet is seen
    fn on_literal(&mut self, header: Header, literal: u64);

    /// Called when an operator packet starts
    fn on_operator_start(&mut self, header: Header);

    /// Called when an operator packet finishes
    fn on_operator_end(&mut self);
}

/// Parse the byte array from the hex input
fn parse(s: &str) -> Result<BitStream> {
    let s = s.trim();
    let mut n = 0;
    let bytes = std::iter::from_fn(move || {
        let hex = s.get(n..(n + 2))?;
        n += 2;
        Some(u8::from_str_radix(hex, 16).wrap_err_with(|| format!("For '{hex}'")))
    })
    .collect::<Result<Vec<u8>>>()?
    .into_iter();

    Ok(BitStream {
        bytes,
        current: 0,
        pos: 0,
    })
}

/// Computes the sum of packet versions by folding the stream
fn first_part(mut stream: BitStream) -> Result<u64> {
    struct VersionSum(u64);

    impl FoldStream for VersionSum {
        fn on_literal(&mut self, header: Header, _: u64) {
            self.0 += header.version as u64;
        }

        fn on_operator_start(&mut self, header: Header) {
            self.0 += header.version as u64;
        }

        fn on_operator_end(&mut self) {}
    }

    let mut sum = VersionSum(0);
    fold_stream(&mut stream, &mut sum).map_err(|pos| err!("Failed at {}", pos))?;
    Ok(sum.0)
}

/// Compute the total of the operations in the packets by folding the stream
fn second_part(mut stream: BitStream) -> Result<u64> {
    #[derive(Default)]
    struct Decoder {
        in_progress: Vec<(PacketType, Option<u64>)>,
        result: Option<u64>,
    }

    impl Decoder {
        fn compute(&mut self, value: u64) {
            if let Some((op, acc)) = self.in_progress.last_mut() {
                *acc = match acc {
                    Some(prev) => Some(op.reduce(*prev, value)),
                    None => Some(value),
                };
            } else {
                self.result = Some(value);
            }
        }
    }

    impl FoldStream for Decoder {
        fn on_literal(&mut self, _: Header, literal: u64) {
            self.compute(literal);
        }

        fn on_operator_start(&mut self, header: Header) {
            self.in_progress.push((header.packet_type, None));
        }

        fn on_operator_end(&mut self) {
            if let Some(literal) = self.in_progress.pop().and_then(|(_, done)| done) {
                self.compute(literal);
            }
        }
    }

    let mut decoder = Decoder::default();
    fold_stream(&mut stream, &mut decoder).map_err(|pos| err!("Failed at {}", pos))?;
    Ok(decoder.result.unwrap_or_default())
}

/// Process all packets in the stream recursively, feeding them to a folder
///
/// Returns `Ok(read_bits)` if it succeeded or `Err(failed_at)` if it failed
fn fold_stream<F: FoldStream>(stream: &mut BitStream, fold: &mut F) -> Result<usize, usize> {
    let mut pos = 0;
    let header = stream.read_header().ok_or(pos)?;
    pos += 6;
    if matches!(header.packet_type, PacketType::Literal) {
        let mut literal = 0;
        loop {
            let (flag, block) = stream.read_bit().zip(stream.read_block(4)).ok_or(pos)?;
            literal = (literal << 4) + block as u64;
            pos += 5;
            // If the flag is 1, there are more blocks to read
            if flag == 0 {
                break;
            }
        }

        fold.on_literal(header, literal);
    } else {
        fold.on_operator_start(header);
        // The sub packet size is either in packet number or bit number
        if stream.read_bit().unwrap_or_default() == 0 {
            let mut remaining_bits = stream.read_block(15).ok_or(pos)? as usize;
            pos += 16;
            loop {
                let bits = fold_stream(stream, fold).map_err(|p| p + pos)?;
                pos += bits;
                if remaining_bits <= bits {
                    break;
                } else {
                    remaining_bits -= bits;
                }
            }
        } else {
            let packets = stream.read_block(11).ok_or(pos)?;
            pos += 12;
            for _ in 0..packets {
                pos += fold_stream(stream, fold).map_err(|p| p + pos)?;
            }
        }
        fold.on_operator_end();
    }

    Ok(pos)
}

impl BitStream {
    /// Read the next bit
    fn read_bit(&mut self) -> Option<u8> {
        self.pos = match self.pos.checked_sub(1) {
            Some(p) => p,
            None => {
                self.current = self.bytes.next()?;
                7
            }
        };
        // Isolate the bit at the wanted position and shift it down to O or 1
        Some((self.current & (1 << self.pos)) >> self.pos)
    }

    /// Read the next header
    fn read_header(&mut self) -> Option<Header> {
        let version = self.read_block(3)? as u8;
        let packet_type = match self.read_block(3)? {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Minimum,
            3 => PacketType::Maximum,
            4 => PacketType::Literal,
            5 => PacketType::GreaterThan,
            6 => PacketType::LesserThan,
            7 => PacketType::Equals,
            _ => return None,
        };
        Some(Header {
            version,
            packet_type,
        })
    }

    /// Read the next block of data
    fn read_block(&mut self, n: usize) -> Option<u16> {
        let mut sum = 0;
        for _ in 0..n {
            sum = (sum << 1) + self.read_bit()? as u16;
        }
        Some(sum)
    }
}

#[cfg(test)]
mod tests;
