use tokio::sync::mpsc;

use crate::messages::state::StateMessage;

pub fn spawn_state() -> mpsc::UnboundedSender<StateMessage> {
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Spawn a new task to handle received messages
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            // Use .recv() here for UnboundedReceiver
            println!("Got message: {:?}", msg);
        }
    });

    tx // Return the sender
}
