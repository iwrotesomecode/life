use gameoflife::Universe;
use std::{thread, time};

fn main() {
    let mut universe = Universe::new(40, 40);

    loop {
        println!("{}", universe);
        universe.evolve();
        let duration = time::Duration::from_millis(500);
        thread::sleep(duration);
        print!("{}[2J", 27 as char);
        //print!("\x1B[2J\x1B[1;1H");
        //print!("\033[H\033[2J");
    }
}

/*
let neighbor_counts: Vec<u32> = universe.live_neighbor_counts();
for line in neighbor_counts.as_slice().chunks(universe.width as usize) {
    for &count in line {
        print!("{} ", count);
    }
    println!();
}
*/
