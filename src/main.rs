use std::{fmt::Display, fs::read_to_string, time::Instant};

use arrayvec::ArrayVec;
use const_for::const_for;
use Color::*;
use LowerUpper::*;

/*
    Tile sides:
    .  A  .
    D  .  B
    .  C  .
*/

/* 
    Order of tiles:
    2  1  8
    3  0  7
    4  5  6
*/

const RULES: [Rule; 8] = [
    // (self's side, other tile idx, other tile side)
    Rule::new(Side::C, 0, Side::A),
    Rule::new(Side::B, 1, Side::D),
    Rule::new(Side::A, 2, Side::C).with_second(Side::B, 0, Side::D),
    Rule::new(Side::A, 3, Side::C),
    Rule::new(Side::D, 4, Side::B).with_second(Side::A, 0, Side::C),
    Rule::new(Side::D, 5, Side::B),
    Rule::new(Side::C, 6, Side::A).with_second(Side::D, 0, Side::B),
    Rule::new(Side::C, 7, Side::A).with_second(Side::D, 1, Side::B),
];

#[derive(Copy, Clone)]
enum Side {
    A = 0, 
    B = 1, 
    C = 2, 
    D = 3,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
enum Color {
    Purple = 0b000,
    Green  = 0b001,
    Yellow = 0b010,
    Blue   = 0b011,
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
enum LowerUpper {
    Lower = 0b000,
    Upper = 0b100,
}

#[derive(Copy, Clone)]
struct Part {
    inner: u8,
}

impl Part {
    pub const fn new(color: Color, lower_upper: LowerUpper) -> Self {
        Self { 
            inner: color as u8 | lower_upper as u8
        }
    }

    fn color(&self) -> Color {
        unsafe { std::mem::transmute(self.inner & 0b011) }
    }

    fn lower_upper(&self) -> LowerUpper {
        unsafe { std::mem::transmute(self.inner & 0b100) }
    }

    fn matches(&self, other: Part) -> bool {
        // Same color, but different lower/upper
        self.inner ^ other.inner == 0b100
    }

    fn from(value: char) -> Self {
        match value {
            'P' => Part::new(Purple, Upper),
            'p' => Part::new(Purple, Lower),
            'G' => Part::new(Green, Upper),
            'g' => Part::new(Green, Lower),
            'Y' => Part::new(Yellow, Upper),
            'y' => Part::new(Yellow, Lower),
            'B' => Part::new(Blue, Upper),
            'b' => Part::new(Blue, Lower),
            _ => panic!("Invalid char '{}'", value)
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let letter = match (self.color(), self.lower_upper()) {
            (Purple, Upper) => 'P',
            (Purple, Lower) => 'p',
            (Green, Upper)  => 'G',
            (Green, Lower)  => 'g',
            (Yellow, Upper) => 'Y',
            (Yellow, Lower) => 'y',
            (Blue, Upper)   => 'B',
            (Blue, Lower)   => 'b',
        };
        write!(f, "{}", letter)
    }
}

// Todo: Pack into a single u16?
#[derive(Copy, Clone, Debug)]
struct Tile {
    id: u8,
    parts: u16
}

impl Tile {
    fn new(id: u8, parts_arr: [Part; 4]) -> Self {
        let mut parts = 0;
        const_for!(i in 0..4 => {
            parts |= (parts_arr[i].inner as u16) << (4 * i);
        });
        Tile { id, parts }
    }

    fn new_from_str(id: u8, parts_str: &str) -> Self {
        if parts_str.len() != 4 {
            panic!("Invalid tile string length in '{}'", parts_str);
        }

        let parts = [
            Part::from(parts_str.as_bytes()[0] as char),
            Part::from(parts_str.as_bytes()[1] as char),
            Part::from(parts_str.as_bytes()[2] as char),
            Part::from(parts_str.as_bytes()[3] as char)
        ];
        Tile::new(id, parts)
    }

    fn get(&self, side: Side) -> Part {
        let offset = 4 * side as u16;
        unsafe { std::mem::transmute(((self.parts >> offset) & 0b1111) as u8) }
    }

    fn rotate(&mut self) {
        self.parts = self.parts.rotate_right(4);
    }
}

fn read_tiles_from_file(path: &str) -> [Tile; 9] {
    read_to_string(path).unwrap().lines()
        .enumerate()
        .map(|(id, line)| {
            Tile::new_from_str(id as u8 + 1, line)
        })
        .collect::<ArrayVec<Tile, 9>>()
        .into_inner()
        .expect("Failed to parse file")
}

#[derive(Copy, Clone)]
struct Rule {
    // (self's side, other tile idx, other tile side)
    first: (Side, usize, Side), // Todo: Wrap in packed other type
    second: Option<(Side, usize, Side)>
}

impl Rule {
    const fn new(own_side: Side, other_idx: usize, other_side: Side) -> Self {
        Rule {
            first: (own_side, other_idx, other_side),
            second: None
        }
    }

    const fn with_second(mut self, other_side: Side, other_idx: usize, other_tile_side: Side) -> Self {
        self.second = Some((other_side, other_idx, other_tile_side));
        self
    }
}

fn main() {
    let tiles = read_tiles_from_file("tiles.txt");

    let before = Instant::now();

    let mut recursions = 0;
    let mut rotations = 0;
    let mut prev_tiles = ArrayVec::<Tile, 9>::new();

    for i in 0..tiles.len() {
        prev_tiles.push(tiles[i]);
        let mut remaining_tiles = ArrayVec::<Tile, 9>::from(tiles);
        remaining_tiles.swap_remove(i);

        solve(&mut prev_tiles, remaining_tiles, &mut recursions, &mut rotations);

        prev_tiles.pop();
    }

    #[cfg(debug_assertions)]
    println!("{} rotations\n{} recursions\n{:?}", rotations, recursions, before.elapsed());

    #[cfg(not(debug_assertions))]
    println!("\n{:?}", before.elapsed());
}

fn solve(prev_tiles: &mut ArrayVec<Tile, 9>, 
         remaining_tiles: ArrayVec<Tile, 9>, 
         recursions: &mut usize, rotations: &mut usize) {

    #[cfg(debug_assertions)]
    { *recursions += 1; }
    
    if remaining_tiles.is_empty() {
        println!("{}", prev_tiles.iter()
                                 .map(|t| t.id.to_string())
                                 .collect::<Vec<String>>()
                                 .join(", "));

        #[cfg(debug_assertions)]
        print_tiles(&prev_tiles);
        
        return;
    }

    let tile_no = prev_tiles.len();
    let rule = RULES[tile_no - 1];
    for i in 0..remaining_tiles.len() {
        let mut tile = remaining_tiles[i];
        for _ in 0..4 {
            #[cfg(debug_assertions)]
            { *rotations += 1; }

            tile.rotate();
            let first_ok = check_rule(rule.first, tile, &prev_tiles);
            let second_ok = match rule.second {
                Some(second) => check_rule(second, tile, &prev_tiles),
                None => true,
            };

            if first_ok && second_ok {
                let mut remaining_tiles = remaining_tiles.clone();
                remaining_tiles.swap_remove(i);

                prev_tiles.push(tile);
                solve(prev_tiles, remaining_tiles, recursions, rotations);
                prev_tiles.pop();
            }
        }
    }
}

fn check_rule((own_side, other_idx, other_side): (Side, usize, Side), tile: Tile, tiles: &ArrayVec<Tile, 9>) -> bool {
    let own_part = tile.get(own_side);
    let other_part = tiles[other_idx].get(other_side);
    own_part.matches(other_part)
}

#[cfg(debug_assertions)]
const ORDER: [usize; 9] = [2, 1, 8, 
                           3, 0, 7, 
                           4, 5, 6];

#[cfg(debug_assertions)]
fn print_tiles(tiles: &[Tile]) {
    let mut lines = vec![String::new(); 9];
    for x in 0..3 {
        for y in 0..3 {
            let i = 3 * y + x;
            let tile = tiles[ORDER[i]];
            let line0 = y * 3;
            
            lines[line0].push_str(&format!("  {}  ", tile.get(Side::A)));
            lines[line0 + 1].push_str(&format!("{} {} {}", tile.get(Side::D), tile.id, tile.get(Side::B)));
            lines[line0 + 2].push_str(&format!("  {}  ", tile.get(Side::C)));
        }
    }

    for line in lines {
        println!("{line}")
    }
    println!()
}