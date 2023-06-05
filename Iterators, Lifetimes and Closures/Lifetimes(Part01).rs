   // -------------------------------------------
   // 			Lifetimes  
   //           	- Dangling Reference 
   //           	- Undetermined Lifetimes
   // -------------------------------------------

//    use std::vec;

fn main(){
    let i:&i32; // holds refernece to an integer                
    {                           
                            
        let j = 5;              
        i = &j; // set i as a reference to a vairable j
    }                           
    println!("The value of i = {}", i);   // it gives error the vairable cannot live long
}                               

fn main() {

    let some_int = 10;  
    let additional_int = some_fn(some_int);  // the variable i is no more exists here b/c it is defined inside 
    println!("{}", additional_int); // the ftn an its scpoe is limited to ftn

}

fn some_fn(i: i32) ->  &i32{   // gives error of lifetimes specifier 
    &i                            
}


fn main(){
    let int1 = 5; 
    let int2 = 10; 
    let result = greater(&int1,&int2);
    
}

fn greater(i:&i32,j:&i32) -> &i32 { // this scenenerio is referring to undetermined lifetime case
    // because he don't know which lifetime the returning reference may be corresponding to
    if i> j {
        i                   
    } else {j          
    }

}

// Now

fn main(){
    let s_1 = "Hello";
    
    
    let v;
    {
        let s_2 = String::from("World");       
        v = some_fn(s_1, s_2.as_str());    
    }                                       
    println!("\n\n{} \n\n", v);             
    }
    
    fn some_fn(first_str: &str, second_str: &str) -> &str { // here it gives error at &str becuase he dont know to which 
        // variable this returning reference is pointing to 
        first_str           
                            
    }
    
    