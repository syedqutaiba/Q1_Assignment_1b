fn main() {
    question_1();
    question_2();
    question_3();
    question_4();
    question_5();
    question_6();
    question_7();
}

fn question_1() {
    let number: i32 = 40;

    println!("{}\n", number);
}

fn question_2() {
    
    let score = 75;

    println!("Score of batsman is : {}", score);

    let score = score * 2;

    println!("Score 2x of batsman is : {}\n", score);

}

fn question_3() {
    
    let name = "Syeda Anabia Zehra";
    let age: i32 = 4;
    let class = "KG-4";
    let roll_number: i32 = 24;

    println!("Name is :{}", name);
    println!("Age is : {}", age);   
    println!("Class is : {}", class);
    println!("Roll # is : {}", roll_number);
    println!("\n");

}

fn question_4() {
    let number1: f32 = 3.7;
    let number2: f32 = 3.2;

    let addition = number1 + number2;
    let subtraction = number1 - number2;
    let multplication = number1 * number2;
    let division = number1 / number2;
    let modulus = number1 % number2;

    println!("{} + {} = {}", number1, number2, addition);
    println!("{} - {} = {}", number1, number2, subtraction);
    println!("{} x {} = {}", number1, number2, multplication);
    println!("{} / {} = {}", number1, number2, division);
    println!("{} % {} = {}", number1, number2, modulus);
    println!("\n");

}

fn question_5() {
    let tup = ("Syeda Anabia Zehra", 4, "KG-4",  24);
    let (name, age, class, roll_number) = tup;

    println!("Name : {}", name);
    println!("Age is {}", age);
    println!("Class is : {}", class);
    println!("Roll number is : {}", roll_number);
    println!("\n");
}

fn question_6() {
    let week = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let mut x = 0;

    for counter in 1..8 {
        println!("{}", week[x]);
        x = x + 1;
    }
println!("\n");
}

fn question_7() {
    
    let tup: (i32, f32, char) = (16, 19.85, 'Q');
    let (x, y, z) = tup;
        
        println!("{}", x);
        println!("{}", y);
        println!("{}", z);
        
        println!("Second value if tuple is : {}\n", y);
}