macro_rules! string_concat {
    () => {
        String::new()
    };
    ($some : expr) => {{
        let mut temp = String::new();
        temp.push_str($some);
        temp
    }};


    // if i remove , from below line code then i have to remove it from the main also
    // and space is considereed to be a default delimiter
    // we can also use ; also ($($some_str : expr); *) then in main string_concat!("Usman Manzoor" ;"Ayyan Manzoor");
    // this convention is followed in that case
    ($($some_str : expr), *) => {{   // * = zero or many, + = one or many, ? = 1 or 0
        let mut temp = String::new();
        $(temp.push_str($some_str);)*;
        temp
    }};
}

macro_rules! vec_mac {
    ($($element: expr), *) => {{
        let mut some_vec = Vec::new();
        $(some_vec.push($element);)*
        some_vec
    }};
}

fn main() {
    string_concat!();
    let str_input = string_concat!("Usman Manzoor", "Ayyan Manzoor", "Umer Manzoor");
    println!(" {}  ", str_input);

    let string_vec = vec_mac!(1 ,3, 45, 456, 2, 1,3, 3,234);
    println!("{:?}", string_vec);
}
