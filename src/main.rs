use std::{collections::HashMap, hash::Hash};

use constellation::{canvas::*, stencil::StencilMap};

fn main() {
    let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
    let tile = Tile::new(Color::new(0, 200, 200, true));
    let addition = Position{x:1, y: 0};
    let current_map = HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]);
    let add = HashMap::from([(Position{x:0,y:0}, tile), (Position{x:1, y:0}, tile), (Position{x:0, y:1}, tile), (Position{x:1,y:1}, tile)]);
    let sub: HashMap<Position, Tile> = HashMap::new();
    let mut test_stencil_map: StencilMap = StencilMap{
        origin: Position { x: 0, y: 0 },
        addition_map: add,
        subtraction_map: sub,
        current_map,
    };
    canvas.update(&mut test_stencil_map);
    canvas.draw();
    for _ in 0..100000000{
        let f: f32 = 1.1234251324124;
        let _ = f.sqrt();
    }
    for _ in 1..5{
        test_stencil_map.translate(addition);
        canvas.update(&mut test_stencil_map);
        canvas.draw();
        for _ in 0..100000000{
            let f: f32 = 1.1234251324124;
            let _ = f.sqrt();
        }
    }
}
