# XQR Code (eXtended QR Code) CLI

A minimal CLI for generated extended QR codes.

## Usage

```
eXtended QR Codes CLI

Usage: xqr <COMMAND>

Commands:
  encode             Encode a value into a JWT and optionally into a QR code
  decode             Decode a JWT from a QR code
  generate-key-pair  Generate a new ECDSA (ES256) key pair for use with XQR codes
  help               Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Encode

```
Encode a value into a JWT and optionally into a QR code

Usage: xqr encode [OPTIONS] --private-key <PRIVATE_KEY> --iss <ISS> [VALUE]

Arguments:
  [VALUE]  The value to encode

Options:
      --private-key <PRIVATE_KEY>
      --iss <ISS>                  The issuer to use
      --display                    Display the QR code in the terminal
      --save <SAVE_PATH>           Save the QR code to a file
      --valid-for <VALID_FOR>      How long the XQR is valid for in seconds, if not set the XQR will be valid forever
  -h, --help                       Print help
```

### Decode

```
Decode a JWT from a QR code

Usage: xqr decode [JWT]

Arguments:
  [JWT]  The JWT to decode

Options:
  -h, --help  Print help
```

### Generate key pair

```
Generate a new ECDSA (ES256) key pair for use with XQR codes

Usage: xqr generate-key-pair --save <SAVE_PATH>

Options:
      --save <SAVE_PATH>  Save the key pair to a file
  -h, --help              Print help
```
