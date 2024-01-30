use std::{collections::{hash_map, HashMap, HashSet}, fs::File, hash, io::prelude::*, ops::RangeBounds, option, path::PathBuf};
use proc_macro2::{token_stream, Ident};
use quote::{format_ident, quote};
use prettier_please;
use serde_json::Value;
use crate::raw_types::{Action, ActionArgOptions, ActionIconOptions};

/// Generates all enum variants for the given action dump file in the directory specified by mod_path.
pub fn gen_types<T: Into<PathBuf>>(action_dump_path: T, module_path: T) -> () {
    let action_dump_path = action_dump_path.into();
    let mut file = File::open(action_dump_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let json = serde_json::from_str::<serde_json::Value>(&contents).unwrap();
    let actions = json.as_object().expect("Your action dump file is not a json object!")
        .get("actions").expect("Your action dump file has no actions!")
        .as_array().expect("Your action dump file's actions entry is not an array!");

    let mut module_code = hash_map::HashMap::new();
    module_code.insert("player_event".to_string(), Vec::new());
    module_code.insert("player_action".to_string(), Vec::new());
    module_code.insert("if_player".to_string(), Vec::new());
    module_code.insert("entity_event".to_string(), Vec::new());
    module_code.insert("entity_action".to_string(), Vec::new());
    module_code.insert("if_entity".to_string(), Vec::new());
    module_code.insert("game_action".to_string(), Vec::new());
    module_code.insert("if_game".to_string(), Vec::new());
    module_code.insert("set_variable".to_string(), Vec::new());
    module_code.insert("if_variable".to_string(), Vec::new());
    module_code.insert("repeat".to_string(), Vec::new());
    module_code.insert("control".to_string(), Vec::new());
    module_code.insert("select_object".to_string(), Vec::new());

    let mut enum_names = HashSet::new();
    let mut tags = TagSet::new();

    for (i, action) in actions.iter().enumerate() {
        // Skips call function and call process actions.
        // TODO: See if there's a way to parse these.
        if action.as_object().unwrap().get("name").unwrap().as_str().unwrap() == "dynamic" {
            continue;
        }
        let action = serde_json::from_value::<Action>(action.clone()).expect(&format!("Failed to parse action #{}!", i));
        let action_block = action.codeblock_name.to_ascii_lowercase().replace(" ", "_");
        let action_data = gen_action(action, &mut enum_names, &mut tags);

        module_code.get_mut(&action_block).unwrap().push(action_data);
    }

    let mut module_path: PathBuf = module_path.into();
    for (module_name, module_code) in module_code {
        module_path.push(format!("{}.rs", module_name));
        eprintln!("Writing module {} to {}...", module_name, module_path.to_str().unwrap());
        let mut file = File::create(&module_path).unwrap();

        let enum_name = quote::format_ident!("{}", snake_to_camel_case(&module_name));
        let (action_defs, (compile_functions, tag_defs)): (Vec<_>, (Vec<_>, Vec<_>)) = module_code.into_iter().unzip();
        let module_code = quote!(
            use either::Either;
            use serde_json::Value;
            use crate::types::*;
            use crate::block::block_types::subactions::*;

            pub enum #enum_name {
                #(#action_defs),*
            }

            impl #enum_name {
                pub fn compile(&self) -> Value {
                    match self {
                        #(#compile_functions)*
                    }
                }
            }

            #(#tag_defs)*
        );

        let module_code = syn::parse2(module_code).unwrap();
        let module_code = prettier_please::unparse(&module_code);

        file.write_all(module_code.to_string().as_bytes()).unwrap();

        module_path.pop();
    }
}

/// Generates a single enum variant for a given action object.
fn gen_action(action: Action, used_names: &mut HashSet<String>, tags: &mut TagSet) -> (token_stream::TokenStream, (token_stream::TokenStream, token_stream::TokenStream)) {
    let mut action_name = match &action.icon {
        ActionIconOptions::Icon(icon) => format_name(&icon.name),
        ActionIconOptions::Event(argless) => format_name(&argless.name)
    };
    let unformated_action_name = action.name.clone();
    // if action.aliases.len() > 0 {
    //     action_name = format_name(&action.aliases[0]);
    // }
    if action_name == "" || used_names.contains(&format!("{} {}", action_name, action.codeblock_name)) {
        action_name = format_name(&action.name);
    }
    if used_names.contains(&format!("{} {}", action_name, action.codeblock_name)) {
        action_name = format!("{}N", action_name)
    }
    // if used_names.contains(&action_name) &&
    //     match &action.icon {
    //         ActionIconOptions::Icon(icon) => icon.,
    //         ActionIconOptions::Argless(argless) => format_name(&argless.name)
    //     }
    // {
    //     action_name = format_name(&action.[0]);
    // }
    used_names.insert(format!("{} {}", action_name, action.codeblock_name));

    let action_name = quote::format_ident!("{}", action_name);

    let mut arg_types = Vec::new();
    let args = match action.icon {
        ActionIconOptions::Icon(icon) => icon.arguments,
        ActionIconOptions::Event(_) => Vec::new(),
    };

    let mut arg_names = Vec::new();

    // Turns the argument list into a list of fields with types.
    let mut i = 0;
    let len = args.len();
    while i < len {
        if let ActionArgOptions::Arg(arg ) = &args[i] {
            let mut outer_arg = arg.clone();
            let mut output = arg_type_to_rust(&outer_arg.arg_type);
            loop {
                if len > i+2 {
                    if let ActionArgOptions::Text{ text } = &args[i+1] {
                        if strip_colour(text) == "OR" {
                            if let ActionArgOptions::Arg(arg) = &args[i+2] {
                                if arg.arg_type == "NONE" {
                                    outer_arg.optional = true;
                                }
                                else {
                                    let inner_arg_type = arg_type_to_rust(&arg.arg_type);
                                    output = quote!(Either<#output, #inner_arg_type>);
                                }
                                i += 2;
                                continue;
                            }
                        }
                    }
                }
                break;
            }

            if outer_arg.plural {
                output = quote!(Vec<#output>);
            }
            else if outer_arg.optional {
                output = quote!(Option<#output>);
            }

            let arg_name = quote::format_ident!("{}", remove_leading_nonalpha(&outer_arg.description[0]).replace(" ", "_").replace(|c: char| {!c.is_ascii_alphanumeric() && c != '_'}, "").to_lowercase().replace("type", "type_"));
            arg_names.push(arg_name.clone());
            arg_types.push(quote!(
                #arg_name: #output
            ));
        }
        eprintln!("arg: {:?}", args[i]);
        i += 1;
    }
    eprintln!("args: {:?}", arg_types);

    // turns the tag list into a list of fields with types and a tokenstream of tag enum definitions.
    let mut tag_types = Vec::new();
    let mut tag_names = Vec::new();
    let mut tag_defs: Vec<token_stream::TokenStream> = Vec::new();
    for tag in action.tags {
        let tag_type = format_ident!("{}{}", &format_name(&tag.name), action_name);
        let tag_ident = format_ident!("{}_tag", remove_leading_nonalpha(&tag.name).replace(" ", "_").replace(|c: char| {!c.is_ascii_alphanumeric() && c != '_'}, "").to_lowercase().replace("type", "type_"));
        let tag_name_unformated = tag.name.clone();
        tag_types.push(quote!(#tag_ident: #tag_type));
        tag_names.push(tag_ident.clone());

        let mut tag_default = format_ident!("placeholder");
        let mut tag_options = Vec::new();
        let mut tag_strings = Vec::new();
        for option in tag.options {
            let option_name = if format_name(&replace_numeric(&option.name)) != "" {
                format_ident!("{}", &format_name(&replace_numeric(&option.name)))
            } 
            else if option.aliases.len() > 0 && format_name(&remove_leading_nonalpha(&option.aliases[0])) != "" {
                format_ident!("{}", &format_name(&replace_numeric(&option.aliases[0])))
            }
            else {
                format_ident!("{}", snake_to_camel_case(&format_name(&option.icon.description[0])))
                
            };

            if option.name == tag.default_option {
                tag_default = option_name.clone();
            }

            tag_options.push(option_name.clone());
            tag_strings.push(option.name.clone());
        }

        let tag_def = quote!(
            #[derive(Debug, Clone)]
            pub enum #tag_type {
                #(#tag_options),*
            }

            impl #tag_type {
                pub fn json(&self) -> serde_json::Map<String, Value> {
                    let mut map = serde_json::Map::new();
                    let mut data = serde_json::Map::new();
                    data.insert("option".to_string(),
                    match self {
                        #(#tag_type::#tag_options => Value::String(#tag_strings.to_string())),*
                    });
                    data.insert("tag".to_string(), Value::String(#tag_name_unformated.to_string()));
                    data.insert("action".to_string(), Value::String(#unformated_action_name.to_string()));
                    data.insert("block".to_string(), Value::String(#unformated_action_name.to_string()));

                    map.insert("data".to_string(), Value::Object(data));
                    map.insert("id".to_string(), Value::String("bl_tag".to_string()));
                    map
                }
            }

            impl Default for #tag_type {
                fn default() -> Self {
                    Self::#tag_default
                }
            }
        );

        tag_defs.push(tag_def);
    }
    let tag_defs = quote!(#(#tag_defs)*);

    // Turns the subaction list into a field, if it exists.
    let subactions = if action.sub_action_blocks == vec!["if_entity", "if_var", "if_game"] {
        quote!(subaction: SelectEntity,)
    }
    else if action.sub_action_blocks == vec!["if_player", "if_var", "if_game"] {
        quote!(subaction: SelectPlayer,)
    }
    else if action.sub_action_blocks == vec!["if_player", "if_entity", "if_var", "if_game"] {
        quote!(subaction: AllSubactions,)
    }
    else {
        quote!()
    };

    let enum_var = quote!(
        #action_name {
            #subactions
            #(#arg_types,)*
            #(#tag_types),*
        }
    );

    let subactions = if action.sub_action_blocks.len() > 0 {
        quote!(subaction,)
    }
    else {
        quote!()
    };

    // creates the compile function for the action.
    let block_name = quote::format_ident!("{}", snake_to_camel_case(&action.codeblock_name.to_ascii_lowercase().replace(" ", "_")));
    
    let subaction_compiler = if action.sub_action_blocks.len() > 0 {
        quote!(
            let mut subaction = subaction.compile();
            let value = subaction.as_object_mut().unwrap();
            value.insert("subaction".to_string(), value["action"].clone());
            value.insert("action".to_string(), serde_json::Value::String(#unformated_action_name.to_string()));
            drop(value);
            subaction
        )
    }
    else {
        quote!(serde_json::Value::Object(map))
    };

    let compile_function = quote!(
        #block_name::#action_name {#subactions #(#arg_names,)* #(#tag_names),*} => {
            let mut map = serde_json::Map::new();
            let mut item_args = compile(vec![#(#arg_names.json()),*], vec![#(#tag_names.json()),*]);

            let mut args = serde_json::Map::new();
            args.insert("items".to_string(), serde_json::Value::Array(item_args));

            map.insert("action".to_string(), serde_json::Value::String(#unformated_action_name.to_string()));
            map.insert("args".to_string(), serde_json::Value::Object(args));

            #subaction_compiler
        }
    );

    (enum_var, (compile_function, tag_defs))
}

struct TagSet {
    tags: Vec<Tag>,
    tags_by_name: HashMap<String, usize>,
}

impl TagSet {
    fn new() -> Self {
        Self {
            tags: Vec::new(),
            tags_by_name: HashMap::new(),
        }
    }
}

struct Tag {
    pub name: String,
    pub action_name: String,
    pub block_name: String,
    pub tags: HashSet<String>,
    pub name_style: TagStyle,
}

enum TagStyle {
    Unique,
    UniqueWithBlock,
    UniqueWithAction,
    SharedNonUnique,
}

fn remove_leading_nonalpha(name: &str) -> String {
    let mut output = String::new();
    let mut begining = true;
    for c in name.chars() {
        if c.is_ascii_alphabetic() && begining {
            begining = false;
        }
        if !begining {
            output.push(c);
        }
    }
    output
}

fn replace_numeric(name: &str) -> String {
    name.replace("1", "One")
        .replace("2", "Two")
        .replace("3", "Three")
        .replace("4", "Four")
        .replace("5", "Five")
        .replace("6", "Six")
        .replace("7", "Seven")
        .replace("8", "Eight")
        .replace("9", "Nine")
        .replace("0", "Zero")
}

fn format_name(name: &str) -> String {
    let name = strip_colour(name).replace(" ", "")
        .replace("=", "Eq")
        .replace("<", "LessThan")
        .replace(">", "GreaterThan")
        .replace(|c: char| {!c.is_ascii_alphanumeric() && c != '_'}, "");

    name
}

fn arg_type_to_rust(arg_type: &str) -> token_stream::TokenStream {
    match arg_type {
        "NUMBER" => quote!(Number),
        "TEXT" => quote!(Text),
        "COMPONENT" => quote!(MiniMessage),
        "LOCATION" => quote!(Location),
        "ITEM" => quote!(Item),
        "PARTICLE" => quote!(Particle),
        "VECTOR" => quote!(Vector),
        "SOUND" => quote!(Sound),
        "BLOCK" => quote!(Block),
        "BLOCK_TAG" => quote!(BlockTag),
        "PROJECTILE" => quote!(Projectile),
        "POTION" => quote!(Potion),
        "SPAWN_EGG" => quote!(SpawnEgg),
        "ENTITY_TYPE" => quote!(EntityType),
        "VARIABLE" => quote!(Variable),
        "ANY_TYPE" => quote!(AnyType),
        "DICT" => quote!(Dict),
        "LIST" => quote!(List),
        "VEHICLE" => quote!(Vehicle),
        arg => todo!("arg type: {}", arg)
    }
}

fn strip_colour(s: &str) -> String {
    let mut output = String::new();
    let mut in_colour = false;
    for c in s.chars() {
        if c == '§' {
            in_colour = true;
        }
        else if in_colour {
            in_colour = false;
        }
        else {
            output.push(c);
        }
    }
    output
}

fn snake_to_camel_case(s: &str) -> String {
    let mut output = String::new();
    let mut capitalize = true;
    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        }
        else if capitalize && c.is_ascii_alphabetic() {
            output.push(c.to_ascii_uppercase());
            capitalize = false;
        }
        else {
            if c == ' ' {
                capitalize = true;
            }
            output.push(c);
        }
    }
    output
}