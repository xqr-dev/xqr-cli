extern crate clap;
use clap::{arg, value_parser, Command};
use std::path::PathBuf;

fn cli() -> Command {
    Command::new("xqr")
        .about("eXtended QR Codes CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("encode")
                .about("Encode a value into a JWT and optionally into a QR code")
                .arg(arg!(value: [VALUE] "The value to encode"))
                .arg(
                    arg!(private_key: --"private-key" <PRIVATE_KEY>)
                        .value_parser(value_parser!(PathBuf))
                        .required(true),
                )
                .arg(arg!(kid: --kid <KID> "The key ID to use").required(true))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("decode")
                .about("Decode a JWT from a QR code")
                .arg(arg!(jwt: [JWT] "The JWT to decode"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("encode", sub_matches)) => {
            let priv_key_path = sub_matches.get_one::<PathBuf>("private_key").unwrap();
            let priv_key = match std::fs::read_to_string(priv_key_path) {
                Ok(priv_key) => priv_key,
                Err(e) => {
                    eprintln!("Error reading {}: {}", priv_key_path.display(), e);
                    std::process::exit(1);
                }
            };
            let value = sub_matches.get_one::<String>("value").unwrap();
            let kid = sub_matches.get_one::<String>("kid").unwrap();
            match xqr::encode(&priv_key, value, kid) {
                Ok(encoded_xqr) => println!("{}", encoded_xqr.token),
                Err(e) => {
                    eprintln!("Error encoding: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(("decode", sub_matches)) => {
            let encoded_xqr: xqr::XQR = sub_matches
                .get_one::<String>("jwt")
                .unwrap()
                .to_string()
                .into();
            let kid = match encoded_xqr.get_kid() {
                Some(kid) => kid,
                None => {
                    eprintln!("Error decoding: {}", "Key ID not found in token");
                    std::process::exit(1);
                }
            };
            let pub_key = match xqr::fetch_public_key(&kid) {
                Ok(pub_key) => pub_key,
                Err(e) => {
                    eprintln!("Error fetching public key for key id ({}): {}", kid, e);
                    std::process::exit(1);
                }
            };
            match xqr::decode(&pub_key, encoded_xqr) {
                Ok(decoded_value) => println!("{}", decoded_value),
                Err(e) => {
                    eprintln!("Error decoding: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }
}
