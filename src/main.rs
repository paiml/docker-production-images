fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let uid = std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            s.lines()
                .find(|l| l.starts_with("Uid:"))
                .and_then(|l| l.split_whitespace().nth(1))
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(0);
    std::process::exit(hellod::run(&args, uid, &mut std::io::stdout()));
}
