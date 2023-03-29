use nannou::prelude::*;

pub mod particle;
pub mod ca;

pub const GREEN_PALATE: [Srgb<u8>; 7] = [
    GREEN, GREENYELLOW, DARKOLIVEGREEN, DARKSEAGREEN, FORESTGREEN, MEDIUMSEAGREEN, MEDIUMSPRINGGREEN
];

pub const BLUE_PALATE: [Srgb<u8>; 10] = [
    BLUE, ALICEBLUE, CADETBLUE, CORNFLOWERBLUE, DARKSLATEBLUE, DEEPSKYBLUE, STEELBLUE, POWDERBLUE, ROYALBLUE, MIDNIGHTBLUE
];


pub const RETRO_PALETTE: [(u8, u8, u8); 4] = [
    (227, 253, 253),
    (203, 241, 245),
    (166, 227, 233),
    (113, 201, 206),
];

pub fn get_from_palate(p: Vec<Srgb<u8>>, alpha: Option<u8>) -> Srgba<u8> {
    let i = random_range(0, p.len() - 1);
    let c: (u8, u8, u8) = p[i].into();
    let a = if alpha.is_some() { alpha.unwrap() } else { 240 };
    Srgba::new(c.0, c.1, c.2, a)
}

pub fn get_random_retro(alpha: Option<u8>) -> Srgba<u8> {
    let mut v = vec![];
    for (r, g, b) in RETRO_PALETTE.to_vec().iter() {
        v.push(srgb(*r, *g, *b))
    }
    get_from_palate(v, alpha)
}

pub fn get_random_green(alpha: Option<u8>) -> Srgba<u8> {
    get_from_palate(GREEN_PALATE.to_vec(), alpha)
}

pub fn get_random_blue(alpha: Option<u8>) -> Srgba<u8> {
    get_from_palate(BLUE_PALATE.to_vec(), alpha)
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn draw_background_grid(app: &App, draw: &Draw) {
    let window = app.main_window();
    let win = window.rect();
    draw.background().rgb(0.11, 0.12, 0.13);
    // 100-step and 10-step grids.
    draw_grid(&draw, &win, 100.0, 1.0);
    draw_grid(&draw, &win, 25.0, 0.5);
    // Crosshair.
    let crosshair_color = gray(0.5);
    let ends = [
        win.mid_top(),
        win.mid_right(),
        win.mid_bottom(),
        win.mid_left(),
    ];
    for &end in &ends {
        draw.arrow()
            .start_cap_round()
            .head_length(16.0)
            .head_width(8.0)
            .color(crosshair_color)
            .end(end);
    }
    // Crosshair text.
    let top = format!("{:.1}", win.top());
    let bottom = format!("{:.1}", win.bottom());
    let left = format!("{:.1}", win.left());
    let right = format!("{:.1}", win.right());
    let x_off = 30.0;
    let y_off = 20.0;
    draw.text("0.0")
        .x_y(15.0, 15.0)
        .color(crosshair_color)
        .font_size(14);
    draw.text(&top)
        .h(win.h())
        .font_size(14)
        .align_text_top()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&bottom)
        .h(win.h())
        .font_size(14)
        .align_text_bottom()
        .color(crosshair_color)
        .x(x_off);
    draw.text(&left)
        .w(win.w())
        .font_size(14)
        .left_justify()
        .color(crosshair_color)
        .y(y_off);
    draw.text(&right)
        .w(win.w())
        .font_size(14)
        .right_justify()
        .color(crosshair_color)
        .y(y_off);
    // Window and monitor details.
    if let Some(monitor) = window.current_monitor() {
        let w_scale_factor = window.scale_factor();
        let m_scale_factor = monitor.scale_factor();
        let mon_phys = monitor.size();
        let mon = mon_phys.to_logical(w_scale_factor as f64);
        let mon_w: f32 = mon.width;
        let mon_h: f32 = mon.height;
        let text = format!(
            "
        Window size: [{:.0}, {:.0}]
        Window ratio: {:.2}
        Window scale factor: {:.2}
        Monitor size: [{:.0}, {:.0}]
        Monitor ratio: {:.2}
        Monitor scale factor: {:.2}
        ",
            win.w(),
            win.h(),
            win.w() / win.h(),
            w_scale_factor,
            mon_w,
            mon_h,
            mon_w / mon_h,
            m_scale_factor
        );
        let pad = 6.0;
        draw.text(&text)
            .h(win.pad(pad).h())
            .w(win.pad(pad).w())
            .line_spacing(pad)
            .font_size(14)
            .align_text_bottom()
            .color(crosshair_color)
            .left_justify();
        // Ellipse at mouse.
        draw.ellipse().wh([5.0; 2].into()).xy(app.mouse.position());
        // Mouse position text.
        let mouse = app.mouse.position();
        let pos = format!("[{:.1}, {:.1}]", mouse.x, mouse.y);
        draw.text(&pos)
            .xy(mouse + vec2(0.0, 20.0))
            .font_size(14)
            .color(WHITE);
    }
}

fn draw_grid(draw: &Draw, win: &Rect, step: f32, weight: f32) {
    let step_by = || (0..).map(|i| i as f32 * step);
    let r_iter = step_by().take_while(|&f| f < win.right());
    let l_iter = step_by().map(|f| -f).take_while(|&f| f > win.left());
    let x_iter = r_iter.chain(l_iter);
    for x in x_iter {
        draw.line()
            .weight(weight)
            .points(pt2(x, win.bottom()), pt2(x, win.top()));
    }
    let t_iter = step_by().take_while(|&f| f < win.top());
    let b_iter = step_by().map(|f| -f).take_while(|&f| f > win.bottom());
    let y_iter = t_iter.chain(b_iter);
    for y in y_iter {
        draw.line()
            .weight(weight)
            .points(pt2(win.left(), y), pt2(win.right(), y));
    }
}

pub fn get_random_position(size: Vec2) -> Vec2 {
    Vec2::new(
        random_range(-size.x as f32 / 2., size.x as f32 / 2.),
        random_range(-size.y as f32 / 2., size.y as f32 / 2.),
    )
}

pub fn get_random_color() -> Srgba<u8> {
    let r = random_range(0, 255);
    let g = random_range(0, 255);
    let b = random_range(0, 255);
    Srgba::new(r, g, b, 255)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
