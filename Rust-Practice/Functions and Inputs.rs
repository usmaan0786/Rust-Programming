
fn function_with_inputs(name: &str, salary: i32){
    println!("My name is {} and my salary is round about {}", name, salary );
}

fn output_functions(a: i32, b: i32) -> i32{
    a * b
}

fn multiple_output(a: i32, b: i32) -> (i32, i32, i32, i32){
    (a * b, a + b, a - b, a/b)
}

fn main() {

    // let name = "UsmanManzoor";
    // let salary = 50_000;
    // function_with_inputs(name, salary);

    // let result  = output_functions(123, 12312);
    // println!("Result = {}", result);

    // let (multiplication, addition, subtraction, division) = multiple_output(1234, 123);
    // println!("Multiplication = {}, Addition = {}, Subtraction = {}, Division = {}",multiplication, addition, subtraction, division);
    // OR

    // let Result = multiple_output(14124, 1212);
    // println!("Result = {:?}", Result); // and you can destructure it....


    // CODE BLOCKS can be quite handy in arranigng your code and initializing variables and isolating small computations
    // from the rest of the code. This brings clarity to the code.
//     let full_names

    // HOW TO TAKE INPUTS IN RUST
    // rust uses standard input and output library to take input

    let mut n= String::new(); 
    std::io::stdin()
    .read_line(&mut n)
    .expect("Failed to print");

   // trim() ftn is used to trim the trailing white spaces of the string
   // pare() this is used to parses the string into another type whihc in this case is f64.
   // expect() this is execute in case of errors

   // through this we can take inputs in different formats

    let n: f64 = n.trim().parse().expect("failed");  // HERE WE are specifying that input should be in the f64 format
    println!("input = {:?}",n);
}