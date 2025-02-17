use anyhow::{Error, Ok};
use v040_memory::configuration::Configuration;
use v040_memory::app_builder::run_app;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut conf = Configuration::parse();
    
    for candidate in conf.candidates.clone() {
        if candidate == *"White" || candidate == *"Null" {
            println!("/Warning\\ \"{candidate}\" is automatically added, it's not need in arguments.");
        } else {
            conf.candidates.push(candidate);
        }
    }

    if conf.candidates.is_empty() {
        println!("/Warning\\ You didn't input any candidates, this poll is useless.");
    }

    conf.candidates.push(String::from("White"));
    conf.candidates.push(String::from("Null"));

    run_app(conf).await?;

    Ok(())
}
