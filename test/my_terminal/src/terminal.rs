use std::io::{self, stdout, Write, Seek};

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

    fn move_index(&mut self, is_left: bool) {
        self.index = (if is_left {self.index.saturating_sub(1)} else {self.index + 1})
            .min(self.content.len());
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
    cursor_row: usize,
}

impl CursorController {
    fn new() -> CursorController {
        Self { cursor_x: 0, cursor_y: 0, cursor_row: 0 }
    }

    fn move_cursor(&mut self, direction: KeyCode, window_size: &(usize, usize)) {
        match direction {
            KeyCode::Left => {
                if self.cursor_x == 0 {
                    self.cursor_x = window_size.1 - 1;
                    self.cursor_y = self.cursor_y.saturating_sub(1);
                } else {
                    self.cursor_x = self.cursor_x.saturating_sub(1);
                }
            }
            KeyCode::Right => {
                if self.cursor_x == window_size.0 - 1 {
                    self.cursor_x = 0;
                } else {
                    self.cursor_x += 1;
                    if self.cursor_y != window_size.1 - 1 {
                        self.cursor_y += 1;
                    }
                }
            }
            _ => unimplemented!(),
        }
    }

    fn update_from_content_index(&mut self, index: usize, window_size: &(usize, usize)) {
        let index = index + 2;
        self.cursor_x = index % window_size.0;
        self.cursor_y = self.cursor_row + (index / window_size.0);
        self.cursor_y = self.cursor_y.min(window_size.1 - 1);
    }

    fn update_cursor_row(&mut self, row: usize) {
        self.cursor_row = row;
    }

    fn get_refresh_cursor_row(&mut self, content_len: usize, window_size: &(usize, usize)) -> (usize, usize) {
        let size = content_len + 2;
        let y_length = size / window_size.0 + 1;
        let new_cursor_row = self.cursor_row.min(window_size.1.saturating_sub(y_length));
        (new_cursor_row, self.cursor_row - new_cursor_row)
    }
}


struct Output {
    win_size: (usize, usize),
    row_contents: RowContents,
    editor_contents: EditorContents,
    cursor_controller: CursorController,
}

impl Output {
    fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            row_contents: RowContents::new(),
            editor_contents: EditorContents::new(),
            cursor_controller: CursorController::new(),
        }
    }

    fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    fn refresh_row(&mut self, new_cursor_row: usize) {
        self.editor_contents.push_str("> ");
        self.editor_contents.push_str(self.row_contents.get_content());
        self.cursor_controller.update_cursor_row(new_cursor_row);
        self.cursor_controller.update_from_content_index(
            self.row_contents.get_index(),
            &self.win_size
        );
    }

    fn refresh_screen(&mut self) -> crossterm::Result<()> {
        let (new_cursor_row, diff) = self.cursor_controller.get_refresh_cursor_row(
            self.row_contents.get_content().len(),
            &self.win_size
        );
        if diff > 0 {
            queue!(
                self.editor_contents,
                terminal::ScrollDown(diff as u16),
            )?;
        }
        queue!(
            self.editor_contents,
            cursor::Hide,
            cursor::MoveTo(0, new_cursor_row as u16),
            terminal::Clear(ClearType::FromCursorDown),
        )?;
        self.refresh_row(new_cursor_row);
        let cursor_x = self.cursor_controller.cursor_x;
        let cursor_y = self.cursor_controller.cursor_y;
        queue!(
            self.editor_contents,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )?;
        self.editor_contents.flush()
    }

    fn move_cursor(&mut self, direction: KeyCode) {
        self.row_contents.move_index(direction == KeyCode::Left);
        // self.cursor_controller.move_cursor(direction, &self.win_size);
    }

    fn insert_char(&mut self, c: char) {
        self.row_contents.insert(c)
    }

    fn enter(&mut self) -> crossterm::Result<Box<String>> {
        self.cursor_controller.update_from_content_index(
            self.row_contents.get_content().len(),
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
            cursor::Show,
        )?;
        self.editor_contents.push_str("\r\n");
        Ok(self.row_contents.move_content())
    }

    fn backspace(&mut self) {
        self.row_contents.remove();
    }

    fn output_string(&mut self, string: &str) -> crossterm::Result<()>{
        self.editor_contents.push_str(string);
        self.editor_contents.flush()?;
        let pos = cursor::position()?;
        self.cursor_controller.update_cursor_row(pos.1 as usize);
        Ok(())
    }
}


pub struct TerminalController {
    reader: Reader,
    output: Output,
}

impl TerminalController {
    pub fn new() -> Self {
        Output::clear_screen().expect("Could not clear screen");
        TerminalController {
            reader: Reader::new(),
            output: Output::new()
        }
    }

    fn process_keypress(&mut self, event: KeyEvent) -> crossterm::Result<TerminalEvent> {
        match event {
            KeyEvent {
                code: KeyCode::Char('q' | 'c'),
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
            } => self.output.move_cursor(direction),
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            } => self.output.insert_char(c),
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
            } => self.output.backspace(),
            _ => {/* wip */}
        }
        Ok(TerminalEvent::Continue)
    }

    pub fn run(&mut self) -> crossterm::Result<TerminalEvent> {
        let _cleanup = CleanUp::new();
        self.output.refresh_screen()?;
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
}
