mod generator;

pub fn print_random_number(){
    let n = generator::gen_run();
    println!("Random u8: {}", n);
}
