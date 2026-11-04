#[derive(Debug, Clone, PartialEq)]
pub enum WindowLayout { Tiling, Floating, Fullscreen }
#[derive(Debug, Clone)]
pub struct Window { pub id: u32, pub title: String, pub focused: bool }
impl Window {
    pub fn new(id: u32, title: &str) -> Self { Self { id, title: title.to_string(), focused: false } }
}
#[derive(Debug, Clone)]
pub struct Workspace { pub id: u32, pub name: String, pub layout: WindowLayout, windows: Vec<Window> }
impl Workspace {
    pub fn new(id: u32, name: &str) -> Self {
        Self { id, name: name.to_string(), layout: WindowLayout::Tiling, windows: Vec::new() }
    }
    pub fn add_window(&mut self, window: Window) { self.windows.push(window); }
    pub fn remove_window(&mut self, id: u32) { self.windows.retain(|w| w.id != id); }
    pub fn window_count(&self) -> usize { self.windows.len() }
}
pub struct WindowManager { workspaces: Vec<Workspace>, active: u32 }
impl WindowManager {
    pub fn new() -> Self {
        let mut wm = Self { workspaces: Vec::new(), active: 1 };
        wm.workspaces.push(Workspace::new(1, "Main"));
        wm
    }
    pub fn add_workspace(&mut self, ws: Workspace) { self.workspaces.push(ws); }
    pub fn workspace_count(&self) -> usize { self.workspaces.len() }
    pub fn active_workspace(&self) -> Option<&Workspace> { self.workspaces.iter().find(|ws| ws.id == self.active) }
    pub fn switch_to(&mut self, id: u32) -> Result<(), String> {
        if self.workspaces.iter().any(|ws| ws.id == id) { self.active = id; Ok(()) }
        else { Err(format!("workspace {} not found", id)) }
    }
}
impl Default for WindowManager { fn default() -> Self { Self::new() } }
