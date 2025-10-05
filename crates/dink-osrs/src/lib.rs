use std::collections::HashMap;

// https://github.com/pajlads/DinkPlugin/blob/master/docs/json-examples.md
// Structs for the Dink Plugin Message types
/*
  {
    "content": "Text message as set by the user",
    "extra": {},
    "type": "NOTIFICATION_TYPE",
    "playerName": "your rsn",
    "accountType": "NORMAL | IRONMAN | HARDCORE_IRONMAN",
    "seasonalWorld": "true | false",
    "dinkAccountHash": "abcdefghijklmnopqrstuvwxyz1234abcdefghijklmnopqrstuvwxyz",
    "embeds": []
  }
*/

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationType {
    Death,
    Collection,
    Level,
    XpMilestone,
    Loot,
    Slayer,
    Quest,
    Clue,
    KillCount,
    CombatAchievement,
    AchievementDiary,
    Pet,
    Speedrun,
    BarbarianAssaultGamble,
    PlayerKill,
    GroupStorage,
    GrandExchange,
    Trade,
    LeaguesArea,
    LeaguesMastery,
    LeaguesRelic,
    LeaguesTask,
    Chat,
    ExternalPlugin,
    Login,
    Logout,
    ToaUnique,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub content: String,
    pub extra: HashMap<String, String>,
    pub notification_type: NotificationType,
    pub player_name: String,
    pub account_type: String,
    pub seasonal_world: String,
    pub dink_account_hash: String,
    // pub embeds: Vec<Embed>, ignore for now
}

impl Notification {
    /// Creates a short message summarizing the notification
    pub fn message(&self) -> String {
        match &self.notification_type {
            NotificationType::Death => {
                let value_lost = self
                    .extra
                    .get("valueLost")
                    .map(|s| s.as_str())
                    .unwrap_or("0");
                let is_pvp = self
                    .extra
                    .get("isPvp")
                    .map(|s| s.as_str())
                    .unwrap_or("false");
                if is_pvp == "true" {
                    format!("{} died in PvP, lost {} gp", self.player_name, value_lost)
                } else {
                    format!("{} died, lost {} gp", self.player_name, value_lost)
                }
            }
            NotificationType::Collection => {
                let item_name = self
                    .extra
                    .get("itemName")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown item");
                let price = self.extra.get("price").map(|s| s.as_str()).unwrap_or("0");
                format!(
                    "{} added {} to collection ({} gp)",
                    self.player_name, item_name, price
                )
            }
            NotificationType::Level => {
                let skills: Vec<String> = self
                    .extra
                    .get("levelledSkills")
                    .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
                    .unwrap_or_default();
                if skills.is_empty() {
                    format!("{} leveled up", self.player_name)
                } else {
                    format!("{} leveled up: {}", self.player_name, skills.join(", "))
                }
            }
            NotificationType::XpMilestone => {
                let milestone = self
                    .extra
                    .get("milestoneAchieved")
                    .map(|s| {
                        s.split(',')
                            .map(|s| s.trim().to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    })
                    .unwrap_or_else(|| "skills".to_string());
                format!("{} hit XP milestone in {}", self.player_name, milestone)
            }
            NotificationType::Loot => {
                let source = self
                    .extra
                    .get("source")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown source");
                let items_count = self
                    .extra
                    .get("items")
                    .map(|s| s.split(',').count())
                    .unwrap_or(1);
                format!(
                    "{} looted {} items from {}",
                    self.player_name, items_count, source
                )
            }
            NotificationType::Slayer => {
                let task = self
                    .extra
                    .get("slayerTask")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown task");
                let points = self
                    .extra
                    .get("slayerPoints")
                    .map(|s| s.as_str())
                    .unwrap_or("0");
                format!(
                    "{} completed {} slayer task (+{} points)",
                    self.player_name, task, points
                )
            }
            NotificationType::Quest => {
                let quest = self
                    .extra
                    .get("questName")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown quest");
                format!("{} completed quest: {}", self.player_name, quest)
            }
            NotificationType::Clue => {
                let clue_type = self
                    .extra
                    .get("clueType")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                let count = self
                    .extra
                    .get("numberCompleted")
                    .map(|s| s.as_str())
                    .unwrap_or("0");
                format!(
                    "{} completed {} clue (#{})",
                    self.player_name, clue_type, count
                )
            }
            NotificationType::KillCount => {
                let boss = self
                    .extra
                    .get("boss")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown boss");
                let count = self.extra.get("count").map(|s| s.as_str()).unwrap_or("0");
                format!("{} defeated {} (count: {})", self.player_name, boss, count)
            }
            NotificationType::CombatAchievement => {
                let task = self
                    .extra
                    .get("task")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown task");
                let tier = self
                    .extra
                    .get("tier")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown tier");
                format!(
                    "{} completed {} combat task: {}",
                    self.player_name, tier, task
                )
            }
            NotificationType::AchievementDiary => {
                let area = self
                    .extra
                    .get("area")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown area");
                let difficulty = self
                    .extra
                    .get("difficulty")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!(
                    "{} completed {} {} diary",
                    self.player_name, difficulty, area
                )
            }
            NotificationType::Pet => {
                let pet_name = self
                    .extra
                    .get("petName")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown pet");
                format!("{} got pet: {}", self.player_name, pet_name)
            }
            NotificationType::Speedrun => {
                let quest = self
                    .extra
                    .get("questName")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown quest");
                let time = self
                    .extra
                    .get("currentTime")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown time");
                format!("{} speedran {} in {}", self.player_name, quest, time)
            }
            NotificationType::BarbarianAssaultGamble => {
                let count = self
                    .extra
                    .get("gambleCount")
                    .map(|s| s.as_str())
                    .unwrap_or("0");
                format!("{} reached {} high gambles", self.player_name, count)
            }
            NotificationType::PlayerKill => {
                let victim = self
                    .extra
                    .get("victimName")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!("{} PK'd {}", self.player_name, victim)
            }
            NotificationType::GroupStorage => {
                let _group = self
                    .extra
                    .get("groupName")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown group");
                let net_value = self
                    .extra
                    .get("netValue")
                    .map(|s| s.as_str())
                    .unwrap_or("0");
                format!(
                    "{} group storage activity ({} gp net)",
                    self.player_name, net_value
                )
            }
            NotificationType::GrandExchange => {
                let status = self
                    .extra
                    .get("status")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                let item_name = self
                    .extra
                    .get("item")
                    .and_then(|s| s.split(',').next())
                    .unwrap_or("Unknown item");
                format!("{} {} on GE: {}", self.player_name, status, item_name)
            }
            NotificationType::Trade => {
                let counterparty = self
                    .extra
                    .get("counterparty")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!("{} traded with {}", self.player_name, counterparty)
            }
            NotificationType::LeaguesArea => {
                let area = self
                    .extra
                    .get("area")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown area");
                format!("{} selected region: {}", self.player_name, area)
            }
            NotificationType::LeaguesMastery => {
                let mastery = self
                    .extra
                    .get("masteryType")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!("{} unlocked {} mastery", self.player_name, mastery)
            }
            NotificationType::LeaguesRelic => {
                let relic = self
                    .extra
                    .get("relic")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown relic");
                let tier = self
                    .extra
                    .get("tier")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!(
                    "{} unlocked Tier {} relic: {}",
                    self.player_name, tier, relic
                )
            }
            NotificationType::LeaguesTask => {
                let task = self
                    .extra
                    .get("taskName")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown task");
                let difficulty = self
                    .extra
                    .get("difficulty")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!(
                    "{} completed {} task: {}",
                    self.player_name, difficulty, task
                )
            }
            NotificationType::Chat => {
                let message_type = self
                    .extra
                    .get("type")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!("{} received {} message", self.player_name, message_type)
            }
            NotificationType::ExternalPlugin => {
                let plugin = self
                    .extra
                    .get("sourcePlugin")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown plugin");
                format!(
                    "{} external plugin notification from {}",
                    self.player_name, plugin
                )
            }
            NotificationType::Login => {
                let world = self
                    .extra
                    .get("world")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!("{} logged into World {}", self.player_name, world)
            }
            NotificationType::Logout => {
                format!("{} logged out", self.player_name)
            }
            NotificationType::ToaUnique => {
                let raid_level = self
                    .extra
                    .get("raidLevels")
                    .map(|s| s.as_str())
                    .unwrap_or("Unknown");
                format!(
                    "{} rolled purple from TOA (level {})",
                    self.player_name, raid_level
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_death_notification_message() {
        let mut extra = HashMap::new();
        extra.insert("valueLost".to_string(), "500".to_string());
        extra.insert("isPvp".to_string(), "false".to_string());
        extra.insert("killerName".to_string(), "Giant Spider".to_string());

        let notification = Notification {
            content: "PlayerName has died...".to_string(),
            extra,
            notification_type: NotificationType::Death,
            player_name: "PlayerName".to_string(),
            account_type: "NORMAL".to_string(),
            seasonal_world: "false".to_string(),
            dink_account_hash: "test_hash".to_string(),
        };

        let message = notification.message();
        assert_eq!(message, "PlayerName died, lost 500 gp");
    }

    #[test]
    fn test_death_pvp_notification_message() {
        let mut extra = HashMap::new();
        extra.insert("valueLost".to_string(), "1000".to_string());
        extra.insert("isPvp".to_string(), "true".to_string());
        extra.insert("killerName".to_string(), "PKerName".to_string());

        let notification = Notification {
            content: "PlayerName has just been PKed by PKerName for 1000 gp...".to_string(),
            extra,
            notification_type: NotificationType::Death,
            player_name: "PlayerName".to_string(),
            account_type: "NORMAL".to_string(),
            seasonal_world: "false".to_string(),
            dink_account_hash: "test_hash".to_string(),
        };

        let message = notification.message();
        assert_eq!(message, "PlayerName died in PvP, lost 1000 gp");
    }

    #[test]
    fn test_collection_notification_message() {
        let mut extra = HashMap::new();
        extra.insert("itemName".to_string(), "Zamorak chaps".to_string());
        extra.insert("itemId".to_string(), "10372".to_string());
        extra.insert("price".to_string(), "500812".to_string());
        extra.insert("completedEntries".to_string(), "420".to_string());
        extra.insert("totalEntries".to_string(), "1443".to_string());
        extra.insert("currentRank".to_string(), "IRON".to_string());

        let notification = Notification {
            content: "PlayerName has added Zamorak chaps to their collection".to_string(),
            extra,
            notification_type: NotificationType::Collection,
            player_name: "PlayerName".to_string(),
            account_type: "NORMAL".to_string(),
            seasonal_world: "false".to_string(),
            dink_account_hash: "test_hash".to_string(),
        };

        let message = notification.message();
        assert_eq!(
            message,
            "PlayerName added Zamorak chaps to collection (500812 gp)"
        );
    }

    #[test]
    fn test_loot_valuable_drop_notification_message() {
        let mut extra = HashMap::new();
        extra.insert("source".to_string(), "Tombs of Amascut".to_string());
        extra.insert(
            "items".to_string(),
            "Tumeken's shadow, Osmumten's fang, Lightbearer".to_string(),
        );
        extra.insert("category".to_string(), "EVENT".to_string());
        extra.insert("killCount".to_string(), "60".to_string());
        extra.insert("rarestProbability".to_string(), "0.001".to_string());

        let notification = Notification {
            content: "PlayerName has looted: Tumeken's shadow, Osmumten's fang, Lightbearer\nFrom: Tombs of Amascut".to_string(),
            extra,
            notification_type: NotificationType::Loot,
            player_name: "PlayerName".to_string(),
            account_type: "NORMAL".to_string(),
            seasonal_world: "false".to_string(),
            dink_account_hash: "test_hash".to_string(),
        };

        let message = notification.message();
        assert_eq!(message, "PlayerName looted 3 items from Tombs of Amascut");
    }

    #[test]
    fn test_loot_single_item_notification_message() {
        let mut extra = HashMap::new();
        extra.insert("source".to_string(), "Giant Mole".to_string());
        extra.insert("items".to_string(), "Mole skin".to_string());
        extra.insert("category".to_string(), "NPC".to_string());
        extra.insert("npcId".to_string(), "3340".to_string());

        let notification = Notification {
            content: "PlayerName has looted: Mole skin\nFrom: Giant Mole".to_string(),
            extra,
            notification_type: NotificationType::Loot,
            player_name: "PlayerName".to_string(),
            account_type: "NORMAL".to_string(),
            seasonal_world: "false".to_string(),
            dink_account_hash: "test_hash".to_string(),
        };

        let message = notification.message();
        assert_eq!(message, "PlayerName looted 1 items from Giant Mole");
    }

    #[test]
    fn test_notification_with_missing_extra_data() {
        let extra = HashMap::new();

        let notification = Notification {
            content: "PlayerName has died...".to_string(),
            extra,
            notification_type: NotificationType::Death,
            player_name: "PlayerName".to_string(),
            account_type: "NORMAL".to_string(),
            seasonal_world: "false".to_string(),
            dink_account_hash: "test_hash".to_string(),
        };

        let message = notification.message();
        assert_eq!(message, "PlayerName died, lost 0 gp");
    }
}
