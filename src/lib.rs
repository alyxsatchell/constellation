pub mod canvas;
pub mod  stencil;

#[cfg(test)]
mod tests {
    use crate::{stencil::{Stencil, Transformation}, canvas::{Tile, Color, Canvas, Point}};


    struct TestStencil{

    }

    impl Stencil for TestStencil{
        fn get_map(&self) -> crate::stencil::StencilMap {
            let tile = Tile::new(Color::new(0, 200, 200, true), 1);
            let origin = Point{x:0, y:0};
            return vec![Transformation::new(Point{x:0,y:0}, origin, None, tile), Transformation::new(Point{x:0,y:1}, origin, None, tile), Transformation::new(Point{x:1,y:0}, origin, None, tile), Transformation::new(Point{x:1,y:1}, origin, None, tile)]
            // return vec![(Transformation::new(None, (0,0)), tile), (Transformation::new(None, (0,1)), tile), (Transformation::new(None, (1,0)), tile), (Transformation::new(None, (1,1)), tile)];
        }
    }
    #[test]
    fn test_stencil_draw(){
        let mut canvas = Canvas::new((10,10), (10,10), Color::new(0,0,0,true));
        let test_stencil = TestStencil{};
        canvas.update(test_stencil.get_map());
        canvas.draw();
    }
}