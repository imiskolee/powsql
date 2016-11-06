use super::backend;

#[test]
fn test_mysql_handshake() {
    let mut conn = backend::BackendConnection::new("127.0.0.1:3306", &Vec::from("root"),&Vec::from("root"),&Vec::new());
    let res = conn.init();
    println!("init {:?}",res);
}
