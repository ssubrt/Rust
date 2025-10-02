
pub mod rect;
use rect::Rect;

fn main() {
    println!("Hello, world!");
    let r : React = React{
        width : 10.0,
        height : 20.0
    }
    
    println!("{} {}",r.width,r.height);
    println!("Area is : {}",r.area());
    println!("Permimeter is : {}",r.permimeter());
    React::test();
}
