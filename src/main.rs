use sqlite;
use sqlite::State;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
	let mut option = String::new();

	println!("1: create database\n2: add to database\n3: save html page\nChoose an option");
	io::stdin().read_line(&mut option).expect("error: unable to read user input");

	if option == "1\n" {
		create_table();
	} else if option == "2\n" {
		add_to_table();
	} else if option == "3\n" {
		save_html();
	} else {
		println!("That's not a valid option");
	}
}

fn create_table() {
	let connection = sqlite::open("db.sqlite").unwrap();

	let query = "CREATE TABLE coops (name TEXT, url TEXT, country TEXT, region TEXT, city TEXT, address TEXT, phone_number TEXT, email_address TEXT, notes TEXT)";

	connection.execute(query).unwrap();
}

fn add_to_table() {
	let connection = sqlite::open("db.sqlite").unwrap();

	let mut name = String::new();
	let mut url = String::new();
	let mut country = String::new();
	let mut region = String::new();
	let mut city = String::new();
	let mut address = String::new();
	let mut phone_number = String::new();
	let mut email_address = String::new();
	let mut notes = String::new();

	println!("Enter a name");
	io::stdin().read_line(&mut name).expect("error: unable to read user input");
	println!("Enter a url");
	io::stdin().read_line(&mut url).expect("error: unable to read user input");
	println!("Enter a country");
	io::stdin().read_line(&mut country).expect("error: unable to read user input");
	println!("Enter a region");
	io::stdin().read_line(&mut region).expect("error: unable to read user input");
	println!("Enter a city");
	io::stdin().read_line(&mut city).expect("error: unable to read user input");
	println!("Enter an address");
	io::stdin().read_line(&mut address).expect("error: unable to read user input");
	println!("Enter a phone number");
	io::stdin().read_line(&mut phone_number).expect("error: unable to read user input");
	println!("Enter an email address");
	io::stdin().read_line(&mut email_address).expect("error: unable to read user input");
	println!("Enter a note");
	io::stdin().read_line(&mut notes).expect("error: unable to read user input");

	let query = format!("INSERT INTO coops VALUES ('{}','{}','{}','{}','{}','{}','{}','{}','{}');", name, url, country, region, city, address, phone_number, email_address, notes);

	connection.execute(query).unwrap();
}

fn save_html() {
	let path = Path::new("out.html");
	let display = path.display();

	let mut file = match File::create(&path) {
		Err(why) => panic!("Couldn't create {}: {}", display, why),
		Ok(file) => file,
	};

	let connection = sqlite::open("db.sqlite").unwrap();

	let query = "SELECT * FROM coops";
	let mut statement = connection.prepare(query).unwrap();

	let mut output_text = String::new();

	output_text += "<!DOCTYPE html><html lang=\"en\"><head><title>Worker Coöperatives Directory</title><meta charset=\"UTF-8\"></head><body>";
	output_text += "<table>";
	output_text += "<tr>";
	output_text += "<th>Name</th>";
	output_text += "<th>Country</th>";
	output_text += "<th>Region</th>";
	output_text += "<th>City</th>";
	output_text += "<th>Address</th>";
	output_text += "<th>Phone Number</th>";
	output_text += "<th>Email Address</th>";
	output_text += "<th>Notes</th>";
	output_text += "</tr>";
	while let Ok(State::Row) = statement.next() {
		output_text += "<tr>";
		output_text += &String::from(format!("<td><a href=\"{}\">{}</a></td>", statement.read::<String, _>("url").unwrap(), statement.read::<String, _>("name").unwrap()));
		output_text += &String::from(format!("<td>{}</td>", statement.read::<String, _>("country").unwrap()));
		output_text += &String::from(format!("<td>{}</td>", statement.read::<String, _>("region").unwrap()));
		output_text += &String::from(format!("<td>{}</td>", statement.read::<String, _>("city").unwrap()));
		output_text += &String::from(format!("<td>{}</td>", statement.read::<String, _>("address").unwrap()));
		output_text += &String::from(format!("<td>{}</td>", statement.read::<String, _>("phone_number").unwrap()));
		output_text += &String::from(format!("<td>{}</td>", statement.read::<String, _>("email_address").unwrap()));
		output_text += &String::from(format!("<td>{}</td>", statement.read::<String, _>("notes").unwrap()));
		output_text += "</tr>";
	}		
	output_text += "</table>";
	output_text += "</body></html>";

	match file.write_all(output_text.as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", display, why),
		Ok(_) => println!("successfully wrote to {}", display),
	}
}
