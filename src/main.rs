/*
   A simple application for learning
   named project1
*/

fn add(a: u32, b: u32) -> u32 {
    a + b
}

fn check_number(a: u32) {
    if a < 99 {
        println!("{:?} is small number", a);
    } else if a > 99 && a < 199 {
        println!("{:?} is big number!", a);
    } else if a > 199 && a < 399 {
        println!("{:?} is huge number!", a);
    } else {
        println!("{:?} to be honest I don't how big is this number", a);
    }
}

fn main() {
    let a: u32 = 50;
    let b: u32 = 345;
    let c: u32 = 120;
    let mut d: u32;

    println!("Good Morning Ayaz!");
    println!("I'll add some numbers!");

    
    check_number(a);
    check_number(b);
    check_number(c);

    d = add(a, c);
    check_number(d);
    d = add(b, c);
    check_number(d);


    loop {
        if a >= 100 {
            break;
        }

        a += 1;
    }
    println!("Limit reached!");
}
