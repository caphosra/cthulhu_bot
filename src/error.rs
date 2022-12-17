use anyhow::Result;

/// Provides a way to print all of the errors.
pub trait DisplayErr {
    /// Displays almost all of the information of the errors.
    fn eprint_all(&self);
}

impl DisplayErr for Result<()> {
    fn eprint_all(&self) {
        if let Err(err) = self {
            eprintln!("[BOT ERROR] {}", err);
            err.chain()
                .skip(1)
                .for_each(|cause| eprintln!("[BOT ERROR] because: {}", cause));
        }
    }
}
