use std::io;




fn main() {
    println!("Hello, world!");

    println!("Let's guess a number！");

    println!("Please type a number:");

    // let mut guess = String::from("a");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    // 注意这里stdin().read_line()，会将标准输入的所有内容存放到字符串中，不会覆盖原有的内容
    // 所以如果guess有初始值，只会往后面继续添加

    let guess: u32 = guess.trim().parse().expect("you type a rong number, please check");
    println!("You type a number is {}", guess);

}
