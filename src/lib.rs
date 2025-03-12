#![no_std]

use num::*;
use pc_keyboard::{DecodedKey, KeyCode};
use pluggable_interrupt_os::vga_buffer::{
    is_drawable, plot, Color, ColorCode, BUFFER_HEIGHT, BUFFER_WIDTH, *,
};

use core::{
    clone::Clone,
    cmp::{min, Eq, PartialEq},
    iter::Iterator,
    marker::Copy,
    prelude::rust_2024::derive,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LetterMover {
    collection: usize,
    col: usize,
    row: usize,
    dx: usize,
    dy: usize,
}





pub fn rand1<const LIMIT: usize>(seed: usize) -> usize {
    let mut r = seed ^ ((seed >> 7) ^ (seed << 3));
    // if r&1 != 0 {r=!r^r;}
    return r.mod_floor(&LIMIT);
}


pub fn rand<const LIMIT: usize>(seed: usize) -> usize {
    let mut r = (16361) ^ !((seed << 8) | (seed));
    return r.mod_floor(&LIMIT);
}


// pub fn rand<const LIMIT: usize>(seed: usize) -> usize {
//     let r = (seed >> 1) | (seed >> 2) | (seed >> 3);
//     return r.mod_floor(&LIMIT);
// }




pub fn safe_add<const LIMIT: usize>(a: usize, b: usize) -> usize {
    (a + b).mod_floor(&LIMIT)
}

pub fn add1<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, 1)
}

pub fn sub1<const LIMIT: usize>(value: usize) -> usize {
    safe_add::<LIMIT>(value, LIMIT - 1)
}

impl Default for LetterMover {
    fn default() -> Self {
        Self {
            collection: 0,
            // letters: ['A'; BUFFER_WIDTH],
            // num_letters: 1,
            // next_letter: 1,
            col: BUFFER_WIDTH / 2,
            row: BUFFER_HEIGHT / 2,
            dx: 0,
            dy: 0,
        }
    }
}

impl LetterMover {
    fn letter_columns(&self) -> impl Iterator<Item = usize> + '_ {
        (0..1).map(|n| safe_add::<BUFFER_WIDTH>(n, self.col))
    }

    pub fn tick(&mut self) {
        self.update_current();
        self.update_location();
        self.draw_current();
        // self.check();
    }


    pub fn reset(&mut self) {
        for i in 0..BUFFER_WIDTH {
            for j in 0..BUFFER_HEIGHT {
                plot(' ', i, j, ColorCode::new(Color::Cyan, Color::Cyan));
            }
        }
        for i in 0..10 {
            plot(' ', rand::<BUFFER_WIDTH>(i), rand::<BUFFER_HEIGHT>(i), ColorCode::new(Color::Pink, Color::Pink));
        }
        self.col = BUFFER_WIDTH / 2;
        self.row = BUFFER_HEIGHT / 2;
    }


    fn update_current(&mut self) {
        for i in 1..BUFFER_WIDTH {
            for j in 1..BUFFER_HEIGHT {
        // for x in self.letter_columns() {
                if (j != self.row) || (i != self.col) {
                    if peek(i, j).1.foreground() != Color::from(13)  {
                        plot(' ', i, j, ColorCode::new(Color::Cyan, Color::Cyan));
                    }
                }
                // else if (j == self.row) && (i == self.col) {
                    // if peek(i, j).1.foreground() == Color::from(13)  {
                        // self.collection+=1;
                    // }
                // }
            }
        }
        plot((48+self.collection as u8) as char, 0, 0, ColorCode::new(Color::White, Color::Black));
    }


    fn update_location(&mut self) {
        self.check();
        self.col = safe_add::<BUFFER_WIDTH>(self.col, self.dx);
        self.row = safe_add::<BUFFER_HEIGHT>(self.row, self.dy);
        // self.check();
        self.dx = 0;
        self.dy = 0;
        // self.check();

    }

    fn draw_current(&self) {
        // plot(' ', rand::<BUFFER_WIDTH>(self.row+self.col), rand::<BUFFER_HEIGHT>(self.row+self.col), ColorCode::new(Color::Pink, Color::Pink));
        plot(' ', self.col, self.row, ColorCode::new(Color::Red, Color::Red));
        // plot(' ', self.row, self.col-1, ColorCode::new(Color::Red, Color::Red));
        // plot(' ', self.row-1, self.col, ColorCode::new(Color::Red, Color::Red));
        // for i in 0..BUFFER_WIDTH {
            // for j in 0..BUFFER_HEIGHT {
                // if(j == self.row && i == self.col) {
                    // plot(' ', self.row, self.col, ColorCode::new(Color::Black, Color::Green));
                // }
                // else {
                    // plot(' ', j, i, ColorCode::new(Color::Black, Color::Black));
                // }
            // }
        // }
        // for (i, x) in self.letter_columns().enumerate() {
        // for i in 0..BUFFER_WIDTH {
        //     for j in 0..BUFFER_HEIGHT {
        //     // if circleSDF((self.col as i32)-i as i32, (self.row as i32)-j as i32, 3) > 0 {
        //         plot(
        //             ' ',
        //             i,
        //             j,
        //             ColorCode::new(Color::Red, Color::Red),
        //         );
        //     }
        // }
    }
    // }

    pub fn check(&mut self) {
        let x = safe_add::<{BUFFER_WIDTH}>(self.col, self.dx);
        let y = safe_add::<{BUFFER_WIDTH}>(self.row, self.dy);
        // peek(x ,y).1.background();
        if peek(x ,y).1.background() == Color::from(13) {
            self.collection+=1;
        }
        // self.col = self.col;
        // self.row = self.row;
    }


    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(code) => self.handle_raw(code),
            DecodedKey::Unicode(c) => self.handle_unicode(c),
        }
    }

    fn handle_raw(&mut self, key: KeyCode) {
        match key {
            KeyCode::ArrowLeft => {
                self.dx = sub1::<BUFFER_WIDTH>(self.dx);
            }
            KeyCode::ArrowRight => {
                self.dx = add1::<BUFFER_WIDTH>(self.dx);
            }
            KeyCode::ArrowUp => {
                self.dy = sub1::<BUFFER_HEIGHT>(self.dy);
            }
            KeyCode::ArrowDown => {
                self.dy = add1::<BUFFER_HEIGHT>(self.dy);
            }
            _ => {}
        }
    }

    fn handle_unicode(&mut self, key: char) {
        if is_drawable(key) {
            // self.letters[self.next_letter] = key;
            // self.next_letter = add1::<BUFFER_WIDTH>(self.next_letter);
            // self.num_letters = min(self.num_letters + 1, BUFFER_WIDTH);
        }
    }
}
