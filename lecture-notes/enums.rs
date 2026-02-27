#![allow(unused_variables)]
#![allow(dead_code)]

#[derive(Debug)]
enum TrafficLight {
    Red(u32),    // seconds remaining
    Yellow(u32), // seconds remaining  
    Green(u32),  // seconds remaining
}

// Takes up as much memory as 3 u32 
// Smallest amount is 1 byte 
// Your code here

fn next_light(light: TrafficLight) -> TrafficLight {
    match light {
        TrafficLight::Red(_) => TrafficLight::Green(30),
        TrafficLight::Green(_) => TrafficLight::Yellow(5),
        TrafficLight::Yellow(_) => TrafficLight::Red(45),
    }
}

fn get_light_color(light: &TrafficLight) -> &str {
    match light {
        TrafficLight::Red(num) => "Red",
        TrafficLight::Green(num) => "Green",
        TrafficLight::Yellow(num) => "Yellow",
    }
}

fn get_time_remaining(light: &TrafficLight) -> u32 {
   match light {
       TrafficLight::Red(time) => *time,
       TrafficLight::Green(time) => *time,
       TrafficLight::Yellow(time) => *time,
   }
}


fn main() {
    let mut light: TrafficLight = TrafficLight::Red(45); 
    
    for _ in 0..3 {
        light = next_light(light); 
        println!(
            "Light: {}, Time remaining: {}",
            get_light_color(&light),
            get_time_remaining(&light)
    );
    };
    
    
}

