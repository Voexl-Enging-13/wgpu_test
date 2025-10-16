use winit::{
    application::ApplicationHandler, // winit calls its methods when things happen (start/resume, input, redraw, etc.).
    dpi::LogicalSize,
    event::*,
    event_loop::*, // a handle you get while the loop is running; it lets you do privileged things like create windows or exit.
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes}, //the window type and the “settings” used to create one (title, size, etc.).
};
 // WindowEvent, KeyboardInput, KeyCode, PhysicalKey — the actual input events you’ll receive.

 // Used to post our own events, Test segment not used.
#[derive(Debug, Clone, Copy)]
enum CustomEvent{
    Timer,
}

// The game/app state lives here. We keep an Option<Window> because 
// the window is created later (inside resumed), not at program start.
#[derive(Default)]
struct App{
    window: Option<Window>,
}

impl ApplicationHandler<CustomEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) { // Called when the app starts or comes back from suspension. This is where we create the window.
        
        let attrs = WindowAttributes::default()
            .with_title("wgpu window") // Set title to window
            .with_inner_size(LogicalSize::new(1280.0, 720.0)); // Set size to window
        self.window = Some(event_loop.create_window(attrs).expect("create window"));
    }

    // All OS/window input lands here (close button, resize, keyboard, mouse, etc).
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(), // quits program
            WindowEvent::KeyboardInput { event, .. } => { // when the esc key is pressed it closes the window.
                if let PhysicalKey::Code(KeyCode::Escape) = event.physical_key {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }
    // This is where custem events like CustomEvent::Timer would be hangled
    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: CustomEvent) {
        // handle your CustomEvent::Timer here if you post it via EventLoopProxy
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();

    // If you need custom user events:
    let event_loop = EventLoop::<CustomEvent>::with_user_event().build()?;
    // (If you don’t use user events, just: let event_loop = EventLoop::new()?;)

    // Run continuously (game-style). Use wait if we want lower CPU when udle
    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app); // hands control to winit forever. From now on winit calls your ApplicationHandler methods until you call exit().
    Ok(())
}

/*
  Winit is a cross-platform window creation and management library. 
  Its primary functions include: 
Window Creation: Providing an API to create and manage windows on various operating systems.
Event Handling: Facilitating the handling of window-related events such as resizing, closing, keyboard input, mouse movement, and touch events. This event handling often occurs within an event loop.  

 WGPU is a safe, portable, and pure-Rust graphics library based on the WebGPU API. It enables general-purpose graphics and compute on the GPU, 
offering a unified API across different native graphics backends (Vulkan, Metal, DirectX 12, OpenGL ES) and web platforms (WebGPU, WebGL2 via WebAssembly). 
Key features of WGPU include: 
Graphics API Abstraction: Providing a high-level, safe abstraction over various low-level graphics APIs.
GPU Access: Enabling rendering and computation on the GPU.
Cross-Platform Compatibility: Running natively on multiple graphics APIs and in browsers.


These two libraries are commonly integrated to build graphical applications in Rust. 
Winit handles the creation and management of the application window and its associated events, while WGPU is used to render graphics onto that window. 
The typical workflow involves:
Creating a Winit Window: Initializing a Winit EventLoop and Window instance.
Initializing WGPU: Obtaining a wgpu::Surface from the Winit window, which serves as the target for rendering. 
Then, initializing a wgpu::Device and wgpu::Queue for interacting with the GPU.

Rendering Loop: Within the Winit event loop, handling events like window resizing (which requires reconfiguring the WGPU surface)
 and drawing frames using WGPU's rendering pipeline (e.g., creating render pipelines, binding resources, and issuing draw calls).
 
Presenting Frames: Swapping the rendered image to the screen using the WGPU surface's present method.

*/