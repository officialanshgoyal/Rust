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
    let  tup: (i32,f64,u8) = (100,"hello",45);
    println!("the tuple values are {:?}",tup);
    let (x,y,z) = tup;
    println!("{} ,{},{} ", x, y, z);

    // arrays  
    let mut arr:[i32;5]=[100,200,300,400,500];
    println!("Array Values are {:?}",arr);
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