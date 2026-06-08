pub mod bubble;
pub mod quick;
pub mod cocktail_shaker;
pub mod selection;
pub mod odd_even;
pub mod gnome;
pub mod insertion;
pub mod shell;
pub mod merge;
pub mod bogo;

use crate::app::{Bar};
use ratatui::prelude::Color;

pub trait Stepper {
    fn step(&mut self, bars: &mut Vec<Bar>) -> bool;

    fn highlighted(&mut self) -> &mut Vec<usize>;

    fn clear_highlights(&mut self, bars: &mut Vec<Bar>) {
        let highlighted = self.highlighted();

        for &idx in highlighted.iter() {
            bars[idx].color = Color::White;
        }

        highlighted.clear();
    }

    fn mark(&mut self, bars: &mut Vec<Bar>, idx: usize, color: Color) {
        bars[idx].color = color;
        self.highlighted().push(idx);
    }
}