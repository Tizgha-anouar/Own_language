
#[derive(Debug,PartialEq)]
pub enum TokenType { 
    NumberLiteral,// 1 2 3 4
    Identifier, // a b c name , age
    Equal, // =
    Plus, // +
    Minus,// - 
    Star,// *
    Slash,// /
    Exponentiation,// ^
    Mod, // %
    LeftParan,// (
    RightParan,// )
    NewLine,//\n => delineating statement
}
use TokenType::*;

/*
example : 123
TokenType : NumberLiteral
lexene : 123
*/
#[derive(Debug)]
pub struct TOKEN 
{
    pub token_type:TokenType,
    pub lexene:String,
}

pub fn get_tokenize(source_code : &str)->Vec::<TOKEN>{

    let mut position  = 0;
    let mut result  = Vec::<TOKEN>::new();
    while position < source_code.len() {
        let curr_char = source_code.chars().nth(position).unwrap();

        match curr_char {
            '(' => result.push(TOKEN{token_type:LeftParan,lexene:curr_char.to_string()}),
            ')' => result.push(TOKEN{token_type:RightParan,lexene:curr_char.to_string()}),
            '/' => {
                position +=1 ;
                let next = source_code.chars().nth(position).unwrap();
                if  next == '/' {
                    loop {
                        position += 1;
                        if source_code.chars().nth(position).unwrap() != '\n' {

                        }else{
                            position -= 1;
                            break;
                        }
                    }
                }else{
                    result.push(TOKEN{token_type:Slash,lexene:curr_char.to_string()});
                }
            },
            '*' => result.push(TOKEN{token_type:Star,lexene:curr_char.to_string()}),
            '%' => result.push(TOKEN{token_type:Mod,lexene:curr_char.to_string()}),
            '-' => result.push(TOKEN{token_type:Minus,lexene:curr_char.to_string()}),
            '+' => result.push(TOKEN{token_type:Plus,lexene:curr_char.to_string()}),
            '^' => result.push(TOKEN{token_type:Exponentiation,lexene:curr_char.to_string()}),
            '=' => result.push(TOKEN{token_type:Equal,lexene:curr_char.to_string()}),
            '\n' => result.push(TOKEN{token_type:NewLine,lexene:curr_char.to_string()}),
            x if x.is_digit(10)  =>{
                let mut number_lexen = x.to_string();
                position +=1;
                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();
                    if next_char == ' ' || 
                    next_char =='\n'|| 
                    next_char == ')'|| 
                    next_char == '*'|| 
                    next_char == '/'||
                    next_char == '+'||
                    next_char == '^'||
                    next_char == '%'||
                    next_char == '-' {
                        break;
                    }
                    if next_char.is_digit(10) {
                        number_lexen.push(next_char);
                        position +=1;
                        continue;
                    }else{
                        panic!("Invalid number {:?}", next_char);
                    }

                }

                result.push(TOKEN{token_type:NumberLiteral,lexene:number_lexen.to_string()});
                continue;

            },
            ' ' =>{},
            iden => {// indentifier
                let mut lexene  = iden.to_string();
                position += 1;
                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();
                    if !next_char.is_alphanumeric() && !(next_char == '_') {
                        break;
                    }

                    lexene.push(next_char);
                    position += 1;

                }
                result.push(TOKEN{token_type:Identifier,lexene:lexene.to_string()});
                continue;
            },
        }
        position += 1;
    }
    result
}
