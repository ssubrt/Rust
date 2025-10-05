

fn main() {
    let str = String::from("Hello, world!");
    let ans: &String;

    {
        let str2 = String::from("");
        ans = longest_str(&str,&str2);
        print!("{}", ans);
    }

    print!("{}", ans);
}

fn longest_str<'a>(s1: &'a String,s2: &'a String) -> &'a String{
    if s1.len() > s2.len() { 
        return s1;
    }
    return s2;
}

