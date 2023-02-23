use std::{thread, time::Duration};

mod terminal;

use terminal::{TerminalController, TerminalEvent};


fn interpreter(terminal_controller: &mut TerminalController) -> crossterm::Result<()> {
    loop {
        match terminal_controller.run()? {
            TerminalEvent::Continue => {},
            TerminalEvent::End => break,
            TerminalEvent::String(s) => {
                let string = format!("{}\n", s);
                terminal_controller.output_string(&string)?;
                thread::sleep(Duration::from_secs(2));
            }
        }
    }
    Ok(())
}

fn main() {
    let mut terminal_controller = TerminalController::new();
    match interpreter(&mut terminal_controller) {
        Err(e) => eprintln!("{}", e),
        _ => println!("finish"),
    }
}
