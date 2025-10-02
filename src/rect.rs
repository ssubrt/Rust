pub struct Rect{
    pub width : f32,
    pub height : f32
}

impl Rect{
    //Static Function
    pub fn test(){
        println!("This is a static function");
    }

    pub fn permimeter(&self) -> f32{
        return 2.0 * (self.width + self.height);
    }

    pub fn area(&self) -> f32{
        return self.width * self.height;
    }
}

