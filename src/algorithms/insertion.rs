use crate::{algorithms::Stepper, app};
use ratatui::prelude::Color;

pub struct InsertionState {
    i: usize,
    j: usize,
    current: Option<app::Bar>,
    highlighted: Vec<usize>,
}

impl InsertionState {
    pub fn new() -> Self {
        Self {
            i: 1,
            j: 1,
            current: None,
            highlighted: Vec::new(),
        }
    }
}

impl Stepper for InsertionState {
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {
        self.clear_highlights(bars);

        if self.i >= bars.len() {
            return true;
        }

        if self.current.is_none() {
            self.current = Some(bars[self.i]);
            self.j = self.i;
        }

        let cur = self.current.unwrap();

        if self.j > 0 && cur.value < bars[self.j - 1].value {
            bars[self.j] = bars[self.j - 1];

            self.mark(bars, self.j, Color::Red);
            self.mark(bars, self.j - 1, Color::Red);

            self.j -= 1;
        } 
        else {
            bars[self.j] = cur;

            self.mark(bars, self.j, Color::Green);

            self.i += 1;
            self.current = None;
        }

        false
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}