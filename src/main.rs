
#[derive(Debug)]
enum TokenType { 
    NumberLiteral,// 1 2 3 4
    Identifier, // a b c name , age
    Equal, // =
    Plus, // +
    Minus,// - 
    Star,// *
    Slash,// /
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
struct TOKEN 
{
    TokenType:TokenType,
    lexene:String,
}

fn get_tokenize(source_code : &str)->Vec::<TOKEN>{

    let mut position  = 0;
    let mut result  = Vec::<TOKEN>::new();
    while position < source_code.len() {
        let curr_char = source_code.chars().nth(position).unwrap();

        match curr_char {
            '(' => result.push(TOKEN{TokenType:LeftParan,lexene:curr_char.to_string()}),
            ')' => result.push(TOKEN{TokenType:RightParan,lexene:curr_char.to_string()}),
            '/' => result.push(TOKEN{TokenType:Slash,lexene:curr_char.to_string()}),
            '*' => result.push(TOKEN{TokenType:Star,lexene:curr_char.to_string()}),
            '-' => result.push(TOKEN{TokenType:Minus,lexene:curr_char.to_string()}),
            '+' => result.push(TOKEN{TokenType:Plus,lexene:curr_char.to_string()}),
            '=' => result.push(TOKEN{TokenType:Equal,lexene:curr_char.to_string()}),
            '\n' => result.push(TOKEN{TokenType:NewLine,lexene:curr_char.to_string()}),
            x if x.is_digit(10) =>{
                let mut number_lexen = x.to_string();
                position +=1;
                while position < source_code.len() {
                    let next_char = source_code.chars().nth(position).unwrap();
                    if next_char == ' ' || next_char == '\n' || next_char == ')' {
                        break;
                    }
                    if next_char.is_digit(10) {
                        number_lexen.push(next_char);
                        position +=1;
                        continue;
                    }else{
                        panic!("Invalid character {:?}", next_char);
                    }

                }

                result.push(TOKEN{TokenType:NumberLiteral,lexene:number_lexen.to_string()});
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
                result.push(TOKEN{TokenType:Identifier,lexene:lexene.to_string()});
                continue;
            }




        }
        position += 1;
    }
    result
}


fn main(){
    let source  = "a = 1 + 2
    print(a)
    ";
    let tokens = get_tokenize(source);
    for token in tokens{
    println!("{:?}",token);

    }
 
}