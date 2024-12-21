use std::{fmt::Display, mem, time::Instant};

use Color::*;
use LowerUpper::*;

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
        unsafe { mem::transmute(self.inner & 0b011) }
    }

    fn lower_upper(&self) -> LowerUpper {
        unsafe { mem::transmute(self.inner & 0b100) }
    }

    fn matches(&self, other: Part) -> bool {
        // Same color, but different lower/upper
        self.inner ^ other.inner == 0b100
    }

    fn char(&self) -> char {
        let letter = match self.color() {
            Purple => 'P',
            Green => 'G',
            Yellow => 'Y',
            Blue => 'B',
        };

        match self.lower_upper() {
            Upper => letter,
            Lower => letter.to_ascii_lowercase(),
        }
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} {:?})", self.color(), self.lower_upper())
    }
}

#[derive(Copy, Clone)]
enum Side {
    A = 0, 
    B = 1, 
    C = 2, 
    D = 3,
}

// Todo: Pack into a single u16?
#[derive(Copy, Clone)]
struct Tile {
    id: u8,
    parts: [Part; 4]
}

impl Tile {
    fn get(&self, side: Side) -> Part {
        self.parts[side as usize]
    }

    fn rotate(&mut self) {
        self.parts.rotate_left(1);
    }
}

#[derive(Copy, Clone)]
struct Rule {
    // (self's side, other tile idx, other tile side)
    first: (Side, usize, Side), // Todo: Wrap in packed other type
    second: Option<(Side, usize, Side)>
}

fn print_tiles(tiles: &[Tile], order: [usize; 9]) {
    let mut lines = vec![String::new(); 9];
    for x in 0..3 {
        for y in 0..3 {
            let i = 3 * y + x;
            let tile = tiles[order[i]];
            let line0 = y * 3;
            
            lines[line0].push_str(&format!("  {}  ", tile.get(Side::A).char()));
            lines[line0 + 1].push_str(&format!("{} {} {}", tile.get(Side::D).char(), tile.id, tile.get(Side::B).char()));
            lines[line0 + 2].push_str(&format!("  {}  ", tile.get(Side::C).char()));
        }
    }

    for line in lines {
        println!("{line}")
    }
    println!()
}

fn main() {
    let before = Instant::now();
    let mut checks = 0;

    for i in 0..TILES.len() {
        let mut prev_tiles = vec![TILES[i]];
        let mut remaining_tiles = TILES.to_vec();
        remaining_tiles.swap_remove(i);
        rec_solve(&mut prev_tiles, &mut remaining_tiles, &mut checks);
    }

    println!("{checks} rotations in {:?}", before.elapsed())
}

fn rec_solve(prev_tiles: &mut Vec<Tile>, remaining_tiles: &mut Vec<Tile>, rotations: &mut usize) {
    if remaining_tiles.is_empty() {
        println!("{}", prev_tiles.iter().map(|t| t.id.to_string()).collect::<Vec<String>>().join(", "));
        #[cfg(debug_assertions)]{
            print_tiles(&prev_tiles, [2, 1, 8, 
                                      3, 0, 7, 
                                      4, 5, 6]);
        }
        return;
    }

    let tile_no = prev_tiles.len();
    let rule = RULES[tile_no - 1];
    for i in 0..remaining_tiles.len() {
        let mut tile = remaining_tiles[i];
        for _ in 0..4 {
            *rotations += 1;
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
                rec_solve(prev_tiles, &mut remaining_tiles, rotations);
                prev_tiles.pop();
            }
        }
    }
}

fn check_rule((own_side, other_idx, other_side): (Side, usize, Side), tile: Tile, tiles: &Vec<Tile>) -> bool {
    let own_part = tile.get(own_side);
    let other_part = tiles[other_idx].get(other_side);
    own_part.matches(other_part)
}

const RULES: [Rule; 8] = [
    Rule {
        first: (Side::C, 0, Side::A),
        second: None
    },
    Rule {
        first: (Side::B, 1, Side::D),
        second: None
    },
    Rule {
        first: (Side::A, 2, Side::C),
        second: Some((Side::B, 0, Side::D))
    },
    Rule {
        first: (Side::A, 3, Side::C),
        second: None
    },
    Rule {
        first: (Side::D, 4, Side::B),
        second: Some((Side::A, 0, Side::C))
    },
    Rule {
        first: (Side::D, 5, Side::B),
        second: None
    },
    Rule {
        first: (Side::C, 6, Side::A),
        second: Some((Side::D, 0, Side::B))
    },
    Rule {
        first: (Side::C, 7, Side::A),
        second: Some((Side::D, 1, Side::B))
    },
];

const TILES: [Tile; 9] = [
    Tile {parts: [
        Part::new(Green, Lower), 
        Part::new(Purple, Lower), 
        Part::new(Blue, Upper), 
        Part::new(Yellow, Upper)
    ], id: 1},
    Tile {parts: [
        Part::new(Yellow, Lower), 
        Part::new(Purple, Lower), 
        Part::new(Green, Upper), 
        Part::new(Blue, Upper)
    ], id: 2},
    Tile {parts: [
        Part::new(Blue, Lower), 
        Part::new(Purple, Lower), 
        Part::new(Green, Upper), 
        Part::new(Yellow, Upper)
    ], id: 3},
    Tile {parts: [
        Part::new(Green, Lower), 
        Part::new(Yellow, Lower), 
        Part::new(Green, Upper), 
        Part::new(Purple, Upper)
    ], id: 4},
    Tile {parts: [
        Part::new(Purple, Lower), 
        Part::new(Yellow, Lower), 
        Part::new(Blue, Upper), 
        Part::new(Green, Upper)
    ], id: 5},
    Tile {parts: [
        Part::new(Green, Lower), 
        Part::new(Purple, Lower), 
        Part::new(Blue, Upper), 
        Part::new(Yellow, Upper)
    ], id: 6},
    Tile {parts: [
        Part::new(Blue, Lower), 
        Part::new(Yellow, Lower), 
        Part::new(Green, Upper), 
        Part::new(Purple, Upper)
    ], id: 7},
    Tile {parts: [
        Part::new(Blue, Lower), 
        Part::new(Purple, Lower), 
        Part::new(Blue, Upper), 
        Part::new(Yellow, Upper)
    ], id: 8},
    Tile {parts: [
        Part::new(Yellow, Lower), 
        Part::new(Green, Lower), 
        Part::new(Blue, Upper), 
        Part::new(Purple, Upper)
    ], id: 9},
];