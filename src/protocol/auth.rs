extern crate openssl;
use self::openssl::hash;
use self::openssl::hash::MessageDigest;
use super::data_type::{Bytes,Int1,AsStr};

//https://dev.mysql.com/doc/internals/en/secure-password-authentication.html
pub const AUTH_METHOD_SECURE:&'static str = "mysql_native_password";
//https://dev.mysql.com/doc/internals/en/old-password-authentication.html
pub const AUTH_METHOD_OLD:&'static str = "mysql_old_password";
//https://dev.mysql.com/doc/internals/en/clear-text-authentication.html
pub const AUTH_METHOD_CLEAR:&'static str = "mysql_clear_password";
//https://dev.mysql.com/doc/internals/en/windows-native-authentication.html
pub const AUTH_METHOD_WINDOWS:&'static str = "authentication_windows_client";
//https://dev.mysql.com/doc/internals/en/sha256.html
pub const AUTH_METHOD_SHA256:&'static str = "sha256_password";

pub type PasswordHandle = fn (&Bytes,&Bytes) -> Bytes;

pub fn get_password_handle(name:&'static str) -> Option<PasswordHandle> {
    match name {
        AUTH_METHOD_SECURE => Some(get_secure_password_slat),
        _ => None,
    }
}


//mysql native password
fn get_secure_password_slat(slat:&Bytes,password:&Bytes) -> Bytes {
    let  s1 = hash::hash(hash::MessageDigest::sha1(),password).unwrap();
    let s2 = hash::hash(hash::MessageDigest::sha1(),&s1).unwrap();
    
    let mut slat_cpy = Vec::from(slat.clone());
    //todo fix
    for c in s2.iter() {
        slat_cpy.push(*c);
    }
    let s3 = hash::hash(hash::MessageDigest::sha1(),&slat_cpy).unwrap();

    let mut i = 0;
    let mut ret = Vec::with_capacity(0);
    for c in s1.iter() {
        ret.push(*c ^ (s3[i]) as Int1);
        i += 1;
    }
    return ret
}



