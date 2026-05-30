use crate::{algorithms::Stepper, app};
use ratatui::prelude::Color;
pub struct CocktailState {
    start: usize,
    end: usize,
    i: usize,
    right: bool,
    swapped: bool,
    highlighted: Vec<usize>,
    done: bool,
}

impl CocktailState {
    pub fn new(len: usize) -> Self {
        Self {
            start: 0,
            end: len,
            i: 0,
            right: true,
            swapped: false,
            highlighted: Vec::new(),
            done: false,
        }
    }
}
impl Stepper for CocktailState
{
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {
        if self.done || bars.len() <= 1 {
            self.done  = true;
            return true;
        }
        if self.start >= self.end {
            self.done = true;
            return true;
        }

        self.clear_highlights(bars);

        let end = self.end;

        if self.right {
            if self.i + 1 < end {
                if bars[self.i].value > bars[self.i + 1].value {
                    bars.swap(self.i, self.i + 1);
                    self.swapped = true;
                }

                self.mark(bars, self.i, Color::Red);
                self.mark(bars, self.i + 1, Color::Red);
                
                self.i += 1;
            } 
            else {
                if !self.swapped {
                    self.done = true;
                    return true;
                }

                self.swapped = false;
                self.right = false;
                self.end -= 1;
                self.i = self.end - 1;
            }
        } 
        else {
            if self.i > self.start {
                if bars[self.i].value < bars[self.i - 1].value {
                    bars.swap(self.i, self.i - 1);
                    self.swapped = true;
                }

                self.mark(bars, self.i, Color::Red);
                self.mark(bars, self.i - 1, Color::Red);

                self.i -= 1;
            } 
            else {
                if !self.swapped {
                    self.done = true;
                    return true;
                }

                self.swapped = false;
                self.right = true;
                self.start += 1;
                self.i = self.start;
            }
        }

        self.done
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}