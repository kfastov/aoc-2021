use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;
use SubmarineInstruction::*;
type Result<T> = std::result::Result<T, Box <dyn Error>>;

#[derive(Debug)]
struct SubmarineParseError;
impl Error for SubmarineParseError {}
impl std::fmt::Display for SubmarineParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Failed to parse submarine instruction")
    }
}

fn get_input() -> Result<impl Iterator<Item = String>> {
    let f = File::open("./input.txt")?;
    let reader = BufReader::new(f);
    Ok(reader.lines().filter_map(|res| res.map_or(None, |x| Some(x))))
}

#[derive(PartialEq, Debug)]
enum SubmarineInstruction {
    Up(i32),
    Down(i32),
    Forward(i32),
}

#[derive(Default)]
struct Coordinate (i32, i32);

#[derive(Default)]
struct Course {
    x: i32,
    y: i32,
    aim: i32,
}

impl std::ops::Add<SubmarineInstruction> for Coordinate {
    type Output = Self;
    fn add(self, instr: SubmarineInstruction) -> Self {
        match instr {
            Up(h) => Coordinate(self.0, self.1 - h),
            Down(h) => Coordinate(self.0, self.1 + h),
            Forward(x) => Coordinate(self.0 + x, self.1),
        }
    }
}

impl std::ops::Add<SubmarineInstruction> for Course {
    type Output = Self;
    fn add(self, instr: SubmarineInstruction) -> Self {
        match instr {
            Up(n) => Course { aim: self.aim - n, ..self },
            Down(n) => Course { aim: self.aim + n, ..self },
            Forward(n) => Course { x: self.x + n, y: self.y + self.aim * n, ..self},
        }
    }
}

fn parse_instructions(input: impl Iterator<Item = String>) -> impl Iterator<Item = Result<SubmarineInstruction>> {
    input.map(|line| -> Result<SubmarineInstruction> {
        let s: String = line;
        let (instr, arg) = s.split_once(' ').ok_or(SubmarineParseError)?;
        let meters = arg.parse::<i32>()?;
        match instr {
            "forward" => Ok(Forward(meters)),
            "up" => Ok(Up(meters)),
            "down" => Ok(Down(meters)),
            _ => Err(SubmarineParseError.into()),
        }
    })
}

fn solution1(input: impl Iterator<Item = String>) -> Result<i32> {
    let instructions: Result<Vec<SubmarineInstruction>> = parse_instructions(input).collect();
    let dest = instructions?.into_iter().fold(Coordinate::default(), |c, i| c + i);
    Ok(dest.0 * dest.1)
}

fn solution2(input: impl Iterator<Item = String>) -> Result<i32> {
    let instructions: Result<Vec<SubmarineInstruction>> = parse_instructions(input).collect();
    let dest = instructions?.into_iter().fold(Course::default(), |c, i| c + i);
    Ok(dest.x * dest.y)
}

fn main() -> Result<()> {
    println!("First star: {}", solution1(get_input()?)?);
    println!("Second star: {}", solution2(get_input()?)?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_instruction_parser() {
        let input = vec![
            "forward 4", // move to (4, 0)
            "down 200", // move to (4, 200)
            "up 32", // move to (4, 168)
        ].into_iter().map(String::from);
        let instructions: Result<Vec<SubmarineInstruction>> = parse_instructions(input).collect();
        assert_eq!(instructions.unwrap(), vec![
            Forward(4),
            Down(200),
            Up(32),
        ]);
    }
    #[test]
    fn test_solution() {
        let input = vec![
            "forward 4", // move to (4, 0)
            "down 200", // move to (4, 200)
            "up 32", // move to (4, 168)
        ].into_iter().map(String::from);
        let solution = solution1(input).unwrap();
        assert_eq!(solution, 4 * 168);
    }
}