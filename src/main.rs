/* 

fn main() {
    //variables
    //scalar data types
    let x = 5;
    println!("Hello, {}",x);
    //x is immutable, varibles are immutable
    let mut y = 5;
    println!("Hey, {}",y);

    let mut tf:bool= true;
    tf=false;
    let mut text = "This is my text";
    text="new text";
    println!("the value of tf is {}", tf);
    println!("the text value is {}", text);
}


*/
//scalar data types amd composite data types
//scalar-integers,booleans,floating-point number, characters
//composite stores a list of values

use std::vec;



/* 
fn main(){
    let t =(1,2,3,);//tuple
    println!("The values in t are {},{},{}", t.0,t.1,t.2);

    let (a,b,c)= t;
    println!("The value of b is {}", b);

    //array
    let mut l =[1,2,3,4,5];
    println!("The list has afirst value of {}",l[0]);

    let mut vec_example = vec![1,2,3];//vector
    println!("Starting vector data is {:?}",vec_example);
    vec_example.push(4);
    println!("Added a 4: {:?}", vec_example);
    vec_example.pop();
}
*/


//control flow
fn main(){
    let mut t = (1, "hello", true);//cannot be looped through

    let mut l = [1,2,3,4,5];
    for item in l{
        println!("> {}",item);
    }
    

    let mut vec_example = vec![1,2,3];

    for data in vec_example.iter(){
        if *data == 2{
            println!("> {}", data);
        }
        else{
            println!("others {}", data)
        }
        
    }
}


















