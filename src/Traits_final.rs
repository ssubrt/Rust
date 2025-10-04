trait Shape{
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

struct Circle{
    radius: f64
};

impl Shape for Circle{
    fn area(&self) -> f64{
        return 3.14 * self.radius * self.radius;
    }
}

struct Reactangle{
    width: f64,
    height: f64
};

impl Shape for Reactangle{
    fn area(&self) -> f64{
        return self.width * self.height;
    }

    fn perimeter(&self) -> f64{
        return 2.0 * (self.width + self.height);
    }
}

// Generic function
fn get_area<T: Shape>(s: &T) -> f64{
    s.area();
}

// Using impl Trait
fn get_area2(s: &impl Shape) -> f64{
    s.area();
}

// Using impl Trait in return position
fn get_perimeter<T impl Shape>(s : T) -> f64{
    s.perimeter();
}


macro_rules! say_hello {
    () => {
        println!("Hello!");
    }
}
fn main() {
    println!("Hello, world!");
    let c = Circle{radius: 5.0};
    let r = Reactangle{
        width: 10.0,
        height: 20.0
    };

    say_hello!();

    println!("Circle area: {}", c.area());
    println!("Rectangle area: {}", r.area());
    println!("Rectangle perimeter: {}", r.perimeter());
}
