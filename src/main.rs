use std::collections::HashMap;

fn main(){
    let mut user:HashMap<String,u32>=HashMap::new();
    user.insert(String::from("age"), 22);
    user.insert(String::from("newage"), 23);

    let firstuserage=user.get("age");
    match firstuserage {
        Some(age)=>println!(" age is this {}",age),
        None=>println!("user age not found")
    }
}