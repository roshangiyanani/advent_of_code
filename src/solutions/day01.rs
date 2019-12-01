use std::convert::TryFrom;
use std::num::ParseIntError;
use std::path::Path;

use crate::lib::{read_lines, Solution};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Module {
    mass: u32,
}

impl TryFrom<&str> for Module {
    type Error = ParseIntError;
    fn try_from(input: &str) -> Result<Module, Self::Error> {
        let mass = u32::from_str_radix(&input, 10)?;
        Ok(Module { mass })
    }
}

impl TryFrom<String> for Module {
    type Error = ParseIntError;
    fn try_from(input: String) -> Result<Module, Self::Error> {
        Ok(Module::try_from(input.as_str())?)
    }
}

fn fuel_requirement(mass: u32) -> u32 {
    let fuel_required = (mass / 3) as i64 - 2;
    std::cmp::max(fuel_required, 0) as u32 // can't require less than 0 fuel
}

impl Module {
    /// Fuel requirement not included the requirement of the required fuel.
    fn calculate_fuel_requirement(&self) -> u32 {
        debug_assert_ne!(self.mass, 0, "mass cannot be 0");
        fuel_requirement(self.mass)
    }

    /// Fuel requirement including the requirement of the required fuel.
    fn recursively_calculate_fuel_requirement(&self) -> u32 {
        std::iter::successors(Some(self.mass), |&mass| {
            Some(fuel_requirement(mass))
        })
        .skip(1) // don't include mass of item itself
        .take_while(|&mass| mass != 0)
        .sum()
    }
}

pub struct Day01 {}

impl Day01 {
    fn part_a<'a, I>(modules: I) -> u64
    where
        I: Iterator<Item = &'a Module>,
    {
        modules
            .map(Module::calculate_fuel_requirement)
            .map(|x| x as u64)
            .sum()
    }

    fn part_b<'a, I>(modules: I) -> u64
    where
        I: Iterator<Item = &'a Module>,
    {
        modules
            .map(Module::recursively_calculate_fuel_requirement)
            .map(|x| x as u64)
            .sum()
    }
}

impl Solution<1> for Day01 {
    fn solve(input: &Path) -> Result<(String, String), String> {
        let lines = read_lines(&input).or_else(|err| Err(err.to_string()))?;
        let modules: Vec<Module> = lines
            .map(|x| x.expect("error reading from file"))
            .map(Module::try_from)
            .map(|m| m.expect("invalid module mass"))
            .collect();
        let a = Self::part_a(modules.iter());
        let b = Self::part_b(modules.iter());
        Ok((a.to_string(), b.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lib::get_input_file;

    use test::Bencher;

    #[test]
    fn to_module() {
        let val = Module::try_from("123").unwrap();
        assert_eq!(val, Module { mass: 123 });

        assert!(Module::try_from("-123").is_err());
        assert!(Module::try_from("not a number").is_err());
    }

    #[test]
    fn calculate_fuel_requirement() {
        let module = Module { mass: 14 };
        let fuel_required = module.calculate_fuel_requirement();
        assert_eq!(fuel_required, 2);
    }

    #[test]
    fn recursively_calculate_fuel_requirement() {
        let module = Module { mass: 1969 };
        let fuel_required = module.recursively_calculate_fuel_requirement();
        assert_eq!(fuel_required, 966);
    }

    #[test]
    fn part_a() {
        let masses = vec![12, 14, 1969, 100756];
        let modules: Vec<Module> =
            masses.iter().map(|&mass| Module { mass }).collect();

        let calculated_fuel_requirement = Day01::part_a(modules.iter());
        let fuel_requirement = vec![2, 2, 654, 33583].iter().sum();
        assert_eq!(calculated_fuel_requirement, fuel_requirement);
    }

    #[test]
    fn part_b() {
        let masses = vec![14, 1969, 100756];
        let modules: Vec<Module> =
            masses.iter().map(|&mass| Module { mass }).collect();

        let calculated_fuel_requirement = Day01::part_b(modules.iter());
        let fuel_requirement = vec![2, 966, 50346].iter().sum();
        assert_eq!(calculated_fuel_requirement, fuel_requirement);
    }

    fn init() -> Vec<Module> {
        let resources = Path::new("./resources");
        let input = get_input_file::<Day01, 1>(resources);
        let lines = read_lines(&input)
            .or_else(|err| Err(err.to_string()))
            .unwrap();
        lines
            .map(|x| x.expect("error reading from file"))
            .map(Module::try_from)
            .map(|x| x.expect("invalid module mass"))
            .collect()
    }

    #[bench]
    fn part_a_full(b: &mut Bencher) {
        let modules = init();
        b.iter(|| assert_eq!(Day01::part_a(modules.iter()), 3267638));
    }

    #[bench]
    fn part_b_full(b: &mut Bencher) {
        let modules = init();
        b.iter(|| assert_eq!(Day01::part_b(modules.iter()), 4898585));
    }
}
