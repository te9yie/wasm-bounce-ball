use app::App;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement};

mod app;

fn get_canvas(id: &str) -> HtmlCanvasElement {
    let window = window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id(id).unwrap();
    canvas.dyn_into::<HtmlCanvasElement>().unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    let window = window().unwrap();
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen]
pub fn main(id: &str) {
    let canvas = get_canvas(id);

    let mut app = App::new(canvas);

    {
        const DELTA_LIMIT: f64 = 1000.0 / 30.0;
        let mut last_update_time = 0.0;
        let f = Rc::new(RefCell::new(None));
        let ff = Rc::clone(&f);
        *ff.borrow_mut() = Some(Closure::wrap(Box::new(move |now| {
            let delta: f64 /* why need? */ = now - last_update_time;
            let delta = delta.clamp(0.0, DELTA_LIMIT);
            last_update_time = now;

            app.update(delta);
            app.draw();

            request_animation_frame(f.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f64)>));
        request_animation_frame(ff.borrow().as_ref().unwrap());
    }
}
