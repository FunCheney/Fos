use std::path::Path;
// cli csv -i input.csv -o output json —header -d ','
use clap::{Parser};
use csv::{Reader, StringRecord};
use serde::{Serialize, Deserialize};

#[derive(Debug, Parser)]
#[command(name= "cli", version, author, about, long_about = None) ]
struct Opts {
    #[command(subcommand)]
    cmd: Subcommand,

}

#[derive(Debug, Parser)]
enum Subcommand {
    #[command(name="csv", about = "show CSV, or Convert CSV file to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    #[arg(short, long)] // "output.json".into()
    pub output: Option<String>,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub name: String,
    pub age: u8,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input).unwrap();
            let records = reader.records()
            .map(|r| r.unwrap())
                .collect::<Vec<StringRecord>>();
            println!("{:?}", records);
        }
    }
    println!("Hello, world!");
}

fn verify_file(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
        Ok(path.into())
    }else {
        Err(format!("{} does not exist", path))
    }
}
