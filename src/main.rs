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

fn take(v: Vec<u32>) {
    println!("v[10]: {} v[100]: {}", v[10], v[100]);
}

fn name_matcher(n: &str) {
    // Using match
    match n {
        "Ayaz" => println!("Hi Ayaz, I hope you are doing good!"),
        "Mina" => {
            println!("Hi Mina, I will do here some other things!");
            println!("this is the other thing that I;m ddoing here {}", n);
        },
        "Hikmet" => println!("Hi Hikmet, I hope you are doing good!"),
        _ => println!("Nice to meet you {}", n),
    }
}

fn main() {
    let mut a: u32 = 50;
    let b: u32 = 345;
    let c: u32 = 120;
    let mut d: u32;
    let mut v0 = Vec::new();

    for i in 1..1000 {
        v0.push(i);
    }

    take(v0);
    // println!("Vector[15]: {}", v0[15]);

    println!("Good Morning Ayaz!");
    println!("I'll add some numbers!");

    
    check_number(a);
    check_number(a);
    check_number(b);
    check_number(c);

    d = add(a, c);
    check_number(d);
    d = add(b, c);
    check_number(d);

    // loop example
    loop {
        if a >= 100 {
            break;
        }

        a += 1;
    }

    println!("Limit reached: {:?}", a);

    while a > 0 {
        if a <= 3 {
            println!("{:?}", a);
        }

        a -= 1;
    }

    println!("Limit reached: {:?}", a);

    // ==================================================
    // Using match

    name_matcher("Ayaz");
    name_matcher("Mina");
    name_matcher("Hikmet");
    name_matcher("Gunay");
}
