/*
   A simple application for learning
   named project1
*/

use std::fmt;


fn main() {
    println!("Good Morning Ayaz!");
    let mut a: u32 = 50;
    let b: u32 = 345;
    let c: u32 = 120;
    let mut d: u32;
    let e: u32;

    println!("==================================================");
    println!("Ownership Move Example\n");

    let mut v0 = Vec::new();

    for i in 1..1000 {
        v0.push(i);
    }

    take(v0);
    // println!("Vector[15]: {}", v0[15]);

    println!("==================================================");
    println!("I'll add some numbers!\n");
    
    check_number(a);
    check_number(a);
    check_number(b);
    check_number(c);

    d = add(a, c);
    check_number(d);
    d = add(b, c);
    check_number(d);

    e = if d > b {
        12
    } else {
        8
    };

    check_number(e);

    println!("==================================================");
    println!("Loop Example\n");

    // loop example
    loop {
        if a >= 100 {
            break;
        }

        a += 1;
    }

    let mut count = 0;
    let result = loop {
        if count >= 5 {
            break count + 6;
        }

        count += 1;
    };

    println!("result of loop: {}", result);
    println!("Limit reached: {:?}", a);

    println!("==================================================");
    println!("While Example\n");

    while a > 0 {
        if a <= 3 {
            println!("{:?}", a);
        }

        a -= 1;
    }

    println!("Limit reached: {:?}", a);

    println!("==================================================");
    println!("Using Match option 0\n");

    name_matcher("Ayaz");
    name_matcher("Gunay");
    name_matcher("Mina");
    name_matcher("Hikmet");

    println!("==================================================");
    println!("Working with enum\n");

    print_color(Color::Yellow);
    print_color(Color::Blue);

    println!("==================================================");
    println!("Working with struct\n");

    let item0 = StockItem::new(3, 5);
    let item1 = StockItem::new(54, 7);

    item0.show();
    println!("item0 sum: {}\n", item0.sum());
    item1.show();
    println!("item1 sum: {}\n", item1.sum());

    println!("item0: {}", item0);
    println!("item1: {}", item1);

    println!("item0: {:#?}", item0);
    println!("item1: {:#?}", item1);

    println!("==================================================");
    println!("Loop with label\n");

    let mut count_loop = 0;
    'a: loop {
        println!("Loop a");
        
        'b: loop {
            println!("Loop b");
            if count_loop >= 5 && count_loop < 10 {
                println!("Loop b break");
                break;
            } else if count_loop >= 10 {
                println!("Exited from loop a");
                break 'a;
            }
            count_loop += 1;
        }
        count_loop += 1;
    }

    println!("==================================================");
    println!("Iteration with for\n");

    let list = vec![45, 45, 34, 78, 64, 89, 9];

    for l in list {
        println!("List item: {}", l);
    }

    println!("==================================================");
    println!("Iteration using exclusive range with for\n");

    for num in 1..11 {
        println!("Range item: {}", num);
    }

    println!("==================================================");
    println!("Iteration using inclusive range with for\n");

    for num in 1..=11 {
        println!("Range item: {}", num);
    }

    println!("==================================================");
    println!("Match option 1\n");
    
    let num1 = 55;

    match num1 {
        50 ..= 100 => println!("between 50 - 100"),
        10 | 20 | 30 | 40 => println!("one of 10 | 20 | 30 | 40"),
        _ => println!("Something else"),
    }

    println!("==================================================");
    println!("Match complex option 2\n");

    let pair = (5, 25);

    match pair {
        (x, 25) => println!("y matches, x: {}", x),
        (5, y) => println!("x matches, y: {}", y),
        _ => println!("Something else"),
    }

    println!("==================================================");
    println!("Match more complex option 3\n");

    let pair2 = (5, -5);

    match pair2 {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal zero"),
        (x, _) if x % 2 == 0 => println!("Even"),
        _ => println!("Something else"),
    }

    println!("==================================================");
    println!("Match more complex option 4\n");

    let num2 = 55;

    match num2 {
        tmp_num @ 50 ..= 100 => println!("between 50 - 100: tmp_num: {}", tmp_num),
        tmp_num @ (10 | 20 | 30 | 40) => println!("one of 10 | 20 | 30 | 40: tmp_num: {}", tmp_num),
        _ => println!("Something else"),
    }

    println!("==================================================");
    println!("Match as expression\n");

    let num3 = match num2 {
        tmp_num @ 50 ..= 100 => tmp_num,
        tmp_num @ (10 | 20 | 30 | 40) => tmp_num,
        _ => 0,
    };

    println!("Result of match: {}", num3);

    println!("==================================================");
    println!("Destructuring tuple\n");

    let numbers = one_two_three();
    let (x, y, z) = one_two_three();
    println!("x: {}, numbers.0: {}", x, numbers.0);
    println!("y: {}, numbers.1: {}", y, numbers.1);
    println!("z: {}, numbers.2: {}", z, numbers.2);
}

fn one_two_three() -> (u32, u32, u32) {
    (1, 2, 3)
}

// Trait implementation
impl fmt::Display for StockItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.quantity, self.price)
    }
}


#[derive(Debug)]
struct StockItem {
    quantity: u32,
    price: u64,
}

impl StockItem {
    fn new(quantity: u32, price: u64) -> StockItem {
        StockItem {
            quantity,
            price,
        }
    }
}

impl StockItem {
    fn show(&self) {
        println!("Item.quantity: {:?}", self.quantity);
        println!("Item.price: {:?}", self.price);
    }

    fn sum(&self) -> u64 {
        self.quantity as u64 * self.price
    }
}


enum Color {
    Magenta,
    Yellow,
    Red,
    Blue,
}

fn print_color(c: Color) {
    match c {
        Color::Magenta => println!("Magenta"),
        Color::Yellow => println!("Yellow"),
        Color::Red => println!("Red"),
        _ => println!("other color"),
    }
}

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
        println!("{:?} to be honest, I don't how big is this number", a);
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
