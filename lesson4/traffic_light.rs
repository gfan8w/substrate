pub enum TrafficLight{
    Red,
    Yellow,
    Green
}


pub trait Run{
    fn turn(&self) -> u32;
}

impl Run for TrafficLight{
    fn turn(&self) -> u32 {
        match &self {
            TrafficLight::Red=> {4} ,
            TrafficLight::Yellow=>  {5},
            TrafficLight::Green=>{9}
        }
    }
}


//入口
pub fn run(){
    let traf=TrafficLight::Yellow;
    let traf_result=traf.turn();
    println!("traffic light：{}", traf_result);
}
