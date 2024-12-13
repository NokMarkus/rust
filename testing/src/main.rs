struct Guy {
    name: String,
    age: u32,
    country: String,
    gender: String,
    race: String,
    allergies: String,
}

fn main() {
    let markus = Guy {
        name: String::from("Markus"),
        age: 69,
        country: String::from("Finland"),
        gender: String::from("Male"),
        race: String::from("White"),
        allergies: String::from("None"),
    };
    println!(" ");

    println!("Name: {}", markus.name);
    println!("Age: {}", markus.age);
    println!("Country: {}", markus.country);
    println!("Gender: {}", markus.gender);
    println!("Race: {}", markus.race);
    println!("Allergies: {}", markus.allergies);

    println!(" ");

    // Create function
    fn new_guy(name: String, age: u32, country: String, gender: String, race: String, allergies: String) -> Guy {
        Guy {
            name,
            age,
            country,
            gender,
            race,
            allergies,
        }
    }

    let john = new_guy(String::from("John"), 42, String::from("USA"), String::from("Non-binary"), String::from("Asian"), String::from("Peanuts"));

    println!("Name: {}", john.name);
    println!("Age: {}", john.age);
    println!("Country: {}", john.country);
    println!("Gender: {}", john.gender);
    println!("Race: {}", john.race);
    println!("Allergies: {}", john.allergies);
    
    println!(" ");

}
