mod config;
mod constants;

use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use discord_rich_presence::{
    activity::{Activity, Assets, Button},
    new_client, DiscordIpc,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Starting on client ID {}",
        constants::CONFIG.configuration.client_id
    );
    let mut client = new_client(&constants::CONFIG.configuration.client_id)?;

    client.connect()?;

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        println!("Exiting the RPC....");
        r.store(false, Ordering::Relaxed);
    })?;

    'outer: loop {
        for (i, (k, v)) in constants::CONFIG.statuses.iter().enumerate() {
            if !running.load(Ordering::Relaxed) {
                break 'outer;
            }

            println!(
                "({:?}/{:?}) Currently on {:?}",
                i + 1,
                constants::CONFIG.statuses.len(),
                k
            );

            let mut activity = Activity::new();

            activity = activity.state(&v.state);
            activity = activity.details(&v.details);

            let mut buttons = vec![];
            if let Some(btns) = &v.buttons {
                for button in btns {
                    buttons.push(Button::new(&button.label, &button.url));
                }
            }

            let mut assets = Assets::new();

            let mut large_image = String::new();
            if let Some(img) = &v.large_image {
                large_image = img.to_string();
            }
            assets = assets.large_image(&large_image);

            let mut small_image = String::new();
            if let Some(img) = &v.small_image {
                small_image = img.to_string();
            }
            assets = assets.small_image(&small_image);

            if !large_image.is_empty() {
                activity = activity.assets(assets);
            }

            if !buttons.is_empty() {
                activity = activity.buttons(buttons);
            }

            client.set_activity(activity)?;

            let sleep = Duration::from_secs(constants::CONFIG.configuration.time_between);
            thread::sleep(sleep);
        }
    }

    client.close()?;

    Ok(())
}
