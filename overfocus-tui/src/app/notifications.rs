use anyhow::Result;

#[cfg(target_os = "windows")] mod win_notify;
#[cfg(target_os = "linux")] mod linux_notify;

#[cfg(target_os = "windows")]
pub fn notify(title: &str, msg: &str, long: bool) -> Result<()> {
    win_notify::notify(title, msg, long)
}

#[cfg(target_os = "linux")]
pub fn notify(title: &str, msg: &str, _long: bool) -> Result<()> {
    linux_notify::notify(title, msg)
}