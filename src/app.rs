use std::fmt;
use std::{time::{Duration, Instant}};
use clap::ValueEnum;
use crate::{algorithms::{bogo::BogoState, bubble::{self, BubbleState}, cocktail_shaker::CocktailState, gnome::GnomeState, insertion::InsertionState, odd_even::OddEvenState, quick::{self, QuickState}, selection::SelectionState, shell::ShellState}};
use crate::algorithms::Stepper;
use rand::{RngExt, seq::IndexedRandom};
use crossterm::event::{self, KeyCode};
use ratatui::style::Stylize;
use ratatui::layout::{Rect, Alignment};
use ratatui_core::buffer::Buffer;
use ratatui::prelude::Color;
use ratatui::{Frame, style::Style, widgets::{self, Block, Clear, Paragraph}};
use ratatui_core::widgets::Widget;

enum Phase {
    Appearing,
    WaitRandomizing,
    Randomizing,
    WaitSorting,
    Sorting,
    WaitDone,
    Check,
    Done,
}
#[derive(Clone, Debug, ValueEnum)]
pub enum SortingAlgorithm {
    Quick,
    Bubble,
    Cocktail,
    Selection,
    OddEven,
    Gnome,
    Insertion,
    Shell,
    Bogo,
    Random,
}
enum AlgorithmState
{
    Quick(QuickState),
    Bubble(BubbleState),
    Cocktail(CocktailState),
    Selection(SelectionState),
    OddEven(OddEvenState),
    Gnome(GnomeState),
    Insertion(InsertionState),
    Shell(ShellState),
    Bogo(BogoState),
}
impl fmt::Display for SortingAlgorithm
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl AlgorithmState {
    fn name(&self) -> &'static str {
        match self {
            AlgorithmState::Quick(_) => "Quick Sort",
            AlgorithmState::Bubble(_) => "Bubble Sort",
            AlgorithmState::Cocktail(_) => "Cocktail Sort",
            AlgorithmState::Selection(_) => "Selection Sort",
            AlgorithmState::OddEven(_) => "Odd–even Sort",
            AlgorithmState::Gnome(_) => "Gnome Sort",
            AlgorithmState::Insertion(_) => "Insertion Sort",
            AlgorithmState::Shell(_) => "Shell Sort",
            AlgorithmState::Bogo(_) => "Bogo Sort",
        }
    }
    pub fn step(&mut self, bars: &mut Vec<Bar>) -> bool {
        match self {
            AlgorithmState::Bubble(state) => state.step(bars),
            AlgorithmState::Quick(state) => state.step(bars),
            AlgorithmState::Cocktail(state) => state.step(bars),
            AlgorithmState::Selection(state) => state.step(bars),
            AlgorithmState::OddEven(state) => state.step(bars),
            AlgorithmState::Gnome(state) => state.step(bars),
            AlgorithmState::Insertion(state) => state.step(bars),
            AlgorithmState::Shell(state) => state.step(bars),
            AlgorithmState::Bogo(state) => state.step(bars),
        }
    }
}

pub struct App {
    info: bool,
    border: bool,
    border_color: [u8;3],
    bar_width: Option<u16>,
    looped: bool,
    bars: Vec<Bar>,
    all_bars: Vec<Bar>,
    iter: usize,
    phase: Phase,
    phase_started: Instant,
    highlighted: Option<(usize, usize)>,
    algorithm: AlgorithmState,
    algorithm_name: SortingAlgorithm,
    last_tick: Instant,
    time_elapsed: Instant,
    timed: Duration,
}
impl App {
    fn get_algorithm(algorith_name: &SortingAlgorithm, amount: usize) -> AlgorithmState {
        match algorith_name {
            SortingAlgorithm::Quick => AlgorithmState::Quick(
                quick::QuickState::new(amount)
            ),
            SortingAlgorithm::Bubble => AlgorithmState::Bubble(
                bubble::BubbleState::new(amount)
            ),
            SortingAlgorithm::Cocktail => AlgorithmState::Cocktail(
                CocktailState::new(amount)
            ),
            SortingAlgorithm::Selection => AlgorithmState::Selection(
                SelectionState::new(amount)
            ),
            SortingAlgorithm::OddEven => AlgorithmState::OddEven(
                OddEvenState::new(amount)
            ),
            SortingAlgorithm::Gnome => AlgorithmState::Gnome(
                GnomeState::new(amount)
            ),
            SortingAlgorithm::Insertion => AlgorithmState::Insertion(
                InsertionState::new()
            ),
            SortingAlgorithm::Shell => AlgorithmState::Shell(
                ShellState::new(amount)
            ),
            SortingAlgorithm::Bogo => AlgorithmState::Bogo(
                BogoState::new()
            ),
            SortingAlgorithm::Random => {
                let vs = vec![SortingAlgorithm::Quick,SortingAlgorithm::Bubble,SortingAlgorithm::Cocktail,SortingAlgorithm::Selection,SortingAlgorithm::OddEven,SortingAlgorithm::Gnome,SortingAlgorithm::Insertion,SortingAlgorithm::Shell];
                match vs.choose(&mut rand::rng()) {
                    Some(i) =>  Self::get_algorithm(i, amount),
                    None    => Self::get_algorithm(&SortingAlgorithm::Quick, amount),
                }
            }
        }
    }
    fn create_bars(amount: usize) -> Vec<Bar>
    {
        let n = amount;
        let mut all_bars = Vec::with_capacity(n);

        for i in 0..n {
            all_bars.push(Bar {
                value: ((i + 1)) as u64,
                color: Color::White,
            });
        }
        all_bars
    }
    pub fn new(app_args: &crate::cfg::AppOptions, bar_args: &crate::cfg::BarOptions) -> Self
    {
        let alg = app_args.sorting_algorithm.clone();

        App {
            info: app_args.info,
            border: app_args.border,
            border_color: app_args.border_color,
            bar_width: bar_args.bar_width,
            looped: app_args.looped,
            bars: Vec::with_capacity(app_args.amount),
            all_bars: Self::create_bars(app_args.amount),
            iter: 0,
            phase: Phase::Appearing,
            phase_started: Instant::now(),
            highlighted: Some((0,0)),
            algorithm: Self::get_algorithm(&alg, app_args.amount),
            algorithm_name: alg,
            last_tick: Instant::now(),
            time_elapsed: Instant::now(),
            timed: Duration::ZERO,
        }
    }
    pub fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        let tick_rate = Duration::from_millis(16);

        loop {
            if Self::handle_events()? {
                break Ok(());
            }
            if self.last_tick.elapsed() >= tick_rate {
                match self.phase {
                    Phase::Appearing => {
                        if self.iter < self.all_bars.len() {
                            self.bars.push(self.all_bars[self.iter]);
                            self.iter += 1;
                        } 
                        else {
                            self.iter = 0;
                            self.phase = Phase::WaitRandomizing;
                        }
                    }
                    Phase::WaitRandomizing => {
                        if self.phase_started.elapsed() >= Duration::from_secs(1) {
                            self.phase = Phase::Randomizing;
                            self.phase_started = Instant::now();
                        }
                    }
                    Phase::Randomizing => {
                        if let Some((a, b)) = self.highlighted {
                            self.bars[a].color = Color::White;
                            self.bars[b].color = Color::White;
                        }
                        if self.iter >= self.bars.len() {
                            self.iter = 0;
                            self.phase = Phase::WaitSorting;
                        }
                        else
                        {
                            let mut rng = rand::rng();

                            let i = self.iter;
                            let j = rng.random_range(0..self.bars.len());

                            self.bars[i].color = Color::Red;
                            self.bars[j].color = Color::Red;

                            self.highlighted = Some((i, j));

                            self.bars.swap(i, j);

                            self.iter += 1;
                        }
                    }
                    Phase::WaitSorting => {
                        if self.phase_started.elapsed() >= Duration::from_secs(1) {
                            self.phase = Phase::Sorting;
                            self.phase_started = Instant::now();
                        }
                    }
                    Phase::Sorting => {
                        if self.algorithm.step(&mut self.bars) {
                            self.phase = Phase::Check;
                        }
                    }
                    Phase::Check => {
                        if self.iter < self.all_bars.len() {
                            self.bars[self.iter].color = Color::LightGreen;
                            self.iter += 1;
                        }
                        else
                        {
                            self.iter = 0;
                            self.timed = self.time_elapsed.elapsed();
                            self.phase_started = Instant::now();
                            self.phase = Phase::WaitDone;
                        }
                    },
                    Phase::WaitDone => {
                        if self.phase_started.elapsed() >= Duration::from_secs(3) {
                            if self.looped
                            {
                                self.phase = Phase::Done;
                                self.phase_started = Instant::now();
                            }
                        }
                    },
                    Phase::Done => {
                        let n = self.all_bars.len();
                        self.bars = Vec::with_capacity(n);
                        self.all_bars = Self::create_bars(n);
                        self.phase = Phase::Appearing;
                        self.algorithm = Self::get_algorithm(&self.algorithm_name,n);
                        self.time_elapsed = Instant::now();   
                    }
                }

                self.last_tick = Instant::now();
            }

            terminal.draw(|frame| {
                self.render(frame);
            })?;
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let algorithm = self.algorithm.name();
        let [r,g,b] = self.border_color;
        
        let inner = if self.border {
            let block = Block::bordered().border_type(widgets::BorderType::Rounded).border_style(Style::new().fg(ratatui::style::Color::Rgb(r,g,b))).bold().title(algorithm);
            frame.render_widget(Clear, area);
            frame.render_widget(&block, area);

            block.inner(area)
        } 
        else {
            area
        };

        match self.phase {
            Phase::WaitDone => {
                if self.info {
                    let text = Paragraph::new(format!("Sorted!\n\nelapsed: {:.3?}\nSorting Algorithm: {}\nBar Count: {}\n\n{}", self.timed, algorithm, self.all_bars.len(), if !self.looped { "Press 'q' to quit" } else { "" })).style(Color::White).alignment(Alignment::Left);                    
                    frame.render_widget(text, inner);
                }
                frame.render_widget(
                    BarsWidget {
                        bars: &self.bars,
                        max: self.bars.len() as u64,
                        bar_width: self.bar_width,
                    },
                    inner,
                );
            }
            _ => {
                frame.render_widget(
                    BarsWidget {
                        bars: &self.bars,
                        max: self.bars.len() as u64,
                        bar_width: self.bar_width,
                    },
                    inner,
                );
            }
        }
    }
    fn handle_events() -> std::io::Result<bool> {
        let timeout = Duration::from_secs_f64(1.0 / 50.0);
        if !event::poll(timeout)? {
            return Ok(false);
        }
        if let Some(key) = event::read()?.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                _ => {}
            };
        }
        Ok(false)
    }
}
#[derive(Clone, Copy)]
pub struct Bar {
    pub value: u64,
    pub color: Color,
}

struct BarsWidget<'a> {
    bars: &'a [Bar],
    max: u64,
    bar_width: Option<u16>,
}

impl Widget for BarsWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if self.bars.is_empty() {
            return;
        }

        for (index, bar) in self.bars.iter().enumerate() {
            let x_start = area.x + ((index * area.width as usize) / self.bars.len()) as u16;
            let x_end = area.x + (((index + 1) * area.width as usize) / self.bars.len()) as u16;

            let max_width = x_end.saturating_sub(x_start).max(1);

            let bar_width = self.bar_width.unwrap_or(max_width).min(max_width);

            let height = ((bar.value as f32 / self.max as f32) * area.height as f32).floor() as u16;

            for y in 0..height {
                let draw_y = area.bottom().saturating_sub(1 + y);

                for w in 0..bar_width {
                    let draw_x = x_start + w;

                    if draw_x >= x_end || draw_x >= area.right() {
                        break;
                    }

                    if let Some(cell) = buf.cell_mut((draw_x, draw_y)) {
                        cell.set_symbol("█").set_style(Style::default().fg(bar.color));
                    }
                }
            }
        }
    }
}