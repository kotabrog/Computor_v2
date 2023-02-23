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
                thread::sleep(Duration::from_secs(2));
                terminal_controller.output_string(&string)?;
            }
        }
    }
    Ok(())
}

fn main() {
    let mut terminal_controller = TerminalController::new().expect("Interpreter initialization failed");
    match interpreter(&mut terminal_controller) {
        Err(e) => eprintln!("{}", e),
        _ => println!("finish"),
    }
}
