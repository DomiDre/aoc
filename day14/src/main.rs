use std::io::{Error, BufRead, BufReader};
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;

struct NanoFactory {
    recipes: HashMap<String, Element>,
    surplus_book: HashMap<String, u64>,
    ore_count: u64
}

impl NanoFactory {
    fn new(recipes: HashMap<String, Element>) -> NanoFactory {
        NanoFactory {
            recipes,
            surplus_book: HashMap::new(),
            ore_count: 0
        }
    }

    fn reset(&mut self) {
        self.surplus_book = HashMap::new();
        self.ore_count = 0;
    }
    
    /// get the required elements to produce an given element
    fn get_requirements(&mut self, element_name: &String, amount: u64) {
        
        let mut production_line = Vec::new();
        // get the recipe for the element
        let element_recipe = self.recipes.get(element_name).unwrap();
        
        let mut amount_to_produce = amount;
        
        // check if there is surplus on the book from earlier recipes
        if let Some(surplus) = self.surplus_book.get_mut(element_name) {
            if *surplus > amount_to_produce {
                *surplus = *surplus - amount_to_produce;
                amount_to_produce = 0;
            } else {
                amount_to_produce = amount_to_produce - *surplus;
                *surplus = 0;
            }
        }
        
        // determine if the desired amount divides the recipe result or if there
        // is a remainder
        let modulo = amount_to_produce % element_recipe.amount_of_product;
        if modulo != 0 {
            let surplus = element_recipe.amount_of_product - modulo;
            amount_to_produce += surplus;
            *self.surplus_book.entry(element_name.to_owned()).or_insert(0) += surplus;
        }


        for (educt_amount, educt_name) in element_recipe.requirements.iter() {
            let needed_amount = amount_to_produce/element_recipe.amount_of_product*educt_amount;
            if educt_name == "ORE" {
                self.ore_count += needed_amount;
            } else {
                production_line.push((educt_name.clone(), needed_amount));
            }
        }

        for element in production_line.iter() {
            self.get_requirements(&element.0, element.1);
        }
    }
}

#[derive(Debug, Clone)]
struct Element {
    name: String, // name of element
    requirements: Vec<(u64, String)>, // whats needed to produce it
    amount_of_product: u64 // how many are produced from requirements
}

impl Element {
    /// seperate a line into an Element object, containing the name of the element
    /// the amount that is produced, and the required elements for production with
    /// their respective amounts
    fn new(recipe: String) -> Option<Element> {
        let line_pattern = Regex::new(r"([\w\d\s,]+) => (\d+) (\w+)").unwrap();
        let line_re = line_pattern.captures(Box::leak(recipe.into_boxed_str()));

        let req_pattern = Regex::new(r"(\d+) (\w+)").unwrap();
        if let Some(value) = line_re {
            // if motivated: at better error message in any case anything fails
            let requirement_strings: Vec<_> = value.get(1)
            .map(|m| m.as_str()).unwrap()
            .split(',').map(|x| x.trim().to_string())
            .collect();
            let mut requirements: Vec<(u64, String)> = Vec::new();
            for element in requirement_strings {
                let req_re = req_pattern.captures(Box::leak(element.into_boxed_str()));
                if let Some(matched_req) = req_re {
                    let number = matched_req.get(1).map(|m| m.as_str()).unwrap()
                    .parse::<u64>().unwrap();
                    let name = matched_req.get(2).map(|m| m.as_str()).unwrap();
                    requirements.push((number, name.to_string()))
                }
            }

            let amount_of_product = value.get(2).map(|m| m.as_str()).unwrap()
            .parse::<u64>().unwrap();
            let name = value.get(3).map(|m| m.as_str()).unwrap().to_string();
            Some(Element {
                name,
                requirements,
                amount_of_product
            })
        } else {
            None
        }

    }
}

fn read_input() -> Result<HashMap<String, Element>, Error> {
    let file = File::open("./input")?;
    let buffer = BufReader::new(file);

    let mut recipe: HashMap<String, Element> = HashMap::new();
    for line in buffer.lines() {
        if let Some(element) = Element::new(line.unwrap()) {
            recipe.insert(element.name.clone(), element);
        }
    }
    Ok(recipe)
}

fn part1(recipes: HashMap<String, Element>) {
    println!("PART 1");
    // keep track if more is produced to satisfy amount_of_product minimum
    let mut factory = NanoFactory::new(recipes);
    factory.get_requirements(&"FUEL".to_string(), 1);
    println!("Minimally needed ore to produce 1 fuel: {}", factory.ore_count);
}


fn part2(recipes: HashMap<String, Element>) {
    println!("PART 2");
    let available_ore: u64 = 1_000_000_000_000;
    let mut factory = NanoFactory::new(recipes);

    let mut limit_left = 1_000_000;
    let mut limit_right = 100_000_000;
    loop {
        let amount_produced = (limit_left + limit_right)/2;
        factory.get_requirements(&"FUEL".to_string(), amount_produced);
        println!("{} ORE for {} FUEL", factory.ore_count, amount_produced);
        // if more is needed than available, shift interval to larger values
        // otherwise to smaller
        if factory.ore_count > available_ore {
            limit_right = amount_produced;
        } else {
            limit_left = amount_produced;
        }

        // if both limits next to each other -> converged
        if limit_left == limit_right - 1 {
            break;
        }
        factory.reset();
    }
    // the smaller value is the solution
    println!("With one trillion ORE one can produce {} FUEL", limit_left);
}

fn main() {
    let recipe = read_input().unwrap();
    part1(recipe.clone());
    part2(recipe);
}
