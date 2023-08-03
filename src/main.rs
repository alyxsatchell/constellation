use constellation::canvas::*;

fn main() {
    // println!("\x1b[46m          \x1b[0m\n\x1b[45m          \x1b[0m\n\x1b[47m          \x1b[0m\n\x1b[45m          \x1b[0m\n\x1b[46m          ");
    let mut test_stroke_list = StrokeList::new();
    test_stroke_list.insert(Stroke::new(
            Color::new(50, 168, 82, true),
            20
        )
    );
    test_stroke_list.insert(Stroke::new(
            Color::new(255, 168, 82, true),
            20
        )
    );
    test_stroke_list.draw();
}
