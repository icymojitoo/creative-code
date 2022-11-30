use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set backgound to blue
    draw.background().color(PINK);

    // put everythin on the frame
    draw.to_frame(app, &frame).unwrap();
}
