fn main() {
    // rust has two main data type of string

    // that is STR and string
    // string slice are imutable. 

    // let some_string = "my name is usman";
    // println!("{}",some_string );

    // let mut capital_string = String::from("my name is usman");
    // println!("{}" , capital_string);

    // capital_string.push('s');
    // println!("{}", capital_string);

    
    // capital_string.pop();
    // // capital_string.pop();
    // // capital_string.pop();
    // // capital_string.pop();
    // println!("{}", capital_string);

    // capital_string.push_str(" and my father name is Manzoor Ellahi");

    // println!("{}", capital_string);

    // // there are also multiple built in functions on strings

    // let number = 6;
    // let num_str = number.to_string();

    // println!("is the  umber really a string = {}", num_str == "6");

    // let some_chr = 'a';
    // let char_str = some_chr.to_string();

    // println!("is the char really a string  = {}", char_str == "a");

    // let s1= "usman".to_string();
    // let s2= "manzoor".to_string();

    // let s3 = format!("my firstname is {} and my lastname is {}",s1,s2 );
    // println!("{}", s3);


    // tuples can't grow in size

    let my_info = ("My salary", 50_000);
    println!("the first element is ({}) and the second element ({})",my_info.0 , my_info.1);
    println!("to print the full tuple  = {:?}", my_info);

    let (salary, salary_variable) = my_info; // D structuring

    // also you can grab first and second values and assign them to diff vairables

    let nested_tuple = (5, 5.0, (3,(68.9, 100.0, 12), 2), "Usman Manzoor");
    let element = nested_tuple.2.1.2;
    println!("{}",element);

    let empty_tuple = ();

    let mut number_array = [1, 2, 4, 5, 6];
    for variable in number_array{
        println!("{}", variable);
    }

    println!("........................................", );
    println!("{:?}",number_array[0]);
    println!("{:?}",number_array );


    let array_with_same_elements = [0; 10];

    let mut string_arr0 = ["apple", "mango", "grapes"];
    let mut string_arr1 = ["Usman Manzoor"; 5 ];

    let mut number_arr = [1, 2, 3, 4, 5, 6];
    let subset_arr = &number_arr[0..3];  // array slicing

    println!("{:?}", number_arr);
    println!("{:?}", subset_arr);
    println!("{:?}",array_with_same_elements);
    println!("{:?}",string_arr0 );
    println!("{:?}",string_arr1 );

    println!("the array is occupying ({}) bytes ", std::mem::size_of_val(&number_arr));

    let check_index = number_arr.get(2); // if value not found it will return None
    println!("Check index  = {:?}",check_index);
    
    



    



}

