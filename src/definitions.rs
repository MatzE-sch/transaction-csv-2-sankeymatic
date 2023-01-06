use regex::Regex;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug)]
pub struct CsvRecord {
    pub begunst: String,
    pub verwend: String,
    pub betrag: String,
    pub datum: String,
}
impl ToString for CsvRecord {
    fn to_string(&self) -> String {
        format!("{} ## {} ## {}", self.begunst, self.verwend, self.betrag)
    }
}

pub struct RegLabels {
    pub reg: Regex,
    pub labels: Vec<String>,
}
pub trait LabelsStr {
    fn labels_str(&self) -> String;
}
impl LabelsStr for RegLabels {
    fn labels_str(&self) -> String {
        // String::from("labels")
        self.labels.join(" ")
    }
}

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(
        short = "t",
        long = "transactions",
        parse(from_os_str),
        default_value = "data/transactions.csv"
    )]
    pub transactions: std::path::PathBuf,
    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        default_value = "data/output.txt"
    )]
    pub output: std::path::PathBuf,
    #[structopt(
        short = "r",
        long = "regex",
        parse(from_os_str),
        default_value = "data/regex_labels.csv"
    )]
    pub regex: std::path::PathBuf,
    #[structopt(
        short = "i",
        long = "ignore",
        parse(from_os_str),
        default_value = "data/ignored_labels.txt"
    )]
    pub ignored: std::path::PathBuf,
}

pub fn cli_to_files(cli_args: Cli) -> Files {
    let transaction_path = Path::new(cli_args.transactions.to_str().unwrap());
    let regex_path = Path::new(cli_args.regex.to_str().unwrap());
    let output_path = Path::new(cli_args.output.to_str().unwrap());
    let ignore_path = Path::new(cli_args.ignored.to_str().unwrap());

    Files {
        transactions: match fs::metadata(transaction_path) {
            Ok(_) => transaction_path.to_str().unwrap().to_string(),
            Err(err) => panic!("transactions: {:?} {}", transaction_path, err),
        },
        output: match fs::metadata(output_path) {
            Ok(_) => output_path.to_str().unwrap().to_string(),
            Err(err) => panic!("output: {:?} {}", output_path, err),
        },
        regex: match fs::metadata(regex_path) {
            Ok(_) => regex_path.to_str().unwrap().to_string(),
            Err(err) => panic!("regex: {:?} {}", regex_path, err),
        },
        ignored: match fs::metadata(ignore_path) {
            Ok(_) => ignore_path.to_str().unwrap().to_string(),
            Err(_err) => {
                println!("ignored labels file not found: {:?}", ignore_path);
                String::from("")
                // panic!("ignored: {:?} {}", ignore_path, err)
            }
        },
    }
}

pub struct Files {
    pub transactions: String,
    pub output: String,
    pub regex: String,
    pub ignored: String,
}
