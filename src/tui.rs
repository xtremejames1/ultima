use std::{io, vec};
use num_traits::cast::FromPrimitive;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use chrono::{Datelike, Days, Month, Months, NaiveDate, TimeDelta, Weekday};

//TODO refactor out separate calendar stuff, so make separate year, month, and weekly views

#[derive(Debug, Default)]
pub struct CalendarTextUserInterface {
    current_date: NaiveDate,
    selected_date: NaiveDate,
    width: u16,
    selected_column: u16,
    saved_column: u16,
    exit: bool,
}

impl CalendarTextUserInterface {
    pub fn new(initial_date: NaiveDate) -> Self {
        let current_date = initial_date.clone();
        let selected_date = initial_date.clone();
        let width = 37;
        let selected_column = Self::get_column(selected_date, width);
        let saved_column = selected_column;
        let exit = false;
        Self {
            current_date,
            selected_date,
            selected_column,
            width,
            saved_column,
            exit
        }
    }

    fn get_column(date: NaiveDate, width: u16) -> u16 {
        let day = date.day();
        let month = date.month();
        let year = date.year();

        let jan_31_weekday: u16 = NaiveDate::from_ymd_opt(year, 1, 31).unwrap().weekday() as u16;

        let month_num_days: u16 = Month::from_u32(month).unwrap().num_days(year).unwrap().into();
        let month_last_weekday = NaiveDate::from_ymd_opt(year, month, month_num_days.into()).unwrap().weekday() as u16;

        // Find the column which needs to be highlighted
        let weekday_offset = (jan_31_weekday + 7 - month_last_weekday) % 7;
        width - weekday_offset - month_num_days + (day as u16)
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        use Constraint::{Fill, Length, Min};

        let vertical = Layout::vertical([Length(1), Min(0), Length(15)]);
        let [title_area, main_area, calendar_vertical] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([Fill(1); 2]);
        let calendar_horizontal = Layout::horizontal([Min(128), Min(12)]);
        let [left_area, right_area] = horizontal.areas(main_area);
        let [calendar, date] = calendar_horizontal.areas(calendar_vertical);

        frame.render_widget(Block::bordered().title("[ultima forsan]"), title_area);
        frame.render_widget(self, calendar);

        let date_block = Block::bordered()
            .border_set(border::THICK)
            .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM);

        let date_text = Text::from(self.build_date());

        let date_paragraph = Paragraph::new(date_text)
            .centered()
            .block(date_block);

        frame.render_widget(date_paragraph, date);

        //TODO maybe tasks on left and image on right?
        frame.render_widget(Block::bordered().title("Left"), left_area);
        frame.render_widget(Block::bordered().title("Right"), right_area);
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('h') => self.back(),
            KeyCode::Char('j') => self.down(),
            KeyCode::Char('k') => self.up(),
            KeyCode::Char('l') => self.forward(),
            KeyCode::Char('u') => self.back_year(),
            KeyCode::Char('d') => self.forward_year(),
            KeyCode::Char('t') => self.set_date(self.current_date),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn back(&mut self) {
        if self.selected_date.day() != 1 {
            self.selected_date = self.selected_date - Days::new(1);
            self.selected_column -= 1;
        }
        else {
            self.selected_date = self.selected_date - Days::new(1);
            self.selected_column = Self::get_column(self.selected_date, self.width);
        }
        self.saved_column = self.selected_column;
    }

    fn forward(&mut self) {
        if self.selected_date.day() != Month::from_u32(self.selected_date.month().into()).unwrap().num_days(self.selected_date.year()).unwrap() as u32 {
            self.selected_date = self.selected_date + Days::new(1);
            self.selected_column += 1;
        }
        else {
            self.selected_date = self.selected_date + Days::new(1);
            self.selected_column = Self::get_column(self.selected_date, self.width);
        }
        self.saved_column = self.selected_column;
    }

    fn up(&mut self) {
        let new_date = self.selected_date - Months::new(1);
        let new_num_days = Month::from_u32(new_date.month()).unwrap().num_days(new_date.year()).unwrap();
        let day_offset: i16 = ((self.saved_column as i16) - (Self::get_column(new_date, self.width) as i16)).clamp(-(new_date.day() as i16) + 1, new_num_days as i16 - new_date.day() as i16);
        self.selected_date = new_date + TimeDelta::days(day_offset.into());
        self.selected_column = Self::get_column(self.selected_date, self.width);
    }

    fn down(&mut self) {
        let new_date = self.selected_date + Months::new(1);
        let new_num_days = Month::from_u32(new_date.month()).unwrap().num_days(new_date.year()).unwrap();
        let day_offset: i16 = ((self.saved_column as i16) - (Self::get_column(new_date, self.width) as i16)).clamp(-(new_date.day() as i16) + 1, new_num_days as i16 - new_date.day() as i16);
        self.selected_date = new_date + TimeDelta::days(day_offset.into());
        self.selected_column = Self::get_column(self.selected_date, self.width);
    }

    fn back_year(&mut self) {
        let new_date = self.selected_date - Months::new(12);
        let new_num_days = Month::from_u32(new_date.month()).unwrap().num_days(new_date.year()).unwrap();
        let day_offset: i16 = ((self.saved_column as i16) - (Self::get_column(new_date, self.width) as i16)).clamp(-(new_date.day() as i16) + 1, new_num_days as i16 - new_date.day() as i16);
        self.selected_date = new_date + TimeDelta::days(day_offset.into());
        self.selected_column = Self::get_column(self.selected_date, self.width);
    }

    fn forward_year(&mut self) {
        let new_date = self.selected_date + Months::new(12);
        let new_num_days = Month::from_u32(new_date.month()).unwrap().num_days(new_date.year()).unwrap();
        let day_offset: i16 = ((self.saved_column as i16) - (Self::get_column(new_date, self.width) as i16)).clamp(-(new_date.day() as i16) + 1, new_num_days as i16 - new_date.day() as i16);
        self.selected_date = new_date + TimeDelta::days(day_offset.into());
        self.selected_column = Self::get_column(self.selected_date, self.width);
    }

    fn set_date(&mut self, date: NaiveDate) {
        self.selected_date = date;
        self.selected_column = Self::get_column(date, self.width);
    }

    fn build_calendar(&self) -> Vec<Line> {
        // We need to find the earliest day of the week, with respect to 31 January, since that
        // will be at the end of the calendar. This is with respect to the year corresponding to
        // the selected_date.

        let mut calendar_text = Vec::new();

        let selected_day = self.selected_date.day();
        let selected_month = self.selected_date.month();
        let selected_year = self.selected_date.year();

        let current_day = self.current_date.day();
        let current_month = self.current_date.month();
        let current_year = self.current_date.year();

        let jan_31_weekday: u16 = NaiveDate::from_ymd_opt(selected_year, 1, 31).unwrap().weekday() as u16;

        // COLORS
        let column_row_highlight = Color::Indexed(017);
        let selected_color = Color::Yellow;
        let current_color = Color::Green;

        let mut weekday_label: Vec<Span> = Vec::new();
        for n in 1..=self.width {
            let weekday = 
                match Weekday::from_u16((7 * (self.width / 7 + 1) + n - self.width + jan_31_weekday) % 7).unwrap() { // Make sure no integer underflow by adding the right multiple of 7s based on the width.
                    Weekday::Mon => " M ",
                    Weekday::Tue => " T ",
                    Weekday::Wed => " W ",
                    Weekday::Thu => " H ",
                    Weekday::Fri => " F ",
                    Weekday::Sat => " S ",
                    Weekday::Sun => " S ",
                };
            weekday_label.push(
                if n == self.selected_column {
                    weekday.bg(column_row_highlight)
                }
                else {
                    weekday.into()
                }
            );
        }
        weekday_label.push("         ".into());

        calendar_text.push(Line::from(weekday_label));

        // For every day of each month, print a symbol underneath its corresponding weekday.

        for m in 1..13 {
            let mut month_line: Vec<Span> = vec!["   ".into(); self.width.into()];
            let month_enum = Month::from_u32(m).unwrap();
            let num_days = month_enum.num_days(selected_year).unwrap();

            // Find position where the last day should be placed to maintain alignment
            let target_position = Self::get_column(NaiveDate::from_ymd_opt(selected_year, m, num_days.into()).unwrap(), self.width);

            let month_label: Span<'_> = match Month::from_u32(m).unwrap() {
                Month::January => " JAN".into(),
                Month::February => " FEB".into(),
                Month::March => " MAR".into(),
                Month::April => " APR".into(),
                Month::May => " MAY".into(),
                Month::June => " JUN".into(),
                Month::July => " JUL".into(),
                Month::August => " AUG".into(),
                Month::September => " SEP".into(),
                Month::October => " OCT".into(),
                Month::November => " NOV".into(),
                Month::December => " DEC".into(),
            };

            month_line.push(
                if m == selected_month {
                    month_label.bg(column_row_highlight)
                } else {
                    month_label
                });

            month_line.push(match Month::from_u32(m).unwrap() {
                Month::May => format!("    {}", selected_year / 1000).into(),
                Month::June => format!("    {}", selected_year % 1000 / 100).into(),
                Month::July => format!("    {}", selected_year % 100 / 10).into(),
                Month::August => format!("    {}", selected_year % 10).into(),
                _ => {"     ".into()}
            });

            for d in 1..=num_days {
                let day = num_days + 1 - d;
                let day_position = target_position as i32 - (d as i32);

                let day_str = format!(" {}{}", day, if day < 10 { " " } else { "" });
                let mut day_span: Span = day_str.into();

                if m == selected_month || day_position + 1 == self.selected_column as i32 {
                    day_span = day_span.bg(column_row_highlight)
                }
                if m == selected_month && (day as u32) == selected_day {
                    day_span = day_span.bg(selected_color)
                }
                if m == current_month && (day as u32) == current_day && selected_year == current_year {
                    day_span = day_span.fg(current_color).italic()
                }

                month_line[day_position as usize] = day_span;
            }

            // TODO if selected month is this month, then set all spaces to the color, else, make
            // sure that if there is empty sapces on the column, set those

            if m == selected_month {
                for i in 0..(target_position as usize - num_days as usize) {
                    month_line[i] = "   ".bg(column_row_highlight);
                }
                for i in (target_position as usize)..self.width as usize {
                    month_line[i] = "   ".bg(column_row_highlight);
                }
            }
            else {
                if self.selected_column < target_position - num_days as u16 + 1 || self.selected_column > target_position {
                    month_line[self.selected_column as usize - 1] = "   ".bg(column_row_highlight);
                }
            }

            calendar_text.push(Line::from(month_line));
        }

        calendar_text
    }

    pub fn build_date(&self) -> Vec<Line> {
        let first_line: Line = match self.selected_date.weekday() { // Make sure no integer underflow by adding the right multiple of 7s based on the width.
            Weekday::Mon => "Monday".into(),
            Weekday::Tue => "Tuesday".into(),
            Weekday::Wed => "Wednesday".into(),
            Weekday::Thu => "Thursday".into(),
            Weekday::Fri => "Friday".into(),
            Weekday::Sat => "Saturday".into(),
            Weekday::Sun => "Sunday".into(),
        };
        let second_line: Line = format!("{}{}{}",
            self.selected_date.day(),
            match Month::from_u32(self.selected_date.month()).unwrap() {
            Month::January => "JAN",
            Month::February => "FEB",
            Month::March => "MAR",
            Month::April => "APR",
            Month::May => "MAY",
            Month::June => "JUN",
            Month::July => "JUL",
            Month::August => "AUG",
            Month::September => "SEP",
            Month::October => "OCT",
            Month::November => "NOV",
            Month::December => "DEC",
        }, self.selected_date.year()).into();

        vec![first_line, second_line.into()]
    }
}

impl Widget for &CalendarTextUserInterface {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" [calendar] ".bold());
        let instructions = Line::from(vec![
            " Back (day) ".into(),
            "<h>".blue().bold(),
            " Down (month) ".into(),
            "<j>".blue().bold(),
            " Up (month) ".into(),
            "<k>".blue().bold(),
            " Forward (day) ".into(),
            "<l>".blue().bold(),
            " Today ".into(),
            "<t>".blue().bold(),
            " Quit ".into(),
            "<q>".blue().bold(),
        ]);
        let calendar_block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK)
            .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

        let calendar_text = Text::from(self.build_calendar());

        Paragraph::new(calendar_text)
            .centered()
            .block(calendar_block)
            .render(area, buf);
    }
}
