use std::{io::prelude::*, path::PathBuf, fs::File, collections::hash_map, ops::RangeBounds};
use proc_macro2::token_stream;
use quote::quote;
use prettier_please;
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

    for (i, action) in actions.iter().enumerate() {
        // Skips call function and call process actions.
        // TODO: See if there's a way to parse these.
        if action.as_object().unwrap().get("name").unwrap().as_str().unwrap() == "dynamic" {
            continue;
        }
        let action = serde_json::from_value::<Action>(action.clone()).expect(&format!("Failed to parse action #{}!", i));
        let action_block = action.codeblock_name.to_ascii_lowercase().replace(" ", "_");
        let action = gen_action(action);

        module_code.get_mut(&action_block).unwrap().push(action);
    }

    let mut module_path: PathBuf = module_path.into();
    for (module_name, module_code) in module_code {
        module_path.push(format!("{}.rs", module_name));
        eprintln!("Writing module {} to {}...", module_name, module_path.to_str().unwrap());
        let mut file = File::create(&module_path).unwrap();

        let enum_name = quote::format_ident!("{}", module_name);
        let module_code = quote!(
            pub enum #enum_name {
                #(#module_code)*
            }
        );

        // let module_code = syn::parse2(module_code).unwrap();
        // let module_code = prettier_please::unparse(&module_code);

        file.write_all(module_code.to_string().as_bytes()).unwrap();

        module_path.pop();
    }
}

/// Generates a single enum variant for a given action object.
fn gen_action(action: Action) -> token_stream::TokenStream {
    let mut action_name = match &action.icon {
        ActionIconOptions::Icon(icon) => icon.name.replace(|c: char| {!c.is_ascii_alphabetic()}, ""),
        ActionIconOptions::Argless(argless) => argless.name.replace(|c: char| {!c.is_ascii_alphabetic()}, "")
    };
    if action_name == "" {
        action_name = action.name.replace(|c: char| {!c.is_ascii_alphabetic()}, "");
    }
    let action_name = quote::format_ident!("{}", action_name);

    let mut arg_types = Vec::new();
    let args = match action.icon {
        ActionIconOptions::Icon(icon) => icon.arguments,
        ActionIconOptions::Argless(_) => Vec::new(),
    };

    let mut i = 0;
    let len = args.len();
    while i < len {
        if let ActionArgOptions::Arg(arg ) = &args[i] {
            let mut outer_arg = arg.clone();
            let mut output = arg_type_to_rust(outer_arg.arg_type.clone());
            if len > i+2 {
                if let ActionArgOptions::Text{ text } = &args[i+1] {
                    if text == "OR" {
                        if let ActionArgOptions::Arg(arg) = &args[i+2] {
                            if arg.arg_type == "NONE" {
                                outer_arg.optional = true;
                            }
                            else {
                                let inner_arg_type = arg_type_to_rust(arg.arg_type.clone());
                                output = quote!(Either<#output, #inner_arg_type>);
                            }
                            i += 2;
                        }
                    }
                }
            }

            if outer_arg.plural {
                output = quote!(Vec<#output>);
            }
            if outer_arg.optional {
                output = quote!(Option<#output>);
            }

            let arg_name = quote::format_ident!("{}", outer_arg.description[0].replace(" ", "_").replace(|c: char| {!c.is_ascii_alphanumeric() && c != '_'}, "").to_lowercase());
            arg_types.push(quote!(
                #arg_name: #output
            ));
        }
        eprintln!("arg: {:?}", args[i]);
        i += 1;
    }
    eprintln!("args: {:?}", arg_types);
    let enum_var = quote!(
        #action_name {
            #(#arg_types),*
        },
    );

    enum_var
}

fn arg_type_to_rust(arg_type: String) -> token_stream::TokenStream {
    quote!(usize) // TODO: Implement this
}