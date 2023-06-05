   // -------------------------------------------
   // 			Iterators     
   //           	- Basics
   //          		- Some useful functions for iterators
   //           	- Common statistics 
   // -------------------------------------------

   fn main() 
   {
       let some_vec = vec![1, 2, 3,4,5,6,7];
       let mut iter = some_vec.iter();
       //The variable now is an iterator, which we can use to iterate through all the values of the vector.

       // it is not doing anything usefull it is useful when we have to do some operations on them
       println!("The iterator : {:?}", iter); 
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next());
       println!("{:?}",iter.next()); 

   let a:Vec<u32> = vec![0,1, 2, 4,5,6,9,8,7];
       
   let mut check = a.iter().any(|&x| x > 0);     // it uses the reference to values that's why we will pass the values by reference by writing and x.
   println!("The value of the any fucntion is {}",check);
   
   let check = a.iter().all(|&x| x > 0);  // this function will return a true if all the elements satisfies the given condition.  
   println!("The value of the all fucntion is {}",check);
   
   let check = a.iter().find(|&&x| x > 0); // wrote && because, as pointed out before, the DOT title uses references, so the value is given to the refernce 
  // Moreover, the function also uses references to values, so we are required to use double references 
   println!("The value of the  function find  is {}",check.unwrap());
   
   
   let check = a.iter().position(|&x| x > 4);  // return the index or the position of the element satisfying a certain condition.
   println!("The value of the  function position is {}",check.unwrap());
   
   let check = a.iter().rposition(|&x| x >4);  // if interested in finding out the index position from the ending part or from the extreme right
   println!("The value of the function rposition is {}",check.unwrap());
   
   
   let check = a.iter().max();     
   println!("The value of the function max is {}",check.unwrap());
 
   let check = a.iter().min();  
   println!("The value of the function min is {}",check.unwrap());
 
   let check:u32 = a.iter().sum();
   let check:u32 = a.iter().product(); 
   println!(" {:?}", check); 
   
   
   let mut iter = a.iter().rev(); // reverse the order of the values in the iterator.
   println!("The result of applying the rev fucntion {:?}", iter); 
   println!(" {:?}", a); 
}