use constellation::canvas::*;

fn main() {
    // println!("\x1b[46m          \x1b[0m\n\x1b[45m          \x1b[0m\n\x1b[47m          \x1b[0m\n\x1b[45m          \x1b[0m\n\x1b[46m          ");
    let mut canvas = Canvas::new((10,10));
    println!("test");
    canvas.draw();
    println!("test2");
}
