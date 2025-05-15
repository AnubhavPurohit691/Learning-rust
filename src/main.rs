fn main(){
    let greeting = "anubhav";
    let char1= greeting.chars().nth(0);
    match char1 {
        Some(c)=>println!("{}",c),
        None=>println!("no character found at")
    }
    
}