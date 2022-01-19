use std::{env, panic, str::FromStr};

use crate::{format_backtrace, BacktraceConfig};

pub fn install_panic_handler() -> Result<(), ()> {
    let config =
        BacktraceConfig::from_str(&env::var("BETTER_BACKTRACE").unwrap_or_else(|_| "".into()))?;

    panic::set_hook(Box::new(move |panic_info| {
        eprintln!("{}", panic_info);
        eprintln!("Backtrace:");
        format_backtrace(&config, std::io::stderr());
    }));
    Ok(())
}
