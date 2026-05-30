use crate::{algorithms::Stepper, app};
use ratatui::style::Color;

#[derive(Clone, Copy)]
enum Phase {
    Select,
    Compare,
    Shift,
    Insert,
}

pub struct ShellState {
    len: usize,
    gap: usize,
    start: usize,
    i: usize,
    pos: usize,
    current: Option<app::Bar>,
    phase: Phase,
    highlighted: Vec<usize>,
}

impl ShellState {
    pub fn new(len: usize) -> Self {
        let gap = len / 2;

        Self {
            len,
            gap,
            start: 0,
            i: gap,
            pos: gap,
            current: None,
            phase: Phase::Select,
            highlighted: Vec::new(),
        }
    }

    fn mark(&mut self, bars: &mut [app::Bar], index: usize, color: Color) {
        bars[index].color = color;
        self.highlighted.push(index);
    }

    fn clear_highlights(&mut self, bars: &mut [app::Bar]) {
        for &i in &self.highlighted {
            bars[i].color = Color::White;
        }

        self.highlighted.clear();
    }

    fn next_sublist(&mut self) {
        self.start += 1;

        if self.start >= self.gap {
            self.gap /= 2;
            self.start = 0;
        }

        self.i = self.start + self.gap;
        self.pos = self.i;

        self.current = None;
        self.phase = Phase::Select;
    }
}

impl Stepper for ShellState {
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {
        self.clear_highlights(bars);

        if self.gap == 0 {
            return true;
        }

        match self.phase {
            Phase::Select => {
                if self.i >= self.len {
                    self.next_sublist();

                    return false;
                }

                self.current = Some(bars[self.i]);

                self.pos = self.i;

                self.mark(bars, self.i, Color::Red);

                self.phase = Phase::Compare;
            }

            Phase::Compare => {
                let current = self.current.unwrap();

                if self.pos >= self.gap {
                    let prev = self.pos - self.gap;

                    self.mark(bars, self.pos, Color::Yellow);
                    self.mark(bars, prev, Color::Yellow);

                    if bars[prev].value > current.value {
                        self.phase = Phase::Shift;
                    } 
                    else {
                        self.phase = Phase::Insert;
                    }
                } 
                else {
                    self.phase = Phase::Insert;
                }
            }

            Phase::Shift => {
                let prev = self.pos - self.gap;

                self.mark(bars, prev, Color::Red);
                self.mark(bars, self.pos, Color::Red);

                bars[self.pos] = bars[prev];

                self.pos -= self.gap;

                self.phase = Phase::Compare;
            }

            Phase::Insert => {
                let current = self.current.unwrap();

                bars[self.pos] = current;

                self.mark(bars, self.pos, Color::Green);

                self.i += self.gap;

                self.phase = Phase::Select;
            }
        }

        false
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}