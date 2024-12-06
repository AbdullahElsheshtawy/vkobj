mod app;
mod renderer;

fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;

    let mut app = app::App::new(renderer::Renderer::new(event_loop.owned_display_handle())?);

    event_loop.run_app(&mut app)?;

    Ok(())
}
