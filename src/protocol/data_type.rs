/**
mysql protocol basic type

doc: https://dev.mysql.com/doc/internals/en/integer.html

**/
extern crate byteorder;
use std::io::Result;
use std::io::Error;
use std::io;
use std::mem;

use self::byteorder::LittleEndian;
pub type Int1 = u8;
pub type Int2 = u16;
pub type Int3 = u32;
pub type Int4 = u32;
pub type Int8 = u64;

pub type Bytes = Vec<u8>;

pub const INT1MAX:Int1 = 0xfc -1;
pub const INT2MAX:Int2 = 0xffff -1;
pub const INT3MAX:Int3 = 0xffffff -1;
pub const INT4MAX:Int4 = 0xffffffff -1;
pub const INT8MAX:Int8 = 0xffffffffffffffff -1;

pub trait AsStr {
      fn as_str(&self) -> &'static str;
}

impl AsStr for Bytes {
     fn  as_str(&self) -> &'static str {
        unsafe{
            mem::transmute(self.iter().as_slice())
        }
    }
}

macro_rules! read_bytes {
    ($s:ident,$bits:expr,$t:ty) => {
        $s.read_uint::<LittleEndian>($bits).map(|num| num as $t)
    };
}

macro_rules! write_bytes {
    ($s:ident,$num:expr,$bits:expr) => {
        $s.write_uint::<LittleEndian>($num as u64,$bits)
    };
}

pub trait ReadBytesExt : byteorder::ReadBytesExt + io::Read{
    #[inline]
    fn read_int1(&mut self) -> Result<Int1> {
        read_bytes!(self,1,Int1)
    }
    #[inline]
    fn read_int2(&mut self) -> Result<Int2> {
        read_bytes!(self,2,Int2)
    }

    #[inline]
    fn read_int3(&mut self) -> Result<Int3> {
        read_bytes!(self,3,Int3)
    }
    
    #[inline]
    fn read_int4(&mut self) -> Result<Int4> {
        read_bytes!(self,4,Int4)
    }
    
    #[inline]
    fn read_int8(&mut self) -> Result<Int8> {
        read_bytes!(self,8,Int8)
    }
    
    #[inline]
    fn read_int_enclen(&mut self) -> Result<Int8> {
        self.read_int1()
            .and_then(|num|{
                println!("{}",num);
             match num as i32 {
                0 ... 0x00fb => {
                    Ok(num as Int8)
                },
                0xfc => {
                    self.read_int2().map(|num|{num as Int8})
                },
                 0xfd => {
                     self.read_int3().map(|num| {num as Int8})
                 },
                 0xfe => {
                     self.read_int8().map(|num|{num as Int8})
                 }
                 //0xff as the first byte of a length-encoded integer is undefined.
                 _ => Err(io::Error::new(io::ErrorKind::InvalidInput,"undefined behaviour"))
            }
        })
    }

    fn read_str_eof(&mut self) ->Result<Bytes> {
        let mut s = Vec::new();
        loop {
            match self.read_int1() {
                Ok(i) => {
                    if i == 0 {
                        break;
                    }
                    s.push(i);
                },
                Err(e) => return Err(e),
            }
        };
        Ok(s)
    }
    
    //Protocol::NulTerminatedBytesing
    fn read_str_nul(&mut self) -> Result<Bytes> {
        self.read_str_eof()
    }

    //Protocol::VariableLengthBytesing
    fn read_str_varlen(&mut self,len:Int8) -> Result<Bytes> {
        let mut s = Vec::new();
        for _ in 0..(len) {
           let r =  self.read_int1().map(|i|{
                s.push(i);
                i
           });
            match r {
                Err(e) => return Err(e),
                _ => {},
            }
        }
        Ok(s)
    }
    
    //Protocol::LengthEncodedBytesing
    fn read_str_enclen(&mut self) -> Result<Bytes> {
        self.read_int_enclen().and_then(|len| {
            self.read_str_varlen(len as Int8)
        })
    }

}

impl<R: io::Read + ?Sized> ReadBytesExt for R {}


pub trait WriteBytesExt : byteorder::WriteBytesExt + io::Write{

    fn write_int1(&mut self,num:Int1) -> Result<()> {
        write_bytes!(self,num,1)
    }

    #[inline]
    fn write_int2(&mut self,num:Int2) -> Result<()> {
        write_bytes!(self,num,2)
    }

    #[inline]
    fn write_int3(&mut self,num:Int3) -> Result<()> {
        write_bytes!(self,num,3)
    }

    #[inline]
    fn write_int4(&mut self,num:Int4) -> Result<()> {
        write_bytes!(self,num,4)
    }

    #[inline]
    fn write_int8(&mut self,num:Int8) -> Result<()> {
        write_bytes!(self,num,8)
    }

    #[inline]
    fn write_int_enclen(&mut self,num:Int8) -> Result<()> {
        if num <= INT1MAX as Int8 {
            self.write_int1(num as Int1)
        }else if num > INT1MAX as Int8 && num <= INT2MAX as Int8 {
            write_bytes!(self,0xfc,1).and(self.write_int2(num as Int2))
        }else if num > INT2MAX as Int8 && num <= INT3MAX as Int8 {
            write_bytes!(self,0xfd,1).and(self.write_int3(num as Int3))
        }else  {
            write_bytes!(self,0xfe,1).and(self.write_int8(num))
        }
    }
    
    #[inline]
    fn write_str_varlen(&mut self,data:&Bytes,num:Int8) -> Result<()> {

        if data.len() < (num as usize) {
            println!("debug {} {:?}",data.as_str(),data);
            return Err(Error::new(io::ErrorKind::InvalidInput,"num is lt str.len()"))
        }
        self.write_all(&(data)[0..(num as usize)])
    }

    #[inline]
    fn write_str_enclen(&mut self,data:&Bytes) -> Result<()> {
        self.write_int_enclen(data.len() as Int8)
            .and(
                self.write_all(&(data[0..(data.len() as usize)]))
            )
    }

    #[inline]
    fn write_str_eof(&mut self,data:&Bytes) -> Result<()> {

        let iter = data.iter();
        let mut is_eof = false;
        for elem in iter {
            if *elem == 0 {
                is_eof = true;
                break
            }
            self.write_int1(*elem as Int1);
        }
        if !is_eof {
            self.write_int1(0 as Int1);
        }
        Ok(())
    }

}

impl<R: io::Write + ?Sized> WriteBytesExt for R {}


pub fn calculate_str_enclen_bytes(len:Int8) -> Int8 {

    if len < INT1MAX as Int8 {
        return len  + 1;
    }
    if len < INT2MAX as Int8 {
        return len +2;
    }
    if len < INT3MAX as Int8 {
        return len + 3;
    }
    if len < INT4MAX as Int8 {
        return len + 4;
    }
    return len + 8;
}




