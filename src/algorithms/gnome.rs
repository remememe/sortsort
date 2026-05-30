use crate::{algorithms::Stepper, app};
use ratatui::prelude::Color;

pub struct GnomeState {
    len: usize,
    highlighted: Vec<usize>,
    pos: usize,
}

impl GnomeState {
    pub fn new(len: usize) -> Self {
        Self { len: len, highlighted: Vec::new(), pos: 1 }
    }
}

impl Stepper for GnomeState {
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {

        self.clear_highlights(bars);

        if self.pos >= self.len {
            return true;
        }

        if self.pos == 0 || bars[self.pos].value >= bars[self.pos - 1].value
        {
            if self.pos < self.len {
                self.mark(bars, self.pos, Color::Green);
            }

            self.pos += 1;
        } 
        else {
            bars.swap(self.pos, self.pos - 1);

            self.mark(bars, self.pos, Color::Red);
            self.mark(bars, self.pos - 1, Color::Red);

            self.pos -= 1;
        }

        false
    }
    
    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}