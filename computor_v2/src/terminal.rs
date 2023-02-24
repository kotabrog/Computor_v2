use std::io::{self, stdout, Write};

use crossterm::{terminal, cursor, event, execute, queue};
use crossterm::terminal::ClearType;
use crossterm::event::{Event, KeyEvent, KeyCode, KeyModifiers, KeyEventKind, KeyEventState};

struct CleanUp;

impl CleanUp {
    fn new() -> Self {
        terminal::enable_raw_mode().expect("Could not activate raw mode");
        Self {}
    }
}

impl Drop for CleanUp {

    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

pub enum TerminalEvent {
    String(Box<String>),
    Continue,
    End,
    Up(Box<String>),
    Down(Box<String>),
}


struct Reader;

impl Reader {
    fn new() -> Self { Self {} }

    fn read(&self) -> crossterm::Result<Event> {
        loop {
            match event::read()? {
                Event::Key(event) => return Ok(Event::Key(event)),
                _ => {/* wip */},
            }
        }
    }
}


struct RowContents {
    content: Box<String>,
    index: usize,
}

impl RowContents {
    fn new() -> Self {
        Self {
            content: Box::new(String::new()),
            index: 0
        }
    }

    fn insert(&mut self, ch: char) {
        self.content.insert(self.index, ch);
        self.index += 1;
    }

    fn insert_str(&mut self, string: &str) {
        self.content.insert_str(self.index, string);
        self.index += string.len();
    }

    fn remove(&mut self) {
        if !self.content.is_empty() && self.index != 0 {
            self.index -= 1;
            self.content.remove(self.index);
        }
    }

    fn get_index(&self) -> usize {
        self.index
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    fn move_index(&mut self, is_left: bool) -> bool {
        if is_left {
            if self.index == 0 {
                return false
            }
            self.index -= 1;
        } else {
            if self.index == self.content.len() {
                return false
            }
            self.index += 1;
        }
        true
    }

    fn move_content(&mut self) -> Box<String> {
        self.index = 0;
        std::mem::replace(&mut self.content, Box::new(String::new()))
    }
}


struct EditorContents {
    content: String,
}

impl EditorContents {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    fn push(&mut self, c: char) {
        self.content.push(c)
    }

    fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl io::Write for EditorContents {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}


struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
}

impl CursorController {
    fn new() -> crossterm::Result<Self> {
        let mut cursor_controller = Self { cursor_x: 0, cursor_y: 0 };
        cursor_controller.update_position()?;
        Ok(cursor_controller)
    }

    fn move_cursor(&mut self, direction: KeyCode, window_size: &(usize, usize)) -> i32 {
        match direction {
            KeyCode::Left => {
                if self.cursor_x == 0 {
                    self.cursor_x = window_size.0 - 1;
                    if self.cursor_y == 0 {
                        return -1
                    } else {
                        self.cursor_y -= 1;
                    }
                } else {
                    self.cursor_x -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_x == window_size.0 - 1 {
                    self.cursor_x = 0;
                    if self.cursor_y != window_size.1 - 1 {
                        self.cursor_y += 1;
                    } else {
                        return 1
                    }
                } else {
                    self.cursor_x += 1;
                }
            }
            _ => unimplemented!(),
        }
        0
    }

    fn update_position(&mut self) -> crossterm::Result<()> {
        let pos = cursor::position()?;
        self.cursor_x = pos.0 as usize;
        self.cursor_y = pos.1 as usize;
        Ok(())
    }

    fn update_from_value(&mut self, value: usize, window_size: &(usize, usize)) {
        self.cursor_x = self.cursor_x + value;
        self.cursor_y = (self.cursor_y + ((self.cursor_x - 1) / window_size.0))
            .min(window_size.1 - 1);
        self.cursor_x = self.cursor_x % window_size.0;
    }

    fn update_from_minus_value(&mut self, value: usize, window_size: &(usize, usize)) {
        let mut x = self.cursor_x as i64 - value as i64;
        if x < 0 {
            let mut y = 0;
            y = (x.abs() - 1) / window_size.0 as i64 + 1;
            self.cursor_x = (x + y * window_size.0 as i64) as usize;
            self.cursor_y = self.cursor_y.saturating_sub(y as usize);
        } else {
            self.cursor_x = x as usize;
        }
    }

    fn get_position(&self) -> cursor::MoveTo {
        cursor::MoveTo(
            self.cursor_x as u16,
            self.cursor_y as u16
        )
    }
}


struct Output {
    win_size: (usize, usize),
    row_contents: RowContents,
    editor_contents: EditorContents,
    cursor_controller: CursorController,
}

impl Output {
    fn new() -> crossterm::Result<Self> {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Ok(Self {
            win_size,
            row_contents: RowContents::new(),
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new()?,
        })
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))?;
        print!("> ");
        stdout().flush()?;
        Ok(())
    }

    fn move_cursor(&mut self, direction: KeyCode) -> crossterm::Result<()> {
        let move_flag = self.row_contents.move_index(direction == KeyCode::Left);
        if move_flag {
            queue!(self.editor_contents, cursor::Hide)?;
            let move_count = self.cursor_controller.move_cursor(direction, &self.win_size);
            match move_count {
                1 => queue!(self.editor_contents, terminal::ScrollUp(1))?,
                -1 => queue!(self.editor_contents, terminal::ScrollDown(1))?,
                _ => {},
            }
            queue!(
                self.editor_contents,
                self.cursor_controller.get_position(),
                cursor::Show,
            )?;
            self.editor_contents.flush()?;
        }
        Ok(())
    }

    fn insert_char(&mut self, c: char) -> crossterm::Result<()> {
        queue!(self.editor_contents, cursor::Hide)?;

        // Check if scrolldown is required
        let index = self.row_contents.get_index();
        let content_length = self.row_contents.get_content().len();
        let diff = content_length - index;
        let x = self.cursor_controller.cursor_x + diff;
        let y = self.cursor_controller.cursor_y + ((x + 1) / self.win_size.0);
        if y >= self.win_size.1 {
            let value = y - self.win_size.1 + 1;
            self.cursor_controller.cursor_y = self.cursor_controller.cursor_y.saturating_sub(value);
            queue!(
                self.editor_contents,
                terminal::ScrollUp(value as u16),
                self.cursor_controller.get_position(),
            )?;
        }

        self.cursor_controller.move_cursor(KeyCode::Right, &self.win_size);

        queue!(
            self.editor_contents,
            terminal::Clear(ClearType::FromCursorDown)
        )?;

        self.row_contents.insert(c);
        let index = self.row_contents.get_index() - 1;
        for (i, c) in self.row_contents.get_content().chars().enumerate() {
            if i >= index {
                self.editor_contents.push(c);
            }
        }

        queue!(
            self.editor_contents,
            self.cursor_controller.get_position(),
            cursor::Show,
        )?;
        self.editor_contents.flush()?;
        Ok(())
    }

    fn enter(&mut self) -> crossterm::Result<Box<String>> {
        self.cursor_controller.update_from_value(
            self.row_contents.get_content().len() - self.row_contents.get_index(),
            &self.win_size
        );
        queue!(
            self.editor_contents,
            cursor::Hide,
            cursor::MoveTo(
                self.cursor_controller.cursor_x as u16,
                self.cursor_controller.cursor_y as u16
            ),
            terminal::Clear(ClearType::FromCursorDown),
        )?;
        self.editor_contents.push_str("\r\n");
        queue!(self.editor_contents, cursor::Show)?;
        self.editor_contents.flush()?;
        self.cursor_controller.update_position()?;
        Ok(self.row_contents.move_content())
    }

    fn backspace(&mut self) -> crossterm::Result<()> {
        let index = self.row_contents.get_index();
        if index == 0 {
            return Ok(())
        }
        self.row_contents.remove();
        queue!(self.editor_contents, cursor::Hide)?;
        let move_count = self.cursor_controller.move_cursor(KeyCode::Left, &self.win_size);
        match move_count {
            -1 => queue!(self.editor_contents, terminal::ScrollUp(1))?,
            _ => {},
        }
        queue!(
            self.editor_contents,
            self.cursor_controller.get_position(),
            terminal::Clear(ClearType::FromCursorDown),
        )?;
        let index = self.row_contents.get_index();
        for (i, c) in self.row_contents.get_content().chars().enumerate() {
            if i >= index {
                self.editor_contents.push(c);
            }
        }
        queue!(
            self.editor_contents,
            self.cursor_controller.get_position(),
            cursor::Show,
        )?;
        self.editor_contents.flush()?;
        Ok(())
    }

    fn output_string(&mut self, string: &str) -> crossterm::Result<()> {
        queue!(
            self.editor_contents,
            cursor::Hide,
            self.cursor_controller.get_position(),
        )?;
        self.editor_contents.flush()?;

        terminal::disable_raw_mode().expect("Could not disable raw mode");
        self.editor_contents.push_str(string);
        self.editor_contents.flush()?;
        terminal::enable_raw_mode().expect("Could not activate raw mode");

        self.editor_contents.push('\r');
        self.editor_contents.push_str("> ");
        queue!(self.editor_contents, cursor::Show)?;
        self.editor_contents.flush()?;
        self.cursor_controller.update_position()?;
        Ok(())
    }

    fn up_and_down(&mut self, is_up: bool) -> TerminalEvent {
        self.cursor_controller.update_from_minus_value(
            self.row_contents.get_index(),
            &self.win_size
        );
        let string = self.row_contents.move_content();
        if is_up {
            TerminalEvent::Up(string)
        } else {
            TerminalEvent::Down(string)
        }
    }

    fn change_content(&mut self, string: &str) -> crossterm::Result<()> {
        queue!(
            self.editor_contents,
            cursor::Hide,
            self.cursor_controller.get_position(),
            terminal::Clear(ClearType::FromCursorDown),
        )?;
        self.row_contents.insert_str(string);
        self.editor_contents.push_str(string);
        queue!(self.editor_contents, cursor::Show)?;
        self.editor_contents.flush()?;
        self.cursor_controller.update_position()?;
        Ok(())
    }
}


pub struct TerminalController {
    reader: Reader,
    output: Output,
}

impl TerminalController {
    pub fn new() -> crossterm::Result<Self> {
        Output::clear_screen().expect("Could not clear screen");
        Ok(TerminalController {
            reader: Reader::new(),
            output: Output::new()?,
        })
    }

    fn process_keypress(&mut self, event: KeyEvent) -> crossterm::Result<TerminalEvent> {
        match event {
            KeyEvent {
                code: KeyCode::Char('q' | 'c' | 'd'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => return Ok(TerminalEvent::End),
            KeyEvent {
                code: direction @
                    (KeyCode::Left
                    | KeyCode::Right),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.output.move_cursor(direction)?,
            KeyEvent {
                code: direction @
                    (KeyCode::Up
                    | KeyCode::Down),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => return Ok(self.output.up_and_down(direction == KeyCode::Up)),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.output.insert_char(c)?,
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => return Ok(TerminalEvent::String(self.output.enter()?)),
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.output.backspace()?,
            _ => {/* wip */}
        }
        Ok(TerminalEvent::Continue)
    }

    pub fn run(&mut self) -> crossterm::Result<TerminalEvent> {
        let _cleanup = CleanUp::new();
        let ret = match self.reader.read()? {
            Event::Key(event) => self.process_keypress(event),
            _ => todo!("wip key"),
        };
        ret
    }

    pub fn output_string(&mut self, string: &str) -> crossterm::Result<()> {
        let _cleanup = CleanUp::new();
        self.output.output_string(string)
    }

    pub fn change_content(&mut self, string: &str) -> crossterm::Result<()> {
        let _cleanup = CleanUp::new();
        self.output.change_content(string)
    }
}
