fn new_stack(maxsize: usize) -> Vec<u32>{
    let vec = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack : &mut Vec<u32>) -> Option<u32> { 
    let poped_val = stack.pop();
    println!("The Poped value is : {:?}", poped_val);
    poped_val
}

fn push(stack : &mut Vec<u32>, item: u32, maxsize : usize){
    if stack.len() == maxsize{
        println!("Stack : {:?}", stack);
    }
    else {
        stack.push(item);
        println!("Stack: {:?}", stack);
    }
}

fn size(stack : &Vec<u32>) -> usize{
    stack.len()
}

fn input() -> usize{
    let mut n  = String::new();
    std::io::stdin()
    .read_line(&mut n).expect("Failed to read input");

    let n = n.trim().parse().expect("invalid input");
    n
}
fn main() {
 
    // some application of stack include string traversal, expression evaluation, backtracking and syntax parsing
    // we will use vector for implementing the stack

    println!("Let us first create a stack");
    println!("Please mention the size of stack");

    let size_stack = input();

    let mut stack = new_stack(size_stack as usize);

    loop {
        println!("\n\n ******* MENU ******* \n\n");
        println!("1. Push \n 2. Pop \n 3. Display \n 4. Size \n 5. Exit");
        println!("\n Enter your choice : ");

        let choice = input();

        match choice{
            1 => {
                println!("Enter the value to insert");
                let item : usize = input();
                push(&mut stack, item,  size_stack as usize);
            }

            2 => {
                println!("The element which is poped as {:?}", pop(&mut stack));
            }

            3 =>{
                println!("The elements are {:?}", stack);
            }
            4 => {
                println!("The size of the stack is {}", size(&stack));
            }

            5 => {
                println!("*****Exiting*****");
                break;
            } 
        }
        println!("Do you want to Continue  ********1 = Yes, 2 = No********");
        let status= input();

        if status == 1{
              continue;
        }
        else {
            break;
        }  
    }

}