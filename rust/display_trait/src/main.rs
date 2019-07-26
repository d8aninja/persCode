struct Person {
    name: String,
    age: u8,
}

impl std::fmt::Display for Person {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} {}", self.name, self.age);
        Ok(())
    }
}

struct People {
    name: String,
    population: Vec<Person>,
    as_of_year: u16,
}

impl std::fmt::Display for People {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let mut i = self.population.len() - 1;
        loop {
            write!(fmt, "{} {} {}\n", self.name, self.population[i], self.as_of_year);
            if i == 0 {
                break;
            } else {
                i -= 1;
            }
        }
        Ok(())
    }
}

fn main() {
    let p_1 = Person{ name: String::from("Jeff"), age: 34};
    let p_2 = Person{ name: String::from("Todd"), age: 37};
    let p_3 = Person{ name: String::from("Frank"), age: 30};

    let ppl = People{name: String::from("Verizon"), population: vec![p_1, p_2, p_3], as_of_year: 2019};

    let len = ppl.population.len();

    println!("{}", ppl);
    println!("{}", len);
}