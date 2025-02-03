use anyhow::{Error, Ok};
use v030_domain::configuration::Configuration;
use v030_domain::app_builder::run_app;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_delimiter = ' ', num_args = 1..)]
    candidates: Vec<String>,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();
    let mut conf = Configuration {
        candidates : Vec::new()
    };
    
    for candidate in args.candidates {
        if candidate == String::from("White") || candidate == String::from("Null") {
            println!("/Warning\\ \"{candidate}\" is automatically added, it's not needed in arguments.");
        } else {
            conf.candidates.push(candidate);
        }
    }

    if conf.candidates.len() == 0 {
        println!("/Warning\\ You didn't input any candidates, this poll is useless.");
    }

    run_app(conf).await?;

    Ok(())
}
