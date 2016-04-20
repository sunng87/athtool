extern crate toml;
extern crate oath;
extern crate clap;
extern crate base32;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::path::Path;
use std::env;

use toml::{Parser, Table};
use clap::App;
use oath::totp_raw;

fn load_config(path: &Path) -> io::Result<String> {
    let mut file = try!(File::open(path));
    let mut s = String::new();
    try!(file.read_to_string(&mut s));
    Ok(s)
}

fn parse_config() -> Option<Table> {
    let mut p = env::home_dir().unwrap();
    p.push(".athtool.toml");
    load_config(&p).ok().and_then(|c| Parser::new(&c).parse())
}

fn main() {
    let matches = App::new("secrets")
                      .version(env!("CARGO_PKG_VERSION"))
                      .author("Ning Sun <sunng@about.me>")
                      .about("OTP generator from command line")
                      .args_from_usage("[service]... 'The service you generate password for'")
                      .get_matches();
    if let Some(v) = matches.value_of("service") {
        if let Some(cfg) = parse_config() {
            if let Some(secret) = cfg.get("secrets")
                                     .and_then(|f| f.as_table())
                                     .and_then(|f| f.get(v))
                                     .and_then(|f| f.as_str()) {
                let secret_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false },
                                                  secret)
                                       .unwrap();
                println!("{}", totp_raw(&secret_bytes, 6, 0, 30));
            } else {
                println!("service not found");
            }
        } else {
            println!("failed to load config");
        }
    } else {
        println!("service not given");
    }
}
