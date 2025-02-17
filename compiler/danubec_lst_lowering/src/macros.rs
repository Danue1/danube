macro_rules! report {
    ($level:expr, $($tt:tt)*) => {{
        use danubec_diagnostic::{Diagnostic, Level};

        let mut diagnostic = Diagnostic::new();
        diagnostic.report($level, format!($($tt)*));

        Err(diagnostic)?
    }};
}

#[macro_export]
macro_rules! opt {
    ($opt:expr, $($tt:tt)*) => {
        match $opt {
            Some(ret) => ret,
            None => report!(Level::ERROR, $($tt)*),
        }
    };
}

#[macro_export]
macro_rules! error {
    ($($tt:tt)*) => {
        report!(Level::ERROR, $($tt)*)
    };
}
