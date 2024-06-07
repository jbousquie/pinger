use std::{fs, process};

const INPUT_FILE: &str = "a_generer.txt";
const OUTPUT_FILE: &str = "adresses.txt";

fn parse_input_file(filename: &str) -> String {
    let mut generated = "".to_string();
    if let Ok(input_file) = fs::read_to_string(filename) {
        let lines: Vec<&str> = input_file.split("\n").collect();
        for line in lines {
            // si la ligne contient un caractère spécial "-" ou "*", on l'analyse
            if line.contains("-") || line.contains("*") {
                generated = generated + "\n" + generate_addrs(line).as_str();
            }
            // sinon on la recopie directement dans la string générée
            else {
                generated = generated + line + "\n";
            }

        }
        generated
    }
    else {
        println!("Erreur de lecture du fichier d'entrée {filename}");
        process::exit(0x0100);
    }
}

fn generate_addrs(line: &str) -> String {
    let mut ips = "".to_string();
    let blank = "".to_string();
    let octets: Vec<&str> = line.split(".").collect();
    for i in 1..octets.len() {
        let octet = octets[1];
        let mut start;
        let mut end ;
        // soit l'octet courant est égal à "*"
        if octet == "*" {
            start = 0;
            end = 255;
        } else
        // soit l'octet courant contient "-"
        if octet.contains("-") {

            let start_end: Vec<&str> = octet.split("-").collect();
            if start_end.len() == 2 {
                let str_start = start_end[0];
                let str_end = start_end[1];
                if let Ok(st) = str_start.parse() {
                    start = st;
                } else {
                    print_line_error(line);
                    return blank;
                }
                if let Ok(ed) = str_end.parse() {
                    end = ed;
                } else {
                    print_line_error(line);
                    return blank;
                }
            }
            else {
                print_line_error(line);
                return blank;
            }
        }
        // sinon erreur
        else {
            print_line_error(line);
            return blank;
        }
    }




    ips
}

fn print_line_error(line: &str) {
    println!("Erreur sur adresse à générer à partir de la ligne {line}. Ligne ignorée.");
}

fn main() {
    let generated_addrs = parse_input_file(INPUT_FILE);
}
