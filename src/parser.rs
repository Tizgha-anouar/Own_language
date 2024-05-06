
use crate::tokenize::{TOKEN, TokenType};

#[derive(Debug)]
pub enum Expr
{
Assignment(AssigImpl),
Operation(OperaImpl),
Number(NumbImpl),
Variable(VarImpl),
FunCall(FunImpl),
Test,
}
#[derive(Debug)]
pub struct AssigImpl
{
    pub target:VarImpl,
    pub value:Box<Expr>,
}
#[derive(Debug)]
pub struct OperaImpl
{
    pub lhs: Box<Expr>,
    pub operator:TOKEN,
    pub rhs:Box<Expr>
}
#[derive(Debug)]
pub struct NumbImpl
{
    pub value: i32,
    pub token:TOKEN //think about it ?????
}
#[derive(Debug)]
pub struct VarImpl
{
    pub token:TOKEN
}
#[derive(Debug)]
pub struct FunImpl
{
    pub name:VarImpl,
    pub arg:Box<Expr>
}

pub fn expect(expected:TokenType,tokens :&mut Vec<TOKEN>  ){

    match tokens.pop() 
    {
        None   => {},
        Some( value ) =>{ if expected != value.token_type {panic!("Expected new line", );}}
    }
}

pub fn parser(mut tokens:Vec<TOKEN>) -> Vec<Expr>{

    tokens.reverse();
    let mut result  = Vec::<Expr>::new();
    while tokens.len() > 0 {
          
            let curr = &tokens[tokens.len() - 1];
            if curr.token_type == TokenType::NewLine {
                tokens.pop();
                continue;
            }


        let expr = parse_expr(&mut tokens);
        expect(TokenType::NewLine, &mut tokens); //delineating
        result.push(expr);

    }
    return result;
}

pub fn parse_expr( tokens :&mut Vec<TOKEN> )-> Expr{

return parse_assignment(tokens);
}

pub fn parse_assignment( tokens :&mut Vec<TOKEN> ) -> Expr{
    /*
        a = 2 + 1
    */
    if tokens.len() > 1 && TokenType::Equal == tokens[tokens.len() - 2 ].token_type {
        let variable = parse_variable(tokens);
        expect(TokenType::Equal, tokens);
        let value = parse_expr(tokens);

        return Expr::Assignment(AssigImpl{
            //target:variable,
            target:variable,
            value:Box::new(value)
        })

    }else{
         
        return parse_term(tokens);
    }
}

pub fn parse_variable( tokens :&mut Vec<TOKEN> ) -> VarImpl{
    let token =  tokens.pop().unwrap();
    return VarImpl{token:token};
}



pub fn parse_term( tokens: &mut Vec<TOKEN> ) -> Expr {
  
    //term : factor ( ("+" | "-") factor )*
    let mut result = parse_factor(tokens);
   
    while tokens.len() > 0 {
        let next_token = &tokens[tokens.len() - 1];

        match next_token.token_type {
            TokenType::Plus | TokenType::Minus => {
                let operator:TOKEN = tokens.pop().unwrap();

                let rhs = parse_factor(tokens);
                result = Expr::Operation(OperaImpl{
                        lhs: Box::new(result),
                        operator:operator,
                        rhs:Box::new(rhs),
                });
            },
            _=>{break;}
        }
    }

    return result;
}
pub fn parse_factor(tokens: &mut Vec<TOKEN> )-> Expr{
    
    let mut result = parse_primary(tokens);
    while tokens.len() > 0 {
        let next_token =  &tokens[tokens.len() -1];
        match next_token.token_type {
            TokenType::Slash | TokenType::Star | TokenType::Exponentiation | TokenType::Mod => {
                let operator = tokens.pop().unwrap();
                let rhs = parse_primary(tokens);

                result = Expr::Operation(OperaImpl { lhs: Box::new(result), operator: operator, rhs: Box::new(rhs) });
            },
            _=>{break;}
        }
    }
    
    return result;
}

pub fn parse_primary(tokens : &mut Vec<TOKEN>)-> Expr{
    let token  = tokens.pop().unwrap();
    match token.token_type 
    {
        TokenType::NumberLiteral =>{
            return Expr::Number(NumbImpl { value: parse_number(&token.lexene), token: token })
        },
        TokenType::Identifier =>{
            if tokens.len() > 0 {
                let next_token = &tokens[tokens.len() -1];
                if next_token.token_type == TokenType::LeftParan {
                    let fun_name = VarImpl{token};
                    tokens.pop().unwrap(); //left paran
                    let arg = parse_expr(tokens);
                    expect(TokenType::RightParan, tokens);
                    return Expr::FunCall(FunImpl { name: fun_name, arg: Box::new(arg) });
                }
            }
            return Expr::Variable(VarImpl { token: token });
        },
        TokenType::LeftParan=>{
            
            if tokens.len() > 0 {
                    let result = parse_expr(tokens);
                    expect(TokenType::RightParan, tokens);
                    return result;
            }
            panic!("not completed Expression");
        },
        TokenType::NewLine=>{
            return parse_expr(tokens);
        },

        err=>{
            println!("error {:?}", err);
            return Expr::Test},
    }
}



pub fn parse_number( value :&str )-> i32{
return value.parse::<i32>().unwrap();
}