//In This we practise Loops in Rust
//There are 3 types of loops in rust: loop , For and while loop.  
//Loop is a keyword that allows you to repeatedly execute a block of code as long as the provided condition evaluates to true. 


fn main(){

    first()
  
}
// Loop
fn first(){
    let mut x=0;
    loop{
        x +=1;
    println!("The value of x is {}",x);
    if x >=5 {
        println!("\nLoop stopped");
        break ;
    }
}
}