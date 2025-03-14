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





pub fn rand<const LIMIT: usize>(seed: usize) -> usize {
    let mut r =
    (seed) ^ ((seed >> 3) ^ (seed << 7));
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
        for i in 0..13 {
            let mut c = rand::<16>(i+self.randv);
            let mut e = 0;
            while c == 4 || c == 13 {
                c = rand::<16>(i+e+self.randv);
                e+=1;
            }
            self.colors[i] = Color::from(c as u8);
        }
    }


    pub fn incLevel(&mut self) {
        if self.current_collection >= 3 {
            self.randv+=rand::<100>(self.col*self.level+self.collection);
            self.current_collection = 0;
            self.level+=1;
            self.reset();
        }
    }

    pub fn clear(&mut self) {
        for i in 0..BUFFER_WIDTH {
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
            let l = rand::<12>(self.level+i+self.randv)%(1+(self.level/5));
            let chr = rand::<94>(self.level+i+self.randv)%(1+(self.level/5));
            let mut c = self.colors[l];
            if chr == 0 {
                c = Color::Pink
            }
            plot(((chr+32) as u8) as char, rand::<{BUFFER_WIDTH-1}>(x), rand::<BUFFER_HEIGHT>(x|BUFFER_HEIGHT), ColorCode::new(Color::Pink, c));
        }
    }

    pub fn tick(&mut self) {
        self.runGame();
    }

    pub fn runGame(&mut self) {
        self.incLevel();
        self.update_current();
        self.draw_current();
        self.update_location();
    }


    pub fn endAll(&mut self) {
            self.colors = [Color::Cyan; 13];
            self.collection = 0;
            self.current_collection = 0;
            self.chars = 0;
            self.level = 0;
            self.dx = 0;
            self.dy = 0;
            self.col = BUFFER_WIDTH / 2;
            self.row = BUFFER_HEIGHT / 2;
    }

    pub fn end(&mut self) {
        for i in 0..BUFFER_WIDTH {
            for j in 0..BUFFER_HEIGHT {
                    plot(' ', i, 0, ColorCode::new(Color::Green, Color::Green));
            }
        }
        let strr = ['p','l','a','y',' ','a','g','a','i','n'];
        for i in 0..strr.len() {
            plot(strr[i], i, 0, ColorCode::new(Color::Green, Color::Black));
        }
    }


    pub fn reset(&mut self) {
        self.setColor();
        self.clear();
        self.populate();
        self.col = BUFFER_WIDTH / 2;
        self.row = BUFFER_HEIGHT / 2;
    }


    fn update_current(&mut self) {
        for i in 0..BUFFER_WIDTH {
            for j in 1..BUFFER_HEIGHT {
                if (((j != self.row) || (i != self.col)) && ( (i != BUFFER_WIDTH / 2) || (j != BUFFER_HEIGHT / 2))) {
                    if peek(i, j).1.foreground() != Color::from(13)  {

                        let l = rand::<12>(self.level+j*i+self.randv)%(1+(self.level/5));
                        let chr = rand::<94>(self.level+j*i+self.randv)%(1+(self.level/5));
                        let c1 = self.colors[l];
                        let c2 = self.colors[l+1];
                        plot(((chr+32) as u8) as char, i, j, ColorCode::new(c1, c2));
                    }
                }
            }
        }
        plot('S', BUFFER_WIDTH / 2, BUFFER_HEIGHT / 2, ColorCode::new(Color::Green, Color::Pink));
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
        if peek(x ,y).1.background() == Color::from(0) {
            self.endAll();
        }
        self.col = x;
        self.row = y;
        self.dx = 0;
        self.dy = 0;

    }

    fn draw_current(&self) {
        plot(' ', self.col, self.row, ColorCode::new(Color::Red, Color::Red));
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
                KeyCode::LShift => {
                    self.current_collection = 3;
                }
                _ => {}
            }
    }

    fn handle_unicode(&mut self, key: char) {
        if is_drawable(key) {
        }
    }
}
