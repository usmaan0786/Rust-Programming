   // -------------------------------------------
   // 			Closures   
   //           	- A quick recap
   //           	- Borrow by immutable reference
   //           	- Borrow by a mutable reference
   //           	- Moving of a value into a closure
   // -------------------------------------------


/*
recap: 
The first syntax was to explicitly define the inputs and outputs.
The second syntax is to skip the types of the inputs and outputs.
The third syntax was when we have only a single statement in the body of the closure. In this case, we do not add the curly brackets.
 */

fn main(){

let some_closure_1 = |x: u32| -> u32 { x + 1 }; 
let some_closure_2 = |x| { x + 1 };    
let some_closure_3 = |x|  x + 1 ; 
} 




fn main(){
let mut vec_1 = vec![1, 2,3];
let some_closure = || {
    // vec_1 is being used by reference.  
    println!("Vec 1 : {:?}", vec_1);     
};

println!("Vec_1: {:?}",vec_1);              
some_closure();    
                   
vec_1[1] = 15;  // Now, if I add a statement of, let's say VEC and then within the square brackets one equals to 15, cimpiler will explain
// This is because the vector is being borrowed as immutable and we cannot mutate or change its value until the immutable reference goes out of scope.
}

/*
We will first look at the case when the closure uses an immutable reference to a variable
Let us now try to access the same vector outside the closure in a print statement.
Now Finally we invoke the closure.
now see what happened in this case
The first thing to note is that we do not have mentioned the input to this closure.
However, since the closure captures its environment, so all the variables which are currently in scope will be available inside the closure also.

Rust compiler will be engaged in some inference since the value is not being updated
So by default the rest compiler will assume that the value will be used inside the closure as an immutable
reference and therefore it will not change since the value inside the closure is being used as an immutable
Therefore, VEC underscore one is available for us as it is being used by reference.

Remember the rest ownership rules according to which multiple readings of the values of variables or reference to the value of variables are okay and does not create any issues.
Therefore, the print statement outside the body of the closure is not creating any issues.

 */


fn main(){
    let mut vec_1 = vec![4,5,6];
    let mut some_closure = || {
        
        vec_1.push(35); 
       
    };
    
    //println!("vec_2 {:?}", vec_2);  
    vec_1[1] = 15;    // this also gives error because multiple updation are not allowed to the mutable refernce 
    // In other words, we cannot borrow as mutable twice. this is against the rust ownership rules


    // According to the ownership rules, we can only have a single mutable reference
    // Not only the immutable but mutable references are also not allowed when a variable is being used as
    // Unless we are done with the call to the closure and the variable representing the closure is no more in scope.
    // we are free to do whatever with vector once the call has been done That is after the line of some underscore closure.
    println!("vec_1 {:?}", vec_1); //   when a variable is being borrowed as mutable, therefore the rest compiler will complain once the updation
                                   // is done we may proceed using the value              
    some_closure();    
                        
    
    // vec_1[2] = 15; 
    
    }
    
    /*
    Now let us see what happens when a closure uses a mutable reference to some variable.
    

     */
   
fn main(){
    let mut vec_1  = vec![1,2,3];
    let some_closure = || {
        
        let vec_2 = vec_1; // value is moved inside the closure
    };

    // Basically what is happening here is 
    // The rust compiler will try again to infer how to use the value of the variable Vec underscore one since
    // the line of code is actually moving its value and transferring the ownership from from VEC one to that of vec_2
    // Therefore, the rest compiler will use the original values and note as a reference since the ownership has been  transferred inside the closure.
    // Therefore, the use of VEC underscore word inside the print statement is not valid because the variable vec underscore word is is no more in scope and has lost the ownership.
    // Moreover, the value of VEC underscore two is also dropped since its scope was limited to the body of the closure
    // So therefore the use of VEC two inside the print statement is not valid.
    some_closure();     
    println!("vec 1 = {:?} ", vec_1);          
    println!("vec 2 = {:?} ", vec_2);          
    
}
    
    /*
    Now,
    We will be looking at a case when a value is being moved inside a closure.  
     */


    /*
    SO at last
    we have looked at three scenarios in which the value of a variable
    may be used inside the closure that is by immutable reference, by mutable reference and by actual value.
     */