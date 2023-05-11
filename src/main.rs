use chrono::NaiveDate;
use std::env;

use std::io::Error;
use std::path::PathBuf;
use std::process;

#[derive(Debug)]
struct Event {
    date: NaiveDate,
    category: String,
    description: String,
}

#[derive(Debug)]
struct CsvData {
    days_path: PathBuf,
    temp_path: PathBuf,
    events: Vec<Event>,
}

fn print_day_format(event: &Event) {
    let delta = (event.date - chrono::Utc::now().naive_utc().date()).num_days();
    let line = event.date.format("%Y-%m-%d").to_string()
        + ": "
        + &event.description
        + " ("
        + &event.category
        + ") - "
        + &format_delta(&delta);
    println!("{}", line);
}

fn format_delta(delta: &i64) -> String {
    if delta < &0 {
        format!("{} days ago", delta.abs())
    } else if delta > &0 {
        format!("in {} days", delta)
    } else {
        "today".to_string()
    }
}

fn read_csv_file() -> Result<CsvData, Error> {
    let home_directory_string = match env::var("HOME") {
        Ok(home_string) => home_string,
        Err(_) => match env::var("USERPROFILE") {
            Ok(user_profile_string) => user_profile_string,
            Err(_) => {
                eprintln!("Unable to determine home directory");
                process::exit(1);
            }
        },
    };

    let mut days_path = PathBuf::from(home_directory_string);
    days_path.push(".days");

    if !days_path.exists() {
        println!("{} does not exist, please create it", days_path.display());
        process::exit(1);
    }

    let events_path = days_path.join("events.csv");
    let temp_path = days_path.join("events.csv.tmp");

    let csv = std::fs::read_to_string(events_path)?;

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(csv.as_bytes());

    let mut events = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let date_str = &record[0];
        let category = &record[1];
        let description = &record[2];

        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
        if let Ok(date) = date {
            events.push(Event {
                date,
                category: category.to_owned(),
                description: description.to_owned(),
            });
        } else {
            eprintln!("bad date: {}", date_str);
        }
    }

    if events.is_empty() {
        println!("No events found");
        process::exit(0);
    }

    Ok(CsvData {
        days_path,
        temp_path,
        events,
    })
}

#[allow(dead_code)]
fn main() {
    let mut days_path = PathBuf::new();
    let mut temp_path = PathBuf::new();
    let mut events_vector = Vec::new();

    match read_csv_file() {
        Ok(csv_data) => {
            days_path = csv_data.days_path;
            temp_path = csv_data.temp_path;
            events_vector = csv_data.events;
        }
        Err(e) => {
            eprintln!("Error reading events.csv file: {}", e);
            process::exit(1);
        }
    }

    // Current time
    let now = chrono::Utc::now().naive_utc().date();
    let args: Vec<String> = env::args().collect();
    //println!("The arguments are: {:?} ja pituus {}", args, args.len());

    let arg_list = "list";
    let arg_today = "--today";
    let arg_before_date = "--before-date";
    let arg_after_date = "--after-date";
    let arg_date = "--date";
    let arg_categories = "--categories";
    let arg_no_category = "--no-category";

    // let arg_add = "--add";

    if args[1] == arg_list {
        if args.len() == 2 {
            for e in events_vector.iter() {
                print_day_format(&e);
            }
        }

        if args.len() > 2 && args[2] == arg_today {
            for e in events_vector.iter() {
                if e.date == now {
                    print_day_format(&e);
                }
            }
        }

        if args.len() > 2 && args[2] == arg_before_date {
            println!("Events before date");
            return;
        }

        if args.len() > 2 && args[2] == arg_after_date {
            println!("Events after date");
            return;
        }

        if args.len() > 2 && args[2] == arg_date {
            println!("Events on date");
        }

        if args.len() > 2 && args[2] == arg_categories {
            println!("Events in categories");
        }

        if args.len() > 2 && args[2] == arg_no_category {
            println!("Events with no category");
        }
    }
}
