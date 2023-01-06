use regex::Regex;
use std::fs;
use std::path::Path;
use structopt::StructOpt;

use csv::ReaderBuilder;
use csv::StringRecord;
use serde::Deserialize;
use std::error::Error;



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
        default_value = "data/transactions.csv",
        help = "path to csv transaction export"
    )]
    pub transactions: std::path::PathBuf,
    #[structopt(
        short = "o",
        long = "output",
        parse(from_os_str),
        default_value = "data/output.txt",
        help = "path of output file, (todo: will create file?)"
    )]
    pub output: std::path::PathBuf,
    #[structopt(
        short = "r",
        long = "regex",
        parse(from_os_str),
        default_value = "data/regex_labels.csv",
        help = "path to csv file for search patterns and how to label transactions (TODO: will create the file?)"
    )]
    pub regex: std::path::PathBuf,
    #[structopt(
        short = "i",
        long = "ignore",
        parse(from_os_str),
        default_value = "data/ignored_labels.txt",
        help = "path to file with lables to be ignored"
    )]
    pub ignored: std::path::PathBuf,
    #[structopt(
        short = "f",
        long = "format",
        default_value = "sparkasse",
        help = "possible values: sparkasse, vrbank (...get_possible_formats_str() (TODO: acrually run this function here without failing on ownership problems again))"
    )]
    pub format: String,
}

pub fn cli_to_files(cli_args: &Cli) -> Files {
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

// there must be a better way of switching between csv formats, but i tried a lot and failed a lot... not worth the time anylonger
pub fn read_vrbank_records(file_name: String) -> Result<Vec<CsvRecord>, Box<dyn Error>> {
    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Record {
        bezeichnung_auftragskonto: String,
        iban_auftragskonto: String,
        bic_auftragskonto: String,
        bankname_auftragskonto: String,
        buchungstag: String,
        valutadatum: String,
        name_zahlungsbeteiligter: String,
        iban_zahlungsbeteiligter: String,
        bic_swift_code_zahlungsbeteiligter: String,
        buchungstext: String,
        verwendungszweck: String,
        betrag: String,
        waehrung: String,
        saldo_nach_buchung: String,
        bemerkung: String,
        kategorie: String,
        steuerrelevant: String,
        glaeubiger_id: String,
        mandatsreferenz: String,
    }
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_path(file_name)?;
    let header = StringRecord::from(vec![
        // Bezeichnung Auftragskonto;IBAN Auftragskonto;BIC Auftragskonto;Bankname Auftragskonto;Buchungstag;Valutadatum;Name Zahlungsbeteiligter;IBAN Zahlungsbeteiligter;BIC (SWIFT-Code) Zahlungsbeteiligter;Buchungstext;Verwendungszweck;Betrag;Waehrung;Saldo nach Buchung;Bemerkung;Kategorie;Steuerrelevant;Glaeubiger ID;Mandatsreferenz
        "bezeichnung_auftragskonto",
        "iban_auftragskonto",
        "bic_auftragskonto",
        "bankname_auftragskonto",
        "buchungstag",
        "valutadatum",
        "name_zahlungsbeteiligter",
        "iban_zahlungsbeteiligter",
        "bic_swift_code_zahlungsbeteiligter",
        "buchungstext",
        "verwendungszweck",
        "betrag",
        "waehrung",
        "saldo_nach_buchung",
        "bemerkung",
        "kategorie",
        "steuerrelevant",
        "glaeubiger_id",
        "mandatsreferenz",
    ]);
    rdr.set_headers(header);

    let mut records: Vec<CsvRecord> = vec![];
    for result in rdr.records().skip(1) {
        //skip header, why ever
        let record = result?;
        let header = StringRecord::from(vec![
            // Bezeichnung Auftragskonto;IBAN Auftragskonto;BIC Auftragskonto;Bankname Auftragskonto;Buchungstag;Valutadatum;Name Zahlungsbeteiligter;IBAN Zahlungsbeteiligter;BIC (SWIFT-Code) Zahlungsbeteiligter;Buchungstext;Verwendungszweck;Betrag;Waehrung;Saldo nach Buchung;Bemerkung;Kategorie;Steuerrelevant;Glaeubiger ID;Mandatsreferenz
            "bezeichnung_auftragskonto",
            "iban_auftragskonto",
            "bic_auftragskonto",
            "bankname_auftragskonto",
            "buchungstag",
            "valutadatum",
            "name_zahlungsbeteiligter",
            "iban_zahlungsbeteiligter",
            "bic_swift_code_zahlungsbeteiligter",
            "buchungstext",
            "verwendungszweck",
            "betrag",
            "waehrung",
            "saldo_nach_buchung",
            "bemerkung",
            "kategorie",
            "steuerrelevant",
            "glaeubiger_id",
            "mandatsreferenz",
        ]); // einfach nochmal weil borrow move zeugs, keine ahnung, machs besser...
        let row: Record = match record.deserialize(Some(&header)){
            Ok(row) => row,
            Err(error) => {
                println!("Are you shure vrbank is the right format for your csv?");
                panic!("{}",error)
            }
        };
        // println!("{:?}", row);
        records.push(CsvRecord {
            begunst: row.name_zahlungsbeteiligter,
            verwend: row.verwendungszweck,
            betrag: row.betrag,
            datum: row.buchungstag,
        });
    }
    Ok(records)
}

pub fn read_sparkasse_records(file_name: String) -> Result<Vec<CsvRecord>, Box<dyn Error>> {
    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Record {
        auftragskonto: String,
        buchungstag: String,
        valutadatum: String,
        buchungstext: String,
        verwendungszweck: String,
        glaeubiger_id: String,
        mandatsreferenz: String,
        kundenreferenz: String,
        sammlerreferenz: String,
        lastschrift_ursprungsbetrag: String,
        auslagenersatz_ruecklastschrift: String,
        beguenstigter_zahlungspflichtiger: String,
        iban: String,
        bic: String,
        betrag: String,
        waehrung: String,
        info: String,
    }
    let mut rdr = ReaderBuilder::new().delimiter(b';').from_path(file_name)?;
    let header = StringRecord::from(vec![
        // "Auftragskonto","Buchungstag","Valutadatum","Buchungstext","Verwendungszweck","Glaeubiger ID","Mandatsreferenz","Kundenreferenz (End-to-End)","Sammlerreferenz","Lastschrift Ursprungsbetrag","Auslagenersatz Ruecklastschrift","Beguenstigter/Zahlungspflichtiger","Kontonummer/IBAN","BIC (SWIFT-Code)","Betrag","Waehrung","Info"
        "auftragskonto",
        "buchungstag",
        "valutadatum",
        "buchungstext",
        "verwendungszweck",
        "glaeubiger_id",
        "mandatsreferenz",
        "kundenreferenz",
        "sammlerreferenz",
        "lastschrift_ursprungsbetrag",
        "auslagenersatz_ruecklastschrift",
        "beguenstigter_zahlungspflichtiger",
        "iban",
        "bic",
        "betrag",
        "waehrung",
        "info",
    ]);
    rdr.set_headers(header);

    let mut records: Vec<CsvRecord> = vec![];
    for result in rdr.records().skip(1) {
        //skip header, why ever
        let record = result?;
        let header = StringRecord::from(vec![
            // "Auftragskonto","Buchungstag","Valutadatum","Buchungstext","Verwendungszweck","Glaeubiger ID","Mandatsreferenz","Kundenreferenz (End-to-End)","Sammlerreferenz","Lastschrift Ursprungsbetrag","Auslagenersatz Ruecklastschrift","Beguenstigter/Zahlungspflichtiger","Kontonummer/IBAN","BIC (SWIFT-Code)","Betrag","Waehrung","Info"
            "auftragskonto",
            "buchungstag",
            "valutadatum",
            "buchungstext",
            "verwendungszweck",
            "glaeubiger_id",
            "mandatsreferenz",
            "kundenreferenz",
            "sammlerreferenz",
            "lastschrift_ursprungsbetrag",
            "auslagenersatz_ruecklastschrift",
            "beguenstigter_zahlungspflichtiger",
            "iban",
            "bic",
            "betrag",
            "waehrung",
            "info",
        ]); // einfach nochmal weil borrow move zeugs, keine ahnung, machs besser...
        let row: Record = match record.deserialize(Some(&header)){
            Ok(row) => row,
            Err(error) => {
                println!("Are you shure sparkasse is the right format for your csv?");
                panic!("{}",error)
            }
        };        // println!("{:?}", row);
        records.push(CsvRecord {
            begunst: row.beguenstigter_zahlungspflichtiger,
            verwend: row.verwendungszweck,
            betrag: row.betrag,
            datum: row.buchungstag,
        });
    }
    Ok(records)
}
