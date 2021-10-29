use std::io::Result;

fn print_lines(s: &str) {
    for line in s.lines() {
        print_line(line);
    }
}

fn print_file(path: &str) -> Result<()> {
    use std::fs::File;
    use std::io::BufRead as _;
    use std::io::BufReader;

    let reader = BufReader::new(File::open(path)?);

    for line in reader.lines() {
        let line = line?;
        print_line(&line);
    }

    Ok(())
}

fn print_line(s: &str) {
    if s.is_empty() {
        return;
    }
    let bytes = s.as_bytes();
    print!("{:02x}", bytes[0]);

    for &b in &bytes[1..] {
        print!("  {:02x}", b);
    }
    print!("\n");
}

fn exec(args: &mut impl Iterator<Item = String>) -> Option<Result<()>> {
    if let Some(first) = args.next() {
        if first == "-f" {
            if let Some(second) = args.next() {
                Some(print_file(&second))
            } else {
                None
            }
        } else {
            print_lines(&first);
            Some(Ok(()))
        }
    } else {
        None
    }
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    while let Some(result) = exec(&mut args) {
        result?;
    }
    Ok(())
}
