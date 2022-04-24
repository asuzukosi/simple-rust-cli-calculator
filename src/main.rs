use::std::env::{args, Args};


fn main() {
    let mut _args: Args = args();

    let first: f32 = _args.nth(1).unwrap().parse().unwrap();
    let operator:char = _args.nth(0).unwrap().parse().unwrap();
    let second: f32 = _args.nth(0).unwrap().parse().unwrap();

    println!("{:?} {:?} {:?}", first, operator, second);
    println!("The result is {}", operation(operator, first, second));
}


fn operation(operator: char, n1:f32, n2:f32) -> f32{
    if operator == '+' {
        return n1 + n2;
    } else if operator == '-' {
        return n1 - n2;
    } else if operator == '/' {
        return n1 / n2;
    } else if operator == '*' {
        return n1 * n2;
    } else {
        return 0.0;
    }
}