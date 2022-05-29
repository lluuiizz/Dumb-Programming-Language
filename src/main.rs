use std::env;
use std::fs;
use std::process;

            //  Definitions of tokens recognizables by the tokenizer 
const KEYWORDS: &'static [&'static str] =
    &["print", "dup", "swap", "over", "drop", "->", "loop", "end"];
const OPERATIONS: &'static [&'static str] = &["+", "-", "*", "/", "%"];
const TYPES: &'static [&'static str] = &["int"];
            //  Defining the int variables who goes to the stack
struct Declaration {
    identifier: String,
    value: i64,
}

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
    let mut tokens: Vec<String> = Vec::new();       //  Vectors who stores the tokens given to the tokenizer

    tokenizer(&contents, &mut tokens);

    let mut stack: Vec<i64> = Vec::new();
    let mut declarations: Vec<Declaration> = Vec::new();
    interpreter(&mut stack, &tokens, &mut declarations);
}

fn tokenizer(contents: &String, tokens_vector: &mut Vec<String>){       //  The only thing that this function does is check if the token given is a separator and the anterior token is not, case true saves 
    let mut last_token: usize = 0;                                      //  a new token in the vector, case false the loop continues...
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

fn is_separator(ch: char) -> bool {         //  Checks if a char is a separator
    match ch {
        ' ' | ';' | '\n' | '\t' | '\\' | '\x7f' | '\r' | '\0' => true,
        _ => false,
    }
}

fn interpreter(stack: &mut Vec<i64>, tokens: &Vec<String>, declarations: &mut Vec<Declaration>) {       //  This function basically will certify that the token is a valid one, then will call a function who 
    let mut token_slice_init: usize = 0;                                                                //  does what the token is intend for, and then repeat until reaches the last token. If a token is not
    let mut token_slice_finish: usize;                                                                  //  a valid one, then the loop finishes and the program gives an error message!

    enum States {           //  States is how the interpreter knows what he is doing in the moment
        LOOP,               //  When he is in a loop state for example, the interpreter will search the token who
        ASSIGNMENT,         //  Ends the LOOP, then the interpreter pass a vector slice containing the tokens
        NOTHING,            //  that were inside the loop to the function "loop_instructions", who will recursively calls interpreter again
                            //  but only passing the new vector slice as arguments, when the loop
                            //  ends, the main interpreter session returns where he stopped.
    }                       //  Obs: To make nested loops possible the interpreter, when the state is an LOOP one, will search the last END keyword
                            //  and then make a vector slice from there, in the vector slice will
                            //  contain another loop, who when the interpreter be executed again,
                            //  will do the same thing

    let mut search_state = States::NOTHING as u8;

    for (i, token) in tokens.iter().enumerate() {
        if token.chars().all(char::is_numeric) && search_state == States::NOTHING as u8 {
            stack.push(token.parse().unwrap());
        } else {
            let find_in_keywords = KEYWORDS.iter().find(|&x| x == token);
            let find_in_keywords = match find_in_keywords {
                Option::Some(result) => result,
                Option::None => "Not Found",
            };

            if find_in_keywords != "Not Found" {
                if search_state == States::NOTHING as u8 {
                    if find_in_keywords == "print" {
                        print_stack(stack);
                    } else if find_in_keywords == "dup" {
                        duplicate(stack);
                    } else if find_in_keywords == "swap" {
                        swap(stack);
                    } else if find_in_keywords == "over" {
                        over(stack);
                    } else if find_in_keywords == "drop" {
                        drop_top(stack);
                    } else if find_in_keywords == "loop" {
                        token_slice_init = i + 1;
                        search_state = States::LOOP as u8;
                    } else if find_in_keywords == "->" {
                        token_slice_init = i + 1;
                        search_state = States::ASSIGNMENT as u8;
                    }
                } else {
                    if find_in_keywords == "end" {
                        token_slice_finish = i;
                        if search_state == States::LOOP as u8 {
                            let slice = &tokens[token_slice_finish..];
                            let mut prev_indx_end_was_encountered = 0;
                            for (j, new_token) in slice.iter().enumerate() {
                                if new_token == "end" {
                                    token_slice_finish += j - prev_indx_end_was_encountered;
                                    prev_indx_end_was_encountered = j;
                                }
                            }
                            loop_instructions(
                                stack,
                                &tokens[token_slice_init..token_slice_finish],
                                declarations,
                            );
                        } else if search_state == States::ASSIGNMENT as u8 {
                            assignment(
                                stack,
                                declarations,
                                &tokens[token_slice_init..token_slice_finish],
                            );
                        }
                        search_state = States::NOTHING as u8;
                    };
                }
                continue;
            }
            if search_state == States::NOTHING as u8 {
                let find_in_operations = OPERATIONS.iter().find(|&x| x == token);
                let find_in_operations = match find_in_operations {
                    Option::Some(result) => result,
                    Option::None => "Not Found",
                };

                if find_in_operations != "Not Found" {
                    if find_in_operations == "+" {
                        sum(stack);
                    } else if find_in_operations == "-" {
                        sub(stack);
                    } else if find_in_operations == "*" {
                        multy(stack);
                    } else if find_in_operations == "/" {
                        div(stack);
                    } else if find_in_operations == "%" {
                        rdiv(stack);
                    };
                    continue;
                }
                let mut encountered_in_declaration: bool = false;
                for declaration in declarations.iter() {
                    if *token == declaration.identifier {
                        stack.push(declaration.value);
                        encountered_in_declaration = true;
                        break;
                    }
                }
                if !encountered_in_declaration {
                    println!("Token unknown \"{}\". Exiting with error!", token);
                    process::exit(-1);
                }
            }
        }
    }
}
fn print_stack(stack: &mut Vec<i64>) {  //  Print the element in the top of the stack
    let len = stack.len();
    if len < 1 {
        println!("ERROR: STACK EMPTY!");
        process::exit(-1);
    }

    println!("{}", stack[len - 1]);
    stack.pop().unwrap();
}

fn duplicate(stack: &mut Vec<i64>) {    //  Duplicate the element in the top of the stack
    let len = stack.len();

    if len < 1 {
        println!("Insufficient number of elements in stack, got 0 need 1");
        process::exit(-1);
    }

    stack.push(stack[len - 1]);
}

fn swap(stack: &mut Vec<i64>) {         //  Swap the two first elements of the stack
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

fn over(stack: &mut Vec<i64>) {         //  Copy the element bellow the top of the stack to the top
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

fn drop_top(stack: &mut Vec<i64>) {     //  Drop the element in the top of the stack
    let len = stack.len();

    if len < 1 {
        println!("Insufficient number of elements in stack, got 0 need 1");
        process::exit(-1);
    }

    stack.pop().unwrap();
}

fn sum(stack: &mut Vec<i64>) {          //  Sum the first two elements in the top of the stack and drop them, then puts the result in the top 
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

fn sub(stack: &mut Vec<i64>) {           //  Subtract the first two elements in the top of the stack and drop them, then puts the result in the top 

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

fn multy(stack: &mut Vec<i64>) {           //  Multiply the first two elements in the top of the stack and drop them, then puts the result in the top 

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

fn div(stack: &mut Vec<i64>) {           //  Divide the first two elements in the top of the stack and drop them, then puts the result in the top 

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

fn rdiv(stack: &mut Vec<i64>) {            //  Takes the rest of division of the first two elements in the top of the stack and drop them, then puts the result in the top 
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

fn assignment(stack: &mut Vec<i64>, declarations: &mut Vec<Declaration>, token_slice: &[String]) {      //  Assign the value in the top of the stack to a variable
    if token_slice.len() < 2 {
        println!("Too few arguments to variable assignment!");
        process::exit(-1);
    } else if token_slice.len() > 2 {
        println!("Too much arguments to variable assignment!");
        process::exit(-1);
    }
    let valid_type: bool;
    let result = TYPES.iter().find(|x| x == &&token_slice[1].as_str());
    match result {
        Some(_p) => valid_type = true,
        None => valid_type = false,
    };

    if !valid_type {
        println!("Invalid type!");
        process::exit(-1);
    }

    if stack.len() < 1 {
        println!("No elements in the stack for assignment");
        process::exit(-1);
    }
    let check_if_identifier_is_valid = token_slice[0].chars().all(char::is_alphanumeric);

    if !check_if_identifier_is_valid {
        println!("Identifier {} is not a valid one", token_slice[0]);
        process::exit(-1);
    }

    let mut identifier_already_declared = false;

    for (indice, declaration) in declarations.iter().enumerate() {
        if declaration.identifier == token_slice[0] {
            identifier_already_declared = true;
            declarations[indice].value = stack[stack.len() - 1];
            break;
        }
    }

    if !identifier_already_declared {
        let new_declaration = Declaration {
            identifier: token_slice[0].to_string(),
            value: stack[stack.len() - 1],
        };
        declarations.push(new_declaration);
    }

    stack.pop();
}

fn loop_instructions(       //  Loop x number of instructions n times given by the top of the stack
    stack: &mut Vec<i64>,
    instructions: &[String],
    declarations: &mut Vec<Declaration>,
) {
    let len = stack.len();

    if len < 1 {
        println!("No arguments provided to loop in the stack, need 1, received 0");
        process::exit(-1);
    }

    let times_to_loop = stack[len - 1];
    let mut counter = 0;
    stack.pop();

    while counter < times_to_loop {
        interpreter(stack, &instructions.to_vec(), declarations);
        counter += 1;
    }
}
