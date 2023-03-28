mod observer;
mod sender; 
use sender::*; 
use observer::*;
use rand::prelude::*;
use iota_client::{MqttEvent, MqttPayload, Client, Result, Topic};
use etherparse::{Ipv4Header, TcpHeader, PacketBuilder};

pub const RNG: ThreadRng = rand::thread_rng();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut test_setup = Sender::new();
        test_setup.payload("test");
    }
}
