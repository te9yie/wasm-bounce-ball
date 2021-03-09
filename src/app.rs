use std::f64::consts::PI;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

fn get_context(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    let context = canvas.get_context("2d").unwrap();
    let context = context.unwrap();
    let context = context.dyn_into::<CanvasRenderingContext2d>().unwrap();
    context
}

struct Ball {
    radius: f64,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Ball {
    fn new(radius: f64) -> Self {
        Self {
            radius,
            x: radius,
            y: radius,
            vx: 0.1,
            vy: 0.1,
        }
    }

    fn update(&mut self, delta: f64) {
        self.x += self.vx * delta;
        self.y += self.vy * delta;
    }

    fn bounce(&mut self, width: f64, height: f64) {
        if (self.vx < 0.0 && self.x < self.radius)
            || (self.vx > 0.0 && self.x > width - self.radius)
        {
            self.vx = -self.vx;
        }
        if (self.vy < 0.0 && self.y < self.radius)
            || (self.vy > 0.0 && self.y > height - self.radius)
        {
            self.vy = -self.vy;
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc(self.x, self.y, self.radius, 0.0, 2.0 * PI).unwrap();
        ctx.fill();
    }
}

pub struct App {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    delta: f64,
    ball: Ball,
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let context = get_context(&canvas);
        context.set_font("100% monospace");
        let ball = Ball::new(20.0);
        Self {
            canvas,
            context,
            delta: 0.0,
            ball,
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.delta = delta;
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;

        self.ball.update(delta);
        self.ball.bounce(width, height);
    }

    pub fn draw(&self) {
        let width = self.canvas.width() as f64;
        let height = self.canvas.height() as f64;
        let ctx = &self.context;

        ctx.clear_rect(0.0, 0.0, width, height);
        ctx.set_fill_style(&"#595857".into());

        self.ball.draw(ctx);

        ctx.stroke_rect(0.0, 0.0, width, height);
        ctx.fill_text(&format!("delta: {:.2}", self.delta), 10.0, 20.0)
            .unwrap();
    }
}
