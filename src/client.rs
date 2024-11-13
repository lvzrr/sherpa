use crate::filehandler::{get_file_contents, lookup, Database};
use crate::server::get_local_ip;
use reqwest;
use std::io::{stdin, stdout, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::process::exit;
use std::str::FromStr;
pub struct Client {
    //ip: IpAddr,
    dest_port: SocketAddr,
    connection: TcpStream,
}

pub fn get_user_opt() -> i8 {
    let mut opt: String = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut opt).expect("Failed\n");
    return opt.trim().parse().unwrap_or_default();
}

pub fn get_user_input() -> String {
    let mut opt: String = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut opt).expect("Failed\n");
    return opt.trim().to_string();
}

impl Client {
    pub fn init(host_ip: &SocketAddr) -> Client {
        let mut client: Client = Client {
            //ip: get_local_ip(),
            dest_port: host_ip.to_owned(),
            connection: TcpStream::connect(&host_ip.to_owned()).expect("ERROR WHILE CONNECTING"),
        };
        client
    }

    pub fn request_filesystem(&mut self) {
        self.connection.write("GET".as_bytes()).unwrap();
        self.connection.flush();
    }

    pub fn run(&mut self) {
        self.request_filesystem();
        let menu: String = String::from_str("+-----SHERPA-----+\n\n\t1 - List files\n\t2 - Get file\n\t3 - Push File\n\t4 - Exit\n\nChoose: ").unwrap();
        loop {
            print!("\n{}", menu);
            manage_option(get_user_opt());
        }
    }
}

fn manage_option(option: i8) {
    print!("\n");
    match option {
        1 => {
            println!("\nDATABASE HERE\n");
        }
        2 => {
            print!("File to get:");
            let filename: String = get_user_input();
            print!("\n");
            println!("FILE LOOKUP: {}\n", filename);
        }
        3 => {
            print!("File to push:");
            let filename: String = get_user_input();
            print!("\n");
            println!("FILE PUSHING: {}\n", filename);
        }
        4 => exit(0),
        _ => (),
    }
}
