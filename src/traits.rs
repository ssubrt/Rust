use std::f64::consts::PI;

trait Shape{
    fn area(&self) -> f64;
}


struct Circle{
    radius: f64,
}

impl Shape for Circle{
    fn area(&self) -> f64{
        return PI * self.radius * self.radius;
    };
}

impl <T : std::fmt::Mul<Output = T> + Copy> Circle<T>{
    fn area(&self) -> f64{
        return (self.radius) * (self.radius) * PI;
    };
}

struct Rectangle{
    width: f64,
    height: f64,
}

impl Shape for Rectangle{
    fn area(&self) -> f64{
        return self.width * self.height;
    };
}

impl <T: std::fmt::Mul<Output = T> + Copy> Rectangle<T,T>{
    fn area(&self) -> f64{
        return (self.height) * (self.width);
    };
}

fn shape_of_area<T: Shape>(s: T) -> f64{
    return s.area();
}

fn main(){
    let c = Circle{
        radius: 5.0;
    }

    let r = Rectangle{
        width: 4.0;
        height: 5.0;
    }

    shape_of_area(c);
    shape_of_area(r);
}