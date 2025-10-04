//Note : Strings do not implement the Copy trait because they manage heap-allocated memory.
// When a String is copied, the heap memory would be duplicated, leading to potential double-free
// errors and memory safety issues. Instead, Strings implement the Clone trait, which allows for
// explicit deep copying of the data when needed.   

// Copy Tarit is used for Booleans, integers, floats, char types and tuples


#[derive(Debug, Clone, Copy)] // Deriving Copy and Clone traits for Point struct


struct Point{
    x : u32,
    y: u32
}


fn main(){
    let p1 = Point{x: 10, y: 20};
    let p2 = p1; // p1 is copied to p2, not moved
    let p2 = p1.clone(); // clone method creates a copy of p1
    // clone is explicit where as Copy is implicit


    println!("p1: {:?}, p2: {:?}", p1, p2); // Both p1 and p2 can be used
}