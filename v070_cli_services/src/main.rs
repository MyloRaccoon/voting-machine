use anyhow::{Error, Ok};
use v070_cli_services::configuration::{Configuration, StorageType};
use v070_cli_services::app_builder::dispatch_service;
use v070_cli_services::storages::file::FileStore;
use v070_cli_services::storages::memory::MemoryStore;
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
        StorageType::File => dispatch_service::<FileStore>(conf).await?,
        StorageType::Memory => dispatch_service::<MemoryStore>(conf).await?,
    };

    // match conf.storage {
    //     StorageType::File => handle_lines::<FileStore, StdioService<FileStore>>(conf).await?,
    //     StorageType::Memory => handle_lines::<MemoryStore, StdioService<MemoryStore>>(conf).await?,
    // };
    
    Ok(())
}
