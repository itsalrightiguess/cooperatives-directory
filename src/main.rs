use sqlite;
use sqlite::State;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use debug_print::{debug_print, debug_println, debug_eprint, debug_eprintln};

fn main() {
	let mut option = String::new();

	loop {
		option = String::new();

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
}

fn create_table() {
	let connection = sqlite::open("db.sqlite").unwrap();

	let query = "CREATE TABLE coops (name TEXT, products TEXT, products_type TEXT, url TEXT, country TEXT, region TEXT, vicinity TEXT, city TEXT, address TEXT, phone_number TEXT, email_address TEXT, notes TEXT)";

	connection.execute(query).unwrap();
}

fn add_to_table() {
	let connection = sqlite::open("db.sqlite").unwrap();

	let mut name = String::new();
	let mut products = String::new();
	let mut products_type = String::new();
	let mut url = String::new();
	let mut country = String::new();
	let mut region = String::new();
	let mut vicinity = String::new();
	let mut city = String::new();
	let mut address = String::new();
	let mut phone_number = String::new();
	let mut email_address = String::new();
	let mut notes = String::new();

	println!("Enter a name");
	io::stdin().read_line(&mut name).expect("error: unable to read user input");
	let mut name_final = name.clone();
	name_final.pop();

	println!("Enter products");
	io::stdin().read_line(&mut products).expect("error: unable to read user input");
	let mut products_final = products.clone();
	products_final.pop();

	println!("Enter products type");
	io::stdin().read_line(&mut products_type).expect("error: unable to read user input");
	let mut products_type_final = products_type.clone();
	products_type_final.pop();

	println!("Enter a url");
	io::stdin().read_line(&mut url).expect("error: unable to read user input");
	let mut url_final = url.clone();
	url_final.pop();

	println!("Enter a country");
	io::stdin().read_line(&mut country).expect("error: unable to read user input");
	let mut country_final = country.clone();
	country_final.pop();

	println!("Enter a region");
	io::stdin().read_line(&mut region).expect("error: unable to read user input");
	let mut region_final = region.clone();
	region_final.pop();

	println!("Enter a vicinity");
	io::stdin().read_line(&mut vicinity).expect("error: unable to read user input");
	let mut vicinity_final = vicinity.clone();
	vicinity_final.pop();

	println!("Enter a city");
	io::stdin().read_line(&mut city).expect("error: unable to read user input");
	let mut city_final = city.clone();
	city_final.pop();

	println!("Enter an address");
	io::stdin().read_line(&mut address).expect("error: unable to read user input");
	let mut address_final = address.clone();
	address_final.pop();

	println!("Enter a phone number");
	io::stdin().read_line(&mut phone_number).expect("error: unable to read user input");
	let mut phone_number_final = phone_number.clone();
	phone_number_final.pop();

	println!("Enter an email address");
	io::stdin().read_line(&mut email_address).expect("error: unable to read user input");
	let mut email_address_final = email_address.clone();
	email_address_final.pop();

	println!("Enter a note");
	io::stdin().read_line(&mut notes).expect("error: unable to read user input");
	let mut notes_final = notes.clone();
	notes_final.pop();

	let query = format!("INSERT INTO coops VALUES ('{}','{}','{}','{}','{}','{}','{}','{}','{}','{}','{}', '{}');", name_final, products_final, products_type_final, url_final, country_final, region_final, vicinity_final, city_final, address_final, phone_number_final, email_address_final, notes_final);

	connection.execute(query).unwrap();
}

fn save_html() {
	let connection = sqlite::open("db.sqlite").unwrap();

	let query = "SELECT * FROM coops";
	let mut statement = connection.prepare(query).unwrap();

	let mut mytable : HashMap<String, Vec<HashMap<String, String>>> = HashMap::new();
	let mut myindex : HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

	while let Ok(State::Row) = statement.next() {
		if (myindex.contains_key(&statement.read::<String, _>("country").unwrap()) == false) {
			myindex.insert(statement.read::<String, _>("country").unwrap(), HashMap::new());
		}

		if (myindex.get(&statement.read::<String, _>("country").unwrap()).unwrap().contains_key(&statement.read::<String, _>("region").unwrap()) == false) {
			let mut myindex_country = myindex.get_mut(&statement.read::<String, _>("country").unwrap()).unwrap();
			myindex_country.insert(statement.read::<String, _>("region").unwrap(), Vec::new());
		}

		if (myindex.get(&statement.read::<String, _>("country").unwrap()).unwrap().get(&statement.read::<String, _>("region").unwrap()).unwrap().contains(&statement.read::<String, _>("vicinity").unwrap()) == false) {
			let mut myindex_country = myindex.get_mut(&statement.read::<String, _>("country").unwrap()).unwrap();
			let mut myindex_region = myindex_country.get_mut(&statement.read::<String, _>("region").unwrap()).unwrap();
			myindex_region.push(statement.read::<String, _>("vicinity").unwrap());
		}

		let mashup = format!("{}_{}_{}", &statement.read::<String, _>("country").unwrap(), &statement.read::<String, _>("region").unwrap(), &statement.read::<String, _>("vicinity").unwrap());
		if (mytable.contains_key(&mashup) == false) {
			mytable.insert(mashup.clone(), Vec::new());
		}

		let mut the_vector = mytable.get_mut(&mashup).unwrap();
		let mut the_item = HashMap::new();

		the_item.insert(String::from("name"), statement.read::<String, _>("name").unwrap());
		the_item.insert(String::from("products"),  statement.read::<String, _>("products").unwrap());
		the_item.insert(String::from("products_type"), statement.read::<String, _>("products_type").unwrap());
		the_item.insert(String::from("url"), statement.read::<String, _>("url").unwrap());
		the_item.insert(String::from("country"), statement.read::<String, _>("country").unwrap());
		the_item.insert(String::from("region"), statement.read::<String, _>("region").unwrap());
		the_item.insert(String::from("vicinity"), statement.read::<String, _>("vicinity").unwrap());
		the_item.insert(String::from("city"), statement.read::<String, _>("city").unwrap());
		the_item.insert(String::from("address"), statement.read::<String, _>("address").unwrap());
		the_item.insert(String::from("phone_number"), statement.read::<String, _>("phone_number").unwrap());
		the_item.insert(String::from("email_address"), statement.read::<String, _>("email_address").unwrap());
		the_item.insert(String::from("notes"), statement.read::<String, _>("notes").unwrap());

		the_vector.push(the_item);
	}

	for x in mytable {
		save_page(x.0, x.1);
	}

	save_index(myindex);
}

fn save_index(index : HashMap<String, HashMap<String, Vec<String>>>) {
	// write the countries index
	let mut index_output4 = String::new();
	index_output4 += "<!DOCTYPE html><html lang=\"en\"><head><title>Worker Coöperatives Directory</title><meta charset=\"UTF-8\"></head><body>";

	index_output4 += "<ul>";

	let x = index.keys();
	for y in x {
		let z = y.clone();
		index_output4 += &format!("<li><a href=\"{}.html\">{}</a></li>", y, z);
	}
	index_output4 += "</ul>";

	index_output4 += "</body></html>";

	let my2path = Path::new("index.html");
	let my2display = my2path.display();
	let mut my2file = match File::create(&my2path) {
		Err(why) => panic!("Couldn't create {}: {}", my2display, why),
		Ok(my2file) => my2file,
	};

	match my2file.write_all(index_output4.as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", my2display, why),
		Ok(_) => println!("successfully wrote to {}", my2display),
	}

	// write the regions indexes
	for x in &index {
		let mut region_output = String::new();
		region_output += "<!DOCTYPE html><html lang=\"en\"><head><title>Worker Coöperatives Directory</title><meta charset=\"UTF-8\"></head><body>";

		region_output += "<ul>";
		for y in x.1 {
			region_output += &format!("<li><a href=\"{}.html\">{}</a></li>", y.0, y.0);
		}
		region_output += "</ul>";

		region_output += "</body></html>";

		let region_path_str = String::from(&format!("{}.html", x.0));
		let region_path = Path::new(&region_path_str);
		let region_display = region_path.display();
		let mut region_file = match File::create(&region_path) {
			Err(why) => panic!("Couldn't create {}: {}", region_display, why),
			Ok(region_file) => region_file,
		};

		match region_file.write_all(region_output.as_bytes()) {
			Err(why) => panic!("couldn't write to {}: {}", region_display, why),
			Ok(_) => println!("successfully wrote to {}", region_display),
		}
	}

	// write the vicinity indexes
	for x in &index {
		for y in x.1 {
			let mut vicinity_output = String::new();
			vicinity_output += "<!DOCTYPE html><html lang=\"en\"><head><title>Worker Coöperatives Directory</title><meta charset=\"UTF-8\"></head><body>";

			vicinity_output += "<ul>";
			for z in y.1 {
				let melded = format!("{}_{}_{}", x.0, y.0, z);
				vicinity_output += &format!("<li><a href=\"{}.html\">{}</a></li>", melded, z);
			}
			vicinity_output += "</ul>";

			vicinity_output += "</body></html>";

			let vicinity_path_str = String::from(&format!("{}.html", y.0));
			let vicinity_path = Path::new(&vicinity_path_str);
			let vicinity_display = vicinity_path.display();
			let mut vicinity_file = match File::create(&vicinity_path) {
				Err(why) => panic!("Couldn't create {}: {}", vicinity_display, why),
				Ok(vicinity_file) => vicinity_file,
			};

			match vicinity_file.write_all(vicinity_output.as_bytes()) {
				Err(why) => panic!("couldn't write to {}: {}", vicinity_display, why),
				Ok(_) => println!("successfully wrote to {}", vicinity_display),
			}
		}
	}
}

fn save_page(file_path: String, table: Vec<HashMap<String, String>>) {
	let mut path_provisional = file_path.clone();
	path_provisional += ".html";
	let path = Path::new(&path_provisional);
	let display = path.display();
	let mut file = match File::create(&path) {
		Err(why) => panic!("Couldn't create {}: {}", display, why),
		Ok(file) => file,
	};

	let mut output_text = String::new();
	output_text += "<!DOCTYPE html><html lang=\"en\"><head><title>Worker Coöperatives Directory</title><meta charset=\"UTF-8\"></head><body>";

	output_text += "<table>";
	output_text += "<tr>";
	output_text += "<th>Name</th>";
	output_text += "<th>Products</th>";
	output_text += "<th>Products Type</th>";
	output_text += "<th>Country</th>";
	output_text += "<th>Region</th>";
	output_text += "<th>City</th>";
	output_text += "<th>Address</th>";
	output_text += "<th>Phone Number</th>";
	output_text += "<th>Email Address</th>";
	output_text += "<th>Notes</th>";
	output_text += "</tr>";

	for i in &table {
		output_text += "<tr>";
		output_text += &String::from(format!("<td><a href=\"{}\">{}</a></td>", i.get(&String::from("url")).unwrap_or(&String::from("")), i.get(&String::from("name")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("products")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("products_type")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("country")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("region")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("city")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("address")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("phone_number")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("email_address")).unwrap_or(&String::from(""))));
		output_text += &String::from(format!("<td>{}</td>", i.get(&String::from("notes")).unwrap_or(&String::from(""))));
		output_text += "</tr>";
	}

	output_text += "</table>";
	output_text += "</body></html>";

	match file.write_all(output_text.as_bytes()) {
		Err(why) => panic!("couldn't write to {}: {}", display, why),
		Ok(_) => println!("successfully wrote to {}", display),
	}
}
