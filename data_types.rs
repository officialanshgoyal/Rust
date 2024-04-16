// Scaler Type
// Integer , Floating Point, String , char, Boolean
fn main() {
    let no = 5;
    println!("The value of number is {}",no);
    
    let flt_num : f32 = 6.789;
    println!("The float value of the number is {:?}",flt_num);

    let strng = "Hello World";
    println!("The string value is {}",strng);

    let ch = 'A';
    println!("The character value is {}",ch);

    let boolval = true;
    println!("Is it True? {}",boolval);

    // compund types -- where we have more than one data type in a single variable 
    // array ,tuple, dictonary

    // tuples 
    let mut tup = (100,"hello",45);
    println!("the tuple values are {:?}",tup);
    println! ("the first value in the tuple is {}",tup.0 );
    println!( "the second value in the tuple is {}",tup.1 );
    tup.0=200;
    println!("after changing the value of the first element to 200 the tuple becomes {:?}",tup);
    let (x,y,z) = tup;
    println!("{} ,{},{} ", x, y, z);

    // arrays  
    let mut arr:[i32;5]=[100,200,300,400,500];
    println!("Array Values are {:?}",arr);
    println!("First Value In Array Is {}",arr[0]);
    arr[1]=2000;
    println!("Modified Array Value {:?}",arr);

    // dictionary or hashmap
    use std::collections::HashMap;
    let mut hm= HashMap::new();
    hm.insert("one".to_string(),1);
    hm.insert("two".to_string(),2);
    hm.insert("three".to_string(),3);
    println!("Dictionary Values are {:?} ",hm);

}