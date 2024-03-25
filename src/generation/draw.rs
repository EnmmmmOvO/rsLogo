use unsvg::*;

pub struct Draw<'a> {
    x: f32,
    y: f32,
    draw: bool,
    color: Color,
    direction: i32,
    image: &'a mut Image,
}

impl<'a> Draw<'a> {
    pub fn new(x: f32, y: f32, image: &'a mut Image) -> Self {
        Self {
            x: x / 2.0,
            y: y / 2.0,
            draw: false,
            color: COLORS[7],
            direction: 0,
            image,
        }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn color(&self) -> usize {
        COLORS.iter().position(|&c| c == self.color).unwrap()
    }

    pub fn direction(&self) -> i32 {
        self.direction
    }

    pub fn pen_up(&mut self) {
        self.draw = false;
    }

    pub fn pen_down(&mut self) {
        self.draw = true;
    }

    pub fn pen_move(&mut self, direction: i32, mut distance: f32) {
        let mut dir = self.direction + direction;

        if distance < 0f32 {
            dir += 180;
            distance = distance.abs();
        }

        if !self.draw {
            let temp = get_end_coordinates(self.x, self.y, dir, distance);
            self.x = temp.0;
            self.y = temp.1;
        } else {
            match self
                .image
                .draw_simple_line(self.x, self.y, dir, distance, self.color)
            {
                Ok(result) => {
                    self.x = result.0;
                    self.y = result.1;
                }
                Err(e) => eprintln!("Error drawing line: {e}"),
            }
        }
    }

    pub fn set_pen_color(&mut self, index: usize) {
        self.color = COLORS[index];
    }

    pub fn turn(&mut self, angle: i32) {
        self.direction += angle;
    }

    pub fn set_heading(&mut self, angle: i32) {
        self.direction = angle;
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
}
