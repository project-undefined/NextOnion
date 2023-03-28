

use crate::*; 

pub struct Sender { 
    server_ip:   Option<[u8; 4]>,
    server_port: Option<u32> 
}

impl Sender {
    
    pub fn new() ->  Sender { 
        Sender {
            server_ip:   Option::None,
            server_port: Option::None,
        }
    }

    // create a new IP with a unique fake identifier
    pub fn payload(mut self, pl: &str) {
    
        let mut ip_header = Ipv4Header::new(
            pl.len() as u16, 
            40, 
            0, 
            Self::ip_gen(), 
            self.server_ip.expect("[SENDER]: Server IP not set")
        );

        // setup the fake TPC packet
        let mut tcp_header = TcpHeader::new(
            //generate a fake source port
               Self::port_gen(), 
               // get the destination server port
          self.server_port.expect("[SENDER]: Server port not set"),
            0,  // sequence number is irrelivant, since the client will not be recieving ACK or anything else
                500 // window size is irrelivant, since the client will not be recieving any data through this conneciton 
        );

        

    
    
    }

    // Randomly generate a new Ip for each new IP packet
    fn ip_gen() -> [u8; 4] {
        let ip: [u8; 4] = [0,0,0,0];
        ip[0] = RNG.gen_range(2..217);
        ip[1] = RNG.gen_range(1..197);
        ip[2] = RNG.gen_range(1..193);
        ip[3] = RNG.gen_range(0..254);
        ip

    }

    // Generate a new random port
    fn port_gen() -> u32 {
        RNG.gen_range(1111..8888)
    }

}