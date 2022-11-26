pub fn draw_tile(
    frame: &mut [u8],
    row_len: usize,
    col_len: usize,
    pos_x: usize,
    pos_y: usize,
    width: usize,
    color: [u8; 4],
) {
    let new_pos_y = col_len - pos_y * width;
    let new_pos_x = width * pos_x;
    for i in 1..width + 1 {
        let start = ((new_pos_y - i) * row_len + new_pos_x) * 4;
        let end = start + 4 * width;
        for pixel in frame[start..end].chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }
}

pub fn clear_frame(frame: &mut [u8], clear_color: [u8; 4]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&clear_color);
    }
}

fn radius_error(x: i32, y: i32, r: i32) -> i32 {
    (x * x + y * y - r * r).abs()
}

fn trans(x: i32, t: usize) -> usize {
    (x + t as i32) as usize
}

//Midpoint rule circle algorithm
pub fn draw_circle(
    frame: &mut [u8],
    rows: usize,
    cols: usize,
    tsize: usize,
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
        draw_tile(frame, rows, cols, trans(x, cx), trans(y, cy), tsize, color);
        //second octant, reflect first in x=y
        (x, y) = (y, x);
        draw_tile(frame, rows, cols, trans(x, cx), trans(y, cy), tsize, color);
        //third octant, reflect second in x
        (x, y) = (-x, y);
        draw_tile(frame, rows, cols, trans(x, cx), trans(y, cy), tsize, color);
        //fourth octant, reflect third in x=-y;
        (x, y) = (-y, -x);
        draw_tile(frame, rows, cols, trans(x, cx), trans(y, cy), tsize, color);
        //fith octant, reflect fourth in y
        (x, y) = (x, -y);
        draw_tile(frame, rows, cols, trans(x, cx), trans(y, cy), tsize, color);
        //sixth octant, reflect fith in x=y
        (x, y) = (y, x);
        draw_tile(frame, rows, cols, trans(x, cx), trans(y, cy), tsize, color);
        //seventh octant, reflect sixth in x
        (x, y) = (-x, y);
        draw_tile(frame, rows, cols, trans(x, cx), trans(y, cy), tsize, color);
        //eith octant, reflect seventh in x=-y
        (x, y) = (-y, -x);
        draw_tile(frame, rows, cols, trans(x, cx), trans(y, cy), tsize, color);
        //update
        if radius_error(xi - 1, yi + 1, r as i32) < radius_error(xi, yi + 1, r as i32) {
            xi -= 1;
            yi += 1;
        } else {
            yi += 1;
        }
    }
}
