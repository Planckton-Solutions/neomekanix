pub mod events;
pub mod state;
pub mod application;

use winit::event_loop::EventLoop;

pub fn init() -> anyhow::Result<()> {
    log::info!("Initalized Log");

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = application::App::new();

    event_loop.run_app(&mut app);

    Ok(())
}