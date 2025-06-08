use lib1::hello_from_lib1;
use lib2::hello_from_lib2;

fn main() {
    println!("test");
    println!("{}", hello_from_lib1());
    println!("{}", hello_from_lib2());
}



