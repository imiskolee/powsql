use super::consts;
use super::packet::PacketStream;
use std::io::Cursor;


#[test]
fn test_packet_ok() {
    let mut buffer:[u8;11] =[0x07,0x00,0x00,0x02,0x00,0x00,0x00,0x02,0x00,0x00,0x00];
    let mut cursor = Cursor::new(&mut buffer[..]);
    let packet = cursor.read_raw_packet();
}
