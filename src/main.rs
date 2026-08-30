mod input;

fn main() {
    let rx = input::spawn();

    loop {
        if let Ok(input) = rx.recv() {
            println!("{input:?}");
        }
    }
}
