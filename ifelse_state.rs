// If , If else statement practise 
fn main(){
    first();
    second();
}
fn first(){
    let no =4;

    if no <5 {
        println!("Condition is true");
    }else{
       println!("Condition is false");  
    }
}

// If else 
fn second() {
    let num=18;
    
    if num % 2 ==0 && num%3==0 {
         println!("Number is divisible by both 2 and 3");
    }else if num%2==0{
         println!("The number is even"); 
    }else if num%3==0{
          println!("The number is divisible by 3");
    }else{
        println!("None of the conditions are met")
    }
}