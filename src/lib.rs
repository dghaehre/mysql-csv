extern crate csv;
use std::process;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::collections::HashMap;

pub struct Arguments {
    pub input: String,
    pub output: String
}

struct Rows {
    id: u16,
    data: HashMap<String, String>
}

struct CsvData {
    filename: String,
    rows: Vec<Rows>,
    header: Vec<String>
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        let length = args.len();
        if length >= 4 || length <= 2 {
            return Err("Wrong arguments");
        } else {
            return Ok(Arguments { input: args[1].clone(), output: args[2].clone() });
        }
    }
}


impl Rows {
    fn new(id: u16) -> Rows {
        return Rows { id: id, data: HashMap::new()}
    }
    fn add_line(&mut self, line: String) {
        if let Some((key, value)) = parse_line(&line) {
            self.data.insert(String::from(key), String::from(value));
        }
    }
}

fn parse_line(line: &String) -> Option<(String, String)> {
        let vec: Vec<&str> = line.split(":").collect();
        if vec.len() > 1 {
            let key = vec[0];
            let value = vec[1..]
                            .iter()
                            .fold(String::from(""), |value, text| {
                                value + text.trim()
                            });

            Some((key.trim().to_string(), value))
        } else {
            None
        }
}

fn get_id(line: &str) -> u16 {
    let vec: Vec<&str> = line.split("").collect();
    let id: u16 = vec
        .iter()
        .map(|chr| {
            let parsed = chr.parse::<u16>();
            let nr = match parsed {
                Ok(nr) => (true, nr),
                Err(_) => (false, 0)
            };
            nr
        })
        .fold(0, |id, (is_number, nr)| {
            if is_number {
                if id == 0 {
                    nr
                } else {
                    if nr < 10 {
                        id * 10 + nr
                    } else {
                        nr
                    }
                }
            } else {
                id
            }
        });
    id
}

fn parse_file(lines: Vec<&str>) -> Result<Vec<Rows>, &'static str> {
    let notstar = String::from("notstar").chars().next().unwrap();
    let rows: Vec<Rows> = lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                let first_char = match line.chars().next() {
                    Some(f) => f,
                    None => notstar
                };
                if first_char == '*' {
                    let id: u16 = get_id(&line);
                    (true, line, id)
                } else {
                    (false, line, i as u16)
                }
            })
            .fold(vec![], | mut rows, (new_row, line, id)| {
                if new_row {
                    rows.push(Rows::new(id));
                } else {
                    let i = rows.len() - 1;
                    rows[i].add_line(String::from(line.clone()));
                }
                rows
            });
    Ok(rows)
}


pub fn parse_arguments(args: &[String]) -> Arguments {
    Arguments::new(&args).unwrap_or_else(|err| {
        println!("{} \r\nCould not understand your arguments.. \r\nArguments should include = [filepath] [output filename]", err);
        process::exit(1);
    })
}

fn create_header(rows: &Vec<Rows>) -> Result<Vec<String>, &'static str> {
    if rows.len() < 2 {
        return Err("Could find enough lines");
    }
    let mut header: Vec<String> = Vec::new();
    let row = rows[0].data.clone();
    for (key, value) in &row {
        if key != "id" && key != "id " {
            header.push(key.to_string());
        }
    }
    Ok(header)
}

fn write_file(data: CsvData) -> Result<(), Box<Error>> {
    let mut wtr = csv::Writer::from_path(&data.filename)?;
    let default_no = String::from("NOHEADER");

    wtr.write_record(&data.header)?;

    for line in &data.rows {
        // TODO: Match header!

        let vec: Vec<String> = data.header
                                .iter()
                                .map(|head| {
                                    match line.data.get(head) {
                                        Some(value) => str::replace(value, ",", "| ").clone(),
                                        None => String::from(" ")
                                    }
                                })
                                .collect();

        //  let vec: Vec<&String> = line.data
        //                         .iter()
        //                         .map(|(key, value)| {
        //                             println!("{:?} - {:?}", &key, &value);
        //                             value
                                // })
                                // .collect();

        // let vec: Vec<String> = line.data
        //                     .iter()
        //                     .map(|(key, value)| {
        //                         let exist = &data.header
        //                                 .iter()
        //                                 .filter(|head| head == &key)
        //                                 .count();

        //                         if exist.count_ones() > 0 {
        //                             value.clone()
        //                         } else {
        //                             default_no.clone()
        //                         }
        //                     })
        //                     .filter(|value| value != &default_no)
        //                     .collect();

        // println!("{:?}", &vec);
        wtr.write_record(vec)?;
    }
    // PRINT HEADER FOR DEBUG
    for head in &data.header {
        println!("{:?}", head);
    }

    wtr.flush()?;
    Ok(())
} 


pub fn run(args: Arguments) -> Result<(), Box<Error>> {
    let mut file = File::open(args.input).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let split = contents.lines();
    let lines: Vec<&str> = split.collect();
    let lines_length = lines.len();
    let rows = parse_file(lines)?;
    let rows_length = rows.len();

    let header: Vec<String> = create_header(&rows)?;

    println!("Lines in document {:?}", &lines_length);
    println!("Unique Id's in document {:?}", &rows_length);

    write_file( CsvData { filename: String::from(args.output), header: header, rows: rows })

}

    
