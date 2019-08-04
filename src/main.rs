#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate maud;

use self::modules::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use maud::{html, Markup};
use std::env;

pub mod modules;
pub mod schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> Markup {
    let user_name = "Leon";
    html! {
        html {
            head {
                title { "Test website" }
                meta charset="UTF-8";
            }
            body {
                header {
                   nav {
                       ul {
                            li { "Home" }
                            li { "Contact Us" }
                       }
                    }
                }
                main {
                    h1 { "Welcome to our test template!" }
                    p { "Hello, " (user_name) "!" }
                }
                footer {
                    p { "Copyright © 2019 - Leon Ebel" }
                }
            }
        }
    }
}

fn main() {
    use schema::users::dsl::*;
    let connection = establish_connection();
    let all_users = users
        .load::<User>(&connection)
        .expect("Error loading users");

    println!("{:?}", all_users);

    rocket::ignite().mount("/", routes![index]).launch();
}
