use core::Hua;

fn main() {
    // get argument from command line
    let args: Vec<String> = std::env::args().collect();
    let file = &args[1];

    // read file
    let contents = std::fs::read_to_string(file).expect("Something went wrong reading the file");

    let mut hua = Hua::new();
    hua.populate().unwrap();
    hua.evaluate(&contents).unwrap();
}
