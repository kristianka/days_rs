use chrono::NaiveDate;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error, Write};
use std::path::PathBuf;
use std::process;

mod help_prints;

#[derive(Debug)]
struct Event {
    /// YYYY-MM-DD, like 2023-05-11
    date: NaiveDate,
    category: String,
    description: String,
}

// Create a new event
impl Event {
    fn new(date: NaiveDate, category: String, description: String) -> Self {
        Self {
            date,
            category,
            description,
        }
    }
}

#[derive(Debug)]
struct CsvData {
    events_path: PathBuf,
    temp_path: PathBuf,
    events: Vec<Event>,
}

/// Prints the event in the format
/// YYYY-MM-DD: description (category) - in X days /  X days ago / today
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

/// Select the correct string to print for the delta
fn format_delta(delta: &i64) -> String {
    if delta < &0 {
        if delta == &-1 {
            return "yesterday".to_string();
        } else {
            format!("{} days ago", delta.abs())
        }
    } else if delta > &0 {
        format!("in {} days", delta)
    } else {
        "today".to_string()
    }
}

/// Read the events.csv file and return event vector, events and tmp paths
fn read_csv_file() -> Result<CsvData, Error> {
    // Check if user is on Windows or Linux
    let home_directory_string = match env::var("HOME") {
        Ok(home_string) => home_string,
        // If not on Linux, check if on Windows
        Err(_) => match env::var("USERPROFILE") {
            Ok(user_profile_string) => user_profile_string,
            Err(_) => {
                eprintln!("Unable to determine home directory");
                process::exit(1);
            }
        },
    };

    // Path for events.csv file
    let mut days_path = PathBuf::from(home_directory_string);
    days_path.push(".days");

    if !days_path.exists() {
        println!("{} does not exist, please create it", days_path.display());
        process::exit(1);
    }

    // Paths to events.csv and events.csv.tmp
    let events_path = days_path.join("events.csv");
    let temp_path = days_path.join("events.csv.tmp");

    // Vector for events
    let mut events = Vec::new();
    let csv = std::fs::read_to_string(&events_path)?;

    // Reader options
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .from_reader(csv.as_bytes());

    // Read the csv file
    for result in rdr.records() {
        let record = result?;
        let date_str = &record[0];
        let category = &record[1];
        let description = &record[2];

        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d");

        // Check if date is valid and push to events vector
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

    // Return the paths and events
    Ok(CsvData {
        events_path,
        temp_path,
        events,
    })
}

fn delete_an_event(
    events_path: &PathBuf,
    temp_path: &PathBuf,
    line_to_delete: String,
    event: &Event,
) {
    match File::open(&events_path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            // Open the temporary file for writing
            match OpenOptions::new().write(true).create(true).open(&temp_path) {
                Ok(file) => {
                    let mut writer = BufWriter::new(file);

                    // Add lines to new file that don't contain given event
                    for line in reader.lines() {
                        if let Ok(line) = line {
                            if !line.contains(line_to_delete.as_str()) {
                                writeln!(writer, "{}", line).unwrap();
                            }
                        }
                    }

                    // Flush the buffer to write the changes to disk
                    writer.flush().unwrap();

                    // Rename temporary events.csv.tmp to overwrite the original file
                    std::fs::rename(&temp_path, &events_path).unwrap();
                    println!(
                        "Successfully deleted event {}: {} ({})",
                        event.date, event.description, event.category
                    );
                }
                Err(e) => {
                    eprintln!("Error opening file: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error opening file: {}", e);
        }
    }
}

/// Format string to be like line in events.csv
/// YYYY-MM-DD,category,description
fn csv_format_to_event(event: &Event) -> String {
    format!(
        "{},{},{}",
        event.date.format("%Y-%m-%d"),
        event.category,
        event.description
    )
}

/// Removes commas from a string and makes it into a vector
fn separate_args_to_vector(args: &String) -> Vec<String> {
    let separated_args: Vec<String> = args.split(",").map(|s| s.to_string()).collect();
    separated_args
}

// Needed to surpress warnings for paths
#[allow(unused_assignments)]
fn main() {
    // Current time
    let now = chrono::Utc::now().naive_utc().date();
    // Arguments to vector
    let args: Vec<String> = env::args().collect();
    //println!("The arguments are: {:?} ja pituus {}", args, args.len());

    // Counter for found events
    let mut counter = 0;
    // Paths
    let mut events_path = PathBuf::new();
    let mut temp_path = PathBuf::new();
    // Vector for events
    let mut events_vector = Vec::new();

    match read_csv_file() {
        Ok(csv_data) => {
            events_path = csv_data.events_path;
            temp_path = csv_data.temp_path;
            events_vector = csv_data.events;
        }
        Err(e) => {
            eprintln!("Error reading events.csv file: {}", e);
            process::exit(1);
        }
    }

    // Arguments to compare args to. For listing, adding and deleting events
    let arg_list = "list";
    let arg_today = "--today";
    let arg_before_date = "--before-date";
    let arg_after_date = "--after-date";
    let arg_date = "--date";
    let arg_categories = "--categories";
    let arg_exclude = "--exclude";
    let arg_no_category = "--no-category";
    let arg_add = "add";
    let arg_category = "--category";
    let arg_description = "--description";
    let mut category = "";
    let mut description = "";
    let arg_delete = "delete";
    let arg_dry_run = "--dry-run";
    let arg_all = "--all";
    let arg_between = "--between";

    if args.len() == 1 {
        println!("No arguments entered. Use --help for help.");
        process::exit(0);
    }

    let arg_help = "--help";
    if args[1] == arg_help {
        counter += 1;
        if args.len() > 2 {
            if args[2] == arg_list {
                help_prints::help_list();
            }
            if args[2] == arg_add {
                help_prints::help_add();
            }
            if args[2] == arg_delete {
                help_prints::help_delete();
            }
        } else {
            println!("Available help commands are: list, add, delete. Example: 'days --help list'");
        }
    }

    // Arguments starting with list
    if args.len() > 1 && args[1] == arg_list {
        // List all events
        if args.len() == 2 {
            for e in events_vector.iter() {
                print_day_format(&e);
                counter += 1;
            }
        }

        // List events by today
        if args.len() > 2 && args[2] == arg_today {
            for e in events_vector.iter() {
                if e.date == now {
                    print_day_format(&e);
                    counter += 1;
                }
            }
        }

        // List events by before, after or both
        if args.len() > 2
            && (args[2] == arg_before_date || args[2] == arg_after_date || args[2] == arg_date)
        {
            if args.len() < 4 {
                eprintln!("No date given");
                process::exit(1);
            }

            let mut before = false;
            let mut after = false;
            let mut on_this_date = false;
            let mut date1 = NaiveDate::parse_from_str(&args[3], "%Y-%m-%d");
            let mut date2 = NaiveDate::parse_from_str(&args[3], "%Y-%m-%d");

            if args[2] == arg_before_date {
                before = true;
            }

            if args[2] == arg_after_date {
                after = true;
            }

            if args.len() > 4 && args[2] == arg_before_date && args[4] == arg_after_date {
                before = true;
                after = true;
                date1 = NaiveDate::parse_from_str(&args[3], "%Y-%m-%d");
                date2 = NaiveDate::parse_from_str(&args[5], "%Y-%m-%d");
            }

            if args[2] == arg_date {
                on_this_date = true;
            }

            for e in events_vector.iter() {
                if before {
                    if e.date < date1.unwrap() {
                        print_day_format(&e);
                        counter += 1;
                    }
                }
                if after {
                    if e.date > date2.unwrap() {
                        print_day_format(&e);
                        counter += 1;
                    }
                }
                if on_this_date {
                    if e.date == date1.unwrap() {
                        print_day_format(&e);
                        counter += 1;
                    }
                }
            }
        }

        // Categories, still inside list
        // if argument after list is --categories
        if args.len() > 2 && (args[2] == arg_categories || args[2] == arg_no_category) {
            // Print events with no categories
            if args[2] == arg_no_category {
                for e in events_vector.iter() {
                    if e.category == "" {
                        print_day_format(&e);
                        counter += 1;
                    }
                }
            } else {
                // Check if there are enough arguments
                if args.len() < 4 {
                    eprintln!("No category given");
                    process::exit(1);
                }

                // Separate the categories into a vector
                let arg_categories: Vec<String> = separate_args_to_vector(&args[3]);
                // Check if user gave --exclude argument
                let exclude: bool = args.len() > 4 && args[4] == arg_exclude;

                for event in events_vector.iter() {
                    if exclude {
                        if !arg_categories.contains(&event.category) {
                            print_day_format(&event);
                            counter += 1;
                        }
                    } else {
                        if arg_categories.contains(&event.category) {
                            print_day_format(&event);
                            counter += 1;
                        }
                    }
                }
            }
        }
    }

    // Arguments starting with add
    if args.len() > 1 && args[1] == arg_add {
        if args.len() < 4 {
            eprintln!("No date given or too few arguments");
            process::exit(1);
        }

        let mut date = NaiveDate::parse_from_str(&args[3], "%Y-%m-%d");

        if args[2] != arg_date {
            date = Ok(now);
        }

        if date.is_err() {
            eprintln!("Bad date given");
            process::exit(1);
        }

        for i in 2..args.len() {
            if args[i] == arg_category {
                category = &args[i + 1];
            }
            if args[i] == arg_description {
                description = &args[i + 1];
            }
        }
        let event = Event::new(date.unwrap(), category.to_string(), description.to_string());
        let event_formatted = csv_format_to_event(&event);

        match OpenOptions::new().append(true).open(&events_path) {
            Ok(file) => {
                let mut writer = BufWriter::new(file);

                match writeln!(writer, "{}", event_formatted) {
                    Ok(_) => {
                        println!(
                            "Successfully added event {}: {} ({})",
                            event.date, event.description, event.category
                        );
                        counter += 1;
                    }
                    Err(e) => {
                        eprintln!("Error writing to file: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error opening file: {}", e);
            }
        }
    }

    // Arguments starting with delete
    if args.len() > 1 && args[1] == arg_delete {
        if args.len() < 3 {
            eprintln!("No date or no enough arguments given.\n");
            process::exit(1);
        }

        let length = args.len() - 1;

        // If --description is given as first argument after delete
        if args[2] == arg_description || args[2] == arg_category {
            let is_description: bool = args[2] == arg_description;

            for event in events_vector.iter() {
                // Check if the given arguments matches, if user gave --description
                // search for matching descriptions and ignore categories
                if (is_description && event.description.starts_with(&args[3]))
                    || (!is_description && event.category == args[3])
                {
                    // Check for dry-run
                    if args.len() > 4 && args[length] == arg_dry_run {
                        println!(
                            "{}: {} ({}) would have been deleted without dry-run",
                            event.date, event.category, event.description
                        );
                    // Delete events for real if dry-run not given
                    } else {
                        let event_formatted = csv_format_to_event(&event);
                        delete_an_event(&events_path, &temp_path, event_formatted, &event);
                    }
                    counter += 1;
                }
            }
        }

        // If --date is given as first argument after delete
        if args.len() > 3 && args[2] == arg_date {
            let date = NaiveDate::parse_from_str(&args[3], "%Y-%m-%d");

            if date.is_err() {
                eprintln!("Bad date given");
                process::exit(1);
            }

            let has_category: bool = args.len() > 5 && args[4] == arg_category;
            let has_description: bool = args.len() > 6 && args[4] == arg_description;

            if has_category {
                for i in 2..args.len() {
                    if args[i] == arg_category {
                        category = &args[i + 1];
                    }
                    if args[i] == arg_description {
                        description = &args[i + 1];
                    }
                }
            }

            for event in events_vector.iter() {
                // If category is given, find events with given date and category
                if has_category {
                    // If description is not given, find events with given date and category
                    if !has_description {
                        if event.date == date.unwrap() && event.category == category {
                            // Check for dry-run
                            if args.len() > 4 && args[length] == arg_dry_run {
                                println!(
                                    "{}: {} ({}) would have been deleted without dry-run",
                                    event.date, event.description, event.category
                                );
                            // Delete events for real if dry-run not given
                            } else {
                                let event_formatted = csv_format_to_event(&event);
                                delete_an_event(&events_path, &temp_path, event_formatted, &event);
                            }
                            counter += 1;
                        }
                    }
                    // If description is given, find events with given date, category and description
                    if has_description {
                        if event.date == date.unwrap()
                            && event.category == category
                            && event.description.starts_with(description)
                        {
                            // Check for dry-run
                            if args.len() > 4 && args[length] == arg_dry_run {
                                println!(
                                    "{}: {} ({}) would have been deleted without dry-run",
                                    event.date, event.description, event.category
                                );
                            // Delete events for real if dry-run not given
                            } else {
                                let event_formatted = csv_format_to_event(&event);
                                delete_an_event(&events_path, &temp_path, event_formatted, event);
                            }
                            counter += 1;
                        }
                    }
                }

                // If category is not given, find events just with given date
                if !has_category {
                    if event.date == date.unwrap() {
                        // Check for dry-run
                        if args.len() > 4 && args[length] == arg_dry_run {
                            println!(
                                "{}: {} ({}) would have been deleted without dry-run",
                                event.date, event.description, event.category
                            );
                        // Delete events for real if dry-run not given
                        } else {
                            let event_formatted = csv_format_to_event(&event);
                            delete_an_event(&events_path, &temp_path, event_formatted, &event);
                        }
                        counter += 1;
                    }
                }
            }
        }

        if args[2] == arg_all {
            // Check for dry-run
            if args.len() > 3 && args[length] == arg_dry_run {
                for event in events_vector.iter() {
                    println!(
                        "{}: {} ({}) would have been deleted without dry-run",
                        event.date, event.description, event.category
                    );
                }
            }
            // Delete events for real if dry-run not given
            if args.len() == 3 {
                for event in events_vector.iter() {
                    let event_formatted = csv_format_to_event(&event);
                    // submits all lines and deletes them
                    delete_an_event(&events_path, &temp_path, event_formatted, &event);
                }
            }
            counter += 1;
        }

        if args[2] == arg_between {
            if args.len() != 5 && args.len() != 6 {
                eprintln!("No dates given or wrong formatting.\n");
                process::exit(1);
            }

            let date1 = NaiveDate::parse_from_str(&args[3], "%Y-%m-%d");
            let date2 = NaiveDate::parse_from_str(&args[4], "%Y-%m-%d");

            if date1.is_err() || date2.is_err() {
                eprintln!("Bad date given");
                process::exit(1);
            }

            for event in events_vector {
                if event.date >= date1.unwrap() && event.date <= date2.unwrap() {
                    // Check for dry-run
                    if args.len() > 5 && args[length] == arg_dry_run {
                        println!(
                            "{}: {} ({}) would have been deleted without dry-run",
                            event.date, event.description, event.category
                        );
                    // Delete events for real if dry-run not given
                    } else {
                        let event_formatted = csv_format_to_event(&event);
                        delete_an_event(&events_path, &temp_path, event_formatted, &event);
                    }
                    counter += 1;
                }
            }
        }
    }

    // If no events were printed, print this
    if counter == 0 {
        println!("No events found");
    }
    // Empty line for readability in the command line
    println!("");
}
