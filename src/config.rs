use std::io::{Read, Write};

use serde::{Deserialize, Serialize};

use log;
use serenity::model::prelude::{GuildId, RoleId};

#[derive(Debug, Serialize, Deserialize)]
pub struct BotConfig {
    bot_login_token: String,
    pub guild_id: GuildId,
    pub catify_id: Option<RoleId>,
}
impl Default for BotConfig {
    fn default() -> Self {
        Self {
            bot_login_token: Default::default(),
            guild_id: Default::default(),
            catify_id: None,
        }
    }
}

impl BotConfig {
    /// loads a [BotConfig] from a json file at file_path and returns it
    ///
    /// ### Panics
    /// Panics if the specifed path does not exist <br>
    /// Panics if the process lacks permission to read from the file
    /// ### Errors
    /// returns an [`std::io::Error`]
    pub fn load_from_path(file_path: &std::path::Path) -> Result<Self, std::io::Error> {
        let mut _buffer = String::new();
        let mut _file = match std::fs::File::open(file_path) {
            Ok(f) => f,
            Err(why) => {
                log::error!("{}", why);
                return Err(why);
            }
        };
        let _ = _file.read_to_string(&mut _buffer);

        Ok(serde_json::from_str::<BotConfig>(&_buffer).unwrap())
    }
    pub fn save_to_path(&self, file_path: &std::path::Path) -> Result<(), std::io::Error> {
        let buffer = serde_json::to_string_pretty(self).unwrap();
        let mut file = match std::fs::File::create(file_path) {
            Ok(f) => f,
            Err(why) => {
                log::error!("{}", why);
                return Err(why);
            }
        };
        let _ = match file.write(buffer.as_bytes()) {
            Ok(s) => s,
            Err(why) => {
                log::error!("{}", why);
                return Err(why);
            }
        };
        Ok(())
    }

    pub fn get_bot_login_token(&self) -> String {
        self.bot_login_token.clone()
    }
    pub fn set_bot_login_token(&mut self, token: String) {
        self.bot_login_token = token;
    }
}

#[cfg(test)]
mod tests {
    use serenity::model::prelude::GuildId;

    use super::BotConfig;

    #[test]
    fn test_config_save() {
        let mut cf = BotConfig::default();
        cf.set_bot_login_token(String::from("test_bot_token"));
        let file_path = std::path::Path::new("./test.config.json");

        cf.guild_id = GuildId(123_123);
        cf.save_to_path(file_path).unwrap();
    }

    #[test]
    fn test_config_load() {
        let file_path = std::path::Path::new("./test.config.json");
        let cf = BotConfig::load_from_path(file_path).unwrap();

        assert_eq!(cf.get_bot_login_token(), String::from("test_bot_token"));
        assert_eq!(cf.guild_id, GuildId(123_123));
        assert_eq!(cf.catify_id, None);
    }
}
