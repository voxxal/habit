use server::*;
use self::models::*;
use self::schema::users::dsl::*;
use diesel::prelude::*;

fn main() {
    let connection = establish_connection();
    let results = users.limit(10).load::<Users>(&connection).expect("error loading users");
    println!("recieved {} users", results.len());
}
