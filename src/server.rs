use local_ip_address::local_ip;
use reqwest::Request;
use std::io::Result;
use std::net::{IpAddr, SocketAddr, TcpListener, TcpStream};
use std::ops::Index;
use std::path::PathBuf;
use std::str::FromStr;
use std::usize;
use std::{io::prelude::*, time::*, u128};

use crate::filehandler;

pub struct Server {
    up: bool,
    ip: IpAddr,
    init_time: SystemTime,
    port: SocketAddr,
    id: String,
    password: String,
    listener: TcpListener,
}

pub fn get_local_ip() -> IpAddr {
    return local_ip().unwrap();
}

impl Server {
    pub fn init(port: u16) -> Server {
        get_local_ip();
        println!("\nInitializing Server...\n");
        let up: SystemTime = SystemTime::now();
        let host: Server = Server {
            ip: get_local_ip(),
            port: SocketAddr::new(get_local_ip(), port),
            init_time: SystemTime::now(),
            password: create_server_password(up),
            id: create_server_id(up),
            up: true,
            listener: TcpListener::bind(SocketAddr::new(get_local_ip(), port))
                .expect("Cannot listen on specified port"),
        };

        println!(
            "\nSuccessfully Created Sherpa Server:\nIp: {}\nPort: {}\nId: {}\nPassword: {}\n",
            host.ip.to_string(),
            host.port.to_string(),
            host.id,
            host.password,
        );
        return host;
    }
    pub fn run(&mut self) {
        for listener_item in self.listener.incoming() {
            let stream: TcpStream = listener_item.unwrap();
            connection_handler(&stream).expect("Connection failed");
        }
    }
}

enum Req_Type {
    Push,
    Get,
    Unknown,
}
struct Req {
    rtype: Req_Type,
    file: String,
    contents: String,
}

/* TODO: APPEND FILE PATH TO STATIC SERVER FOLDER*/
fn parse_command(command: &String) -> Req {
    let parsed: Vec<&str> = command.split("&").collect();
    match parsed.get(0).unwrap().to_owned() {
        "GET" => {
            return Req {
                rtype: Req_Type::Get,
                file: parsed.get(1).unwrap().to_owned().to_string(),
                contents: "".to_string(),
            }
        }
        "PUSH" => {
            return Req {
                rtype: Req_Type::Push,
                file: parsed.get(1).unwrap().to_owned().to_string(),
                contents: parsed.get(2).unwrap().to_owned().to_string(),
            }
        }
        &_ => {
            return Req {
                rtype: Req_Type::Unknown,
                file: "".to_string(),
                contents: "".to_string(),
            }
        }
    }
}

fn connection_handler(mut stream: &TcpStream) -> Result<Req> {
    let mut buffer: [u8; 4096] = [0; 4096];
    stream
        .read(&mut buffer)
        .expect("Failed to handle connection");
    let resp_command: String = String::from_utf8_lossy(&buffer[..]).to_string();
    println!("Request taken: {}", resp_command);
    let re = parse_command(&resp_command);

    /*
     *TODO: gen resp and send
     */

    return Ok(re);
}

fn gen_body_response(re: Req) -> String {
    /*
     * GET FILE&FILENAME&
     * PUSH FILE&FILENAME&
     * GET DATABASE
     */

    /*TODO: IMPLEMENT PUSH */
    match re.rtype {
        Req_Type::Push => (),

        Req_Type::Get => match re.file.as_str() {
            "DATABASE" => (), /* this should get full db, but rn idk where it is accessed from here lol*/
            &_ => (),         /* This should cover the files or dirs*/
        },
        _ => (),
    }

    return "ERROR".to_string();
}
fn setseed(init_time: SystemTime) -> u128 {
    println!("\nInitializing Seed...\n");

    let seed: u128 = init_time.elapsed().unwrap().as_nanos();
    return seed;
}

fn create_server_id(init_time: SystemTime) -> String {
    println!("\nInitializing ServerID...\n");

    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUWXYZ"
        .chars()
        .collect();
    let mut seed_id: u128 = setseed(init_time);
    let mut id: String = String::new();
    while id.len() < 150 {
        let index: usize = (seed_id % (chars.len() as u128)) as usize;
        id.push(chars[index]);
        seed_id -= 100;
    }
    return id;
}

fn create_server_password(init_time: SystemTime) -> String {
    println!("\nInitializing Server Password...\n");
    let chars: Vec<char> = "123456789%&@#abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUWXYZ"
        .chars()
        .collect();
    let mut seed_id: u128 = setseed(init_time);
    let mut id: String = String::new();
    while id.len() < 300 {
        let index: usize = (seed_id % (chars.len() as u128)) as usize;
        id.push(chars[index]);
        seed_id -= 50;
    }
    return id;
}
