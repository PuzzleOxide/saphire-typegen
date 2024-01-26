use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub name: String,
    pub codeblock_name: String,
    pub tags: Vec<ActionTags>,
    pub aliases: Vec<String>,
    pub icon: ActionIconOptions,
    #[serde(default)]
    pub sub_action_blocks: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionTags {
    pub name: String,
    pub options: Vec<ActionTagOption>,
    pub default_option: String,
    pub slot: usize,
    //pub aliases: Vec<String>, // apparently this was removed, leaving it here incase it comes back
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionTagOption {
    pub name: String,
    pub icon: ActionTagIcon,
    pub aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionTagIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<String>,
    pub description: Vec<String>,
    pub example: Vec<String>,
    pub works_with: Vec<String>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActionIconOptions {
    Icon (
        ActionIcon,
    ),
    Event (
        ActionEventIcon,
    ),
}

impl Default for ActionIconOptions {
    fn default() -> Self {
        ActionIconOptions::Event(ActionEventIcon::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionEventIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<String>,
    pub description: Vec<String>,
    pub example: Vec<String>,
    pub works_with: Vec<String>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionIcon {
    pub material: String,
    pub name: String,
    pub deprecated_note: Vec<String>,
    pub description: Vec<String>,
    pub example: Vec<String>,
    pub works_with: Vec<String>,
    pub additional_info: Vec<Vec<String>>,
    pub required_rank: String,
    pub require_tokens: bool,
    pub require_rank_and_tokens: bool,
    pub advanced: bool,
    pub loaded_item: String,
    //pub tags: usize, // TODO: change action_icon to be an enum for support of icons that dont have tags or arguments
    pub arguments: Vec<ActionArgOptions>,
    pub return_values: Vec<ActionIconReturnTypeOption>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActionArgOptions {
    Text {
        text: String,
    },
    Arg (
        ActionArg,
    )
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ActionArg {
    #[serde(rename = "type")]
    pub arg_type: String,
    pub plural: bool,
    pub optional: bool,
    pub description: Vec<String>,
    pub notes: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum ActionIconReturnTypeOption {
    Text {
        text: String,
    },
    Arg (
        ActionIconReturnType,
    )
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ActionIconReturnType {
    #[serde(rename = "type")]
    pub type_name: String,
    pub description: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_action() {
        let encoded = serde_json::to_string_pretty(&Action::default()).unwrap();
        println!("{}", encoded);
        let json = r#"
        {
            "name": "SetReducedDebug",
            "codeblockName": "PLAYER ACTION",
            "tags": [
                {
                    "name": "Reduced Debug Info Enabled",
                    "options": [
                        {
                            "name": "True",
                            "icon": {
                                "material": "LIME_DYE",
                                "name": "",
                                "deprecatedNote": [],
                                "description": [],
                                "example": [],
                                "worksWith": [],
                                "additionalInfo": [],
                                "requiredRank": "",
                                "requireTokens": false,
                                "requireRankAndTokens": false,
                                "advanced": false,
                                "loadedItem": ""
                            },
                            "aliases": []
                        },
                        {
                            "name": "False",
                            "icon": {
                                "material": "RED_DYE",
                                "name": "",
                                "deprecatedNote": [],
                                "description": [],
                                "example": [],
                                "worksWith": [],
                                "additionalInfo": [],
                                "requiredRank": "",
                                "requireTokens": false,
                                "requireRankAndTokens": false,
                                "advanced": false,
                                "loadedItem": ""
                            },
                            "aliases": []
                        }
                    ],
                    "defaultOption": "True",
                    "slot": 26
                }
            ],
            "aliases": [],
            "icon": {
                "material": "COMPASS",
                "name": "§9Set Reduced Debug Info Enabled",
                "deprecatedNote": [],
                "description": [
                    "When enabled, a player won't be",
                    "able to see their coordinates,",
                    "block info, or other info."
                ],
                "example": [],
                "worksWith": [],
                "additionalInfo": [],
                "requiredRank": "",
                "requireTokens": false,
                "requireRankAndTokens": false,
                "advanced": false,
                "loadedItem": "",
                "tags": 1,
                "arguments": [],
                "returnValues": []
            }
        }
        "#;

        let action = serde_json::from_str::<Action>(json).unwrap();
        println!("{:#?}", action);
    }

    #[test]
    fn test_action_icon() {
        let encoded = serde_json::to_string_pretty(&ActionIconOptions::default()).unwrap();
        println!("{}", encoded);
        let json = r#"
        {
            "material": "DROPPER",
            "name": "Set Hotbar Items",
            "deprecatedNote": [],
            "description": [
                "Sets items in a player's",
                "hotbar."
            ],
            "example": [],
            "worksWith": [],
            "additionalInfo": [],
            "requiredRank": "",
            "requireTokens": false,
            "requireRankAndTokens": false,
            "advanced": false,
            "loadedItem": "",
            "tags": 0,
            "arguments": [
                {
                    "type": "ITEM",
                    "plural": true,
                    "optional": false,
                    "description": [
                        "Item(s) to set"
                    ],
                    "notes": [
                        [
                            "Slots 1-9"
                        ]
                    ]
                }
            ]
        }
        "#;
        let action_icon = serde_json::from_str::<ActionIconOptions>(json).unwrap();
        println!("{:#?}", action_icon);
    }

    #[test]
    fn test_action_args() {
        let encoded = serde_json::to_string_pretty(&ActionArgOptions::Arg(ActionArg::default())).unwrap();
        println!("{}", encoded);
        let json = r#"
        {
            "text": "OR"
        }
        "#;

        let action_args = serde_json::from_str::<ActionArgOptions>(json).unwrap();
        println!("{:#?}", action_args);
    }
}