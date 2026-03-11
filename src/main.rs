use clap::Parser;
use rand::RngExt;

#[derive(Parser)]
#[command(author = "CapThunder19", version = "1.0", about = "Password generator")]
struct Args {
    #[arg(short, long, default_value_t = 12)]
    length: usize,
}

fn password_generator(length: usize) -> String {
   let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                  abcdefghijklmnopqrstuvwxyz\
                  0123456789!@#$%^&*";
   let mut rng = rand::rng();
   (0..length)
         .map(|_| {
                let index = rng.random_range(0..charset.len());
                charset.chars().nth(index).unwrap()
         })
 .collect()

}

fn main() {
    println!("|-----------------------------|");
    println!("|   Password Generator        |");
    println!("|-----------------------------|");

    let args = Args::parse();
    let password = password_generator(args.length);
    println!("Generated Password: {}", password);
}