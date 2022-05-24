#![allow(non_snake_case)]

//Image canvas 
const CANVAS_WIDTH : i16 = 256; 
const CANVAS_HEIGHT : i16 = 256; 

fn main() {
    println!("P3\n{} {}\n255\n", CANVAS_WIDTH, CANVAS_HEIGHT);
    for j in (0..CANVAS_HEIGHT).rev() { 
        for i in 0..CANVAS_WIDTH {
            let r: f32 = i as f32/ CANVAS_WIDTH as f32 ; 
            let g: f32  = j as f32/ CANVAS_HEIGHT as f32 ;    
            let b: f32 = 0.25; 

            let ir: i32 = (255.999 * r) as i32; 
            let ig: i32 = (255.999 * g) as i32;            
            let ib: i32 = (255.999 * b) as i32; 

            println!("{} {} {}", ir, ig, ib); 
        }
    }
}
