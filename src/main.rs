use prompted::input;

fn main() {
    println!("Welcome to the magical Height-to-Redbull converter!");

    let mut _height_in_cm: i32;
    let redbull_355_ml_height = 15.74;

    println!("Please input your height in centimeters:");

    let mut _user_height_input = String::new();
    let _user_height_input = input!("Height in cm: ");

    let _height_in_cm: i32 = _user_height_input
        .trim()
        .parse()
        .expect("Please type a number!");

    let height_in_redbulls = _height_in_cm as f32 / redbull_355_ml_height;

    println!(
        "Your height in Redbulls is {} 12oz cans!",
        height_in_redbulls
    );
}
