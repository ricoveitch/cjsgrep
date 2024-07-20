use crate::{
    ast::{
        self, ASTNode, BlockStatement, CallExpression, FunctionStatement, Identifier,
        MemberExpression, ObjectPattern, Program, VariableExpression,
    },
    lexer::{self, TokenType},
};

pub struct Parser {
    lexer: lexer::Lexer,
    curr_token: TokenType,
}

fn exit(message: &str) -> ! {
    eprintln!("{}", message);
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

    // fn lookahead(&mut self, distance: usize) -> TokenType {
    //     match distance {
    //         0 => self.curr_token.clone(),
    //         _ => self.lexer.lookahead(distance),
    //     }
    // }

    fn advance_token(&mut self) {
        self.curr_token = self.lexer.next_token();
    }

    fn advance_token_till(&mut self, pred: impl Fn(&TokenType) -> bool) {
        loop {
            if pred(&self.curr_token) {
                break;
            }
            self.advance_token();
        }
    }

    fn advance_token_against(&mut self, target: Vec<TokenType>) -> bool {
        for t in target {
            if &self.curr_token != &t {
                return false;
            }
            self.advance_token();
        }

        true
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

    fn eat_identifier(&mut self) -> Option<String> {
        let curr_token = self.curr_token.clone();
        match curr_token {
            TokenType::Identifier(ident) => {
                self.advance_token();
                Some(ident)
            }
            _ => None,
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
            TokenType::Identifier(ident) => match ident.as_str() {
                "function" => self.function_expression(),
                "const" | "var" | "let" => self.variable_statement(),
                "module" => self.export_statement(),
                "if" => {
                    self.advance_token();
                    None
                }
                _ => Some(self.parse_identifier(ident.to_string())),
            },
            TokenType::ForwardSlash => {
                self.parse_backslash();
                None
            }
            _ => {
                self.advance_token();
                None
            }
        }
    }

    fn export_statement(&mut self) -> Option<ASTNode> {
        if !self.advance_token_against(vec![
            TokenType::Identifier(String::from("module")),
            TokenType::Dot,
            TokenType::Identifier(String::from("exports")),
            TokenType::Equals,
        ]) {
            return None;
        }

        match &self.curr_token {
            TokenType::OpenBraces => {
                Some(ASTNode::ExportStatement(self.object_pattern_expression()))
            }
            _ => None,
        }
    }

    fn parse_backslash(&mut self) {
        self.advance_token();

        match &self.curr_token {
            TokenType::Asterisk => loop {
                self.advance_token_till(|t| t == &TokenType::Asterisk);
                self.advance_token();
                if self.curr_token == TokenType::ForwardSlash {
                    return;
                }
            },
            TokenType::ForwardSlash => {
                self.advance_token_till(|t| match t {
                    TokenType::Newline | TokenType::EOF => true,
                    _ => false,
                });
            }
            _ => (),
        };
    }

    fn object_pattern_expression(&mut self) -> ObjectPattern {
        let obj_pat_start = self.lexer.cursor.line_num;
        let mut properties = vec![];
        self.eat(&TokenType::OpenBraces);

        loop {
            if &self.curr_token == &TokenType::Newline {
                self.advance_token();
            }

            if &self.curr_token == &TokenType::CloseBraces {
                self.advance_token();
                break;
            }

            let key = match self.eat_identifier() {
                Some(ident) => ident,
                None => break,
            };

            let mut value = key.clone();

            if &self.curr_token == &TokenType::Colon {
                self.advance_token();
                if let TokenType::Identifier(ident) = &self.curr_token {
                    value = ident.clone();
                    self.advance_token();
                };
            }

            properties.push(ast::Property { key, value });

            if &self.curr_token == &TokenType::Comma {
                self.advance_token();
            }
        }

        ObjectPattern {
            properties,
            start: obj_pat_start,
            end: self.lexer.cursor.line_num,
        }
    }

    fn arrow_function_statement(&mut self, name: &str) -> ASTNode {
        let start = self.lexer.cursor.line_num;
        self.advance_token_till(|t| t == &TokenType::OpenBraces);
        let body = self.block_statement();

        ASTNode::FunctionStatement(FunctionStatement {
            name: String::from(name),
            body: Box::new(body),
            start,
            end: self.lexer.cursor.line_num,
        })
    }

    fn variable_statement(&mut self) -> Option<ASTNode> {
        let start = self.lexer.cursor.line_num;
        self.advance_token();

        let lhs = match &self.curr_token {
            TokenType::Identifier(ident) => self.parse_identifier(ident.clone()),
            TokenType::OpenBraces => ASTNode::ObjectPattern(self.object_pattern_expression()),
            _ => return None,
        };

        self.eat(&TokenType::Equals);

        match &self.curr_token {
            TokenType::OpenParen => match &lhs {
                ASTNode::Identifier(ident) => Some(self.arrow_function_statement(&ident.name)),
                _ => None,
            },
            TokenType::OpenBraces => {
                //TODO: object declaration...
                None
            }
            TokenType::Identifier(ident) => {
                let rhs = self.parse_identifier(ident.to_string());
                Some(ASTNode::VariableExpression(VariableExpression {
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                    start,
                    end: self.lexer.cursor.line_num,
                }))
            }
            _ => None,
        }
    }

    fn call_expression(&mut self, base: ASTNode) -> ASTNode {
        let start = base.get_start().to_owned();
        self.eat(&TokenType::OpenParen);

        // only care about the first parameter if its a string, for require().
        let param = match &self.curr_token {
            TokenType::String(s) => Some(s.to_owned()),
            _ => None,
        };

        self.advance_token_till(|t| t == &TokenType::CloseParen);
        self.eat(&TokenType::CloseParen);

        let call_expression = ASTNode::CallExpression(CallExpression {
            base: Box::new(base),
            param,
            start,
            end: self.lexer.cursor.line_num,
        });

        call_expression
    }

    fn parse_identifier(&mut self, ident: String) -> ASTNode {
        let start = self.lexer.cursor.line_num;
        let ident_node = ASTNode::Identifier(Identifier {
            name: ident,
            start,
            end: self.lexer.cursor.line_num,
        });
        self.advance_token();

        match &self.curr_token {
            TokenType::OpenParen => self.call_expression(ident_node),
            TokenType::Dot => self.member_expression(ident_node),
            _ => ident_node,
        }
    }

    fn member_expression(&mut self, base: ASTNode) -> ASTNode {
        let mut base = base;
        loop {
            let (new_base, more) = self.member_prefix_expression(base);
            base = new_base;
            if !more {
                break;
            }
        }

        base
    }

    fn member_prefix_expression(&mut self, base: ASTNode) -> (ASTNode, bool) {
        let expression = match &self.curr_token {
            &TokenType::Dot => {
                self.eat(&TokenType::Dot);
                let property = match self.eat_identifier() {
                    Some(ident) => ident,
                    None => return (base, false),
                };
                let me = MemberExpression {
                    base: Box::new(base),
                    property,
                    start: 0,
                    end: 0,
                };

                ASTNode::MemberExpression(me)
            }
            &TokenType::OpenParen => self.call_expression(base),
            _ => return (base, false),
        };

        (expression, true)
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

    fn function_expression(&mut self) -> Option<ASTNode> {
        let start = self.lexer.cursor.line_num;
        self.advance_token();
        let name = match self.eat_identifier() {
            Some(ident) => ident,
            None => return None,
        };
        self.advance_token_till(|t| t == &TokenType::OpenBraces);
        let body = self.block_statement();

        Some(ASTNode::FunctionStatement(FunctionStatement {
            name,
            body: Box::new(body),
            start,
            end: self.lexer.cursor.line_num,
        }))
    }
}
