use anyhow::Result;
use notify_rust::Notification;

pub fn notify(title: &str, msg: &str) -> Result<()> {
    match Notification::new().summary(title).body(msg).show() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)?,
    }
}