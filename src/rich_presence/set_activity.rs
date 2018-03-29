use models::Command;
use utils::pid;


#[derive(Debug, Default, Serialize)]
pub struct SetActivityArgs {
    pid: i32,
    activity: SetActivity,
}

impl SetActivityArgs {
    pub fn command(args: SetActivity) -> Command<Self> {
        Command::new("SET_ACTIVITY", Self {
            pid: pid(),
            activity: args
        })
    }
}

message_format![SetActivity
    state:      String,
    details:    String,
    instance:   bool,
    timestamps: SetActivityTimestamps func,
    assets:     SetActivityAssets func,
    party:      SetActivityParty func,
    secrets:    SetActivitySecrets func,
];

message_format![SetActivityTimestamps
    start: u32,
    end: u32,
];

message_format![SetActivityAssets
    large_image: String,
    large_text: String,
    small_image: String,
    small_text: String,
];

message_format![SetActivityParty
    id: u32,
    size: (u32, u32),
];

message_format![SetActivitySecrets
    join: String,
    spectate: String,
    game: String alias = "match",
];

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    const FULL_JSON: &'static str =
r###"{
  "state": "rusting",
  "details": "detailed",
  "instance": true,
  "timestamps": {
    "start": 1000,
    "end": 2000
  },
  "assets": {
    "large_image": "ferris",
    "large_text": "Ferris",
    "small_image": "rusting",
    "small_text": "Rusting..."
  },
  "party": {
    "id": 1,
    "size": [
      3,
      6
    ]
  },
  "secrets": {
    "join": "025ed05c71f639de8bfaa0d679d7c94b2fdce12f",
    "spectate": "e7eb30d2ee025ed05c71ea495f770b76454ee4e0",
    "match": "4b2fdce12f639de8bfa7e3591b71a0d679d7c93f"
  }
}"###;

    #[test]
    fn test_serialize_full_activity() {
        let activity = SetActivity::new()
            .state("rusting")
            .details("detailed")
            .instance(true)
            .timestamps(|t| t
                .start(1000)
                .end(2000))
            .assets(|a| a
                .large_image("ferris")
                .large_text("Ferris")
                .small_image("rusting")
                .small_text("Rusting..."))
            .party(|p| p
                .id(1)
                .size((3, 6)))
            .secrets(|s| s
                .join("025ed05c71f639de8bfaa0d679d7c94b2fdce12f")
                .spectate("e7eb30d2ee025ed05c71ea495f770b76454ee4e0")
                .game("4b2fdce12f639de8bfa7e3591b71a0d679d7c93f"));

        let json = serde_json::to_string_pretty(&activity).unwrap();

        assert_eq![json, FULL_JSON];
    }

    #[test]
    fn test_serialize_empty_activity() {
        let activity = SetActivity::new();
        let json = serde_json::to_string(&activity).unwrap();
        assert_eq![json, "{}"];
    }
}
