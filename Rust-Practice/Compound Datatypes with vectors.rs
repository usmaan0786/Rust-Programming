fn main() {

    // vectors

    let mut number_vec = vec![1, 2, 2, 4, 4, 5];

    println!("{}", number_vec[3]);
    println!("{:?}", number_vec );

    let mut array_with_same_element = vec![0;10];
    println!("{:?}", array_with_same_element );


    array_with_same_element[2] = 3455;

    println!("{:?}", array_with_same_element );


    // vector slicing
    let subset_vec = &&number_vec[0..3];

    println!("{:?}", subset_vec);

    // also we have built in functions there 
    
    // like .len() and .get(index:100)

    number_vec.push(34);
    number_vec.push(121);

    println!("{:?}", number_vec );

    // to remove the value

    number_vec.remove(3);

    println!("{:?}", number_vec );    

    println!("to check the value is in the array = {:?}", number_vec.contains(&121));  // i assigned & becuase this func reqiures 
    // a reference to a number as an input

}