use std::thread;

fn main(){
let handle=thread::spawn(||{
    for _i in 0..500000000{
        println!("hi from spawed");
    }
});

for _i in 0..500000000{
    println!("hi from main")
};
handle.join().unwrap();
}
