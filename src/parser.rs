use crate::{
    ast::{ASTNode, BlockStatement, CallExpression, FunctionStatement, Identifier, Program},
    lexer::{self, TokenType},
};

pub struct Parser {
    lexer: lexer::Lexer,
    curr_token: TokenType,
}

fn exit(message: &str) -> ! {
    println!("{}", message);
    std::process::exit(1)
}

impl Parser {
    pub fn new(src: &str) -> Parser {
        let mut lexer = lexer::Lexer::new(src);
        let curr_token = lexer.next_token();
        Parser { lexer, curr_token }
    }

    pub fn parse(&mut self) -> ASTNode {
        self.program()
    }

    fn lookahead(&mut self, distance: usize) -> TokenType {
        match distance {
            0 => self.curr_token.clone(),
            _ => self.lexer.lookahead(distance),
        }
    }

    fn advance_token(&mut self) {
        self.curr_token = self.lexer.next_token();
    }

    fn advance_token_till(&mut self, token: TokenType) {
        self.advance_token();
        loop {
            if self.curr_token == token {
                break;
            }
            self.advance_token();
        }
    }

    fn program(&mut self) -> ASTNode {
        let start = self.lexer.cursor.line_num;
        let statement_list = self.statement_list();
        ASTNode::Program(Program {
            lines: Box::new(statement_list),
            start,
            end: self.lexer.cursor.line_num,
        })
    }

    fn eat(&mut self, expected_token: &TokenType) -> TokenType {
        if self.curr_token == TokenType::EOF {
            exit(format!("EOF").as_str());
        }

        if expected_token != &self.curr_token {
            exit(format!("unexpected token '{}'", self.curr_token).as_str());
        }

        let previous_token = self.curr_token.clone();
        self.advance_token();
        previous_token
    }

    fn eat_identifier(&mut self) -> String {
        let curr_token = self.curr_token.clone();
        match curr_token {
            TokenType::Identifier(ident) => {
                self.advance_token();
                ident
            }
            _ => exit(
                format!(
                    "unexpected token '{}', expected an identifier",
                    self.curr_token
                )
                .as_str(),
            ),
        }
    }

    fn statement_list(&mut self) -> Vec<ASTNode> {
        let mut statements = vec![];

        while self.curr_token != TokenType::EOF {
            if let Some(statement) = self.statement() {
                statements.push(statement);
            }
        }

        statements
    }

    fn statement(&mut self) -> Option<ASTNode> {
        match &self.curr_token {
            TokenType::OpenBraces => Some(self.block_statement()),
            TokenType::Identifier(ident) => {
                match ident.as_str() {
                    "function" => Some(self.function_expression()),
                    "const" | "var" | "let" => self.variable_statement(),
                    // TODO: if const|var|let check if arrow function
                    "if" => {
                        self.advance_token();
                        None
                    }
                    _ => self.parse_identifier(ident.to_string()),
                }
            }
            _ => {
                self.advance_token();
                None
            }
        }
    }

    fn variable_statement(&mut self) -> Option<ASTNode> {
        let start = self.lexer.cursor.line_num;
        self.advance_token();
        let var_name = self.eat_identifier();
        self.eat(&TokenType::Equals);

        match &self.curr_token {
            TokenType::OpenParen => {
                self.advance_token_till(TokenType::OpenBraces);
                let body = self.block_statement();

                Some(ASTNode::FunctionStatement(FunctionStatement {
                    name: var_name,
                    body: Box::new(body),
                    start,
                    end: self.lexer.cursor.line_num,
                }))
            }
            TokenType::OpenBraces => {
                //TODO: object declaration...
                None
            }
            TokenType::Identifier(ident) => self.parse_identifier(ident.to_string()),
            _ => None,
        }
    }

    fn call_expression(&mut self, base: ASTNode) -> ASTNode {
        let start = self.lexer.cursor.line_num;
        self.advance_token_till(TokenType::CloseParen);
        self.eat(&TokenType::CloseParen);

        let call_expression = ASTNode::CallExpression(CallExpression {
            base: Box::new(base),
            start,
            end: self.lexer.cursor.line_num,
        });

        if self.curr_token == TokenType::Dot {
            //return self.member_expression(call_expression);
        }

        call_expression
    }

    fn parse_identifier(&mut self, ident: String) -> Option<ASTNode> {
        match self.lookahead(1) {
            TokenType::OpenParen => {
                let start = self.lexer.cursor.line_num;
                self.advance_token();
                Some(self.call_expression(ASTNode::Identifier(Identifier {
                    name: ident,
                    start,
                    end: self.lexer.cursor.line_num,
                })))
            }
            _ => {
                self.advance_token();
                None
            }
        }
    }

    fn block_body(&mut self) -> Vec<ASTNode> {
        let mut statements = vec![];
        while self.curr_token != TokenType::CloseBraces {
            if let Some(statement) = self.statement() {
                statements.push(statement);
            }
        }

        statements
    }

    fn block_statement(&mut self) -> ASTNode {
        let start = self.lexer.cursor.line_num;
        self.eat(&TokenType::OpenBraces);
        let body = self.block_body();
        self.eat(&TokenType::CloseBraces);

        ASTNode::BlockStatement(BlockStatement {
            body: Box::new(body),
            start,
            end: self.lexer.cursor.line_num,
        })
    }

    fn function_expression(&mut self) -> ASTNode {
        let start = self.lexer.cursor.line_num;
        self.advance_token();
        let name = self.eat_identifier();
        self.advance_token_till(TokenType::OpenBraces);
        let body = self.block_statement();

        ASTNode::FunctionStatement(FunctionStatement {
            name,
            body: Box::new(body),
            start,
            end: self.lexer.cursor.line_num,
        })
    }
}
