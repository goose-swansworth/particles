use std::collections::HashSet;
use image::{GenericImageView, DynamicImage};



pub struct Canvas {
    row_len: usize,
    col_len: usize,
    tile_size: usize
}

fn radius_error(x: i32, y: i32, r: i32) -> i32 {
    (x * x + y * y - r * r).abs()
}

fn trans(x: i32, t: usize) -> usize {
    (x + t as i32) as usize
}

impl Canvas {
    pub fn init(row_len: usize, col_len: usize, tile_size: usize) -> Canvas {
        Canvas {row_len, col_len, tile_size}
    }
    
    pub fn clear(&self, frame: &mut [u8], clear_color: [u8; 4]) {
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&clear_color);
        }
    }
    
    pub fn draw_sprite(&self, frame: &mut [u8], sprite: &DynamicImage, x: usize, y: usize) {
        let (n, m) = sprite.dimensions();
        let rows = n as usize;
        for (i, pixel) in sprite.pixels().enumerate() {
            if pixel.2[3] != 0 {
                let color = [pixel.2[0], pixel.2[1], pixel.2[2], pixel.2[3]];
                self.draw_tile(frame, x + (i % rows), y + (i / rows), color);
                
            }
        }
    }
    
    pub fn fill(&self, frame: &mut [u8], sx: i32, sy: i32, occupied: &mut HashSet<(i32, i32)>, color: [u8; 4]) -> HashSet<(i32, i32)> {
        let mut stack = Vec::new();
        let mut set = HashSet::new();
        stack.push((sx, sy));
        loop {
            let (x, y) = match stack.pop() {
                None => break,
                Some((x, y)) => (x, y)
            };
            if !occupied.contains(&(x, y)) && !set.contains(&(x, y))  {
                self.draw_tile(frame, x as usize, y as usize, color);
                set.insert((x, y));
                if x > 0 {
                    stack.push((x - 1, y));
                }
                if x < self.row_len as i32 / self.tile_size as i32 {
                    stack.push((x + 1, y));
                }
                if y > 0 {
                    stack.push((x, y - 1));
                }
                if y < self.col_len as i32 / self.tile_size as i32 {
                    stack.push((x, y + 1));
                }
            }
        }
        set
    }
    
    pub fn mouse_to_canvas(&self, xcoord: bool, value: f32) -> usize {
        if xcoord {
            let gridx = (value / 2.0 / self.tile_size as f32) as usize;
            if gridx <= self.row_len / self.tile_size {
                return gridx;
            }
            return self.row_len / self.tile_size;
        }
        let gridy = ((self.col_len as f32 * 2.0 - value) / 2.0 / self.tile_size as f32) as usize;
        if gridy <= self.col_len / self.tile_size {
            return gridy;
        }
        self.col_len / self.tile_size
    }
    
    

    //Draw a square at the given canvas coordinates
    pub fn draw_tile(&self, frame: &mut [u8], pos_x: usize, pos_y: usize, color: [u8; 4]) {
        let new_pos_y = self.col_len - pos_y * self.tile_size;
        let new_pos_x = self.tile_size * pos_x;
        for i in 1..self.tile_size + 1 {
            let start = ((new_pos_y - i) * self.row_len + new_pos_x) * 4;
            let end = start + 4 * self.tile_size;
            for pixel in frame[start..end].chunks_exact_mut(4) {
                pixel.copy_from_slice(&color);
            }
        }
    }
    
    //Bresenham's line algorithm
    pub fn draw_line(&self, frame: &mut [u8], (x0, y0): (i32, i32), (x1, y1): (i32, i32), color: [u8; 4]) {
        if (y1 - y0).abs() < (x1 - x0).abs() {
            if x0 > x1 {
                self.draw_line_low(frame, (x1, y1), (x0, y0), color);
            } else {
                self.draw_line_low(frame, (x0, y0), (x1, y1), color);
            }
        } else {
            if y0 > y1 {
                self.draw_line_high(frame, (x1, y1), (x0, y0), color);
            } else {
                self.draw_line_high(frame, (x0, y0), (x1, y1), color);
            }
        }
    }
    
    fn draw_line_low(&self, frame: &mut [u8], (x0, y0): (i32, i32), (x1, y1): (i32, i32), color: [u8; 4]) {
        let dx = x1 - x0;
        let mut dy = y1 - y0;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut delta = (2 * dy) - dx;
        let mut y = y0;
        for x in x0..x1+1 {
            self.draw_tile(frame, x as usize, y as usize, color);
            if delta > 0 {
                y += yi;
                delta += 2 * (dy - dx);
            } else {
                delta += 2*dy;
            }
        }
    }
    
    fn draw_line_high(&self, frame: &mut [u8], (x0, y0): (i32, i32), (x1, y1): (i32, i32), color: [u8; 4]) {
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut delta = 2 * dx - dy;
        let mut x = x0;
        for y in y0..y1+1 {
            self.draw_tile(frame, x as usize, y as usize, color);
            if delta > 0 {
                x += xi;
                delta += 2 * (dx - dy);
            } else {
                delta += 2*dx;
            }
        }
    }
    
    //Midpoint rule circle algorithm
    pub fn draw_circle(&self,
        frame: &mut [u8],
        cx: usize,
        cy: usize,
        r: u32,
        color: [u8; 4],
    ) {
        let mut xi: i32 = r as i32;
        let mut yi: i32 = 0;
        while xi >= yi {
            let mut x = xi;
            let mut y = yi;
            //first octant
            self.draw_tile(frame, trans(x, cx), trans(y, cy), color);
            //second octant, reflect first in x=y
            (x, y) = (y, x);
            self.draw_tile(frame, trans(x, cx), trans(y, cy), color);
            //third octant, reflect second in x
            (x, y) = (-x, y);
            self.draw_tile(frame, trans(x, cx), trans(y, cy), color);
            //fourth octant, reflect third in x=-y;
            (x, y) = (-y, -x);
            self.draw_tile(frame, trans(x, cx), trans(y, cy), color);
            //fith octant, reflect fourth in y
            (x, y) = (x, -y);
            self.draw_tile(frame, trans(x, cx), trans(y, cy), color);
            //sixth octant, reflect fith in x=y
            (x, y) = (y, x);
            self.draw_tile(frame, trans(x, cx), trans(y, cy), color);
            //seventh octant, reflect sixth in x
            (x, y) = (-x, y);
            self.draw_tile(frame, trans(x, cx), trans(y, cy), color);
            //eith octant, reflect seventh in x=-y
            (x, y) = (-y, -x);
            self.draw_tile(frame, trans(x, cx), trans(y, cy), color);
            //update
            if radius_error(xi - 1, yi + 1, r as i32) < radius_error(xi, yi + 1, r as i32) {
                xi -= 1;
                yi += 1;
            } else {
                yi += 1;
            }
        }
    }

}

fn trans_point(x: i32, y: i32, cx: i32, cy: i32) -> (i32, i32) {
    (x + cx, y + cy)
}

pub fn circle_coords(cx: i32, cy: i32, r: i32) -> Vec<(i32, i32)> {
    let mut xi: i32 = r;
    let mut yi: i32 = 0;
    let mut points = Vec::new();
    while xi >= yi {
        let mut x = xi;
        let mut y = yi;
        //first octant
        points.push(trans_point(x, y, cx, cy));
        //second octant, reflect first in x=y
        (x, y) = (y, x);
        points.push(trans_point(x, y, cx, cy));
        //third octant, reflect second in x
        (x, y) = (-x, y);
        points.push(trans_point(x, y, cx, cy));
        //fourth octant, reflect third in x=-y;
        (x, y) = (-y, -x);
        points.push(trans_point(x, y, cx, cy));
        //fith octant, reflect fourth in y
        (x, y) = (x, -y);
        points.push(trans_point(x, y, cx, cy));
        //sixth octant, reflect fith in x=y
        (x, y) = (y, x);
        points.push(trans_point(x, y, cx, cy));
        //seventh octant, reflect sixth in x
        (x, y) = (-x, y);
        points.push(trans_point(x, y, cx, cy));
        //eith octant, reflect seventh in x=-y
        (x, y) = (-y, -x);
        points.push(trans_point(x, y, cx, cy));
        //update
        if radius_error(xi - 1, yi + 1, r as i32) < radius_error(xi, yi + 1, r as i32) {
            xi -= 1;
            yi += 1;
        } else {
            yi += 1;
        }
    
    }
    points
}
pub fn line_points((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> Vec<(i32, i32)> {
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            return points_line_low((x1, y1), (x0, y0));
        } else {
            return points_line_low((x0, y0), (x1, y1));
        }
    } else {
        if y0 > y1 {
            return points_line_high((x1, y1), (x0, y0));
        } else {
            return points_line_high((x0, y0), (x1, y1));
        }
    }
}

fn points_line_low((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut delta = (2 * dy) - dx;
    let mut y = y0;
    for x in x0..x1+1 {
        points.push((x, y));
        if delta > 0 {
            y += yi;
            delta += 2 * (dy - dx);
        } else {
            delta += 2*dy;
        }
    }
    points
}

fn points_line_high((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut delta = 2 * dx - dy;
    let mut x = x0;
    for y in y0..y1+1 {
        points.push((x, y));
        if delta > 0 {
            x += xi;
            delta += 2 * (dx - dy);
        } else {
            delta += 2*dx;
        }
    }
    points
}
