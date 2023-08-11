extern crate clap;
use clap::{arg, value_parser, Arg, ArgAction, Command};
use image::Luma;
use qrcode::QrCode;
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
                .arg(
                    Arg::new("display")
                        .long("display")
                        .help("Display the QR code in the terminal")
                        .action(ArgAction::SetTrue)
                        .conflicts_with("save"),
                )
                .arg(
                    arg!(save: --save <SAVE_PATH> "Save the QR code to a file")
                        .value_parser(value_parser!(PathBuf))
                        .conflicts_with("display"),
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("decode")
                .about("Decode a JWT from a QR code")
                .arg(arg!(jwt: [JWT] "The JWT to decode"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("generate-key-pair")
                .about("Generate a new ECDSA (ES256) key pair for use with XQR codes")
                .arg(
                    arg!(save: --save <SAVE_PATH> "Save the key pair to a file")
                        .value_parser(value_parser!(PathBuf))
                        .required(true),
                )
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
                Ok(encoded_xqr) => {
                    if sub_matches.get_flag("display") {
                        match qr2term::print_qr(&encoded_xqr.token) {
                            Ok(_) => {}
                            Err(e) => {
                                eprintln!("Error displaying QR code: {}", e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        match sub_matches.get_one::<PathBuf>("save") {
                            Some(save_path) => {
                                let code = match QrCode::new(encoded_xqr.token) {
                                    Ok(code) => code,
                                    Err(e) => {
                                        eprintln!("Error creating QR code: {}", e);
                                        std::process::exit(1);
                                    }
                                };
                                let image = code.render::<Luma<u8>>().build();
                                match image.save(save_path) {
                                    Ok(_) => println!("Saved QR code to {}", save_path.display()),
                                    Err(e) => {
                                        eprintln!("Error saving QR code: {}", e);
                                        std::process::exit(1);
                                    }
                                }
                            }
                            None => println!("{}", encoded_xqr.token),
                        }
                    }
                }
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
        Some(("generate-key-pair", sub_matches)) => {
            let save_path = sub_matches.get_one::<PathBuf>("save").unwrap();
            let pub_key_path = save_path.with_extension("pub");
            let key_pair = xqr::generate_key_pair();
            let private_key_pem = match key_pair.to_pem() {
                Ok(private_key_pem) => private_key_pem,
                Err(e) => {
                    eprintln!("Error generating private key: {}", e);
                    std::process::exit(1);
                }
            };
            let pub_key_pem = match key_pair.public_key().to_pem() {
                Ok(pub_key_pem) => pub_key_pem,
                Err(e) => {
                    eprintln!("Error generating public key: {}", e);
                    std::process::exit(1);
                }
            };

            match std::fs::write(save_path, private_key_pem) {
                Ok(_) => println!("Saved private key to {}", save_path.display()),
                Err(e) => {
                    eprintln!("Error saving private key: {}", e);
                    std::process::exit(1);
                }
            }
            match std::fs::write(pub_key_path.clone(), pub_key_pem) {
                Ok(_) => println!("Saved public key to {}", pub_key_path.display()),
                Err(e) => {
                    eprintln!("Error saving public key: {}", e);
                    std::process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }
}
