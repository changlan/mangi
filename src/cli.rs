use clap::{App, Arg, Error,SubCommand};
use clap;
use std::{ffi::OsString,path::PathBuf};
use std::net::{IpAddr,Ipv4Addr,Ipv6Addr};
use std::str::FromStr;

#[derive(Debug,Clone)]
pub struct Server {
    pub bind_addr: String,
    pub port: u16,
    pub key: String,
    pub dns: IpAddr
}

#[derive(Debug,Clone)]
pub struct Client {
    pub remote_addr: String,
    pub port: u16,
    pub key: String,
}


#[derive(Debug,Clone)]
pub enum Args {
    Client(Client),
    Server(Server)
}

pub fn get_args() -> Result<Args,String> {
    let matches = App::new("kytan: High Performance Peer-to-Peer VPN")
                            .version("1.0")
                            .subcommand(SubCommand::with_name("server")
                                        .arg(Arg::with_name("bind")
                                            .short("l")
                                            .long("listen")
                                            .default_value("0.0.0.0")
                                            .help("set the listen address")
                                            .takes_value(true))
                                        .arg(Arg::with_name("port")
                                            .short("p")
                                            .long("port")
                                            .default_value("9527")
                                            .help("set the listen port")
                                            .takes_value(true))
                                        .arg(Arg::with_name("key")
                                            .short("k")
                                            .long("key")
                                            .help("set the key for encryption communication")
                                            .takes_value(true))
                                        .arg(Arg::with_name("dns")
                                            .short("d")
                                            .long("dns")
                                            .default_value("8.8.8.8")
                                            .help("set dns for client, default 8.8.8.8")
                                            .takes_value(true))
                            )
                            .subcommand(SubCommand::with_name("client")
                                        .arg(Arg::with_name("server")
                                            .short("s")
                                            .long("server")
                                            .help("set the remote server address")
                                            .takes_value(true))
                                        .arg(Arg::with_name("port")
                                            .short("p")
                                            .long("port")
                                            .help("set the remote port")
                                            .takes_value(true))
                                        .arg(Arg::with_name("key")
                                            .short("k")
                                            .long("key")
                                            .help("set the key for encryption communication")
                                            .takes_value(true))
                            ).get_matches();
    if let Some(matches) = matches.subcommand_matches("client"){ 
        let ip_str = matches.value_of("server").ok_or_else(|| "can not find client host value").unwrap();
        let port_str = matches.value_of("port").ok_or_else(|| "can not find client port value").unwrap();
        let key_str = matches.value_of("key").ok_or_else(|| "can not find client key value").unwrap();
        // let remote_addr = IpAddr::V4(Ipv4Addr::from_str(ip_str).map_err(|e| e.to_string())?);
        let port = port_str.parse::<u16>().map_err(|e| e.to_string())?;
        Ok(Args::Client(Client{
            remote_addr: ip_str.to_string(),
            port: port,
            key: key_str.to_string(),
        }))
    } else if let Some(matches) = matches.subcommand_matches("server") {
        let ip_str = matches.value_of("bind").ok_or_else(|| "can not find server host value").unwrap();
        let port_str = matches.value_of("port").ok_or_else(|| "can not find server port value").unwrap();
        let key_str = matches.value_of("key").ok_or_else(|| "can not find server key value").unwrap();
        let dns = matches.value_of("dns").ok_or_else(|| "can not find dns value")?;
        // let bind_addr = IpAddr::V4(Ipv4Addr::from_str(ip_str).map_err(|e| e.to_string())?);
        let dns = IpAddr::V4(Ipv4Addr::from_str(dns).map_err(|e| e.to_string())?);
        let port = port_str.parse::<u16>().map_err(|e| e.to_string())?;
        Ok(Args::Server( Server {
            bind_addr: ip_str.to_string(),
            port: port,
            key: key_str.to_string(),
            dns: dns
        }))
    } else {
        unimplemented!()
    }
}

// pub fn get_args() -> Result<Args,String> {
//     let matches = App::new("My Super Program")
//                         .version("1.0")
//                         .author("Kevin K. <kbknapp@gmail.com>")
//                         .about("Does awesome things")
//                         .arg(Arg::with_name("config")
//                             .long("config")
//                             .value_name("FILE")
//                             .help("Sets a custom config file")
//                             .takes_value(true))
//                         .arg(Arg::with_name("client")
//                             .short("c")
//                             .long("client")
//                             .help("Sets the mode as client"))
//                         .arg(Arg::with_name("server")
//                             .short("s")
//                             .long("server")
//                             .help("Sets the mode as server"))
//                         .arg(Arg::with_name("address")
//                             .short("a")
//                             .long("address")
//                             .help("set the target address or bind address")
//                             .takes_value(true))
//                         .arg(Arg::with_name("port")
//                             .short("p")
//                             .long("port")
//                             .help("set the target port")
//                             .takes_value(true))
//                         .arg(Arg::with_name("key")
//                             .short("k")
//                             .long("key")
//                             .help("password of your remote server")
//                             .takes_value(true))
//                         .arg(Arg::with_name("dns")
//                             .short("d")
//                             .long("dns")
//                             .default_value("8.8.8.8")
//                             .help("set the dns, default value 8.8.8.8")
//                             .takes_value(true)
//                         )
//                         .get_matches();
//     // if matches.is_present("config") {
//     //     let matches = load_yaml!(matches.value_of("config").map_err(|e| e.to_string())?);
//     // }
    
//     let mut mode = "";
//     if matches.is_present("server") {
//         mode = "server";
//     } else {
//         if matches.is_present("client") {
//             mode = "client";
//         } else {
//             panic!("please select work mode");
//         }
//     }

//     let mut host = "";
//     if matches.is_present("address") {
//         host = matches.value_of("address").unwrap();
//     }

//     let mut port: u16 = 0;
//     if matches.is_present("port") {
//         let port_str = matches.value_of("port").unwrap();
//         port = port_str.parse::<u16>().map_err(|e| e.to_string())?;
//     }

//     let mut key = "";
//     if matches.is_present("key") {
//         key = matches.value_of("key").unwrap();
//     }
//     Ok(Args {
//         mode: mode.to_string(),
//         port: port,
//         host: host.to_string(),
//         key: key.to_string(),
//         dns: matches.value_of("dns").unwrap().to_string(),
//     })
// }