use std::fmt::{Display, Formatter};

use commons::{Result, WrapErr};

use super::int_code::{IntCodeInput, Processor, Status};

pub const TITLE: &str = "Day 23: Category Six";
const NETWORK_SIZE: usize = 50;

pub fn run(raw: String) -> Result<()> {
    let memory = parse(&raw)?.data;
    let output = run_until_nat_packet(&memory)
        .wrap_err("No NAT packet received, but the network has stopped")?;
    println!("First NAT packet was : {output}\n");

    println!("Starting network with NAT ON");
    let output = run_until_duplicate_wakeup(&memory)
        .wrap_err("The network stopped before sending twice the same wakeup")?;
    println!("Duplicate Y wake-up was : {output}");

    Ok(())
}

fn parse(s: &str) -> Result<IntCodeInput> {
    Ok(s.parse()?)
}

/// Runs the network for first part.
/// There is no NAT to handle packets to 255, and the first packet for it is returned.
fn run_until_nat_packet(memory: &[i64]) -> Option<Packet> {
    let mut computers: Vec<Processor> = (0..NETWORK_SIZE)
        .map(|port| Processor::with_initial_inputs(memory, &[port as i64]))
        .collect();

    loop {
        for n in 0..NETWORK_SIZE {
            match collect_packet(&mut computers[n]) {
                Ok(packet) => {
                    if let Err(not_sent) = send_packet(&mut computers, packet) {
                        return if not_sent.destination == 255 {
                            Some(not_sent)
                        } else {
                            println!("Error : unhandled packet {not_sent}");
                            None
                        };
                    }
                }
                Err(Status::RequireInput) => computers[n].write_int(-1),
                Err(_) => return None,
            }
        }
    }
}

/// Runs the network for the second part.
/// There is a NAT receiving packets for 255, and it will wake up 0 when network is idle.
fn run_until_duplicate_wakeup(memory: &[i64]) -> Option<Packet> {
    // We add 1 to a computer idle count when it asks for a packet but does not receive any.
    // When the computer produces a packet, its idle count is reset.
    // If the idle count is 2 or higher we take it to mean the computer is blocking on IO
    let mut idle_cycles: [u16; NETWORK_SIZE] = [0; NETWORK_SIZE];
    // The NAT sends the last packet it received to 0 when all states are idle
    let mut nat_buffer: Option<Packet> = None;
    // We want to collect the first Y value sent twice in a row by the NAT
    let mut last_wakeup: Option<Packet> = None;

    let mut computers: Vec<Processor> = (0..NETWORK_SIZE)
        .map(|port| Processor::with_initial_inputs(memory, &[port as i64]))
        .collect();

    loop {
        for n in 0..NETWORK_SIZE {
            match collect_packet(&mut computers[n]) {
                Ok(packet) => {
                    idle_cycles[n] = 0; // The computer sent a packet, so it is no longer idle.
                    if let Err(mut not_sent) = send_packet(&mut computers, packet) {
                        if not_sent.destination == 255 {
                            not_sent.destination = 0;
                            nat_buffer = Some(not_sent);
                        } else {
                            println!("Error : unhandled packet {not_sent}");
                            return None;
                        }
                    };
                }
                Err(Status::RequireInput) => {
                    idle_cycles[n] += 1; // This computer is considered idle for one cycle.
                    computers[n].write_int(-1)
                }
                Err(_) => return None,
            }
        }

        // Enter the NAT if the network is idle, i.e. all idles count are higher than 2
        if idle_cycles.iter().all(|&x| x >= 2) {
            // Send the last buffered packet to the computer 0
            let packet = nat_buffer.take()?;
            let recipient = computers.first_mut()?;
            recipient.write_int(packet.x);
            recipient.write_int(packet.y);
            println!("Wake-up ! {}", &packet);

            // Check if the previous one sent had the same Y, if it is we can return
            if let Some(previous) = last_wakeup.take() {
                if packet.y == previous.y {
                    return Some(packet);
                }
            }

            // Set the last wake-up to be the packet we just sent
            last_wakeup = Some(packet);
        }
    }
}

/// Collect one packet from a Processor as an Ok, or a blocking Status as an Err
fn collect_packet(computer: &mut Processor) -> Result<Packet, Status> {
    let destination = computer.read_next()?;
    let x = computer.read_next()?;
    let y = computer.read_next()?;
    Ok(Packet::new((destination, x, y)))
}

/// Send a packet to one of the computers as an Ok(()) or return it in an Err if it can't be sent
fn send_packet(computers: &mut [Processor], packet: Packet) -> Result<(), Packet> {
    let port = packet.destination;
    if 0 <= port && port < NETWORK_SIZE as i64 {
        let port = port as usize;
        let recipient = &mut computers[port];
        recipient.write_int(packet.x);
        recipient.write_int(packet.y);
        Ok(())
    } else {
        Err(packet)
    }
}

/// Represent a packet in the network.
#[derive(Debug, Clone, Eq, PartialEq)]
struct Packet {
    destination: i64,
    x: i64,
    y: i64,
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(x: {}, y: {}) for port {}",
            self.x, self.y, self.destination
        )
    }
}

impl Packet {
    fn new((destination, x, y): (i64, i64, i64)) -> Self {
        Self { destination, x, y }
    }
}

#[cfg(test)]
mod tests;
