mod to_do;

use rand::prelude::*;
use std::env;
use to_do::structs::done::Done;
use to_do::structs::pending::Pending;
use to_do::ItemTypes;
use to_do::to_do_factory;

/// This function generates a float number using a number
/// generator passed into the function.
///
/// # Arguments
/// * generator (&mut ThreadRng): the random number
/// generator to generate the random number
///
/// # Returns
/// (f64): random number between 0 -> 10
fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.gen();
    return placeholder * 10.0
}

/// This trait defines the struct to be a user.
trait IsUser {
    /// This function proclaims that the struct is a user.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// (bool) true if user, false if not
    fn is_user() -> bool {
        return true
    }
}

/// This struct defines a user
///
/// # Attributes
/// * name (String): the name of the user
/// * age (i8): the age of the user
struct User {
    name: String,
    age: i8
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path: &str = &args[0];

    if path.contains("/debug/") {
        println!("The development app is running");
    }
    else if path.contains("/release/") {
        println!("The production app is running");
    }

    let mut rng: ThreadRng = rand::thread_rng();

    let random_number = generate_float(&mut rng);
    println!("{}", random_number);

    let done: Done = Done::new("shopping");
    println!("{}", done.super_struct.title);
    println!("{}", done.super_struct.status);

    let pending: Pending = Pending::new("laundry");
    println!("{}", pending.super_struct.title);
    println!("{}", pending.super_struct.status);

    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "make");

    match to_do_item.unwrap() {
            ItemTypes::Pending(item) => println!(
                "it's a pending item with the title: {}",
                item.super_struct.title),

            ItemTypes::Done(item) => println!(
                    "it's a done item with the title: {}",
                    item.super_struct.title),
    }
}
