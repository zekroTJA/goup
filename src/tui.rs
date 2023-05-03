use console::{style, Term};
use std::io::{self, Write};

#[macro_export]
macro_rules! error {
    ( $fmt:expr ) => {
        $crate::tui::print_error(&format!($fmt));
    };
    ( $fmt:expr, $($args:tt),* ) => {
        $crate::tui::print_error(&format!($fmt, $($arg)*));
    };
}

#[macro_export]
macro_rules! warning {
    ( $fmt:expr ) => {
        $crate::tui::print_warning(&format!($fmt));
    };
    ( $fmt:expr, $($args:tt),* ) => {
        $crate::tui::print_warning(&format!($fmt, $($arg)*));
    };
}

#[allow(unused_must_use)]
pub fn print_status(v: &str) {
    Term::stdout().clear_line();
    print!("{}", style(v).dim().italic());
    io::stdout().flush();
}

#[allow(unused_must_use)]
pub fn print_succes(v: &str) {
    Term::stdout().clear_line();
    println!("{}", style(v).green());
}

#[allow(unused_must_use)]
pub fn print_error(v: &str) {
    Term::stdout().clear_line();
    println!(
        "{}: {}",
        style("error:").red().bold(),
        style(v).red().bright()
    );
}

#[allow(unused_must_use)]
pub fn print_warning(v: &str) {
    Term::stdout().clear_line();
    println!(
        "{}: {}",
        style("warning:").yellow().bold(),
        style(v).yellow().bright()
    );
}
