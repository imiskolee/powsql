use super::super::protocol::data_type::{Int1,Int4,Int8,Bytes,AsStr};
use super::super::protocol::packet::{PacketStream,PacketOption};
use super::super::protocol::handshake::{HandshakeStream,HandshakeResponse41};
use super::super::protocol::auth;
use super::super::protocol::consts;
use std::net::TcpStream;
use std::io::{Result,Error,ErrorKind};

pub struct BackendConnection {
    connection_id : Int4,
    capability:Int4,
    charset:Int1,
    slat: Bytes,
    auth_plugin_name:Bytes,
    stream:Option<TcpStream>,
    username: Bytes,
    password: Bytes,
    addr:&'static str,
    db:Bytes,
}

impl BackendConnection {
    pub fn new(addr:&'static str,username:&Bytes,password:&Bytes,database:&Bytes) -> Self {
        BackendConnection {
            connection_id:0,
            capability:0,
            charset:0,
            slat:Vec::new(),
            auth_plugin_name:Vec::new(),
            addr : addr,
            username:username.clone(),
            password:password.clone(),
            db:database.clone(),
            stream:None,
        }
    }

    pub fn init(&mut self) -> Result<PacketOption> {
        self.connect()
            .and_then(|_|{
                self.read_handshake()
            })
            .and_then(|_|{
                self.write_handshake()
                    .and_then(|packet_option|{
                        println!("{:?}",packet_option);
                        match packet_option {
                            PacketOption::Err(err) => Err(Error::new(ErrorKind::InvalidInput,err.error_message.as_str())),
                            _ => Ok(packet_option)
                        }
                    })
            })
    }
    
    fn connect(&mut self) -> Result<()> {
        println!("connection");
        if self.addr.len() < 1 {
            return Err(Error::new(ErrorKind::InvalidData,"miss addr"))
        }
        match  TcpStream::connect(self.addr) {
            Err(e) => return Err(e),
            Ok(stream) => self.stream = Some(stream)
        }
        Ok(())
    }

    fn read_handshake(&mut self) -> Result<()> {
        println!("read_handshake");
        match self.stream.as_mut().unwrap().read_handshake_v10() {
            Err(e) => return Err(e),
            Ok(packet) => {
                self.connection_id = packet.connection_id;
                self.auth_plugin_name = packet.auth_plugin_name;
                self.charset = packet.charset;
                self.slat.append(&mut packet.auth_plugin_data_part_1.clone());
                self.slat.append(&mut packet.auth_plugin_data_part_2.clone());
                self.capability = packet.capability;
            }
        }
        Ok(())
    }

    fn write_handshake(&mut self) -> Result<PacketOption> {
        println!("write_handshake");
        let mut response:HandshakeResponse41 = HandshakeResponse41::new();

        response.capability =

            consts::CAPABILITY_FLAG_CLIENT_PROTOCOL_41 as Int4
            | consts::CAPABILITY_FLAG_CLIENT_PLUGIN_AUTH as Int4
            | consts::CAPABILITY_FLAG_CLIENT_SECURE_CONNECTION as Int4;

        response.capability = self.capability & response.capability;

        response.charset = self.charset;
        response.username = self.username.clone();
        response.database = self.db.clone();
        response.auth_plugin_name = self.auth_plugin_name.clone();

        //only supported auth method `mysql_native_password`
        match auth::get_password_handle( self.auth_plugin_name.as_str()) {
            None => return Err(Error::new(ErrorKind::InvalidData,"auth method not supported")),
            Some(handle) => {
                let auth_response = handle(&self.slat,&self.password);
                response.auth_response_len = auth_response.len() as Int8;
                response.auth_response = auth_response;
            }
        }
        self.stream.as_mut().unwrap().write_handshake_response_41(&response)
            .and_then(|_|{
                self.stream.as_mut().unwrap().read_raw_packet()
                    .and_then(|mut packet|{
                        println!("packet {:?}",packet);
                        packet.read_packet(response.capability as consts::ProtocolConst)
                    })
            })
    }

}
