use anyhow::Result;

#[cfg(target_os = "windows")] mod win_notify;
#[cfg(target_os = "linux")] mod linux_notify;

pub fn notify(title: &str, msg: &str, long: bool) -> Result<()> {
    #[cfg(target_os = "windows")]
    win_notify::notify(title, msg, long)
}