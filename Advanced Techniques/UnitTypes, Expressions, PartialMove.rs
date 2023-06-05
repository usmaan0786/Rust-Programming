
// Unit Types and Unit Values

/*
fn f1() -> (){ // it will return a unit type 

}

fn f2(){ // no return 

}

fn division(divident : f32, divisor : f32) -> Result<(), String> {
    let answer = match divisor {
        0.0 =>{
            Err(String::from("Error : Division by zero"))
        }
        _ =>{
            Ok(())
        }
    };
    answer
}
fn main(){
    let x = (); // indicated by unit type with nothing inside 

    f1(); // explicitly stating that it should return a unit value 
    f2(); //implicitly returning a unit value

    let x = {
        println!("HI my name is usman");
        println!("HI my name is umer")
    };
  
    division(345.6, 123.0);

} */

// Expressions vs Statements