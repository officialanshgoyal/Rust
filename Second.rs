fn main(){
    let mut x =1;   // we use mut because variables are immutable in rust  by default. 
   
     
     x=x+2;
     
    assert_eq!(x,3);
    println!("success");

    let y: i32 = 5;

    println! ("The value of x is {} The value of y is {}",x,y); // {} is used to  insert a variable into the string

    
    
    


}