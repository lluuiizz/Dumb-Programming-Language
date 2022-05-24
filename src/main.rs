use std::env;
use std::fs;
use std::process;

const KEYWORDS: &'static [&'static str] = &["print", "dup", "swap", "over", "drop", "loop", "end"];
const OPERATIONS: &'static [&'static str] = &["+", "-", "*", "/", "%"];

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Missing argument!!!");
        return;
    }
    let file = String::from(&args[1]);
    let mut format_return: bool = false;

    for (i, ch) in file.chars().enumerate() {
        if ch == '.' {
            let format = &file[i..];
            if format == ".dumb" {
                format_return = true;
                break;
            }
        }
    }

    if !format_return {
        println!("Invalid file format, please insert an '.dumb' file");
        return;
    }

    let contents = fs::read_to_string(file).expect("Error at reading file");
    let mut tokens: Vec<String> = Vec::new();

    tokenizer(&contents, &mut tokens);

    let mut stack: Vec<i64> = Vec::new();

    interpreter(&mut stack, &tokens);
}

fn tokenizer(contents: &String, tokens_vector: &mut Vec<String>) {
    let mut last_token: usize = 0;
    let contents_as_bytes = contents.as_bytes();

    for (i, ch) in contents.chars().enumerate() {
        if i != 0 {
            let can_insert_new_token =
                is_separator(ch) && !is_separator(contents_as_bytes[i - 1] as char);
            let can_not_insert_new_token =
                is_separator(ch) && is_separator(contents_as_bytes[i - 1] as char);

            if can_insert_new_token {
                tokens_vector.push(contents[last_token..i].to_string());
                last_token = i + 1;
            } else if can_not_insert_new_token {
                last_token = i + 1;
            }
        }
    }
}

fn is_separator(ch: char) -> bool {
    match ch {
        ' ' | ';' | '\n' | '\t' | '\\' | '\x7f' | '\r' | '\0' => true,
        _ => false,
    }
}

fn interpreter(stack: &mut Vec<i64>, tokens: &Vec<String>) {
    let mut token_slice_init: usize = 0;
    let mut token_slice_finish: usize;

    const LOOP: u8 = 0;
    const NOTHING: u8 = 1;

    let mut search_state = NOTHING;

    for (i, token) in tokens.iter().enumerate() {
        if token.chars().all(char::is_numeric) && search_state == NOTHING {
            stack.push(token.parse().unwrap());
        } else {
            for (_j, keyword) in KEYWORDS.iter().enumerate() {
                if token == keyword {
                    if search_state == NOTHING {
                        if keyword.to_string() == "print" {
                            print_stack(stack)
                        };
                        if keyword.to_string() == "dup" {
                            duplicate(stack)
                        };
                        if keyword.to_string() == "swap" {
                            swap(stack)
                        };
                        if keyword.to_string() == "over" {
                            over(stack)
                        };
                        if keyword.to_string() == "drop" {
                            drop_top(stack)
                        };
                        if keyword.to_string() == "loop" {
                            token_slice_init = i + 1;
                            search_state = LOOP;
                        }
                    }

                    if keyword.to_string() == "end" {
                        token_slice_finish = i;
                        if search_state == LOOP {
                            loop_instructions(stack, &tokens[token_slice_init..token_slice_finish]);
                        };
                        search_state = NOTHING;
                    };
                    break;
                }
            }
            for operation in OPERATIONS.iter() {
                if search_state == NOTHING {
                    if token == operation {
                        if operation.to_string() == "+" {
                            sum(stack)
                        };

                        if operation.to_string() == "-" {
                            sub(stack)
                        };
                        if operation.to_string() == "*" {
                            multy(stack)
                        };
                        if operation.to_string() == "/" {
                            div(stack)
                        };
                        if operation.to_string() == "%" {
                            rdiv(stack)
                        };
                        break;
                    }
                }
            }
        }
    }
}
fn print_stack(stack: &mut Vec<i64>) {
    let len = stack.len();
    if len < 1 {
        println!("ERROR: STACK EMPTY!");
        process::exit(-1);
    }

    println!("{}", stack[len - 1]);
    stack.pop().unwrap();
}

fn duplicate(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 1 {
        println!("Insufficient number of elements in stack, got 0 need 1");
        process::exit(-1);
    }

    stack.push(stack[len - 1]);
}

fn swap(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 2 {
        println!(
            "Insufficient number of elements in stack, got {} need 2",
            len
        );
        process::exit(-1);
    }
    let buff = stack[len - 1];
    stack[len - 1] = stack[len - 2];
    stack[len - 2] = buff;
}

fn over(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 2 {
        println!(
            "Insufficient number of elements in stack, got {} need 2",
            len
        );
        process::exit(-1);
    }

    stack.push(stack[len - 2]);
}

fn drop_top(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 1 {
        println!("Insufficient number of elements in stack, got 0 need 1");
        process::exit(-1);
    }

    stack.pop().unwrap();
}

fn sum(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 2 {
        println!(
            "Insufficient number of elements in stack, got {} need 2",
            len
        );
        process::exit(-1);
    }
    
    let (result, overflow): (i64, bool) = stack[len - 2].overflowing_add(stack[len - 1]);

    if overflow {
        println!("Sum exceeded the limits of i64!!!");
        process::exit(-1);
    }

    stack.pop();
    stack.pop();
    stack.push(result);
}

fn sub(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 2 {
        println!(
            "Insufficient number of elements in stack, got {} need 2",
            len
        );
        process::exit(-1);
    }

    let (result, overflow): (i64, bool) = stack[len - 2].overflowing_sub(stack[len - 1]);

    if overflow {
        println!("Subtract exceeded the limits of i64!!!");
        process::exit(-1);
    }



    stack.pop();
    stack.pop();
    stack.push(result);
}

fn multy(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 2 {
        println!(
            "Insufficient number of elements in stack, got {} need 2",
            len
        );
        process::exit(-1);
    }

    let (result, overflow): (i64, bool) = stack[len - 2].overflowing_mul(stack[len - 1]);

    if overflow {
        println!("Multiplication exceeded the limits of i64!!!");
        process::exit(-1);
    }


    stack.pop();
    stack.pop();
    stack.push(result);
}

fn div(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 2 {
        println!(
            "Insufficient number of elements in stack, got {} need 2",
            len
        );
        process::exit(-1);
    }

    let (result, overflow): (i64, bool) = stack[len - 2].overflowing_div(stack[len - 1]);

    if overflow {
        println!("Division exceeded the limits of i64!!!");
        process::exit(-1);
    }


    stack.pop();
    stack.pop();
    stack.push(result);
}

fn rdiv(stack: &mut Vec<i64>) {
    let len = stack.len();

    if len < 2 {
        println!(
            "Insufficient number of elements in stack, got {} need 2",
            len
        );
        process::exit(-1);
    }

    let result = stack[len - 2] % stack[len - 1];
    stack.pop();
    stack.pop();
    stack.push(result);
}

fn loop_instructions(stack: &mut Vec<i64>, instructions: &[String]) {
    let len = stack.len();

    if len < 1 {
        println!("No arguments provided to loop in the stack, need 1, received 0");
        process::exit(-1);
    }

    let times_to_loop = stack[len - 1];
    let mut counter = 0;
    stack.pop();

    while counter < times_to_loop {
        interpreter(stack, &instructions.to_vec());
        counter += 1;
    }
}
