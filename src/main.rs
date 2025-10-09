use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::*,
    event_loop::*,
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes},
};

#[derive(Debug, Clone, Copy)]
enum CustomEvent{
    Timer,
}

#[derive(Default)]
struct App{
    window: Option<Window>,
}

impl ApplicationHandler<CustomEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Build attributes instead of using WindowBuilder
        let attrs = WindowAttributes::default()
            .with_title("wgpu window")
            .with_inner_size(LogicalSize::new(1280.0, 720.0));
        self.window = Some(event_loop.create_window(attrs).expect("create window"));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: CustomEvent) {
        // handle your CustomEvent::Timer here if you post it via EventLoopProxy
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // If you need custom user events:
    let event_loop = EventLoop::<CustomEvent>::with_user_event().build()?;
    // (If you don’t use user events, just: let event_loop = EventLoop::new()?;)

    // Choose your loop style (Poll for games, Wait for apps)
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app);
    Ok(())
}
