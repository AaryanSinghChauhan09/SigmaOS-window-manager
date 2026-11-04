pub mod config;
pub mod error;
pub mod service;
pub mod workspace;
pub use config::Config;
pub use error::{ComponentError, Result};
pub use service::{Service, ServiceStatus};
pub use workspace::{Window, WindowLayout, WindowManager, Workspace};
pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> error::Result<()>;
    fn shutdown(&mut self) -> error::Result<()>;
    fn health_check(&self) -> error::Result<()>;
}
pub struct WindowManagerComponent { service: service::Service }
impl WindowManagerComponent {
    pub fn new(config: Config) -> Self { Self { service: service::Service::new(config) } }
}
impl Component for WindowManagerComponent {
    fn name(&self) -> &str { "sigmaos-window-manager" }
    fn version(&self) -> &str { env!("CARGO_PKG_VERSION") }
    fn initialize(&mut self) -> error::Result<()> { self.service.start() }
    fn shutdown(&mut self) -> error::Result<()> { self.service.stop() }
    fn health_check(&self) -> error::Result<()> {
        if self.service.is_running() { Ok(()) }
        else { Err(error::ComponentError::Service("window-manager not running".to_string())) }
    }
}
