use atc_genesis_platform::{EntityId, FrameId, Renderer, Transform};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphicsBackend { Null, Vulkan, Metal, DirectX12, OpenGL, WebGpu }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RenderCapabilities { pub backend: GraphicsBackend, pub max_frames_in_flight: u32 }

pub trait RenderBackend {
    fn capabilities(&self) -> RenderCapabilities;
    fn begin_frame(&mut self, frame: FrameId);
    fn submit(&mut self, entity: EntityId, transform: Transform);
    fn end_frame(&mut self);
}

#[derive(Default)]
pub struct NullBackend { frame: Option<FrameId>, submitted: u64 }
impl NullBackend { pub fn submitted(&self) -> u64 { self.submitted } }
impl RenderBackend for NullBackend {
    fn capabilities(&self) -> RenderCapabilities { RenderCapabilities { backend: GraphicsBackend::Null, max_frames_in_flight: 2 } }
    fn begin_frame(&mut self, frame: FrameId) { self.frame = Some(frame); self.submitted = 0; }
    fn submit(&mut self, _entity: EntityId, _transform: Transform) { if self.frame.is_some() { self.submitted += 1; } }
    fn end_frame(&mut self) { self.frame = None; }
}

pub struct BackendRenderer<B: RenderBackend> { pub backend: B }
impl<B: RenderBackend> Renderer for BackendRenderer<B> {
    fn begin_frame(&mut self, frame: FrameId) { self.backend.begin_frame(frame); }
    fn submit(&mut self, entity: EntityId, transform: Transform) { self.backend.submit(entity, transform); }
    fn end_frame(&mut self) { self.backend.end_frame(); }
}
