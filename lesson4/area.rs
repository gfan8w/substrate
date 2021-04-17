use std::f64::consts::PI;

pub trait CalculateArea{
    fn calc(&self) ->f64;
}


struct Triangle{
    bottom: u32,
    height: u32
}

struct Square {
    length: u32
}

struct Cycle{
    radius: u32
}


impl CalculateArea for Triangle {
    fn calc(&self) -> f64 {
        return (self.bottom * self.height) as f64;
    }
}

impl CalculateArea for Square{
    fn calc(&self) -> f64 {
        return (self.length * self.length) as f64;
    }
}

impl CalculateArea for Cycle {
    fn calc(&self) -> f64 {
        return PI* ((self.radius*self.radius) as f64);
    }
}


pub fn main() {

    let tri= Triangle{height:34,bottom:21};
    let tri_area=tri.calc();
    println!("triangle area is {}", tri_area);

    let sq=Square{ length:20};
    let sq_are=sq.calc();
    println!("square area is {}", sq_are);

    let cy=Cycle{ radius:3};
    let cy_area=cy.calc();
    println!("cycle area is {}", cy_area);

}





