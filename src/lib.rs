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
    colors: [Color; 13],
    chars: usize,
    randv: usize,
    collection: usize,
    current_collection: usize,
    level: usize,
    col: usize,
    row: usize,
    dx: usize,
    dy: usize,
}





pub fn rand1<const LIMIT: usize>(seed: usize) -> usize {
    let mut r = seed ^ ((seed >> 7) ^ (seed << 3));
    // if r&1 != 0 {r=!r}
    return r.mod_floor(&LIMIT);
}


// pub fn rand1<const LIMIT: usize>(seed: usize) -> usize {
    // let mut r = (16361*seed) ^ !((seed << 8) | (seed));
    // return r.mod_floor(&LIMIT);
// }


pub fn rand<const LIMIT: usize>(seed: usize) -> usize {
    let mut r = (seed ^ ((seed >> 7) ^ (seed << 3))) ^ 1;
    return r.mod_floor(&LIMIT);
}




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
            colors: [Color::Cyan; 13],
            collection: 0,
            current_collection: 0,
            chars: 0,
            level: 0,
            randv: 0,
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



    pub fn setColor(&mut self) {
        // self.colors[0] = Color::Cyan;
        for i in 0..13 {
            let mut c = rand::<16>(self.level+self.collection+i+self.randv);
            let mut e = 0;
            while c == 4 || c == 13 {
                // self.collection+=1;
                c = rand::<16>(self.level+self.collection+i+e+self.randv);
                e+=1;
            }
            self.colors[i] = Color::from(c as u8);
        }
    }


    pub fn incLevel(&mut self) {
        if self.current_collection >= 10 {
            self.randv+=rand::<100>(self.col*self.level+self.collection);
            self.current_collection = 0;
            self.level+=1;
            self.reset();
        }
    }

    pub fn clear(&mut self) {
        for i in 1..BUFFER_WIDTH-1 {
            for j in 1..BUFFER_HEIGHT {
                let l = rand::<12>(self.level+j*i+self.randv)%(1+(self.level/5));
                let chr = rand::<94>(self.level+j*i+self.randv)%(1+(self.level/5));
                let c1 = self.colors[l];
                let c2 = self.colors[l+1];
                plot(((chr+32) as u8) as char, i, j, ColorCode::new(c1, c2));
            }
        }
    }

    pub fn populate(&mut self) {
        for i in 0..10 {
            let x = rand::<{BUFFER_WIDTH-1}>(i+self.level*10+self.randv);
            if peek(safe_add::<{BUFFER_WIDTH-1}>(x, 0), safe_add::<BUFFER_HEIGHT>(x, 0)).1.foreground() == Color::from(13)  {
                self.current_collection+=1;
                self.collection+=1;
            }
            // else {
            let l = rand::<12>(self.level+i+self.randv)%(1+(self.level/5));
            let chr = rand::<94>(self.level+i+self.randv)%(1+(self.level/5));
            let mut c = self.colors[l];
            if chr == 0 {
                c = Color::Pink
            }
            plot(((chr+32) as u8) as char, rand::<{BUFFER_WIDTH-1}>(x), rand::<BUFFER_HEIGHT>(x|BUFFER_HEIGHT), ColorCode::new(Color::Pink, c));
            // }
        }
    }

    pub fn tick(&mut self) {
        // self.check();
        self.update_current();
        self.update_location();
        self.draw_current();
        self.incLevel();
    }


    pub fn reset(&mut self) {
        self.setColor();
        self.clear();
        self.populate();
        self.col = BUFFER_WIDTH / 2;
        self.row = BUFFER_HEIGHT / 2;
    }


    fn update_current(&mut self) {
        for i in 1..BUFFER_WIDTH-1 {
            for j in 1..BUFFER_HEIGHT {
        // for x in self.letter_columns() {
                if (j != self.row) || (i != self.col) {
                    if peek(i, j).1.foreground() != Color::from(13)  {

                        let l = rand::<12>(self.level+j*i+self.randv)%(1+(self.level/5));
                        let chr = rand::<94>(self.level+j*i+self.randv)%(1+(self.level/5));
                        let c1 = self.colors[l];
                        let c2 = self.colors[l+1];
                        plot(((chr+32) as u8) as char, i, j, ColorCode::new(c1, c2));
                    }
                }
                // else if (j == self.row) && (i == self.col) {
                    // if peek(i, j).1.foreground() == Color::from(13)  {
                        // self.collection+=1;
                    // }
                // }
            }
        }
        plot((48+((self.collection/100)%100) as u8) as char, 0, 0, ColorCode::new(Color::White, Color::Black));
        plot((48+((self.collection/10)%10) as u8) as char, 1, 0, ColorCode::new(Color::White, Color::Black));
        plot((48+(self.collection%10) as u8) as char, 2, 0, ColorCode::new(Color::White, Color::Black));

        plot((48+((self.level/100)%100) as u8) as char, BUFFER_WIDTH-3, 0, ColorCode::new(Color::White, Color::Black));
        plot((48+((self.level/10)%10) as u8) as char, BUFFER_WIDTH-2, 0, ColorCode::new(Color::White, Color::Black));
        plot((48+(self.level%10) as u8) as char, BUFFER_WIDTH-1, 0, ColorCode::new(Color::White, Color::Black));
    }


    fn update_location(&mut self) {
        let x = safe_add::<BUFFER_WIDTH>(self.col, self.dx);
        let y = safe_add::<BUFFER_HEIGHT>(self.row, self.dy);
        if peek(x ,y).1.foreground() == Color::from(13) {
            self.collection+=1;
            self.current_collection+=1;
        }
        self.col = x;
        self.row = y;
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
            KeyCode::ArrowDown => {
                self.dy = add1::<BUFFER_HEIGHT>(self.dy);
            }
            KeyCode::LShift => {
                self.current_collection = 10;
            }
            // KeyCode::LShift => {
                // self.populate();
            // }
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
