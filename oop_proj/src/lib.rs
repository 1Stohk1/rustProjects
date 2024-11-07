use rgb::RGB;
use draw::*;

pub trait DrawOnScreen {
    fn draw_component(&self, canvas: &mut Canvas);
}

pub struct Screen {
    pub components: Vec<Box<dyn DrawOnScreen>>,
}

impl Screen {
    pub fn run(&self, width: u32, height: u32) {
        let mut canvas: Canvas = Canvas::new(width, height);
        let path = "image.svg";
        for component in self.components.iter() {
            component.draw_component(&mut canvas);
        }
        render::save(
            &canvas, 
            path,
            SvgRenderer::new(),
        )
        .expect("Failed to save the Scalable Vector Graphic.");
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub color: RGB<u8>,
    pub border_width: u32,
    pub x_pos: f32,
    pub y_pos: f32,
    pub label: String,
}

impl DrawOnScreen for Button {
    fn draw_component(&self, canvas: &mut Canvas) {
        let rect: Drawing = Drawing::new()
            .with_shape(Shape::Rectangle { 
                width: (self.width),
                height: (self.height) 
            })
            .with_xy(self.x_pos, self.y_pos)
            .with_style(Style::stroked(5, self.color));
        canvas.display_list.add(rect);
    }
}