use serde::*;

use serde_json::Result;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
  name: String,
  age: u8,
  phones: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddNewMissingFieldPerson {
  name: String,
  age: u8,
  phones: Vec<String>,

  #[serde(default)]
  // This attr will use default value(maybe 0) if missing during
  // deserialization
  locate: String,
}

fn serde_example() -> Result<()> {
  // Some data structure.
  let person = Person {
    name: "John Doe".to_string(),
    age: 30,
    phones: vec!["+44 1234567".to_string(), "+44 2345678".to_string()],
  };

  // Serialize it to a JSON string.
  let person_json = serde_json::to_string(&person)?;

  println!("Serialized: {}", person_json);

  // Deserialize the JSON string back to a Person.
  let pd: Person = serde_json::from_str(&person_json).unwrap();

  println!("Deserialized: {:?}", pd);



  let new_pd: AddNewMissingFieldPerson = serde_json::from_str(&person_json).unwrap();

  println!("Deserialized: {:?}", new_pd);

  Ok(())
}

fn main() {
  match serde_example() {
    Err(err) => println!("Error: {}", err),
    _ => (),
  }
}
