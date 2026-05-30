use crate::{algorithms::Stepper, app};
use rand::seq::SliceRandom;
use ratatui::prelude::Color;

pub struct BogoState {
    make_red: bool,
    highlighted: Vec<usize>,
    done: bool,
}

impl BogoState {
    pub fn new() -> Self {
        Self { make_red: false, highlighted: Vec::new(), done: false }
    }
}
impl Stepper for BogoState {
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {

        if bars.len() < 2 {
            return true;
        }
        
        if !self.make_red
        {
            self.make_red = true;
            for bar in bars.iter_mut() {
                bar.color = Color::Red;
            }
        }

        bars.shuffle(&mut rand::rng());

        self.done = bars.windows(2).all(|w| w[0].value <= w[1].value);

        self.done
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}