#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiPointerButton { Primary, Secondary, Middle }

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UiEvent {
    PointerMove { x: f32, y: f32 },
    PointerDown { x: f32, y: f32, button: UiPointerButton },
    PointerUp { x: f32, y: f32, button: UiPointerButton },
    KeyDown { key: u32 },
    KeyUp { key: u32 },
}

pub trait UiWidget {
    fn bounds(&self) -> Rect;
    fn handle_event(&mut self, event: UiEvent) -> bool;
}
