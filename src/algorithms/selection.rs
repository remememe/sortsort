use crate::{algorithms::Stepper, app};
use ratatui::prelude::Color;
pub struct SelectionState {
    len: usize,
    start: usize,
    current: usize,
    smallest: usize,
    highlighted: Vec<usize>,
    done: bool,
}
impl SelectionState {
    pub fn new(len: usize) -> Self {
        Self { len: len, start: 0, current: 1, smallest: 0, highlighted: Vec::new(), done: false }
    }
}
impl Stepper for SelectionState {
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {
        if self.done || self.len < 2 {
            self.done = true;
            return true;
        }

        self.clear_highlights(bars);

        if self.start >= self.len - 1 {
            self.done = true;
            return true;
        }

        if bars[self.current].value < bars[self.smallest].value {
            self.smallest = self.current;
        }

        self.mark(bars, self.smallest, Color::Red);

        if self.current != self.smallest {
            self.mark(bars, self.current, Color::Yellow);
        }

        self.current += 1;

        if self.current >= self.len {
            bars.swap(self.start, self.smallest);

            self.highlighted.clear();
            self.mark(bars, self.start, Color::Green);

            self.start += 1;
            self.smallest = self.start;
            self.current = self.start + 1;
        }

        false
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}