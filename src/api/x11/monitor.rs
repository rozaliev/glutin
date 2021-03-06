use std::collections::VecDeque;
use std::sync::Arc;

use super::XConnection;
use native_monitor::NativeMonitorId;

pub struct MonitorID(pub Arc<XConnection>, pub u32);

pub fn get_available_monitors(x: &Arc<XConnection>) -> VecDeque<MonitorID> {
    let nb_monitors = unsafe { (x.xlib.XScreenCount)(x.display) };

    let mut monitors = VecDeque::new();
    monitors.extend((0 .. nb_monitors).map(|i| MonitorID(x.clone(), i as u32)));
    monitors
}

pub fn get_primary_monitor(x: &Arc<XConnection>) -> MonitorID {
    let primary_monitor = unsafe { (x.xlib.XDefaultScreen)(x.display) };
    MonitorID(x.clone(), primary_monitor as u32)
}

impl MonitorID {
    pub fn get_name(&self) -> Option<String> {
        let MonitorID(_, screen_num) = *self;
        Some(format!("Monitor #{}", screen_num))
    }

    pub fn get_native_identifier(&self) -> NativeMonitorId {
        NativeMonitorId::Numeric(self.1)
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        let screen = unsafe { (self.0.xlib.XScreenOfDisplay)(self.0.display, self.1 as i32) };
        let width = unsafe { (self.0.xlib.XWidthOfScreen)(screen) };
        let height = unsafe { (self.0.xlib.XHeightOfScreen)(screen) };
        (width as u32, height as u32)
    }
}
