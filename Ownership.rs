// In this we see the concept of ownership in Rust.
fn main(){

     first()
}
fn first(){
    let x=9;

    let y=x;

    println!("{}" , y);



    let a=String::from("Ansh Goyal");
    let b=a.clone();
    println!("{}---{}",a,b);
}