use std::time::Duration;
use std::net::TcpStream;
pub fn scan(ip:&str,port:u16){


    /* make 50 thread */
    /* Default 1000 port will sacned */
    /*  */

    let addr: String = format!("{}:{}",ip,port);
    let socket = addr.parse().unwrap();
    let timeout = Duration::from_secs(2);

    match TcpStream::connect_timeout(&socket, timeout) {
        Ok(_) => { println!("[✅️] Open {}", addr)},
        Err(_) => {}
    };
}