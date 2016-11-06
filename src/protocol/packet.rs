use super::data_type;
use super::data_type::{ReadBytesExt, WriteBytesExt, Int1, Int2,Int3, Int8, Bytes};
use super::consts;
use std::io::{Result,Cursor,Error,ErrorKind,Read,Write};

pub struct PacketRaw {
    pub payload_len:Int3,
    pub sequence_id:Int1,
    pub payload:Cursor<Vec<u8>>,
}

impl PacketRaw {

  pub fn read_packet(&mut self,flag:consts::ProtocolConst) -> Result<PacketOption> {
        let mut packet = PacketOption::None;
        // parse raw packet
     
        //the EOF packet may appear in places where a Protocol::LengthEncodedInteger may appear. You must check whether the packet length is less than 9 to make sure that it is a EOF packet.
        //https://dev.mysql.com/doc/internals/en/packet-EOF_Packet.html
        if self.payload_len < 9 {
            return Ok(PacketOption::Ok(Default::default()));
        }
      self.payload.read_int1().and_then(|first_byte| {

            let common_err = Err(Error::new(ErrorKind::InvalidInput,"unsupprtoed packet type"));

            match first_byte {
                // Ok Packet
                PACKET_HEDAER_OK => {
                    let p = PacketOK::from_reader(flag,&mut  self.payload);
                    match p {
                        Ok(v) => Ok(PacketOption::Ok(v)),
                        _ => common_err,
                    }
                }
                // EOF packet
                PACKET_HEADER_EOF => {
                    let p = PacketEOF::from_reader(flag,&mut  self.payload);
                    match p {
                        Ok(v) => Ok(PacketOption::Eof(v)),
                        _ => common_err,
                    }
                }
                // ERR  Packet
                PACKET_HEADER_ERR => {
                    let p = PacketERR::from_reader(flag,&mut  self.payload);
                    match p {
                        Ok(v) => Ok(PacketOption::Err(v)),
                        _ => common_err,
                    }
                }
                _ => {
                    common_err
                }
            };
            Ok(PacketOption::None)
        })
    }

    pub fn write_packet_ok(&mut self, flag:consts::ProtocolConst,packet: PacketOK) -> Result<()> {
        packet.write(flag,&mut self.payload)
    }

    pub fn write_packet_err(&mut self,flag:consts::ProtocolConst, packet: PacketERR) -> Result<()> {
        packet.write(flag,&mut self.payload)
    }

    pub fn write_packet_eof(&mut self, flag:consts::ProtocolConst,packet: PacketEOF) -> Result<()> {
        packet.write(flag,&mut self.payload)
    }
}



pub struct PacketOK {
    header:Int1,
    pub affected_rows:Int8,
    pub last_insert_id:Int8,
    pub status_flags:Int2,
    pub warnings:Int2,
    pub info: Bytes,
    pub session_state_changes: Bytes
}

pub struct PacketERR {
    header:Int1,
    pub  error_code:Int2,
    pub sql_state_marker: Bytes,
    pub sql_state: Bytes,
    pub error_message: Bytes
}

pub struct PacketEOF {
    header: Int1,
    pub warnings:Int2,
    pub status_flags:Int2
}

const PACKET_HEDAER_OK:Int1 = 0x00;
const PACKET_HEADER_EOF:Int1 = 0xfe;
const PACKET_HEADER_ERR:Int1 = 0xff;

pub enum PacketOption {
    Ok(PacketOK),
    Err(PacketERR),
    Eof(PacketEOF),
    None,
}



// TODO: define macro to mapping common opts
impl PacketOption {

    /// Returns true if the result is `PacketOption::Ok`,otherwise returns false.
    #[inline]
    pub fn is_ok(self) -> bool {
        match self {
            PacketOption::Ok(_) => true,
            _ => false,
        }
    }

    /// Returns true if the result is `PacketOption::Err`,otherwise returns false.
    #[inline]
    pub fn is_err(self) -> bool {
        match self {
            PacketOption::Err(_) => true,
            _ => false,
        }
    }
    /// Returns true if the result is `PacketOption::Eof`,otherwise returns false.
    #[inline]
    pub fn is_eof(self) -> bool {
        match self {
            PacketOption::Eof(_) => true,
            _ => false,
        }
    }

    /// Returns true if the result is `PacketOption::None`,otherwise returns false.
    #[inline]
    pub fn is_none(self) -> bool {
        match self {
            PacketOption::None => true,
            _ => false,
        }
    }
    
    //Panics if the value not `PacketOption::Ok`.
    //Returns `PacketOk` if the value is `PacketOption::Ok`
    #[inline]
    pub fn unwrap_ok(self) -> PacketOK {
        match self {
            PacketOption::Ok(v) => v,
            _ => panic!("{}", "Called `PacketOption::unwrap_ok()` failed."),
        }
    }

    //Panics if the value not `PacketOption::Err`.
    //Returns `PacketOk` if the value is `PacketOption::Err`
    #[inline]
    pub fn unwrap_err(self) -> PacketERR {
        match self {
            PacketOption::Err(v) => v,
            _ => panic!("{}", "Called `PacketOption::unwrap_ok()` failed."),
        }
    }

    //Panics if the value not `PacketOption::Eof`.
    //Returns `PacketOk` if the value is `PacketOption::Eof`
    #[inline]
    pub fn unwrap_eof(self) -> PacketEOF {
        match self {
            PacketOption::Eof(v) => v,
            _ => panic!("{}", "Called `PacketOption::unwrap_ok()` failed."),
        }
    }

    #[inline]
    pub fn map_ok<F: FnOnce(PacketOK) -> Result<PacketOK>>(self, op: F) -> Result<PacketOK> {
        match self {
            PacketOption::Ok(v) => op(v),
            _ => panic!("{}", "Called `PacketOption::map_ok()` failed."),
        }
    }

    #[inline]
    pub fn map_err<F: FnOnce(PacketERR) -> Result<PacketERR>>(self, op: F) -> Result<PacketERR> {
        match self {
            PacketOption::Err(v) => op(v),
            _ => panic!("{}", "Called `PacketOption::map_ok()` failed."),
        }
    }

    #[inline]
    pub fn map_eof<F: FnOnce(PacketEOF) -> Result<PacketEOF>>(self, op: F) -> Result<PacketEOF> {
        match self {
            PacketOption::Eof(v) => op(v),
            _ => panic!("{}", "Called `PacketOption::map_ok()` failed."),
        }
    }
}


impl Default for PacketOK {
     fn default() -> Self {
        PacketOK{
            header:PACKET_HEDAER_OK,
            affected_rows:0,
            last_insert_id:0,
            status_flags:0,
            warnings:0,
            info:Vec::new(),
            session_state_changes:Vec::new(),
        }
    }
}

impl Default for PacketERR {
    fn default () -> Self {
        PacketERR{
            header:PACKET_HEADER_ERR,
             error_code:0,
             sql_state_marker:Vec::new(),
             sql_state:Vec::new(),
             error_message:Vec::new()

        }
    }
}

impl Default for PacketEOF {
    fn default () -> Self {
        PacketEOF{
            header:PACKET_HEADER_EOF,
            warnings:0,
            status_flags:0
        }
    }
}

impl PacketOK {
    
    pub fn from_reader<T:ReadBytesExt + ?Sized>(flag:consts::ProtocolConst,reader:&mut T) -> Result<Self>{

        let mut packet:PacketOK = Default::default();

        if let Err(e) = reader.read_int_enclen()
            .map(|num|{packet.affected_rows = num})
            .and_then(|_|{reader.read_int_enclen().map(|num|{packet.last_insert_id = num})})
            .and_then(|n|{
                if flag & (consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41|
                           consts::CAPABILITY_FLAG_CLIENT_TRANSACTIONS) > 0{
                    return reader.read_int2().map(|num|{packet.status_flags = num});
                }
                Ok(n)
            }

            )
            .and_then(|n|{
                if flag & consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41 > 0 {
                    return reader.read_int2().map(|num|{packet.warnings = num});
                }
                Ok(n)
            })
            .and_then(|n|{
                if let Err(e) = reader.read_str_enclen().map(|s|{packet.info = s}) {
                    return Err(e)
                }
                Ok(n)
            })
            .and_then(|n|{
                if
                    (flag & consts::CAPABILITY_FLAG_CLIENT_SESSION_TRACK > 0) &&
                    (packet.status_flags as consts::ProtocolConst &
                    consts::SERVER_SESSION_STATE_CHANGED > 0)
                {
                    return reader.read_str_enclen().map(|s|{packet.session_state_changes = s});
                }
                Ok(n)
            }) {
                return Err(e);
            }
        Ok(packet)
    }

   
    pub fn write<T:WriteBytesExt + ?Sized>(self,flag:consts::ProtocolConst,writer:&mut T) -> Result<()> {

        writer.write_int1(self.header)
            .and_then(|_|{writer.write_int_enclen(self.affected_rows)})
            .and_then(|_|{writer.write_int_enclen(self.last_insert_id)})
            .and_then(|n|{
                if flag & (consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41 |
                           consts::CAPABILITY_FLAG_CLIENT_TRANSACTIONS) > 0{
                    return writer.write_int2(self.status_flags);
                }
                Ok(n)
            })
            .and_then(|n|{
                if flag &  consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41 > 0 {
                    return writer.write_int2(self.warnings);
                }
                Ok(n)
            })
            .and_then(|n|{
                if flag & consts::CAPABILITY_FLAG_CLIENT_SESSION_TRACK > 0 {
                    return writer.write_str_enclen(&self.info);
                }
                Ok(n)
            })
            .and_then(|n|{
                if
                    flag & consts::CAPABILITY_FLAG_CLIENT_SESSION_TRACK > 0 &&
                    self.status_flags as consts::ProtocolConst & consts::SERVER_SESSION_STATE_CHANGED > 0 {
                    return writer.write_str_enclen(&self.session_state_changes);
                }
                Ok(n)
            })
    }
}


impl PacketERR {

    pub fn from_reader<T:ReadBytesExt+ ?Sized>(flag:consts::ProtocolConst, reader:&mut T) -> Result<PacketERR> {
        let mut packet:PacketERR = Default::default();
        reader.read_int2().map(|num|packet.error_code = num)
            .and_then(|n|{
                if flag & consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41 > 0 {
                    return reader.read_str_varlen(1).map(|s|packet.sql_state_marker=s)
                        .and(reader.read_str_varlen(5).map(|s|packet.sql_state=s));
                }
                Ok(n)
            })
            .and_then(|_|{
                return reader.read_str_eof().map(|s|{packet.error_message  = s})
            })
            .and_then(|_|{
                Ok(packet)
            })
    }

    pub fn write<T:WriteBytesExt + ?Sized>(self,flag:consts::ProtocolConst,writer:&mut T) -> Result<()> {
        writer.write_int2(self.error_code)
            .and_then(|n|{
                if flag & consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41 > 0 {
                    return writer.write_str_varlen(&self.sql_state_marker,1)
                        .and(writer.write_str_varlen(&self.sql_state,5));
                }
                Ok(n)
            })
            .and_then(|_|{
                return writer.write_str_eof(&self.error_message);
            })
    }
}

impl PacketEOF {

    pub fn from_reader<T:ReadBytesExt+ ?Sized>(flag:consts::ProtocolConst,reader:&mut T) -> Result<PacketEOF> {

        let mut packet:PacketEOF = Default::default();

        if flag & consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41 > 0 {

            return reader.read_int2().map(|num|{packet.warnings = num})
                .and_then(|_|{reader.read_int2().map(|num|{packet.status_flags = num})})
                .and_then(|_|{Ok(packet)});
        }
        Ok(packet)
    }

    pub fn write<T:WriteBytesExt + ?Sized>(self,flag:consts::ProtocolConst,writer:&mut T) -> Result<()> {
        if flag & consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41 > 0 {
            return writer.write_int2(self.warnings)
                .and(writer.write_int2(self.status_flags));
        }
        Ok(())
    }
}

pub trait PacketStream: ReadBytesExt + WriteBytesExt {
    fn read_raw_packet(&mut self) -> Result<PacketRaw> {

        let mut packet = PacketRaw{
            payload_len:0,
            sequence_id:0,
            payload:Cursor::new(Vec::new())
        };
        if let Some(err) = self.read_int3()
            .map(|num| packet.payload_len = num)
            .and_then(|_|{self.read_int1()
                          .map(|num| packet.sequence_id = num)})
            .and_then(|_|{self.read_str_varlen(packet.payload_len as Int8)
                          .map(|p|{
                              packet.payload = Cursor::new(p)
                          })
            })
            .err() {
                return Err(err);
            };
         Ok(packet)
    }
    
    fn write_raw_packet(&mut self,packet:&PacketRaw) -> Result<()> {
        self.write_int3(packet.payload_len)
            .and_then(|_|{self.write_int1(packet.sequence_id)})
            .and_then(|_|{self.write_str_varlen(packet.payload.get_ref(),packet.payload_len as Int8)})
    }
}

impl<R: Write + Read + ?Sized > PacketStream for R {}

