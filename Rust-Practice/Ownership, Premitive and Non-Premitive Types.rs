use std::vec;

fn main() {

    // let x = 235;
    // let y = x; // here copy occuered
    // println!("x:{} y:{}",x, y);

    // let s1 = "sumajnh";
    // let s2 = s1; // here copy occured
    // println!("output: {}, {}", s1, s2);
    
    let s1 = String::from("Usman");
    let s2 = &s1; // here move occured and s1 is no more valid so i put & sign with s1 and passes reference of s1 to s2.
    // this will lead to more key concepts of borrowing and referencing
    println!("output: {}, {}", s1, s2);

    let v1 = vec![1, 2, 34, 45];
    let v2 = &v1; // what if i really want ot make a copy of the variable and dont want to cvhange the ownership
    println!("v1{:?}, v2{:?}", v1, v2);

    // here a ftn called clone to make a copy of the variable inst5ed of referencing it.
    let v2 = v1.clone();
    println!("v1{:?}, v2{:?}", v1, v2); // vec 1 and vec2 are diff variable and they have their diff ownership

    // IN case of thord rule The scopes
    // {
    //     let my_name = String::from("Usman Manzoor");
    //     println!("Name: {}",my_name);
    // }
    // println!("Name: {}",my_name);



}