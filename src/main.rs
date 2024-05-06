mod tokenize;
mod parser;
use tokenize::{get_tokenize, TokenType};
use parser::{parser, AssigImpl, Expr, FunImpl, NumbImpl, OperaImpl, VarImpl};
use std::collections::HashMap;
use std::fs;
use std::env;

type Env = HashMap<String, i32>; // aliase

fn interpreter( parser : &Vec<Expr>, env: &mut Env ){

    for parse in parser {
        check(parse, env);

    }
}

//expressions yield values, statement don't
fn check(parse : &Expr, env : &mut Env) -> i32{
    match parse {
        Expr::Number(NumbImpl{ value, ..})=>return *value,
        Expr::Operation(OperaImpl{
            lhs, 
            operator, 
            rhs})=>{
                let lhs_value = check(lhs,env);
                let rhs_value = check(rhs,env);
                match operator.token_type {
                    TokenType::Star =>return lhs_value * rhs_value,
                    TokenType::Slash =>return lhs_value / rhs_value,
                    TokenType::Minus =>return lhs_value - rhs_value,
                    TokenType::Mod =>return lhs_value % rhs_value,
                    TokenType::Plus =>return lhs_value + rhs_value,
                    TokenType::Exponentiation =>return i32::pow(lhs_value, rhs_value as u32),
                    _=>{panic!("this operator not valide")}
                }

        },
        Expr::Assignment(AssigImpl{target,value})=>{
            let result = check(value, env);

            env.insert(target.token.lexene.clone(),result);

            return result;
        },
        Expr::Variable(VarImpl{token})=>{
            if let Some(value) = env.get(&token.lexene){
                return *value;
            }else{
            panic!("this indentifier not found");
            }
        },
        Expr::FunCall(FunImpl{name,arg})=>{
            if name.token.lexene == "print" {   
                let value = check(arg, env);
                println!("{:?}",value);
                return value; // 0 or -1
            }else{
                panic!("this function not implement yet ..");
            }
        }
        _=>{panic!("sorry .....");},

    }

}


fn main(){
    let args : Vec<String>  = env::args().collect();

    if args.len() > 1 {
    let source = fs::read_to_string(&args[1])
        .expect("Should have been able to read the file");

    let tokens = get_tokenize(&source);
    
    // for token in tokens {
    //     println!("{:?}", token);
    // }
    let result = parser(tokens);
    // for expr in result{
    //     println!(" ________{:?}", expr);
    // }
    let mut env : Env = HashMap::new();

    interpreter(&result, &mut env);

    }else{
        println!("path not found");
    }
}