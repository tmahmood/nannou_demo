use nannou::prelude::*;
use nannou_utils::{get_random_color, get_random_green, Pt, srgba8_t};



fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}



fn update(app: &App, model: &mut Model, _update: Update) {

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.to_frame(app, &frame).unwrap();
}
