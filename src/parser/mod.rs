use super::lexer::Lexer;
use super::lexer::token::Token;
use super::lexer::token::TokenType;

pub struct Parser {
    lexer: Lexer,
}

pub fn new(lexer: Lexer) -> Parser {
    Parser{
        lexer,
    }
}

impl Parser {
    pub fn eval(&mut self, mut db: super::db::Database) -> super::db::Database {
        match self.next_token().get_type() {
            TokenType::Put | TokenType::Update => {
                let prime_key = self.next_token().get_literal();
                let second_key = self.next_token().get_literal();
                let value = self.next_token().get_literal();

                db.put(prime_key, second_key, value);
            },
            TokenType::Get => {
                let prime_key = self.next_token().get_literal();
                let second_key = self.next_token().get_literal();
                
                let val = db.get(prime_key, second_key);

                println!("{}", val);
            },
            TokenType::Delete => {
                let prime_key = self.next_token().get_literal();
                let second_key = self.next_token().get_literal();
                
                
                db.delete(prime_key, second_key);
            },

            TokenType::Exit => {
                super::exit();
            }

            _ => println!("invalid command"),
        }

        db
    }

    fn next_token(&mut self) -> Token {
        self.lexer.next_token()
    }
}
