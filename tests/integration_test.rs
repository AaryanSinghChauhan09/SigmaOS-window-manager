use sigmaos_window_manager::{Config, Window, WindowLayout, WindowManager, WindowManagerComponent, Workspace, Component};

#[test]
fn test_wm_initial_workspace_aaa() {
    let wm = WindowManager::new();
    assert_eq!(wm.workspace_count(), 1);
    assert!(wm.active_workspace().is_some());
    assert_eq!(wm.active_workspace().unwrap().name, "Main");
}

#[test]
fn test_workspace_add_window_aaa() {
    let mut ws = Workspace::new(1, "Main");
    ws.add_window(Window::new(100, "Firefox"));
    ws.add_window(Window::new(101, "Terminal"));
    assert_eq!(ws.window_count(), 2);
}

#[test]
fn test_workspace_remove_window_aaa() {
    let mut ws = Workspace::new(1, "Main");
    ws.add_window(Window::new(100, "Firefox"));
    ws.remove_window(100);
    assert_eq!(ws.window_count(), 0);
}

#[test]
fn test_wm_switch_workspace_aaa() {
    let mut wm = WindowManager::new();
    wm.add_workspace(Workspace::new(2, "Dev"));
    let result = wm.switch_to(2);
    assert!(result.is_ok());
}

#[test]
fn test_wm_switch_nonexistent_aaa() {
    let mut wm = WindowManager::new();
    let result = wm.switch_to(99);
    assert!(result.is_err());
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = WindowManagerComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = WindowManagerComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-window-manager");
}
