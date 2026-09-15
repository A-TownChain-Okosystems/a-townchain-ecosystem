use atc_genesis_platform::{EntityId, FrameId, Renderer, Transform};

#[derive(Default)]
pub struct NullRenderer {
    frame: Option<FrameId>,
    submitted: usize,
}

impl NullRenderer {
    pub fn submitted_count(&self) -> usize { self.submitted }
}

impl Renderer for NullRenderer {
    fn begin_frame(&mut self, frame: FrameId) {
        self.frame = Some(frame);
        self.submitted = 0;
    }

    fn submit(&mut self, _entity: EntityId, _transform: Transform) {
        if self.frame.is_some() { self.submitted += 1; }
    }

    fn end_frame(&mut self) { self.frame = None; }
}
