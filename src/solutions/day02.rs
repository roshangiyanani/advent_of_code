use std::convert::From;
use std::fs::read_to_string;
use std::path::Path;

use crate::lib::Solution;

#[derive(Clone, Debug)]
struct Intcode {
    values: Vec<u32>,
    // current_pos: usize,
}

impl Intcode {
    fn new(values: Vec<u32>) -> Intcode {
        Intcode { values }
    }

    fn apply_parameters(&mut self, arg1: u8, arg2: u8) {
        assert!(
            self.values.len() >= 3,
            "not long enough to apply parameters"
        );
        assert!(arg1 <= 99, format!("arg1 ({}) must be <= 99", arg1));
        assert!(arg2 <= 99, format!("arg2 ({}) must be <= 99", arg2));

        self.values[1] = arg1 as u32;
        self.values[2] = arg2 as u32;
    }

    fn restore_1202_code(&mut self) {
        self.apply_parameters(12, 2);
    }

    /// process a single step, and return whether or not to halt
    fn process_step(&mut self, current_pos: usize) -> bool {
        let code = Opcode::from(self.values[current_pos]);
        if code == Opcode::Halt {
            true
        } else {
            debug_assert!(self.values.len() >= current_pos + 4);

            let ind1 = self.values[current_pos + 1] as usize;
            let ind2 = self.values[current_pos + 2] as usize;
            let ind3 = self.values[current_pos + 3] as usize;
            let n1 = self.values[ind1];
            let n2 = self.values[ind2];

            self.values[ind3] = match code {
                Opcode::AddAndStore => n1 + n2,
                Opcode::MultiplyAndStore => n1 * n2,
                _ => unreachable!(),
            };

            false
        }
    }

    /// process the entire input
    fn process(mut self) -> Vec<u32> {
        let mut current_pos = 0;
        let len = self.values.len();
        while current_pos < len && !self.process_step(current_pos) {
            current_pos += 4;
        }
        self.values
    }

    fn guess_parameters(mut self, target: u32) -> Result<(u8, u8), String> {
        for arg1 in 0..=99 {
            for arg2 in 0..=99 {
                let mut guess_memory = self.clone();
                guess_memory.apply_parameters(arg1, arg2);
                let guess_memory = guess_memory.process();
                if guess_memory[0] == target {
                    return Ok((arg1, arg2));
                }
            }
        }
        Err("no parameters result in the given target".to_owned())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Opcode {
    AddAndStore,
    MultiplyAndStore,
    Halt,
}

impl From<u32> for Opcode {
    fn from(code: u32) -> Opcode {
        match code {
            1 => Opcode::AddAndStore,
            2 => Opcode::MultiplyAndStore,
            99 => Opcode::Halt,
            _ => unreachable!(format!("invalid opcode {} received", code)),
        }
    }
}

pub struct Day02 {}

impl Day02 {
    fn part_a(values: Vec<u32>) -> u32 {
        let mut intcode = Intcode::new(values);
        intcode.restore_1202_code();
        let result = intcode.process();
        result[0]
    }

    fn part_b(values: Vec<u32>) -> Result<u32, String> {
        let intcode = Intcode::new(values);
        let (noun, verb) = intcode.guess_parameters(19690720)?;
        Ok((100 * noun as u32) + verb as u32)
    }
}

impl Solution<2> for Day02 {
    fn solve(input: &Path) -> Result<(String, String), String> {
        let input = read_to_string(input).or_else(|e| Err(e.to_string()))?;
        let input: Vec<_> = input
            .split(',')
            .map(|input| input.trim())
            .map(|input| u32::from_str_radix(input, 10))
            .map(|x| x.expect("invalid input"))
            .collect();
        let a = Day02::part_a(input.clone());
        let b = Day02::part_b(input)?;
        Ok((a.to_string(), b.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lib::get_input_file;

    use test::Bencher;

    #[test]
    fn apply_add_and_store() {
        let values = vec![1, 0, 0, 3];
        let mut intcode = Intcode::new(values);
        assert!(!intcode.process_step(0), "should not halt on AddAndStore");
        assert_eq!(intcode.values, vec![1, 0, 0, 2]);
    }

    #[test]
    fn apply_multiply_and_store() {
        let values = vec![2, 3, 2, 3];
        let mut intcode = Intcode::new(values);
        assert!(
            !intcode.process_step(0),
            "should not halt on MultipleAndStore"
        );
        assert_eq!(intcode.values, vec![2, 3, 2, 6]);
    }

    #[test]
    fn apply_halt() {
        let values = vec![1, 0, 0, 3, 99];
        let mut intcode = Intcode::new(values.clone());
        assert!(intcode.process_step(4), "should halt on Halt");
        assert_eq!(intcode.values, values);
    }

    #[test]
    fn apply() {
        let values = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let intcode = Intcode::new(values);
        let result = intcode.process();
        assert_eq!(result, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    // #[test]
    // fn part_b() {
    //     let masses = vec![14, 1969, 100756];
    //     let modules: Vec<Module> =
    //         masses.iter().map(|&mass| Module { mass }).collect();

    //     let calculated_fuel_requirement = Day01::part_b(modules.iter());
    //     let fuel_requirement = vec![2, 966, 50346].iter().sum();
    //     assert_eq!(calculated_fuel_requirement, fuel_requirement);
    // }

    fn init() -> Vec<u32> {
        let resources = Path::new("./resources");
        let input = get_input_file::<Day02, 2>(resources);
        let input = read_to_string(input).unwrap();
        input
            .split(',')
            .map(|input| input.trim())
            .map(|input| u32::from_str_radix(input, 10))
            .map(|x| x.expect("invalid input"))
            .collect()
    }

    #[bench]
    fn part_a_full(b: &mut Bencher) {
        let intcode = init();
        b.iter(|| assert_eq!(Day02::part_a(intcode.clone()), 7210630));
    }

    #[bench]
    fn part_b_full(b: &mut Bencher) {
        let intcode = init();
        b.iter(|| assert_eq!(Day02::part_b(intcode.clone()), Ok(3892)));
    }
}
