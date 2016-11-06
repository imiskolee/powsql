/**
mysql handshake packet reader & writer
https://dev.mysql.com/doc/internals/en/connection-phase-packets.html
**/
use super::data_type;
use super::data_type::{Int1, Int2, Int3, Int4, Int8, Bytes, ReadBytesExt, WriteBytesExt,};
use std::collections::HashMap;
use std::io::Result;
use std::io::Write;
use std::io::Read;
use std::io::Cursor;
use std::cmp;
use super::consts;
use super::packet::{PacketRaw, PacketStream};

const DEFAULT_PROTOCOL_VERSION: Int1 = 0x0a;

#[derive(Debug,Default)]
pub struct HandshakeV10 {
    pub protocol_version: Int1,
    pub server_version:  Bytes,
    pub connection_id: Int4,
    pub auth_plugin_data_part_1:  Bytes,
    pub filler: Int1,
    pub capability: Int4,
    pub charset: Int1,
    pub status_flags: Int2,
    pub auth_plugin_data_len: Int1,
    pub reserved:  Bytes,
    pub auth_plugin_data_part_2:  Bytes,
    pub auth_plugin_name:  Bytes,
}

#[derive(Debug,Default)]
pub struct HandshakeResponse41 {
    pub capability: Int4,
    pub max_packet_size: Int4,
    pub charset: Int1,
    pub reserved:  Bytes,
    pub username:  Bytes,
    pub auth_response_len: Int8,
    pub auth_response:  Bytes,
    pub database:  Bytes,
    pub auth_plugin_name:  Bytes,
    pub attrs: HashMap<Bytes,Bytes>,
}

impl HandshakeV10 {

    pub fn new() -> Self {
        let mut packet:HandshakeV10 = Default::default();
        packet.reserved = Vec::with_capacity(10);
        unsafe{
            packet.reserved.set_len(10);
        }
        packet
    }
}

impl HandshakeResponse41 {
    pub fn new() -> Self {
        let mut packet:HandshakeResponse41 = Default::default();
        packet.reserved = Vec::with_capacity(23);
        packet.max_packet_size = data_type::INT3MAX;
        unsafe{
            packet.reserved.set_len(23);
        }
        packet
    }
}


pub trait HandshakeStream: ReadBytesExt + WriteBytesExt {

    fn write_handshake_v10(&mut self, r: &HandshakeV10) -> Result<()> {
        let mut writer = Vec::new();
        writer.write_int1(r.protocol_version)
            .and_then(|_| writer.write_str_eof(&r.server_version))
            .and_then(|_| writer.write_int4(r.connection_id))
            .and_then(|_| writer.write_str_varlen(&r.auth_plugin_data_part_1, 8))
            .and_then(|_| writer.write_int1(r.filler))
            .and_then(|_| writer.write_int2((r.capability) as Int2))
            .and_then(|_| writer.write_int1(r.charset))
            .and_then(|_| writer.write_int2(r.status_flags))
            .and_then(|_| writer.write_int2((r.capability & (r.capability) >> 16) as Int2))
            .and_then(|_| {
                if r.capability as consts::ProtocolConst &
                   consts::CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH > 0 {
                    return writer.write_int1(r.auth_plugin_data_len);
                }
                return writer.write_int1(0);
            })
            .and_then(|_| writer.write_str_varlen(&r.reserved, 10))
            .and_then(|_| {
                if r.capability as consts::ProtocolConst &
                   consts::CAPABILITY_FLAG_CLIENT_SECURE_CONNECTION > 0 {
                    return writer.write_str_varlen(&r.auth_plugin_data_part_2,
                                                 cmp::min((r.auth_plugin_data_len - 8) as Int8,
                                                          13 as Int8));
                }
                Ok(())
            })
            .and_then(|_| {
                if r.capability as consts::ProtocolConst &
                    consts::CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH > 0 {
                    return writer.write_str_eof(&r.auth_plugin_name);
                }
                Ok(())
            })
            .and_then(|_|{
                let mut packet:PacketRaw = PacketRaw{
                    payload_len : 0,
                    sequence_id : 0,
                    payload: Cursor::new(vec![])
                };
                packet.payload_len = writer.len() as Int3;
                packet.sequence_id = 0;
                packet.payload = Cursor::new(writer);
                self.write_raw_packet(&packet)
            })
    }
    fn read_handshake_v10(&mut self) -> Result<HandshakeV10> {
        let mut raw_packet: PacketRaw;
        match self.read_raw_packet() {
            Err(e) => return Err(e),
            Ok(p) => raw_packet = p,
        }

        let mut packet: HandshakeV10 = Default::default();
        raw_packet.payload
            .read_int1()
            .map(|num| packet.protocol_version = num)
            .and_then(|_| raw_packet.payload.read_str_eof().map(|s| packet.server_version = s))
            .and_then(|_| raw_packet.payload.read_int4().map(|num| packet.connection_id = num))
            .and_then(|_| {

                raw_packet.payload.read_str_varlen(8).map(|s| packet.auth_plugin_data_part_1 = s)
            })
            .and_then(|_| raw_packet.payload.read_int1().map(|num| packet.filler = num as Int1))
            .and_then(|_| {

                raw_packet.payload.read_int2().map(|num| {

                    packet.capability = num as Int4;
                    num
                })
            })
            .and_then(|_| raw_packet.payload.read_int1().map(|num| packet.charset = num))
            .and_then(|_| raw_packet.payload.read_int2().map(|num| packet.status_flags = num))
            .and_then(|_| {

                raw_packet.payload.read_int2().map(|num| {
                    let mut upper: Int4 = num as Int4;
                    upper = upper << 16 | packet.capability;
                    packet.capability = upper;
                    num
                })
            })
            .and_then(|_| {
                raw_packet.payload.read_int1().map(|num| packet.auth_plugin_data_len = num)
            })
            .and_then(|_| raw_packet.payload.read_str_varlen(10).map(|s| packet.reserved = s))
            .and_then(|_| {
                if packet.capability as consts::ProtocolConst &
                   consts::CAPABILITY_FLAG_CLIENT_SECURE_CONNECTION > 0 {
                    return raw_packet.payload
                           .read_str_varlen(cmp::min(13,((packet.auth_plugin_data_len as u8) - 8) as u64))
                        .map(|s| packet.auth_plugin_data_part_2 = s);
                }
                Ok(())
            })
            .and_then(|_| {
                if packet.capability as consts::ProtocolConst &
                   consts::CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH > 0 {
                    return raw_packet.payload.read_str_eof().map(|s| packet.auth_plugin_name = s);
                }
                Ok(())
            })
            .and_then(|_| Ok(packet))
    }

    fn read_handshake_response_41(&mut self) -> Result<HandshakeResponse41> {
        
        let mut raw_packet: PacketRaw;
        match self.read_raw_packet() {
            Err(e) => return Err(e),
            Ok(p) => raw_packet = p,
        }

        let mut packet = HandshakeResponse41::new();

        raw_packet.payload.read_int4().map(|num| packet.capability = num)
            .and_then(|_|{
                raw_packet.payload.read_int4().map(|num| packet.max_packet_size = num)
            })
            .and_then(|_|{
                raw_packet.payload.read_int1().map(|num| packet.charset = num)
            })
            .and_then(|_|{
                raw_packet.payload.read_str_varlen(23).map(|s| packet.reserved = s)
            })
            .and_then(|_|{
                raw_packet.payload.read_str_eof().map(|s| packet.username = s)
            })
            .and_then(|_|{
               
                if packet.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA > 0 {
                    return raw_packet.payload.read_int_enclen().map(|num| packet.auth_response_len = num);
                }else if  packet.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_SECURE_CONNECTION > 0{
                    return raw_packet.payload.read_int1().map(|num| packet.auth_response_len = num as Int8);
                }
                Ok(())
            })
            .and_then(|_|{
              
                if packet.auth_response_len > 0 {
                    raw_packet.payload.read_str_varlen(cmp::min(13,packet.auth_response_len -8)).map(|s| packet.auth_response = s)
                }else {
                    raw_packet.payload.read_str_eof().map(|s| packet.auth_response = s)
                }
            })
            .and_then(|_|{
         
                if packet.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_CONNECT_WITH_DB > 0 {
                  return  raw_packet.payload.read_str_eof().map(|s| packet.database = s);
                }
                Ok(())
            })
            .and_then(|_|{
                if packet.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH > 0 {
                    return raw_packet.payload.read_str_eof().map(|s| packet.auth_plugin_name = s)
                }
                Ok(())
            })
            .and_then(|_|{
                println!("{:?}",packet);
                if packet.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_CONNECT_ATTRS> 0 {
                   return  raw_packet.payload.read_int_enclen()
                        .and_then(|num|{
                            let mut received_bytes:Int8 = 0;
                            loop {
                                if received_bytes >= num {
                                    break
                                }
                                if let Err(e) = raw_packet.payload.read_str_enclen()
                                    .and_then(|key|{
                                        received_bytes += data_type::calculate_str_enclen_bytes(key.len() as Int8);
                                        raw_packet.payload.read_str_enclen().map(|val|{
                                            received_bytes += data_type::calculate_str_enclen_bytes(val.len() as Int8);
                                            packet.attrs.insert(key,val);
                                           ()
                                        })
                                    })
                                {
                                    return Err(e)
                                }
                            }
                            Ok(())
                        })
                }
                Ok(())
            })
            .and_then(|_|{
                Ok(packet)
            })
    }

    fn write_handshake_response_41(&mut self,
                                   r: &HandshakeResponse41)
                                   -> Result<()> {

        let mut writer = Vec::new();

        writer.write_int4(r.capability)
            .and_then(|_| writer.write_int4(r.max_packet_size))
            .and_then(|_| writer.write_int1(r.charset))
            .and_then(|_| writer.write_str_varlen(&r.reserved, 23))
            .and_then(|_| writer.write_str_eof(&r.username))
            .and_then(|_| {
                if (r.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH_LENENC_CLIENT_DATA > 0) ||
                    (r.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_SECURE_CONNECTION > 0) {
                    return writer.write_int_enclen(r.auth_response_len)
                            .and(writer.write_str_varlen(&r.auth_response,cmp::min(13,r.auth_response_len -8)));
                }
                return writer.write_str_eof(&r.auth_response);
            })
            .and_then(|_| {
                if r.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_CONNECT_WITH_DB > 0 {
                    return writer.write_str_eof(&r.database);
                }
                Ok(())
            })
            .and_then(|_| {
                if  r.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH > 0 {
                    return writer.write_str_eof(&r.auth_plugin_name);
                }
                Ok(())
            })
            .and_then(|_| {
                if  r.capability as consts::ProtocolConst & consts::CAPABILITY_FLAG_CLIENT_CONNECT_ATTRS > 0 {
                    let mut temp_buffer = Vec::new();
                    for (k, v) in r.attrs.iter() {
                        if let Err(e) = temp_buffer.write_str_enclen(k) {
                            return Err(e);
                        }
                        if let Err(e) =  temp_buffer.write_str_enclen(v) {
                            return Err(e);
                        }
                    }
                    return writer.write_int_enclen(temp_buffer.len() as Int8)
                        .and_then(|_|{
                            writer.write_str_varlen(&temp_buffer,temp_buffer.len() as Int8)
                        })
                }
                Ok(())
            })
            .and_then(|_|{
                let mut packet:PacketRaw = PacketRaw{
                    payload_len : 0,
                    sequence_id : 0,
                    payload: Cursor::new(vec![])
                };
                packet.payload_len = writer.len() as Int3;
                packet.sequence_id = 0;
                packet.payload = Cursor::new(writer);
                self.write_raw_packet(&packet)
            })
    }
}


impl<R: Write + Read + ?Sized> HandshakeStream for R {}
