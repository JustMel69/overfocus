use anyhow::Result;
use winrt_notification::{Toast, Duration};

pub fn notify(title: &str, msg: &str, long: bool) -> Result<()> {
    match Toast::new(Toast::POWERSHELL_APP_ID).duration(if long { Duration::Long } else { Duration::Short }).title(title).text1(msg).show() {
        Ok(_) => Ok(()),
        Err(e) => Err(e)?,
    }
}