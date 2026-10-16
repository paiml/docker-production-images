use hellod::{greeting, run, status};

struct Broken;
impl std::io::Write for Broken {
    fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
        Err(std::io::Error::other("closed"))
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn greets_world_by_default() {
    assert_eq!(greeting(""), "hello, world");
}

#[test]
fn greets_by_name() {
    assert_eq!(greeting("docker"), "hello, docker");
}

#[test]
fn status_names_root_and_others() {
    assert_eq!(status(0), "status: running as root");
    assert_eq!(status(10001), "status: running as uid 10001");
}

#[test]
fn run_writes_both_lines() {
    let mut out = Vec::new();
    assert_eq!(run(&["x".to_string()], 7, &mut out), 0);
    assert_eq!(
        String::from_utf8(out).unwrap(),
        "hello, x\nstatus: running as uid 7\n"
    );
}

#[test]
fn run_without_args_greets_world() {
    let mut out = Vec::new();
    assert_eq!(run(&[], 0, &mut out), 0);
    assert!(String::from_utf8(out)
        .unwrap()
        .starts_with("hello, world\n"));
}

#[test]
fn run_reports_a_write_error() {
    assert_eq!(run(&[], 0, &mut Broken), 2);
}

#[test]
fn run_reports_a_failure_on_the_second_line() {
    struct OnceThenBroken(bool);
    impl std::io::Write for OnceThenBroken {
        fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
            if self.0 {
                Err(std::io::Error::other("closed"))
            } else {
                self.0 = true;
                Ok(b.len())
            }
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    assert_eq!(run(&[], 0, &mut OnceThenBroken(false)), 2);
}

#[test]
#[ignore]
fn calls_nothing() {}
