//! `hellod`: a deliberately small program. It prints a greeting and one status line
//! so image-building lessons can show what ends up inside a production image.

use std::io::Write;

/// Greeting for `name`; an empty name falls back to "world".
pub fn greeting(name: &str) -> String {
    let who = if name.is_empty() { "world" } else { name };
    format!("hello, {who}")
}

/// Status line the Dockerfile lessons check for: who runs the process.
pub fn status(uid: u32) -> String {
    if uid == 0 {
        "status: running as root".to_string()
    } else {
        format!("status: running as uid {uid}")
    }
}

/// Runs the program; returns the exit code.
pub fn run(args: &[String], uid: u32, out: &mut dyn Write) -> i32 {
    let name = args.first().map(String::as_str).unwrap_or("");
    let ok =
        writeln!(out, "{}", greeting(name)).is_ok() && writeln!(out, "{}", status(uid)).is_ok();
    if ok {
        0
    } else {
        2
    }
}
