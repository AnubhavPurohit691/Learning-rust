use std::fs;

fn main(){
let res=fs::read_to_string("example.txt");
match res {
    Ok(content)=>Ok(content),
    Err(_)=>Err("error reading file".to_string())
};
println!("printing")
}
// go to pdf projects.100xdevs for more example