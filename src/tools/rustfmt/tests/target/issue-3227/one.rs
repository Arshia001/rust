// rustfmt-version: One

fn main() {
    thread::spawn(|| {
        while true {
            println!("iteration");
        }
    });

    thread::spawn(|| loop {
        println!("iteration");
    });
}
