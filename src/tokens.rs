

#[derive(Debug)]
pub enum TokenType {
    SOF         = -1,
    NUMBER      = 0,
    PLUS        = 1,
    MINUS       = 2,
    MULTIPLY    = 3,
    DIVIDE      = 4,
    LPAREN      = 5,
    RPAREN      = 6

}

#[derive(Debug)]
pub struct Token {
    pub _type:TokenType,
    pub val:Option<isize>
}

impl Token {
    pub fn new(_ty: TokenType, va: Option<isize>) -> Token {
        Token {_type:_ty, val:va}
    }
}




// pub fn check_token_file() {
//     println!("Token is working!");
//     let token = Token{
//         _type:TokenType::NUMBER, 
//         val:10
//     };
//     println!("{:?}", token);
//     let s = "Jofin".to_string();
//     for i in s.split("") {
//         println!("{}", i);    
//     }
    

// }