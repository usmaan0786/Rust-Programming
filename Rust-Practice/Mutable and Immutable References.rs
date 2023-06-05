

// one muutable reference in a scope.
// We cam have many immutable references.
        // immutable references are memory safe because these can't change their values
        // The key difference between "&mut referencing" and "&heap_num" is that "&mut referencing" refers 
        // to a variable that exists on the stack, while "&heap_num" refers to a value that exists on the heap. 
// mutable and immutable cannot co exist.
// scope of a reference.
//Data should not change when immutable referencing are in scope.
use std::vec;


fn main() {

    let mut heap_num = vec![1, 32, 4, 67];
    let ref1 = &mut heap_num; 
    let ref2 = &mut heap_num; // here Rule 01 occurs 
    println!("Ref1 = {:?} Ref2 = {:?}", ref1, ref2);

    
    let mut heap_num = vec![1, 32, 4, 67];
    let ref1 = &heap_num; 
    let ref2 = &heap_num; // here Rule 02 occurs
    println!("Ref1 = {:?} Ref2 = {:?}", ref1, ref2);


    let mut heap_num = vec![1, 32, 4, 67];
    let ref1 = &heap_num; 
    let ref2 = &heap_num; // here Rule 02 occurs
    let ref3 = &mut heap_num; // here Rule 03 occurs
    println!("Ref1 = {:?} Ref2 = {:?} ref02 = {:?}", ref1, ref2, ref3);

    // what should we do when we both require mutable and immutable referencing at the same time
    // since they cannot coexist

    // ANSWER: rule03 exists in a certain scope 

    let mut heap_num = vec![1, 32, 4, 67];
    let ref1 = &heap_num; 
    let ref2 = &heap_num;
    println!("Ref1 = {:?} Ref2 = {:?}", ref1, ref2); // ref has scope from line 39 to 40
    let ref3 = &mut heap_num; // here we can use mutable referencing becuase now it is out of the scope of 

    // the scope of reference is when it is first used and last used

}