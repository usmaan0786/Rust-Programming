fn stack_function(mut var: i32){  // this function will accepet a number as an input
    var = 43;
    println!("Value outside the main function = {}", var);
}

fn heap_function(var: &mut Vec<i32>){ // This will accept a vec as an input
    
    var.push(123);
    println!("vector outside the main function = {:?}", var );
}
fn main() {
    let stack_num = 32;
    let mut heap_vec = vec![1, 3, 4];

    stack_function(stack_num);
    println!("Value inside the main function = {}", stack_num);

    heap_function(heap_vec); // ownership is passed to var in heap_function this means that when the function completes and finishes
    its value will automatically be droppepd((((((BY THRIRD RULE ))))))
    println!("Vector inside the main function = {:?}", heap_vec);

    // Now we use referencing here
    heap_function(&mut heap_vec);
    println!("Vector inside the main function = {:?}", heap_vec);

    let data1 = String::from("Usman");
    let data2 = String::from("Manzoor");

    let combine_string = vec![&data1, &data2];  // instead of m,oving them to the another string which take high memory, I simply did used 
    // references to combine both strings
    println!("Result = {:?}", combine_string);
}