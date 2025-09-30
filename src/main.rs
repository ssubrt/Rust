fn main() {
    println!("Hello, world!");


    let ans = sum(1,2);
    println!("1 + 2 = {}",ans);


    let result = is_even(4);
    if result {
        println!("4 is even");
    }
    else{
        println!("4 is odd");
    }
    println!("4 is even? {}",result);

    let name: String = String::from("Alice");
    print!("Hello, {}!\n", name);

    let num : Vec<i32> = vec![1,2,3,4,5];
    // for n in num {
    //     println!("Number: {}",n);
    // }
    
    print!("Numbers list : {:?}\n",num);

    let mut name: String = String::from("Alice");
    name.push_str(" in Borderland");
    println!("Updated name: {}",name);
}

fn sum(a:u32,b:u32) -> u32 {
    return a+b;
}

fn is_even(a:u32) -> bool {
    return a % 2 == 0;
}