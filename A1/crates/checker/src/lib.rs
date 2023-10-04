pub extern crate tokio;

use std::{sync::mpsc, time::Duration};

use generator::Grid;
use thirtyfour::prelude::*;

pub async fn start_checker(
    receiver: mpsc::Receiver<Option<Grid>>,
    sender: mpsc::Sender<Grid>,
) -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    driver.goto("https://www.arukone.bwinf.de/arukone").await?;
    driver
        .set_implicit_wait_timeout(Duration::from_millis(500))
        .await?;
    let button = driver.find(By::Tag("button")).await?;
    let textarea = driver.find(By::Tag("textarea")).await?;
    loop {
        let Ok(grid) = receiver.recv() else {
            continue;
        };
        let Some(grid) = grid else {
            break;
        };
        let s = grid.to_string();
        textarea.clear().await?;
        textarea.send_keys(&s).await?;
        button.click().await?;
        let output = driver.find(By::Tag("strong")).await?;
        let result = output.text().await?.contains("Lösung gefunden:");
        if !result {
            sender.send(grid).expect("Send this shit");
            break;
        }
    }
    driver.quit().await?;
    Ok(())
}
