
enum Shapes{
    Rectangle(f32,f32),
    Circle(f32),
    Square(f32)
}


fn main(){

    println!("Here we go");
    let r : Shapes = Shapes::Rectangle(10.0,20.0);
    let c : Shapes = Shapes::Circle(10.0);
    let s : Shapes = Shapes::Square(10.0);

    println!("Area of Rectangle is : {}",area(r));
    println!("Area of Circle is : {}",area(c));
    println!("Area of Square is : {}",area(s));

}

fn area(s: Shapes) -> f32{
    match s{
        Shapes::Reactangle(w,h) => w*h,
        Shapes::Circle(r) => 3.14 * r * r,
        Shapes::Square(s) => s*s
    }

// NOTE: here if we dont put semicolon it will return the value
        // if we dont put semicolon the complier understand that we want to return the value
        
        /*
        it is above is same as
        let result = match self{
            Shapes::Reactangle(w,h) => 2.0 * (w + h),
            Shapes::Circle(r) => 2.0 * 3.14 * r,
            Shapes::Square(s) => 4.0 * s
        };
        return result;

        And


        return match self{
            Shapes::Reactangle(w,h) => 2.0 * (w + h),
            Shapes::Circle(r) => 2.0 * 3.14 * r,
            Shapes::Square(s) => 4.0 * s
        
         */

 };

fn permimeter(p:Shapes) -> f32{
    match p{
        Shapes::Reactangle(w,h) => 2.0 * (w + h),
        Shapes::Circle(r) => 2.0 * 3.14 * r,
        Shapes::Square(s) => 4.0 * s
    }
}


// struct Shapes{
//     Reactangle(f32,f32),
//     Circle(f32),
//     Square(f32)
// }

// impl Shapes{
//     fn area(&self) -> f32{
//         return match self{
//             Shapes::Reactangle(w,h) => w*h,
//             Shapes::Circle(r) => 3.14 * r * r,
//             Shapes::Square(s) => s*s
//         };
//     }

//     fn permimeter(&self) -> f32{
//          match self{
//             Shapes::Reactangle(w,h) => 2.0 * (w + h),
//             Shapes::Circle(r) => 2.0 * 3.14 * r,
//             Shapes::Square(s) => 4.0 * s
//         }
       
//     }
// }

