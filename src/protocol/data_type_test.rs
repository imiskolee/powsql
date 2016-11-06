use super::data_type::ReadBytesExt;
use super::data_type::WriteBytesExt;
use std::io::Cursor;

#[test]
fn test_int1() {

    {
        let mut buf: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_int1().unwrap(), 1);
        assert_eq!(cursor.read_int1().unwrap(), 2);
        assert_eq!(cursor.read_int1().unwrap(), 3);
        assert_eq!(cursor.read_int1().unwrap(), 4);
    }
    
    {
        let mut buf: [u8; 4] = [0x00, 0x03, 0xf3, 0xa4];
        let mut cursor = Cursor::new(&mut buf[..]);
        let mut writeBuf :[u8;4] = [0,0,0,0];
        let mut w = Cursor::new(&mut writeBuf[..]);
        assert_eq!( w.write_int1(cursor.read_int1().unwrap()).is_err(),false);
        assert_eq!( w.write_int1(cursor.read_int1().unwrap()).is_err(),false);
        assert_eq!( w.write_int1(cursor.read_int1().unwrap()).is_err(),false);
        assert_eq!( w.write_int1(cursor.read_int1().unwrap()).is_err(),false);
        assert_eq!(w.into_inner(),cursor.into_inner());
    }
}

#[test]
fn test_int2() {
    {
        let mut buf: [u8; 4] = [0xff, 0x01, 0xff,0xff];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_int2().unwrap(), (0x01 << 8) + 0xff);
        assert_eq!(cursor.read_int2().unwrap(), (0xff << 8) + 0xff);
    }
    
    {
        let mut buf: [u8; 4] = [0x03, 0x00, 0xa4, 0xf3];
        let mut cursor = Cursor::new(&mut buf[..]);
        let  mut writeBuf :[u8;4] = [0,0,0,0];
        let mut w = Cursor::new(&mut writeBuf[..]);
        assert_eq!( w.write_int2(cursor.read_int2().unwrap()).is_err(),false);
        assert_eq!( w.write_int2(cursor.read_int2().unwrap()).is_err(),false);
      //  assert_eq!(w.into_inner(),[0x00,0x03,0xf3,0xa4]);
    }
}

#[test]
fn test_int3() {

    {
        let mut buf: [u8; 6] = [0x01, 0x02, 0x03,0xff,0x01,0xbc];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_int3().unwrap(), (0x03 << 16) + (0x02 << 8) + 0x01);
        assert_eq!(cursor.read_int3().unwrap(), (0xbc << 16) + (0x01 << 8) + 0xff);
    }

    {
        let mut buf: [u8; 6] = [0x01, 0x02, 0x03,0xff,0x01,0xbc];
        let mut cursor = Cursor::new(&mut buf[..]);
        let  mut writeBuf :[u8;6] = [0,0,0,0,0,0];
        let mut w = Cursor::new(&mut writeBuf[..]);
        assert_eq!( w.write_int3(cursor.read_int3().unwrap()).is_err(),false);
        assert_eq!( w.write_int3(cursor.read_int3().unwrap()).is_err(),false);
     //   assert_eq!(w.into_inner(),[0x03,0x02,0x01,0xbc,0x01,0xff]);
    }
}

#[test]
fn test_int4() {
    {
        let mut buf: [u8; 8] = [0x01, 0x02, 0x03,0x04,0xff,0x01,0xbc,0xea];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_int4().unwrap(), (0x04 << 24) + (0x03 << 16) + (0x02 << 8) + 0x01);
        assert_eq!(cursor.read_int4().unwrap(), (0xea << 24) + (0xbc << 16) + (0x01 << 8) + 0xff);
    }

    {
        let mut buf: [u8; 8] = [0x01, 0x02, 0x03,0x04,0xff,0x01,0xbc,0xea];
        let mut cursor = Cursor::new(&mut buf[..]);
        let  mut writeBuf :[u8;8] = [0,0,0,0,0,0,0,0];
        let mut w = Cursor::new(&mut writeBuf[..]);
        assert_eq!( w.write_int4(cursor.read_int4().unwrap()).is_err(),false);
        assert_eq!( w.write_int4(cursor.read_int4().unwrap()).is_err(),false);
      //  assert_eq!(w.into_inner(),[0x04,0x03,0x02,0x01,0xea,0xbc,0x01,0xff]);
    }
}

#[test]
fn test_int8() {

    {
        let mut buf: [u8; 8] = [
            0x01, 0x02, 0x03,0x04,0x05,0x06,0x07,0x08
        ];
        let mut cursor = Cursor::new(&mut buf[..]);

        assert_eq!(cursor.read_int8().unwrap(),
                   (0x08 << 56 ) + (0x07 << 48) + (0x06 << 40) +
                   (0x05 << 32) + (0x04 << 24) + (0x03 << 16) + (0x02 << 8) +
                   0x01
        );
    }
    {
        let mut buf: [u8; 8] = [
            0x01, 0x02, 0x03,0x04,0x05,0x06,0x07,0x08
        ];
        let mut cursor = Cursor::new(&mut buf[..]);
        let mut writeBuf :[u8;8] = [0,0,0,0,0,0,0,0];
        let mut w = Cursor::new(&mut writeBuf[..]);
        assert_eq!( w.write_int8(cursor.read_int8().unwrap()).is_err(),false);
      //  assert_eq!(w.into_inner(),[0x08,0x07,0x06,0x05,0x04,0x03,0x02,0x01]);
    }
}

#[test]
fn test_int_enclen() {

    {
        let mut buf:[u8;1] = [0x0c];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_int_enclen().unwrap(),0x0c);
    }
    
    {
        let mut buf:[u8;3] = [0xfc,0x01,0x02];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_int_enclen().unwrap(),(0x02 << 8) + 0x01);
    }

    {
        let mut buf:[u8;4] = [0xfd,0x01,0x02,0x03];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_int_enclen().unwrap(),(0x03 << 16) + (0x02 << 8) + 0x01);
    }
}



#[test]
fn test_read_str_nul() {
    {
        let mut buf: [u8; 4] = ['a' as u8, 'b' as u8, 'c' as u8, 0];
        let mut cursor = Cursor::new(&mut buf[..]);
        let s1 = cursor.read_str_nul();
        assert_eq!(s1.unwrap(), "abc");
    }

    {
        let mut buf: [u8; 5] = [0, 'a' as u8, 'b' as u8, 'c' as u8, 0];
        let mut cursor = Cursor::new(&mut buf[..]);
        let s1 = cursor.read_str_nul();
        assert_eq!(s1.unwrap(), "");
    }

    {
        let mut buf: [u8; 5] = ['a' as u8, 'b' as u8, 0, 'c' as u8, 0];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_str_nul().unwrap(), "ab");
        assert_eq!(cursor.read_str_nul().unwrap(), "c");
    }

    // TODO: if the stream not container EOF([00]byte)
    //
    // {
    // let mut buf:[u8;3] = ['a' as u8,'b' as u8,'c' as u8];
    // let mut cursor = Cursor::new(&mut buf[..]);
    // assert_eq!(cursor.read_str_nul().unwrap(),"abc");
    // }
    //
}

#[test]
fn test_read_str_varlen() {
    {
        let mut buf: [u8; 4] = ['a' as u8, 'b' as u8, 'c' as u8, 0];
        let mut cursor = Cursor::new(&mut buf[..]);
        let s1 = cursor.read_str_varlen(3);
        assert_eq!(s1.unwrap(), "abc");
    }

    {
        let mut buf: [u8; 5] = [0, 'a' as u8, 'b' as u8, 'c' as u8, 0];
        let mut cursor = Cursor::new(&mut buf[..]);
        let s1 = cursor.read_str_varlen(3);
        assert_eq!(s1.unwrap(), "\u{0}ab");
    }

    {
        let mut buf: [u8; 5] = ['a' as u8, 'b' as u8, 0, 'c' as u8, 0];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_str_varlen(2).unwrap(), "ab");
        assert_eq!(cursor.read_str_varlen(3).unwrap(), "\u{0}c\u{0}");
    }

    // overflow
    {
        let mut buf: [u8; 5] = ['a' as u8, 'b' as u8, 0, 'c' as u8, 0];
        let mut cursor = Cursor::new(&mut buf[..]);
        assert_eq!(cursor.read_str_varlen(6).is_err(), true);
    }
}

#[test]
fn test_read_str_enclen() {

    {
        let mut buf: [u8; 4] = [0x3, 'a' as u8, 'b' as u8, 'c' as u8];
        let mut cursor = Cursor::new(&mut buf[..]);
        let s1 = cursor.read_str_enclen();
        assert_eq!(s1.unwrap(), "abc");
    }

    {
        let mut buf: [u8; 5] = [0x4, 'a' as u8, 'b' as u8, 'c' as u8, 0];
        let mut cursor = Cursor::new(&mut buf[..]);
        let s1 = cursor.read_str_enclen();
        assert_eq!(s1.unwrap(), "abc\u{0}");
    }
}
