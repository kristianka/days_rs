pub fn help_list() {
    println!("Help for the list command:");
    println!("Usage: days list [options]");
    println!("Options:");
    println!("--today");
    println!("--before-date <date>");
    println!("--after-date <date>");
    println!("--date <date>");
    println!("--categories <category1,category2>");
    println!("--exclude <category1,category2>");
    println!("--no-category");
}

pub fn help_add() {
    println!("Help for the add command:");
    println!("Usage: days add [options]");
    println!("Options:");
    println!("--date <date> --category <category> --description <description>");
    println!("--category <category> --description <description>");
    println!("--description <description>");
}

pub fn help_delete() {
    println!("Help for the delete command:");
    println!("Usage: days delete [options]");
    println!("Note you can end every command with --dry-run to show what it'll delete. There's no undo, be careful!");
    println!("Options:");
    println!("--date <date>");
    println!("--category <category>");
    println!("--date <date> --category <category>");
    println!("--description <description>");
    println!("--date <date> --category <category> --description <description>");
}
