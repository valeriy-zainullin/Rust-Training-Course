// This chapter is dedicated to the object oriented programming features of Rust.

// DYNAMIC DISPATCH
// ================================================================================================

// ----- 1 --------------------------------------
// Design a small simulation of a zoo where you have different animal types that can make noise and
// move. You have to use dynamic dispatch (trait objects) so that a collection can hold a mix of 
// different types of animals and call methods uniformly.
//
// - Define a trait `Animal` with the following methods:
//   - `fn name(&self) -> &str;`
//   - `fn make_noise(&self) -> String;`
//   - `fn move_position(&mut self, delta_x: f64, delta_y: f64);`
//   - `fn position(&self) -> (f64, f64);`
// - Create at least two concrete types implementing `Animal`, e.g. `Lion` and `BritishPigeon`. Each
//   has its own name, own noise (e.g., "Roar!", "Oi mate! Bloody hell I love fish'n'chips brof!'"),
//   and keeps track of its position `(x, y)` as `f64`.
//
// Create a struct `Zoo` that holds a vector of animals. Provide the following methods for this 
// struct:
// - `fn new -> Self`: just a basic constructor.
// - `add_animal`: adds a new animal to the zoo.
// - `make_all_noises -> Vec<String>`: calls `make_noise()` on each animal and collects the 
//   strings.
// - `move_all`: moves every animal by the given delta.
// - `positions -> Vec<(&str, (f64, f64))>`: returns a vector of tuples with each animal’s name and
//   its current position.
//
// White a small example function which creates a zoo, adds your animals there and calls the 
// `make_all_noises`, `move_all` and `positions` Zoo methods to show that they're working correctly.

use std::rc::Rc;

// Create a struct `Zoo` that holds a vector of animals. Provide the following methods for this 
// struct:
// - `fn new -> Self`: just a basic constructor.
// - `add_animal`: adds a new animal to the zoo.
// - `make_all_noises -> Vec<String>`: calls `make_noise()` on each animal and collects the 
//   strings.
// - `move_all`: moves every animal by the given delta.
// - `positions -> Vec<(&str, (f64, f64))>`: returns a vector of tuples with each animal’s name and
//   its current position.
//

trait Animal {
    /// Name of the animal (e.g. Cow)
    fn name(&self) -> &str;

    /// Returns nimal's usual noise.
    fn make_noise(&self) -> String;

    /// Move animal by a specific vector.
    fn move_position(&mut self, delta_x: f64, delta_y: f64);

    // Returns animal position.
    fn position(&self) -> (f64, f64);
}

struct Lion {
    x: f64,
    y: f64,
}

impl Lion {
    /// Creates a new lion at origin
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Animal for Lion {
    /// Returns name of the Lion ("Lion")
    fn name(&self) -> &str {
        "Lion"
    }

    /// Returns "Roar!" for a lion
    fn make_noise(&self) -> String {
        "Roar!".to_string()
    }

    /// Move animal by a specific vector.
    fn move_position(&mut self, delta_x: f64, delta_y: f64) {
        self.x += delta_x;
        self.y += delta_y;
    }

    // Returns animal's position.
    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

struct Cow {
    x: f64,
    y: f64,
}

impl Cow {
    /// Creates a new cow at origin
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

impl Animal for Cow {
    /// Returns name of the cow ("Cow")
    fn name(&self) -> &str {
        "Cow"
    }

    /// Returns "Moo!" for a cow
    fn make_noise(&self) -> String {
        "Moo!".to_string()
    }

    /// Move animal by a specific vector.
    fn move_position(&mut self, delta_x: f64, delta_y: f64) {
        self.x += delta_x;
        self.y += delta_y;
    }

    // Returns animal's position.
    fn position(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}

struct Zoo {
    animals: Vec<Box<dyn Animal>>,
}

impl Zoo {
    /// Creates an empty zoo
    fn new() -> Self {
        Self { animals: Vec::new() }
    }
    
    /// Adds an animal into the zoo
    fn add_animal<T: Animal + 'static>(&mut self, animal: T) {
        self.animals.push(Box::new(animal))
    }

    /// Returns zoo's noise
    fn make_all_noises(&self) -> Vec<String> {
        self.animals.iter()
            .map(|animal| animal.make_noise())
            .collect()
    }

    // Moves animals inside the zoo by a specific vector
    fn move_all(&mut self, delta_x: f64, delta_y: f64) {
        for animal in &mut self.animals {
            animal.move_position(delta_x, delta_y);
        }
    }

    // Returns positions of the animals inside the zoo
    fn positions(&self) -> Vec<(&str, (f64, f64))> {
        self.animals.iter()
            .map(|animal| (animal.name(), animal.position()))
            .collect()
    }
//
// White a small example function which creates a zoo, adds your animals there and calls the 
// `make_all_noises`, `move_all` and `positions` Zoo methods to show that they're working correctly.
}

#[test]
fn test_zoo() {
    let mut zoo = Zoo::new();
    zoo.add_animal(Lion::new());
    zoo.add_animal(Cow::new());
    zoo.add_animal(Cow::new());
    zoo.add_animal(Cow::new());
    zoo.add_animal(Lion::new());

    assert_eq!(zoo.make_all_noises(), vec!["Roar!", "Moo!", "Moo!", "Moo!", "Roar!"]);
    assert_eq!(zoo.positions(), vec![
        ("Lion", (0.0, 0.0)),
        ("Cow", (0.0, 0.0)),
        ("Cow", (0.0, 0.0)),
        ("Cow", (0.0, 0.0)),
        ("Lion", (0.0, 0.0)),
    ]);

    zoo.move_all(1.0, 2.0);
    assert_eq!(zoo.positions(), vec![
        ("Lion", (1.0, 2.0)),
        ("Cow", (1.0, 2.0)),
        ("Cow", (1.0, 2.0)),
        ("Cow", (1.0, 2.0)),
        ("Lion", (1.0, 2.0)),
    ]);
}

// SUPERTRAITS
// ================================================================================================

// ----- 2 --------------------------------------
// Implement the `BackTo2007` trait with the `std::fmt::Display` as a supertrait for it. Implement 
// `BackTo2007` trait for the `Account` struct which consists of `name: String` and 
// `year_of_birth: u32` fields.
//
// This `BackTo2007` trait should have just one `cringify(&self) -> String` method, which will
// make the account much more cringy by adding "★彡Xx_" to the left of the `self.to_string()` and 
// "_xX彡★" to the right. Just like that: ★彡Xx_NAGIBATOR1999_xX彡★
//
// Notice that you also should decide how to display the account.

// IMPLEMENT HERE:

trait BackTo2007: std::fmt::Display {
    /// Returns the name usable in 2007
    fn cringify(&self) -> String;
}

struct Account {
    name: String,
    year_of_birth: u32,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.name.to_uppercase(), self.year_of_birth)
    }
}

impl BackTo2007 for Account {
    fn cringify(&self) -> String {
        format!("★彡Xx_{}_xX彡★", self)
    }
}

// DEFAULT GENERIC TYPE PARAMETERS AND ASSOCIATED TYPES
// ================================================================================================

// ----- 3 --------------------------------------
// Implement a `Converter` trait with `Input` and `Output` associated types. `Input` should have a 
// `String` default type. This trait should have a `convert` method which takes a value of type 
// `Input` and returns a value of type `Output`.
//
// Implement `Converter` for two stucts:
// - `StringToIntConverter`: converts the provided String to `i32`.
// - `IntToHexConverter`: converts the provided `i32` into the String holding its hex
//   representation.

// IMPLEMENT HERE:

trait Converter {
    type Input; /* = String; Associated type defaults is not yet stabilized, this yields an error */
    type Output;

    fn convert(input: <Self as Converter>::Input) -> <Self as Converter>::Output; 
}

struct StringToIntConverter {}
impl StringToIntConverter {
    fn new() -> Self { Self {} }
}
impl Converter for StringToIntConverter {
    type Input = String;
    type Output = i32;

    fn convert(input: <Self as Converter>::Input) -> <Self as Converter>::Output {
        input.parse().unwrap()
    }
}

struct IntToHexConverter {}
impl IntToHexConverter {
    fn new() -> Self { Self {} }
}
impl Converter for IntToHexConverter {
    type Input = i32;
    type Output = String;

    fn convert(input: <Self as Converter>::Input) -> <Self as Converter>::Output {
        format!("{:x}", input)
    }
}
