use anyhow::{Error, Ok};
use v050_use_cases::configuration::{Configuration, StorageType};
use v050_use_cases::app_builder::handle_lines;
use v050_use_cases::storages::file::FileStore;
use v050_use_cases::storages::memory::MemoryStore;
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

    match conf.storage {
        StorageType::File => handle_lines::<FileStore>(conf).await?,
        StorageType::Memory => handle_lines::<MemoryStore>(conf).await?,
    };
    
    Ok(())
}
