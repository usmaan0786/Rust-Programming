/* 
macro_rules! practice {
    () => {
        1 + 1
    }; // no assocaited pattern so it will substitute the code 1 + 1 in the main program

    (someting) => {
        println!("Found Nothing here")
    };

    ($e1 : expr , $e2 : expr) => {
        $e1 + $e2
    };

    ($e1 : expr , $e2 : expr; $e3 : expr) => {
        $e1 * ($e2 + $e3)
    };
}
fn main() {
    // practice!();
    println!("{}", practice!());
    practice!(someting);
    println!("{}", practice!(12 , 334));
   //  println!("{}", practice!("Something", 2;"Nothing")); //matches because the invocation is matched with the last rule

   // can be called with any type of brackets
   practice!();
   practice!{};
   practice![];
}
*/


// TYPE Capture
/* 
macro_rules! name {
    ($t : ty) => {{
        let mut n = String::new();
        std::io::stdin()
            .readline(&mut n)
            .expect("Failed to read input");

        let n: $t = n.trim().parse().expect("invalid input");
        n
    }};
}

fn main() {
    println!("Enter a int number");
    let some_input = name!(f32);
} */


// Identifiers

// macro_rules! Identifiers {
//    () => {
//       let mut n = 6;
//    };
// }

// fn main(){
//    Identifiers!();
//    x = x + 1;
// }

// Solution

/*
macro_rules! Identifiers {
   ($var : ident) => { 
      $var = $var + 1; 
   };
}

fn main(){
   let mut x = 4;
   Identifiers!(x);
}*/

// Ownership of macros
// and All macros practice

macro_rules! create_function {
    ($func_name : ident, $input : ident, $input_type: ty, $output_type : ty) => {
       
       fn $func_name($input : $input_type) -> $output_type{
          println!("You called {:?}() with the input type of {:?}", stringify!($func_name), stringify!($input_type));
          $input
       }
    };
 }
 
 create_function!(f1, x, i32, i32);
 
 fn main(){
    let y = f1(124);
 }