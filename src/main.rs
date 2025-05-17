fn main(){
    let longest;
    let smallstring=String::from("small");
    {
        let longerstring=String::from("longer");
        longest=longestString(&smallstring,&longerstring);
        println!("{}",longest);
    }
}

fn longestString<'a>(small: &'a String, long: &'a String) -> &'a String { //lifetime 'a where ->'a is intersection of both lifespan.
    if small.len() > long.len() {
        return small;
    } else {
        return long;
    }
}