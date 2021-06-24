// This file is auto generated by [`cg`] from [`schema`].
//
// **DO NOT EDIT THIS FILE**,
//
// Edit `cg` or `schema` instead.
//
// [cg]: https://github.com/teloxide/cg
// [`schema`]: https://github.com/WaffleLapkin/tg-methods-schema
use serde::Serialize;

use crate::types::{BotCommand, True};

impl_payload! {
    /// Use this method to change the list of the bot's commands. Returns _True_ on success.
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize)]
    pub SetMyCommands (SetMyCommandsSetters) => True {
        required {
            /// A JSON-serialized list of bot commands to be set as the list of the bot's commands. At most 100 commands can be specified.
            pub commands: Vec<BotCommand> [collect],
        }
    }
}
