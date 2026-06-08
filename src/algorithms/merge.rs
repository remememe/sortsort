use crate::{algorithms::Stepper, app};
use ratatui::prelude::Color;

#[derive(Clone, Copy)]
enum Phase {
    Split,
    Merge,
}

struct Frame {
    start: usize,
    end: usize,
    mid: usize,
    phase: Phase,

    left: Vec<u64>,
    right: Vec<u64>,
    l: usize,
    r: usize,
    i: usize,
    initialized: bool,
}

pub struct MergeState {
    stack: Vec<Frame>,
    highlighted: Vec<usize>,
    done: bool,
}

impl MergeState {
    pub fn new(len: usize) -> Self {
        Self {
            stack: vec![Frame {
                start: 0,
                end: len,
                mid: 0,
                phase: Phase::Split,
                left: Vec::new(),
                right: Vec::new(),
                l: 0,
                r: 0,
                i: 0,
                initialized: false,
            }],
            highlighted: Vec::new(),
            done: false,
        }
    }
}

impl Stepper for MergeState {
    fn step(&mut self, bars: &mut Vec<app::Bar>) -> bool {
        self.clear_highlights(bars);

        if self.stack.is_empty() {
            self.done = true;
            return true;
        }

        let mut push_left = None;
        let mut push_right = None;
        let mut write_op = None;
        let mut pop_current = false;

        {
            let frame = self.stack.last_mut().unwrap();

            match frame.phase {
                Phase::Split => {
                    let len = frame.end - frame.start;

                    if len <= 1 {
                        pop_current = true;
                    } 
                    else {
                        frame.mid = frame.start + len / 2;
                        frame.phase = Phase::Merge;

                        push_right = Some(Frame {
                            start: frame.mid,
                            end: frame.end,
                            mid: 0,
                            phase: Phase::Split,
                            left: Vec::new(),
                            right: Vec::new(),
                            l: 0,
                            r: 0,
                            i: 0,
                            initialized: false,
                        });

                        push_left = Some(Frame {
                            start: frame.start,
                            end: frame.mid,
                            mid: 0,
                            phase: Phase::Split,
                            left: Vec::new(),
                            right: Vec::new(),
                            l: 0,
                            r: 0,
                            i: 0,
                            initialized: false,
                        });
                    }
                }

                Phase::Merge => {
                    if !frame.initialized {
                        frame.left = bars[frame.start..frame.mid]
                            .iter()
                            .map(|b| b.value)
                            .collect();

                        frame.right = bars[frame.mid..frame.end]
                            .iter()
                            .map(|b| b.value)
                            .collect();

                        frame.l = 0;
                        frame.r = 0;
                        frame.i = frame.start;
                        frame.initialized = true;
                    }

                    if frame.i >= frame.end {
                        pop_current = true;
                    } 

                    else {
                        let take_left = frame.r >= frame.right.len() || (frame.l < frame.left.len() && frame.left[frame.l] <= frame.right[frame.r]);

                        if take_left {
                            let v = frame.left[frame.l];
                            write_op = Some((frame.i, v));
                            frame.l += 1;
                        } 
                        else {
                            let v = frame.right[frame.r];
                            write_op = Some((frame.i, v));
                            frame.r += 1;
                        }

                        frame.i += 1;
                    }
                }
            }
        }

        if let Some(frame) = push_right {
            self.stack.push(frame);
        }
        if let Some(frame) = push_left {
            self.stack.push(frame);
        }

        if let Some((idx, value)) = write_op {
            bars[idx].value = value;
            self.mark(bars, idx, Color::Red);
        }

        if pop_current {
            self.stack.pop();
        }

        if self.stack.is_empty() {
            self.done = true;
            return true;
        }

        false
    }

    fn highlighted(&mut self) -> &mut Vec<usize> {
        &mut self.highlighted
    }
}