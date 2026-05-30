use crate::{algorithms::Stepper, app};
use ratatui::prelude::Color;
pub enum QuickPhase {
    ChooseRange,
    Partition,
    FinalSwap,
    Done,
}

pub struct QuickState {
    stack: Vec<(usize, usize)>,
    low: usize,
    high: usize,
    i: usize,
    j: usize,
    highlighted: Vec<usize>,
    phase: QuickPhase,
}

impl QuickState {
    pub fn new(len: usize) -> Self {
        let mut stack = Vec::new();
        if len > 1 {
            stack.push((0, len - 1));
        }

        Self {
            stack,
            low: 0,
            high: 0,
            i: 0,
            j: 0,
            highlighted: Vec::new(),
            phase: QuickPhase::ChooseRange,
        }
    }

    pub fn is_done(&self) -> bool {
        matches!(self.phase, QuickPhase::Done)
    }
}

impl Stepper for QuickState {
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {
        if bars.len() < 2 {
            self.phase = QuickPhase::Done;
            return true;
        }

        self.clear_highlights(bars);

        match self.phase {
            QuickPhase::ChooseRange => {
                while let Some((low, high)) = self.stack.pop() {
                    if low < high {
                        self.low = low;
                        self.high = high;
                        self.i = low;
                        self.j = low;
                        self.phase = QuickPhase::Partition;

                        return false;
                    }
                }

                self.phase = QuickPhase::Done;
            }

            QuickPhase::Partition => {
                let pivot_value = bars[self.high].value;

                self.mark(bars, self.high, Color::Blue);
                self.mark(bars, self.j, Color::Red);
                self.mark(bars, self.i, Color::Red);

                if self.j < self.high {
                    if bars[self.j].value <= pivot_value {
                        bars.swap(self.i, self.j);
                        self.i += 1;
                    }

                    self.j += 1;
                } else {
                    self.phase = QuickPhase::FinalSwap;
                }
            }

            QuickPhase::FinalSwap => {
                bars.swap(self.i, self.high);
                self.mark(bars, self.i, Color::Green);

                if self.low < self.i {
                    self.stack.push((self.low, self.i - 1));
                }

                if self.i + 1 < self.high {
                    self.stack.push((self.i + 1, self.high));
                }

                self.phase = QuickPhase::ChooseRange;
            }

            QuickPhase::Done => {}
        }

        self.is_done()
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}