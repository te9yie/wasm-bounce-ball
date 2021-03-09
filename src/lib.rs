use app::App;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

mod app;

fn create_canvas(width: u32, height: u32) -> HtmlCanvasElement {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
    canvas.set_width(width);
    canvas.set_height(height);
    body.append_child(&canvas).unwrap();
    canvas
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    let window = window().unwrap();
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen(start)]
pub fn main() {
    const WIDTH: u32 = 400;
    const HEIGHT: u32 = 200;
    let canvas = create_canvas(WIDTH, HEIGHT);

    let mut app = App::new(canvas);

    {
        const DELTA_LIMIT: f64 = 1000.0 / 30.0;
        let window = window().unwrap();
        let performance = window.performance().unwrap();
        let mut last_update_time = performance.now();

        let f = Rc::new(RefCell::new(None));
        let ff = Rc::clone(&f);
        *ff.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            let now = performance.now();
            let delta = now - last_update_time;
            let delta = if delta < DELTA_LIMIT {
                delta
            } else {
                DELTA_LIMIT
            };
            last_update_time = now;

            app.update(delta);
            app.draw();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut()>));
        request_animation_frame(ff.borrow().as_ref().unwrap());
    }
}
