
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