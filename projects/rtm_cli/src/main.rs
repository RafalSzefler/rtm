#![deny(warnings)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::unreadable_literal,
    clippy::inline_always,
    clippy::module_name_repetitions,
    clippy::len_without_is_empty,
    clippy::should_implement_trait
)]

mod csv_reader;
mod csv_writer;

use std::path::PathBuf;

use clap::Parser;
use rtm_core::processor::AccountingSystem;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    filename: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if !cli.filename.exists() {
        eprintln!("File does not exist: {}", cli.filename.display());
        return;
    }

    let stream = std::fs::File::open(&cli.filename).unwrap();
    let mut reader = csv_reader::CsvReader::new(stream);
    let iter = match reader.read_iter() {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("Error reading file: {e:?}");
            return;
        }
    };

    let mut accounting_system = AccountingSystem::new();

    for operation in iter {
        if let Err(err) = accounting_system.run_operation(operation) {
            eprintln!("Error processing operation: {err:?}");
            return;
        }
    }

    let mut writer = csv_writer::CsvWriter::new(std::io::stdout());
    for account in accounting_system.iter_accounts() {
        writer.write_client_account(account);
    }
}
