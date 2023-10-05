use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use clap::Parser;

mod methods;

// ANSII codes for formatting
const BLUE: &'static str = "\x1B[34m";
const RED: &'static str = "\x1B[1;31m";
const BOLD: &'static str = "\x1B[1m";
const UNDERLINE: &'static str = "\x1B[4m";
const RESET: &'static str = "\x1B[0m";

// Command line flags
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Simple tool to generate reverse shells for CTFs and pentesting."
)]
struct Args {
    #[arg(short, long, help = "Represents the shell (e.g. bash_-i, python3, etc.)")]
    shell: Option<String>,

    #[arg(
        short,
        long,
        help = "Represents the name of the output file. If this flag is not set, the script will be printed to stdout"
    )]
    output: Option<String>,

    #[arg(short, long="port", help = "Port number")]
    port_num: Option<String>,

    #[arg(short,long="ip", help = "IP address")]
    ip_address: Option<String>,

    #[arg(long, help = "Displays all supported shells")]
    supported_shells: bool,
}

impl Args {
    // Validates command line flags.
    fn validate(&self) -> Result<(), Vec<&str>> {
        let mut validation_errors: Vec<&str> = Vec::new();

        match self.shell {
            Some(_) => {
                if let None = self.port_num {
                    validation_errors.push("Missing port number.");
                } else if let None = self.ip_address {
                    validation_errors.push("Missing IP address.")
                }
            }
            None => {
                if let Some(_) = self.port_num {
                    validation_errors.push("Port number referenced without shell.")
                } else if let Some(_) = self.ip_address {
                    validation_errors.push("IP address referenced without shell.")
                }
            }
        }

        if validation_errors.len() > 0 {
            return Err(validation_errors);
        }

        return Ok(());
    }
}

// Handles logic for getting the script, and creating/displaying the script.
fn handle_script(args: Args) {
    if let Err(validation_errors) = args.validate() {
        println!("{}Input validation failed:{}\n{:?\n}", RED, RESET, validation_errors);
        return;
    }

    if let Some(method) = args.shell {
        let revshell_script = methods::METHODS.get(&method);
        match revshell_script {
            Some(script) => {
                // The values of args.ip_address and args.port_num can be unwrapped because they
                // have been validated with args.validate
                let mut formatted_script = script.replace("IP_ADDR", &args.ip_address.unwrap());
                formatted_script = formatted_script.replace("PORT", &args.port_num.unwrap());
                output_script(&formatted_script, args.output).unwrap();
            }
            None => println!("Invalid method. Run revshell --available-shells to view all shells."),
        }
    }
}

// Handles outputting the script in accordance to the command line flags. It either creates a file
// or it prints the script to stdout.
fn output_script(script: &str, file_name: Option<String>) -> Result<(), Box<dyn Error>> {
    if let Some(name) = file_name {
        let mut file = File::create(name)?;
        file.write_all(script.as_bytes())?;
        Ok(())
    } else {
        println!("{script}");
        Ok(())
    }
}

fn main() {
    let args = Args::parse();

    if args.supported_shells {
        println!("{BOLD}{BLUE}{UNDERLINE}Available Methods:{RESET}");
        for method in methods::METHODS.keys() {
            println!("{method}");
        }
    } else {
        handle_script(args);
    }
}
