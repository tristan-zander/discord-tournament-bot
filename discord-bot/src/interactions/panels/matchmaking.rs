pub mod report_score {
    use std::str::FromStr;

    use twilight_model::{
        application::interaction::MessageComponentInteraction,
        id::{marker::UserMarker, Id},
    };

    pub struct ScoreData {
        // The name of the user
        pub user: String,
        pub score: u32,
    }

    #[non_exhaustive]
    pub enum ScoreMode {
        AddPoints,
        RemovePoints,
    }

    impl FromStr for ScoreMode {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "Add points" => Ok(ScoreMode::AddPoints),
                "Remove points" => Ok(ScoreMode::RemovePoints),
                _ => Err(anyhow!("Unknown value {}", s)),
            }
        }
    }

    impl Into<String> for ScoreMode {
        fn into(self) -> String {
            match self {
                ScoreMode::AddPoints => "Add points".to_string(),
                ScoreMode::RemovePoints => "Remove points".to_string(),
            }
        }
    }

    impl Into<&str> for ScoreMode {
        fn into(self) -> &'static str {
            match self {
                ScoreMode::AddPoints => "Add points",
                ScoreMode::RemovePoints => "Remove points",
            }
        }
    }

    #[non_exhaustive]
    pub enum ReportScoreAction {
        /// Confirm the results, commit to the database if the results are verified by both parties.
        ConfirmResults,
        /// Should show a modal to the user, explaining the abuse.
        ReportAbuse,
        /// Toggle the [ScoreMode](ScoreMode) of the message.
        SwitchMode,
        /// Raise or lower the specified user's score depending on the [ScoreMode](ScoreMode).
        ReportScore(Id<UserMarker>),
    }

    pub struct ReportScorePanel {
        pub user_scores: Vec<ScoreData>,
        pub mode: ScoreMode,
        pub action: ReportScoreAction,
    }

    impl TryFrom<MessageComponentInteraction> for ReportScorePanel {
        type Error = anyhow::Error;

        fn try_from(interaction: MessageComponentInteraction) -> Result<Self, Self::Error> {
            let embed = interaction
                .message
                .embeds
                .get(0)
                .ok_or_else(|| anyhow!("message contained no embed"))?;

            let mut user_scores = Vec::with_capacity(embed.fields.len() - 1);
            for f in &embed.fields {
                if f.inline == false {
                    continue;
                }

                let score = {
                    // Using index of 1 here for the take function
                    let mut char_index: usize = 0;
                    for (i, c) in f.value.chars().enumerate() {
                        if c.is_numeric() {
                            char_index = i + 1;
                        } else {
                            break;
                        }
                    }

                    if char_index == 0 {
                        return Err(anyhow!("could not find the score for {}", f.name));
                    } else {
                        f.value
                            .chars()
                            .into_iter()
                            .take(char_index)
                            .collect::<String>()
                            .parse()?
                    }
                };

                user_scores.push(ScoreData {
                    user: f.name.to_owned(),
                    score,
                });
            }

            let mode = embed
                .fields
                .iter()
                .filter(|f| f.inline == false)
                .nth(1)
                .ok_or_else(|| anyhow!("could not get field for current mode"))?
                .value
                .parse()?;

            let button_id = interaction.data.custom_id.as_str();
            let action = match button_id {
                "switch_mode" => ReportScoreAction::SwitchMode,
                "confirm_results" => ReportScoreAction::ConfirmResults,
                "report_abuse" => ReportScoreAction::ReportAbuse,
                _ => match str::parse::<Id<UserMarker>>(button_id) {
                    Ok(id) => ReportScoreAction::ReportScore(id),
                    Err(e) => return Err(anyhow!(e)),
                },
            };

            Ok(Self {
                user_scores,
                mode,
                action,
            })
        }
    }

    impl ReportScorePanel {
        pub fn add_user(&mut self, name: String, score: u32) {
            self.user_scores.push(ScoreData { user: name, score });
        }
    }
}
