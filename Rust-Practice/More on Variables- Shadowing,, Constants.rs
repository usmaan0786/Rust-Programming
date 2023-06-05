

fn main() {
    
    let(firstname , secondname) = (250, 480.22);
    let largenumber = 1_000_000;

    let x = 255;

    print!("the value of the variable in octal is {:o} and in hexadecimal is {:X} and for the binay {:b}",x,x,x );


    let n1 = 14;
    let n2 = 15.6;

    // let n3 = n1 + n2;

    // typecasting

    let n3 = n1 as f64 + n2; // here n1 is converted into float

    print!("Result = {}",n3);

    // Shadowing

    let s = 5;
    let s = 5*5;

    println!("the value of s is {}",s);

    // let mut p = 5;
    let p = 5*5;
    let p =60;
    println!("the value of p is {}",p );


    const MAX_SALARY:u32 = 100_0000;

    println!("result = {}",MAX_SALARY);
}


