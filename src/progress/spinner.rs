use console::{style, Term};
use core::fmt;
use spinoff::{spinners, Color};

pub struct Spinner {
    spinner: spinoff::Spinner,
}

impl Spinner {
    pub fn new<S: fmt::Display>(message: S) -> Self {
        Term::stdout().clear_line().ok();
        let spinner = spinoff::Spinner::new(
            spinners::Dots,
            style(message).dim().italic().to_string(),
            Color::Green,
        );
        Self { spinner }
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.spinner.clear();
    }
}
