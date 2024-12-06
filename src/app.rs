use crate::renderer;

pub struct App {
    window: Option<winit::window::Window>,
    renderer: renderer::Renderer,
}

impl App {
    pub fn new(renderer: renderer::Renderer) -> Self {
        Self {
            window: None,
            renderer,
        }
    }
}

impl winit::application::ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = winit::window::WindowAttributes::default()
                .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
                .with_title("Vulkan Obj Renderer");
            self.window = Some(event_loop.create_window(window_attributes).unwrap());
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let window = self.window.as_ref().unwrap();

        if window.id() == window_id {
            match event {
                winit::event::WindowEvent::CloseRequested => event_loop.exit(),
                winit::event::WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            physical_key: winit::keyboard::PhysicalKey::Code(key),
                            state: winit::event::ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    use winit::keyboard::KeyCode;
                    match key {
                        KeyCode::Escape => {
                            event_loop.exit();
                        }

                        KeyCode::F11 => {
                            if window.fullscreen().is_none() {
                                window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(
                                    None,
                                )));
                            } else {
                                window.set_fullscreen(None);
                            }
                        }
                        _ => {}
                    }
                }
                winit::event::WindowEvent::RedrawRequested => {
                    window.request_redraw();
                }

                _ => {}
            }
        }
    }
}
