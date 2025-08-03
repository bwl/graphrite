use clap::{Parser, Subcommand};
use graphrite_core::parser::Parser as CoreParser;

#[derive(Parser)]
#[command(name = "graphrite", version, about = "Graphrite CLI")] 
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Lint { input: String },
    Compile { #[arg(long)] json: bool, input: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Lint { input } => {
            let src = std::fs::read_to_string(input)?;
            match CoreParser::parse(&src) {
                Ok(_) => Ok(()) ,
                Err(diags) => {
                    for d in diags { println!("{}\t{}", d.code, d.message); }
                    std::process::exit(1);
                }
            }
        }
        Commands::Compile { json: _, input } => {
            let src = std::fs::read_to_string(input)?;
            match CoreParser::parse(&src) {
                Ok(doc) => {
                    println!("{}", serde_json::to_string_pretty(&doc)?);
                    Ok(())
                }
                Err(diags) => {
                    for d in diags { println!("{}\t{}", d.code, d.message); }
                    std::process::exit(1);
                }
            }
        }
    }
}
