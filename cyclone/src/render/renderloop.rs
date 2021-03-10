use glium::{glutin, Surface};

pub struct WindowInfo<'a> {
    pub title: &'a str,
    pub screen_size: (i32, i32)
}

pub struct RenderInfo (pub glutin::event_loop::EventLoop<()>, pub glium::Display);

pub fn init(window_info: WindowInfo) -> RenderInfo {
    let event_loop = glutin::event_loop::EventLoop::new();
    let screen_size = window_info.screen_size;
    let wb = glutin::window::WindowBuilder::new()
            .with_title(window_info.title)
            .with_inner_size(glutin::dpi::LogicalSize::new(screen_size.0, screen_size.1));
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    RenderInfo (event_loop, display)
}

pub fn draw(display: &glium::Display) {
    
}