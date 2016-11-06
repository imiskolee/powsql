extern crate sha1;
use super::data_type::{Bytes};
use self::sha1::Sha1;

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
    let mut sha1_password = Sha1::new();
    sha1_password.reset();
    sha1_password.update(password);
    let s1 = sha1_password.digest();
    
    let mut sha1_2 = Sha1::new();
    sha1_2.reset();
    sha1_2.update(s1.to_string().as_bytes());
    let s2 = sha1_2.digest();
    
    let mut slat_cpy = Vec::from(slat.clone());
    //todo fix
    for c in s2.to_string().as_bytes().iter() {
        slat_cpy.push(*c);
    }

    let mut sha1_3 = Sha1::new();
    sha1_3.reset();
    sha1_3.update(&slat_cpy);
    let s3 = sha1_3.digest();
    let mut i = 0;
    let mut ret = Vec::with_capacity(s1.to_string().len());
    for c in s1.to_string().as_bytes().iter() {
        ret.push(*c ^ (*s3.to_string().as_bytes())[i]);
        i += 1;
    }
    return ret;
}



