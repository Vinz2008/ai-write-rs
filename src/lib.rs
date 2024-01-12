use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, Lit};
use openai_api_rust::*;
use openai_api_rust::chat::*;

enum Randomness {
    NotRandom = 0,
    Random = 1,
    VeryRandom = 2
}

const START_PROMPT : &'static str = "You are an assistant that only respond with rust code. Do not write normal text";

const START_PROMPT_EXPR : &'static str = "You are an assistant that only respond with rust code. Do not write normal text and don't write functions, assume you are writing code that will be in a function.";

fn get_from_ai(prompt : String, start_prompt : &'static str, randomness : Randomness) -> String {
    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let content = format!("{}.\n {}", start_prompt, prompt);
    let body = ChatBody {
        model: "gpt-3.5-turbo".to_string(),
        max_tokens: Some(15),
        temperature: Some(randomness as i32 as f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message { role: Role::User, content: content.clone() }],
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();
    message.content.clone()
}

fn from_expr_to_string(expr : Expr) -> Result<String, String> {
    match expr {
        Expr::Lit(l) => {
            match l.lit {
                Lit::Str(lit_str) => {
                    Ok(lit_str.value())
                },
                _ => { 
                    Err("expected string in ai_write macro".to_string())
                }
            }
        },
        _ => { 
            Err("expected string in ai_write macro".to_string())
        }
    }
}

#[proc_macro]
pub fn ai_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let arg = from_expr_to_string(input);
    let content = get_from_ai(arg.unwrap(), START_PROMPT, Randomness::NotRandom);
    let return_val = content.as_str().parse();
    match return_val {
        Result::Ok(v) => { 
            v
        }
        Result::Err(e) => { 
            println!("Lexing Error : {}", e); 
            TokenStream::new()
        }
    }
}

#[proc_macro]
pub fn ai_write_expr(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Expr);
    let arg = from_expr_to_string(input);
    let content = get_from_ai(arg.unwrap(), START_PROMPT_EXPR, Randomness::NotRandom);
    let return_val = content.as_str().parse();
    match return_val {
        Result::Ok(v) => { 
            v
        }
        Result::Err(e) => { 
            println!("Lexing Error : {}", e); 
            TokenStream::new()
        }
    }
}