#![allow(warnings)]
use std::collections::HashMap;

fn main() {
    let input = include_str!("../../example.txt");
    let answer = input_to_answer(input);
    println!("The answer is: {answer}");
}

fn input_to_answer(s: &str) -> usize {
    let codes = Codes::try_from(s).unwrap();
    let world = World::default();
    codes.answer(world)
}

impl Default for World {
    fn default() -> Self {
        todo!()
    }
}

impl TryFrom<&str> for Codes {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

struct Codes(Vec<Code>);

struct Point {
    x: u8,
    y: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    North,
    East,
    South,
    West,
    Press,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Instructions(Vec<Instruction>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum NumPadKey {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Press,
}

struct Arm(Point);

struct Code(Vec<NumPadKey>);

trait GoTo<K> {
    fn go_to(self, key: &K) -> (Instructions, Self);
}

impl DPadRoom {
    fn apply_instructions(&self, instructions: Instructions) -> (Instructions, Self) {
        instructions.0.iter().fold(
            (Instructions::default(), self.clone()),
            |(mut instructions, layer), instruction| {
                let (new_instructions, layer) = layer.go_to(instruction);
                instructions.0.extend(new_instructions.0);
                (instructions, layer)
            },
        )
    }
}

impl GoTo<NumPadKey> for NumPadRoom {
    fn go_to(self, key: &NumPadKey) -> (Instructions, Self) {
        todo!()
    }
}

impl GoTo<Instruction> for DPadRoom {
    fn go_to(self, key: &Instruction) -> (Instructions, Self) {
        todo!()
    }
}

struct NumPadRoom {
    key_pad: HashMap<NumPadKey, Point>,
    arm: Arm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DPadRoom {
    key_pad: HashMap<Instruction, Point>,
    arm: Arm,
}

struct World {
    base: NumPadRoom,
    layers: Vec<DPadRoom>,
}

impl Code {
    fn instructions(&self, world: World) -> (Instructions, World) {
        // get base instructions
        // these instructions are executed on inside the numpad room.
        let (base_instructions, base) = self.0.iter().fold(
            (Instructions::default(), world.base),
            |(mut instructions, base), key| {
                let (new_instructions, base) = base.go_to(key);
                instructions.0.extend(new_instructions.0);
                (instructions, base)
            },
        );
        let (instructions, layers) = world.layers.iter().fold(
            (base_instructions, Vec::new()),
            |(instructions, mut layers), layer| {
                let (instructions, layer) = layer.apply_instructions(instructions);
                layers.push(layer);
                (instructions, layers)
            },
        );
        let world = World { base, layers };
        (instructions, world)
    }

    fn score(&self, world: World) -> (usize, World) {
        let (instructions, world) = self.instructions(world);
        let score = instructions.0.len() * self.numeric_value();
        (score, world)
    }

    fn numeric_value(&self) -> usize {
        todo!()
    }
}

impl Codes {
    fn answer(&self, world: World) -> usize {
        self.0
            .iter()
            .fold((0, world), |(_, world), code| code.score(world))
            .0
    }
}
