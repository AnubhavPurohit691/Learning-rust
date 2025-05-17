use std::thread;


fn main(){
    let mut x =1;
    thread::spawn(move ||{
        x=2;
        println!("{}",x);
    }).join().unwrap();
    println!("{}",x);
}