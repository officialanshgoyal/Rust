//In This we practise Loops in Rust
//There are 3 types of loops in rust: loop , For and while loop.  
//Loop is a keyword that allows you to repeatedly execute a block of code as long as the provided condition evaluates to true. 


fn main(){

    first();
   second();
    third();
    fourth();
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
// While Loop
fn second(){
    let mut y =1;
    
    while y < 10 {
        println!("Value of y is {} ",y) ;
        y+=1;
    }
    println!("\nWhile loop completed")
}

fn fourth(){
    let a=[10,20,30,40,50,60,160,170,180];

    let mut index =0;

    while index < 8 {
    println! ("Element at Index {} is {}",index,a[index]);
    index +=1;
    }
}
    
//For Loop
fn third(){
    for num in 1..11{
       println!("Value of num is {}",num);
    }
    println!("\nFor loop completed")
}