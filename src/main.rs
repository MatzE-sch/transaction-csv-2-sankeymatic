mod regex_importer;
use regex_importer::read_regex_labels;

mod definitions;
use definitions::*;

use csv::Writer;
use std::error::Error;

use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io::stdin;

use std::io::Write;

use color_print::cprintln;

use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
    let files = cli_to_files(&args);
    let ignore = fs::read_to_string(files.ignored).unwrap_or_default();

    // println!("transactions: \t{}", args.transactions.to_str().unwrap());
    // println!("output: \t{}", args.output.to_str().unwrap());
    // println!("regex / labels:\t{}", args.regex.to_str().unwrap());
    // println!("ignore:\t{}", ignore);

    init_output(files.output.to_string());

    let read_transaction_records = match &*args.format {
        "sparkasse" => read_sparkasse_records,
        "vrbank" => read_vrbank_records,
        _ => panic!("provide a valid csv format name"),
    };

    let umsatz = read_transaction_records(files.transactions.to_string()).unwrap();

    'row: for row in umsatz {
        if row.betrag == "0,00" {
            continue;
        }
        // println!("{:?}", row);
        let regex_vec = read_regex_labels(files.regex.to_string()).unwrap();

        // let mut index = 0;
        for reg_labels in regex_vec {
            let reg = reg_labels.regex;
            let mut labels = reg_labels.labels;
            let re = Regex::new(reg.as_str()).unwrap();
            // index += 1;
            // println!("Test {}, {}", index, re);
            if re.is_match(&row.to_string()) {
                // skip row if 1 label is ignored labels
                for label in labels.split(' ') {
                    if ignore.contains(label) {
                        println!("ignored:");
                        // println!("label {label}");
                        // println!("regex {reg}");
                        continue 'row;
                    }
                }

                cprintln!(
                    "<bright-green>Match!</> <blue>/{}/</> {}",
                    re,
                    row.to_string()
                );

                let mut output = String::from("");

                // let mut von = "Budget";
                let mut betrag = row.betrag.replace(",", ".").parse::<f64>().unwrap();

                if betrag < 0.0 {
                    betrag = betrag.abs();
                    labels = "Budget ".to_string() + &labels;
                } else {
                    labels = labels + " Budget";
                }
                let labels_iter = labels.split(' ');
                let labels_iter2 = labels.split(' ');
                for (von, nach) in labels_iter.zip(labels_iter2.skip(1)) {
                    output = output + von + " [" + &betrag.to_string() + "] " + nach + "\n";
                }
                write_output(files.output.to_string(), output);
                continue 'row;
            }
        }
        cprintln!("<yellow>No regex matched!</>");
        println!(
                "Please enter a regex for the following entry on {}\n(leave blank to use whole line as regex (experimental)):",
                row.datum
            );
        println!("{}", &row.to_string());

        let mut regex_accepted = false;
        while !regex_accepted {
            let mut reg_opt: Option<Regex> = None;
            while reg_opt.is_none() {
                let user_regex = user_input();

                match Regex::new(&user_regex.trim()) {
                    Ok(re) => reg_opt = Some(re),
                    Err(_) => {
                        println!("/{}/ is not a valid regex! Try again:", user_regex);
                        println!("{}", &row.to_string());
                    }
                }
            }
            let mut reg = reg_opt.unwrap();
            // println!("Entered regex: {:?}", reg);
            if reg.to_string().is_empty() {
                reg = Regex::new(&regex::escape(&row.to_string())).unwrap();
            }
            if reg.is_match(&row.to_string()) {
                regex_accepted = true;
                // println!("Succsessfully registered regex");
                println!("Pleas enter your lables (food pizza):");
                let labels_string = user_input();
                // println!("{:?}", labels_string);
                let labels = labels_string
                    .split(' ')
                    .map(|string_ref| string_ref.to_string())
                    .collect();
                match write_to_csv(
                    files.regex.to_string(),
                    RegLabels {
                        reg: reg,
                        labels: labels,
                    },
                ) {
                    Ok(res) => res,
                    Err(e) => panic!("file not writable? error: {}", e),
                }
            } else {
                cprintln!("<red>Your regex doesn't match your entry! Please try again!</>");
                println!("{}", &row.to_string());
            }
        }

        // println!()
    }
    cprintln!("<bright-green>Conratulation, nothing uncategorized!</>");
    cprintln!("Go to <bright-blue>https://www.sankeymatic.com/build/</> and paste your output")
}

fn user_input() -> String {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");

    String::from(input.trim())
}

fn write_to_csv(file_name: String, reglabels: RegLabels) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();
    let mut wtr = Writer::from_writer(file);
    // wtr.write_record(&["a", "b", "c"])?;
    // wtr.write_record(&["x", "y", "z"])?;
    wtr.write_record(&[reglabels.reg.to_string(), reglabels.labels_str()])?;
    wtr.flush()?;
    Ok(())
}

fn write_output(file_name: String, s: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();
    write!(file, "{}", s).expect("Cant write to file?");
}

fn init_output(file_name: String) {
    let mut file = OpenOptions::new()
        .write(true)
        // .append(false)
        .truncate(true)
        .create(true)
        .open(file_name)
        .unwrap();

    write!(file, "// output format is intended for \n// https://www.sankeymatic.com/build/\n// outputh width: 1600 works better then the default\n")
        .expect("Cant write to file?");
}
