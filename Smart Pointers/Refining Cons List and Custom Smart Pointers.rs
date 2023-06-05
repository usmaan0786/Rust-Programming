    // -------------------------------------------
    // 			Smart Pointers
    //          	- Box Pointers 
    //          	- Issue with Box Pointer implementation of Cons variant in List
    // 			- Custom Defined Smart Pointers 
    // -------------------------------------------

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("{:?}",list);
}


//Example #2:   

enum List {
   Cons(i32, Option<Box<List>>),    
 }
 
use List::{Cons};

fn main() {  
   let list =   List::Cons(1, Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))));
}





    struct MySmartPointer{
        value: i32
    }    // add it at the end 


    impl MySmartPointer {
        fn new(x:i32)-> MySmartPointer{
           MySmartPointer{value: x}
    
        }
    }
    
  
/*    
   trait Deref {
    type Target: name_of_type; // becasue inner value in our type is i32
    fn deref(&self) -> &Self::Target; // the input to this function is a ref to self
   }  
*/

   use std::ops::Deref; 

   impl Deref for MySmartPointer {
   type Target = i32; // becasue inner value in our type is i32
   fn deref(&self) -> &i32 { // the input to this function is a ref to self
      &self.value //returns data
      }
   }
   
    /* 
    pub trait Drop {
    fn drop(&mut self); // this function will be executed when smart pointers goes out of scope
   }                    /// ensures that any memory that is associated and allocated to the smart pointer is being given back.
                        // when the smart pointyers is no more in use
   */

    
    impl Drop for MySmartPointer{
        fn drop(&mut self){
           println!("dropping MySmartPointer object from memory {:?}", self.value);
        }
    }
    
        
     fn main() {
        
        let a = 50;
        let b = &a;   //let b = Box::new(a); 
        println!("{}", 50 == a);
        println!("{}", 50 == *b ); // deref trait
        //println!("{}", a == b); // we cannot comapre a value with a ref
        
          
        let sptr1 = MySmartPointer::new(a);// at last this pointer drops
        let sptr2 = MySmartPointer::new(a+3);// then this pointers drops at second
        let sptr3 = MySmartPointer::new(a+6); // this pointere drops first
         
        println!("{}", a==*sptr1); // *(sptr1.deref()) // In the first place explain the deref methos and returns a ref to inner data contained 
                                                    //in the memory and then in the second case explain the drop or secondly, it follows the reference
                                                    // to access the contents that are being stored over there.
        
        let z = *sptr1; // here z is my smart pointer And when I write a * before the sptr1, then its type will be changed to i32
                        // which in this case is the inner value associated with the pointer.

        println!("The items will be dropped after this line executes");     
        drop(sptr2);  // byt this drop function for sptr2 has been called first and then later remaining pointers will be called in reverse order. 
    }


use std::ops::Deref;

struct Courses {
    courses_list: Vec<String>,
    department: String,
    year: u32,
}

impl Deref for Courses {
    type Target = Vec<String>;

    fn deref(&self) -> &Vec<String> {
        &self.courses_list
    }
}

fn main() {
    let current_courses = Courses {
        courses_list: vec!["CS101".to_string(), "CS303".to_string(), "CS400".to_string()],
        department: String::from("CS"),
        year: 2022,
    };

    println!("{:?}", *current_courses);
}


