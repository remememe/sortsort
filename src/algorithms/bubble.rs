use crate::{algorithms::Stepper, app};
use ratatui::prelude::Color;
pub struct BubbleState {
    len: usize,
    i: usize,
    j: usize,
    highlighted: Vec<usize>,
    done: bool,
}

impl BubbleState {
    pub fn new(len: usize) -> Self {
        Self { len: len, i: 0, j: 0, highlighted: Vec::new(), done: false }
    }
}

impl Stepper for BubbleState
{
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {
        if self.done || bars.len() < 2 {
            self.done = true;
            return self.done;
        }
        
        self.clear_highlights(bars);
        
        if self.j + 1 >= self.len - self.i {
            self.j = 0;
            self.i += 1;

            if self.i >= self.len - 1 {
                self.done = true;
            }
            return self.done;
        }

        self.mark(bars, self.j, Color::Red);
        self.mark(bars, self.j + 1, Color::Red);

        if bars[self.j].value > bars[self.j + 1].value {
            bars.swap(self.j, self.j + 1);
        }

        self.j += 1;

        return false;
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}