use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    while let Some(line) = lines.next() {
        let length: i32 = line.unwrap().trim().parse().unwrap();

        for _ in 0..length {
            let line = lines
                .next()
                .expect("there was no next line")
                .expect("the line could not be read");

            println!("{}", line);
        }
    }

    Ok(())
}
