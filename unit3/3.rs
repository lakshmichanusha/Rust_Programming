fn main() {
  let a=10;
  let b=20;
  let operator='*';
    match operator {
            '+' => {
                println!("{}", a + b);
            },
            '-' => {
                println!("{}", a - b);
            },
            '*' => {
                println!("{}", a * b);
            },
            '/' => {
                if b == 0{
                    println!("Division by 0 is undefined");
                }
                else {
                    println!("{}", a / b);
                }
            },
            '%' => {
                if b == 0{
                    println!("Mod 0 is undefined");
                }
                else {
                    println!("{}", a % b);
                }
            },
            _ => println!("{}", "invalid operator"),
        }
}
