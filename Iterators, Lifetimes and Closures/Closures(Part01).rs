   // -------------------------------------------
   // 			Closures   
   //           	- Basic Syntax 
   //           	- Closure with inputs 
   //           	- Same variable for different closures  
   //           	- Ownership and closures  
   //           	- Inference of the output and inputs
   //           	- Passing closure as a function argument
   // -------------------------------------------



//  |...| { ... } 

fn main(){
    let x  = 5; 
    let square = || println!("\n\n The square of variable x is {} \n\n ",x*x);  // closure is assigned to variable of square
    square(); 
}

/* 
We can call this closure by rating the name of the variable.
Before we proceed further, we should note a key point with regards to closures.
The point is that the closure captures its environment.
This means that the closure will capture the variables from the cord segment in which it is defined.
In this case, the variable X is known inside the closure body.
This behavior is unlike functions inside the body of the function.
inside the body of the functions.
Only those variables are known which are defined inside the body or which are passed to the function
In this case, although we did not pass the variable of X to the closure, however, it is still known inside the body of the closure.
In summary, all the variables in the code segment in which the closure is defined are visible to the closure


 */



fn main(){
        let x  = 5; 
        

        let square = |num:i32| println!("\n\n The square of {} is {} \n\n ",num, num*num); 


        square(x);  

        let y = 15; 
        square(y); 
    }
    
/*
Now defining some inputs to closures.
You may note that for the inputs we need to mention the types as well as as well as the type, just
 */



    fn main(){
        let x  = 5; 
        

        let square = |num:i32| println!("\n\n The square is {}",num*num);
        let square = |num:i32| println!("\n\n The square is {} \n\n",num*num*num); 


        square(x);  

        let y = 15; 
        square(y); 
    }
    

/*
Now, let us look at some interesting things about the closure.
The first point to note is that redefining a variable with a different closure will use the new definition.
Explanation: Instead of printing the square, I will print the cube. so here we update the print statement
This means now that we have two different closures in the two lines.
In fact, we have redefined the variable so that it corresponds to a different closure.

Let us see which closure will be called by the call to square by executing this code.

You may note that in both the calls, the second definition is being called and the code is displaying to us the cube of the value.
The ownership rules in case of closures are applied in pretty much the same way as in connection with the ftns

 */


fn main() {
 
    let print_user_age = |general_info:String, name: &str, age| println!("{}\n\t{}: {}", general_info, name, age);
    let general_info = String::from("The details are"); 
    let (person_name, person_age) = (String::from("Nouman"), 51);  
    
    print_user_age(general_info, &person_name, person_age); 


    println!("The variable has been moved {}", person_name);   
   
}
 
 /*
 let go in some details 
 Now the general underscore info variable ownership will be transferred to that of the variable inside the closure.
While the variable person underscore name ownership will remain with the variable of person, underscore name and will not be transferred to the variable of name inside the closure.
 

On Copy Notes ................

In summary, we have to pay attention to the ownership rules when we are passing values to the closure.

now moving to next point
The closures are able to infer the outputs and inputs themselves and it is unlike functions where we have to explicitly mention and set up the inputs and output types.


  */



fn main(){
        
    
    let square = |num|  num*num; 

    let x = 5; 
    square(x); 
    
    let y = 105.5; 
    square(y);   // will complain   
}

/*
You may note that currently the input and output types are unknown for the closure.
Next, I will define a variable X and will call this closure with the variable x.

You may note that as soon as I call the closure, the type will be updated for the inputs.
subsequent calls to the same closure will use the same inputs and same output types.

and it will locked with that type of closure and when we call it with other types of closure the rust complier will complain

now move to the next topic
We can also pass the closure to a function as a parameter.

 */


// here y is denominator
fn division<F: Fn(f32) -> bool>(x: f32, y:f32, f: F)  { // here the third input is generic
    if f(y) == true                                    // We will also need to mention the same generic just after the name of the function.
    { 
        print!("\n\n The division result is {} \n\n" ,x/y);
    } else {
        println!("\n\n Dvision is Not possible \n\n ");
    } 
}

/*
In this program we will write a function which will be used to compute the division of two numbers.
Moreover, the same the same function will use a closure to check if the denominator of the fraction passed to the function is a zero or not.



 */

fn main() {
    
    let division_status = |y:f32| { if y != 0.0 {true} else {false} };

    division(5.0, 10.0, division_status); 
    division(54.0, 0.0, division_status); 

}

/*
here we defined the closure

here we check the reslt by compile the code
The result is the division of the two numbers, while for the second call, the result is the print message that the division is not possible.
 */