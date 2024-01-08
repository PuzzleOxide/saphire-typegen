use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Action {
    pub name: String,
    pub codeblock_name: String,
    pub tags: Vec<ActionTags>,
    pub aliases: Vec<String>,
    pub icon: ActionIconOptions,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionTags {
    pub name: String,
    pub options: Vec<ActionTagOption>,
    pub default_option: String,
    pub slot: usize,
    pub aliases: Vec<String>,
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
    Argless (
        ActionIconArgless,
    ),
}

impl Default for ActionIconOptions {
    fn default() -> Self {
        ActionIconOptions::Argless(ActionIconArgless::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ActionIconArgless {
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
    pub arguments: Vec<ActionArgOptions>
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_action() {
        let encoded = serde_json::to_string_pretty(&Action::default()).unwrap();
        println!("{}", encoded);
        let json = r#"
        {
            "name": "Raycast",
            "codeblockName": "SET VARIABLE",
            "tags": [
                {
                    "name": "Entity Collision",
                    "options": [
                        {
                            "name": "True",
                            "icon": {
                                "material": "LIME_DYE",
                                "name": "",
                                "deprecatedNote": [],
                                "description": [
                                    "Collides with players",
                                    "and entities."
                                ],
                                "example": [],
                                "worksWith": [],
                                "additionalInfo": [
                                    [
                                        "To select entities on the",
                                        "ray, use 'Filter Selection",
                                        "by Raycast'."
                                    ]
                                ],
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
                    "defaultOption": "False",
                    "slot": 25,
                    "aliases": []
                },
                {
                    "name": "Block Collision",
                    "options": [
                        {
                            "name": "All blocks",
                            "icon": {
                                "material": "STONE_BRICKS",
                                "name": "",
                                "deprecatedNote": [],
                                "description": [
                                    "Passes through air only."
                                ],
                                "example": [],
                                "worksWith": [],
                                "additionalInfo": [
                                    [
                                        "The hit location's",
                                        "direction is set to",
                                        "the hit block side."
                                    ]
                                ],
                                "requiredRank": "",
                                "requireTokens": false,
                                "requireRankAndTokens": false,
                                "advanced": false,
                                "loadedItem": ""
                            },
                            "aliases": []
                        },
                        {
                            "name": "Non-fluid blocks",
                            "icon": {
                                "material": "WATER_BUCKET",
                                "name": "",
                                "deprecatedNote": [],
                                "description": [
                                    "Passes through fluids",
                                    "(water and lava)."
                                ],
                                "example": [],
                                "worksWith": [],
                                "additionalInfo": [
                                    [
                                        "The hit location's",
                                        "direction is set to",
                                        "the hit block side."
                                    ]
                                ],
                                "requiredRank": "",
                                "requireTokens": false,
                                "requireRankAndTokens": false,
                                "advanced": false,
                                "loadedItem": ""
                            },
                            "aliases": []
                        },
                        {
                            "name": "Solid blocks",
                            "icon": {
                                "material": "TALL_GRASS",
                                "name": "",
                                "deprecatedNote": [],
                                "description": [
                                    "Passes through non-solid",
                                    "blocks such as tall grass,",
                                    "and through fluids."
                                ],
                                "example": [],
                                "worksWith": [],
                                "additionalInfo": [
                                    [
                                        "The hit location's",
                                        "direction is set to",
                                        "the hit block side."
                                    ]
                                ],
                                "requiredRank": "",
                                "requireTokens": false,
                                "requireRankAndTokens": false,
                                "advanced": false,
                                "loadedItem": ""
                            },
                            "aliases": []
                        },
                        {
                            "name": "None",
                            "icon": {
                                "material": "FEATHER",
                                "name": "",
                                "deprecatedNote": [],
                                "description": [
                                    "Passes through all blocks."
                                ],
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
                    "defaultOption": "All blocks",
                    "slot": 26,
                    "aliases": []
                }
            ],
            "aliases": [],
            "icon": {
                "material": "SPECTRAL_ARROW",
                "name": "Raycast from Location",
                "deprecatedNote": [],
                "description": [
                    "Raycasts from a location",
                    "to the first intersection."
                ],
                "example": [],
                "worksWith": [],
                "additionalInfo": [],
                "requiredRank": "",
                "requireTokens": false,
                "requireRankAndTokens": false,
                "advanced": false,
                "loadedItem": "",
                "tags": 2,
                "arguments": [
                    {
                        "type": "VARIABLE",
                        "plural": false,
                        "optional": false,
                        "description": [
                            "Variable to set"
                        ],
                        "notes": []
                    },
                    {
                        "type": "LOCATION",
                        "plural": false,
                        "optional": false,
                        "description": [
                            "Ray origin"
                        ],
                        "notes": []
                    },
                    {
                        "type": "NUMBER",
                        "plural": false,
                        "optional": false,
                        "description": [
                            "Ray distance"
                        ],
                        "notes": []
                    }
                ]
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