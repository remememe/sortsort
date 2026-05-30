use crate::{algorithms::Stepper, app};
use ratatui::prelude::Color;
pub struct OddEvenState 
{
    len: usize,
    i: usize,
    odd: bool,
    highlighted: Vec<usize>,
    swapped: bool,
}
impl OddEvenState 
{
    pub fn new(len: usize) -> Self {
        Self { len: len, i: 0, odd: false, highlighted: Vec::new(), swapped: false }
    }
}
impl Stepper for OddEvenState
{
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {

        if self.len < 2 {
            return true;
        }
                
        self.clear_highlights(bars);

        if self.i + 1 < self.len && bars[self.i].value > bars[self.i + 1].value
        {
            bars.swap(self.i, self.i + 1);
            self.swapped = true;
        }

        self.mark(bars, self.i, Color::Red);
        self.mark(bars, self.i + 1, Color::Red);
        
        self.i += 2;

        if self.i >= self.len - 1 {

            if !self.odd {
                self.odd = true;
                self.i = 1;
            }

            else {
                if !self.swapped {
                    for bar in bars.iter_mut() {
                        bar.color = Color::White;
                    }
                    return true;
                }

                self.swapped = false;
                self.odd = false;
                self.i = 0;
            }
        }

        false
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}