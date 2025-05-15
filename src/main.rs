fn main() {
    let sentence = "my life";
    let mut ans=String::from ("");
    println!("{:?}",sentence.chars());
    for c in sentence.chars(){
        ans.push_str(c.to_string().as_str());//here as_str is used when function want &str type in argument
        // ans.push(c);
        if c==' '{
            break
        }
    }
    println!("{}",ans)
}
