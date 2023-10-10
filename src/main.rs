mod modules;

use clap::Parser;
use modules::{domain, arguments};
use crate::modules::domain::DomainParsingError;

#[tokio::main]
async fn main() {
    let args = arguments::Cli::parse();

    let site_domain = args.site_domain;
    println!("[Scanning] Site Domain: {:?}", site_domain);

    match domain::get_domain_age(&site_domain).await {
        Ok(data) => {
            println!("Site Domain: {:?}", data.result.input);
            println!("Domain TLD: {:?}", data.result.tld);
            println!("Domain age (in years): {:?} years {:?} days",
                     data.result.creation.years, data.result.creation.days);
            println!("Domain age (in full days): {:?} days", data.result.creation.fulldays);
            println!("Domain created on: {:?}", data.result.creation.date);
            println!("Classification: {:?}", data.result.creation.classification);
            println!("IPTY Black Listed:  {:?}", data.result.ipty_blacklisted);
            println!("Mystic Owner: {:?}", data.result.mystic_owner);

            if args.whois {
                println!("Whois Info: {:?}", data.result.whois_data);
            }
        }
        Err(err) => {
            match err {
                DomainParsingError::RequestError(req_err) => {
                    println!("[Request Error] {}", req_err);
                }
                DomainParsingError::JsonError(serde_err) => {
                    println!("[JSON Parsing Error] {}", serde_err);
                }
            }
            println!("Unable to parse domain: {:?}.", site_domain);
        }
    }
}

