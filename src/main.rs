use::std::{thread, time};

fn main() {
    let mut canvas = Canvas::new(80,40,".");
    let mut the_x = 0;
    let mut update = |test| {
        canvas.fillRect(0, 0, 80, 40, " ");
        canvas.point(the_x, 20, "o");
        the_x = the_x + 1;
        canvas.display();
        test
    };
    while 1==1 {
        thread::sleep(time::Duration::from_millis(10));
        update(1);
    }
}

struct Canvas<'a> {
    width: usize,
    height: usize,
    pixels: Box<Vec<Vec<&'a str>>>,
}

impl<'a> Canvas<'a> {
    fn new(width: usize, height: usize, fill: &str) -> Canvas {
        let pixels = Box::new(vec![vec![fill; width]; height]);
        Canvas { 
            width,
            height,
            pixels,
        }
    }

    fn fillRect(&mut self, x: usize, y: usize, w: usize, h: usize, fill: &'a str) {
        for x_coor in x..x+w {
            for y_coor in y..y+h {
                self.point(x_coor, y_coor, fill);
            }
        }
    }

    fn display(&self) {
        print!("\x1B[2J\x1B[1;1H");
        for line in self.pixels.iter() {
            println!("{}", line.join(""));
        }
    }

    fn point(&mut self, x: usize, y: usize, draw: &'a str) {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y){
            self.pixels[y][x] = draw;
        }
    }

}
