# Domain Age

`Domain Age` is a command-line utility written in Rust that allows you to determine the age of a domain.

## Features

The tool allows you to query the following information about a domain:

- Site Domain
- Top-level domain (TLD)
- Domain age in years and days
- Date of domain creation
- Classification of the domain
- Check if the domain is blacklisted
- Check if the domain has a mystic owner
- Get Whois information about the domain

## Installation

Before you use Domain Age, you need to have Rust installed. You can install Rust from their [website](https://www.rust-lang.org/). After you have Rust installed, you can clone this repository and run the utility using `cargo run`.


```sh
git clone https://github.com/Bishwas-py/domain-age.git
cd domain-age
cargo run -- -s example.com
```

## Usage

To get information about a domain, you need to pass the `short (-s)` or `long (--site_domain)` parameter with the domain name as an argument.

```sh
domainage -s example.com
```

or

```sh
domainage --site_domain example.com
```

To get the whois information about a domain, you can use the `whois (-w)` flag.

```sh
domainage -s example.com -w
```

Get it on [releases](https://github.com/Bishwas-py/domain-age/releases).

## Development

To contribute to Domain Age, please make sure to checkout your feature branch from `dev`. You are encouraged to use standard Rust formatted Markdown syntax, which is preferred. You can use the provided rustfmt.toml for proper code formatting.

## Contributions

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License

Domain Age is licensed under MIT License. Please see the [LICENSE](LICENSE) file for details.