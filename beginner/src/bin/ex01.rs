fn main() {
    let life = 42;
    println!("hello");
    println!("{:?}", life);
    println!("{:?} {:?}", life, life);

    let a = 99;
    if a > 99 {
        println!("Big number");
    } else {
        println!("Small number");
    }

    let a = 100;
    if a > 200 {
        println!("Big number");
    }
}
