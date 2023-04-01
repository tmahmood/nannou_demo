use nannou::color::IntoLinSrgba;
use nannou::draw::properties::ColorScalar;
use nannou::prelude::*;

pub mod particle;
pub mod ca;

pub type C8 = Srgba<u8>;
pub type CF32 = Srgba<f32>;
pub type CPt = Vec<(Vec2, C8)>;
pub type Pt = Vec<Vec2>;

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

pub const NIGHT_PALETTE: [(u8, u8, u8); 4] = [
    (27, 38, 44),
    (15, 76, 117),
    (50, 130, 184),
    (27, 36, 48),
];

pub fn get_from_palate(p: Vec<Srgb<u8>>, alpha: Option<u8>) -> Srgba<u8> {
    let i = random_range(0, p.len() - 1);
    let c: (u8, u8, u8) = p[i].into();
    let a = if alpha.is_some() { alpha.unwrap() } else { 240 };
    srgba8_t(c, a)
}

pub fn srgba8_t((r, g, b): (u8, u8, u8), a: u8) -> C8  {
    srgba8(r, g, b, a)
}

pub fn get_random_from_palette(palette: Vec<(u8, u8, u8)>, alpha: Option<u8>) -> C8 {
    let i = random_range(0, palette.len() - 1);

    srgba8_t(palette[i], if alpha.is_some() { alpha.unwrap() } else { 240 })
}

pub fn get_random_night(alpha: Option<u8>) -> Srgba<u8> {
    get_random_from_palette(NIGHT_PALETTE.to_vec(), alpha)
}

pub fn get_random_retro(alpha: Option<u8>) -> Srgba<u8> {
    get_random_from_palette(RETRO_PALETTE.to_vec(), alpha)
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


pub fn draw_soft_bg(draw: &Draw, app: &App, color: impl IntoLinSrgba<ColorScalar>, alpha: f32) {
    if app.elapsed_frames() <= 1 {
        draw.background().color(color);
    } else {
        let mut color = color.into_lin_srgba();
        color.alpha = alpha;

        let win = app.window_rect();
        draw.rect().wh(win.wh()).color(color);
    }
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

pub fn get_random_color() -> C8 {
    let r = random_range(0, 255);
    let g = random_range(0, 255);
    let b = random_range(0, 255);
    Srgba::new(r, g, b, 255)
}

pub fn poly_shapes_colored(r: f32, c: C8, angle: usize) -> CPt {
    (0..=360).step_by(angle).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * r;
        let y = radian.cos() * r;
        (pt2(x,y), c)
    }).collect()
}

pub fn poly_shapes(r: f32, c: C8, angle: usize) -> Pt {
    (0..=360).step_by(angle).map(|i| {
        let radian = deg_to_rad(i as f32);
        let x = radian.sin() * r;
        let y = radian.cos() * r;
        pt2(x,y)
    }).collect()
}

pub fn map_sin(v: f32, out_min: f32, out_max: f32) -> f32 {
    map_range(v.sin(), -1., 1., out_min, out_max)
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
