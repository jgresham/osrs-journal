use std::collections::HashMap;

use dink_osrs::{Notification, NotificationType};

// sample usage of the dink_osrs library
fn main() {
    println!("Sample usage of the dink-osrs library");
    let notification = Notification {
        content: "PlayerName has died...".to_string(),
        extra: HashMap::new(),
        notification_type: NotificationType::Death,
        player_name: "PlayerName".to_string(),
        account_type: "NORMAL".to_string(),
        seasonal_world: "false".to_string(),
        dink_account_hash: "test_hash".to_string(),
    };
    println!("{}", notification.message());
}
