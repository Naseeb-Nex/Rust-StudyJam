fn addition(x : i32, y : i32)->i32{
    x+y
}
fn main() {
    println!("First Program");
    println!("Addition");
    let x = 3;
    let y = 2;
    let z = addition(x, y);
    println!("{}", z);
}
