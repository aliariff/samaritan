#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use std::error::Error;
use std::fs::File;
use std::path::Path;

mod user;
use user::User;

#[get("/")]
fn generate() -> &'static str {
    let mut number = 100;
    let mut vec = Vec::new();

    while number != 0 {
        let user = User {
            id: number,
            name: "String".to_string(),
            month: "String".to_string(),
            size: 70,
            electricity_usage: 200,
            water_usage: 400,
            gas_usage: 50,
            zip: "52078".to_string(),
            country: "Germany".to_string(),
        };
        vec.push(user);
        number = number - 1;
    }

    vec.reverse();
    File::create("data.json")
        .map(|file| serde_json::to_writer_pretty(&file, &vec).unwrap())
        .unwrap();

    "Ok"
}

fn read_user_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<User>, Box<Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(file)?;

    // Return the `Users`.
    Ok(u)
}

#[get("/?<zip>&<country>")]
fn data(zip: String, country: String) -> Json<Vec<User>> {
    let mut users = read_user_from_file("data.json").unwrap();
    users = users
        .into_iter()
        .filter(|user| user.zip == zip && user.country == country)
        .collect();
    Json(users)
}

fn main() {
    rocket::ignite()
        .mount("/data", routes![data])
        .mount("/generate", routes![generate])
        .launch();
}
