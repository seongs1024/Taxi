mod environment;

use crate::environment::Env;

fn main() {
    let mut env = Env::default();
    
    println!("{}", env);
}
