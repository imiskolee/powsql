use super::handshake::{HandshakeStream,HandshakeV10,HandshakeResponse41};
use std::net::TcpStream;
use std::io::Cursor;
use super::data_type::AsStr;
use super::consts;

/*
#[test]
fn test_handshakev10_no_auth() {

    let mut buffer:[u8;58] =
        [0x36,0x00,0x00,0x00,
         0x0a,
         0x35,0x2e,0x35,0x2e,0x32,0x2d,0x6d,0x32,0x00,
         0x0b,0x00,0x00,0x00,
         0x64,0x76,0x48,0x40,0x49,0x2d,0x43,0x4a,
         0x00,
         0xff,0xf7,
         0x08,
         0x02,0x00,
         0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
         0x00,0x00,0x00,0x00,0x00,0x2a,0x34,0x64,
         0x7c,0x63,0x5a,0x77,0x6b,0x34,0x5e,0x5d,
         0x3a,0x00
        ];

    let mut cursor = Cursor::new(&mut buffer[..]);
    let packet = cursor.read_handshake_v10().unwrap();

    assert_eq!(packet.protocol_version,0x0a);
    assert_eq!(packet.server_version,"5.5.2-m2");
    assert_eq!(packet.connection_id,11);
    assert_eq!(packet.auth_plugin_data_part_1,"dvH@I-CJ");
    assert_eq!(packet.filler,0);
    assert_eq!(packet.charset,195);
    assert_eq!(packet.status_flags,2231);
    assert_eq!(packet.auth_plugin_data_len,0);
    assert_eq!(packet.reserved,"\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}");
    assert_eq!(packet.auth_plugin_data_part_2,"");
    assert_eq!(packet.auth_plugin_name,"");
}
*/



#[test]
fn test_handshakev10_with_auth() {

    let mut buffer:[u8;78] =
        [74,0,0,0,
         10,
         53,46,55,46,49,54,0,
         67,0,0,0,
         51,99,101,104,119,1,97,22,
         0,
         255,247,
         8,
         2,0,
         255,129,
         21,
         0,0,0,0,0,0,0,0,0,0,
         64,84,93,86,72,9,97,60,119,22,98,68,0,109,121,115,113,108,95,110,97,116,105,118,101,95,112,97,115,115,119,111,114,100,0];
    
    let mut cursor = Cursor::new(&mut buffer[..]);


    let packet = cursor.read_handshake_v10().unwrap();

    println!("{:?}",packet);

    assert_eq!(packet.protocol_version,0x0a);
    assert_eq!(packet.server_version.as_str(),"5.7.16");
    assert_eq!(packet.connection_id,67);
    assert_eq!(packet.auth_plugin_data_part_1.as_str(),"3cehw\u{1}a\u{16}");
    assert_eq!(packet.filler,0);
    assert_eq!(packet.charset,8);
    assert_eq!(packet.status_flags,2);
    assert_eq!(packet.auth_plugin_data_len,21);
    assert_eq!(packet.reserved.as_str(),"\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}\u{0}");
    assert_eq!(packet.auth_plugin_data_part_2.as_str(),"@T]VH\ta<w\u{16}bD\u{0}");
    assert_eq!(packet.auth_plugin_name.as_str(),"mysql_native_password");

    let mut buffer2 = Vec::with_capacity(78);
    unsafe{
        buffer2.set_len(78);
    }
    let mut cursor2 = Cursor::new(&mut buffer2[..]);

    let ret = cursor2.write_handshake_v10(&packet);

    assert_eq!(*cursor.get_ref(),*cursor2.get_ref())
}


#[test]
fn test_handshake_response_41() {

    let mut buffer:[u8;182] =
        [
0xb2,0x00,0x00,0x01,0x85,0xa2,0x1e,0x00,0x00,0x00,0x00,0x40,0x08,0x00,0x00,0x00
,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00
,0x00,0x00,0x00,0x00,0x72,0x6f,0x6f,0x74,0x00,0x14,0x22,0x50,0x79,0xa2,0x12,0xd4
,0xe8,0x82,0xe5,0xb3,0xf4,0x1a,0x97,0x75,0x6b,0xc8,0xbe,0xdb,0x9f,0x80,0x6d,0x79
,0x73,0x71,0x6c,0x5f,0x6e,0x61,0x74,0x69,0x76,0x65,0x5f,0x70,0x61,0x73,0x73,0x77
,0x6f,0x72,0x64,0x00,0x61,0x03,0x5f,0x6f,0x73,0x09,0x64,0x65,0x62,0x69,0x61,0x6e
,0x36,0x2e,0x30,0x0c,0x5f,0x63,0x6c,0x69,0x65,0x6e,0x74,0x5f,0x6e,0x61,0x6d,0x65
,0x08,0x6c,0x69,0x62,0x6d,0x79,0x73,0x71,0x6c,0x04,0x5f,0x70,0x69,0x64,0x05,0x32
,0x32,0x33,0x34,0x34,0x0f,0x5f,0x63,0x6c,0x69,0x65,0x6e,0x74,0x5f,0x76,0x65,0x72
,0x73,0x69,0x6f,0x6e,0x08,0x35,0x2e,0x36,0x2e,0x36,0x2d,0x6d,0x39,0x09,0x5f,0x70
,0x6c,0x61,0x74,0x66,0x6f,0x72,0x6d,0x06,0x78,0x38,0x36,0x5f,0x36,0x34,0x03,0x66
,0x6f,0x6f,0x03,0x62,0x61,0x72
         ];
    
    let mut cursor = Cursor::new(&mut buffer[..]);

    let resp = cursor.read_handshake_response_41();

    
    let mut buffer2 = Vec::with_capacity(182);
    unsafe{
        buffer2.set_len(182);
    }
    let response = resp.unwrap();
    let mut cursor2 = Cursor::new(&mut buffer2[..]);
    let res = cursor2.write_handshake_response_41(&response);
    assert_eq!(res.is_ok(),true);
}



/*
#[test]
fn test_handshakev10_real() {

    let mut stream = TcpStream::connect("127.0.0.1:3306").unwrap();
    let packet = stream.read_handshake_v10().unwrap();
    println!("my packet{:?}",packet);
}
*/
