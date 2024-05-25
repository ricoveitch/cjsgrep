use std::collections::HashMap;

use crate::{
    ast::{ASTNode, BlockStatement, CallExpression, FunctionStatement, Program},
    logger,
};

pub struct ASTVisitor<'a> {
    src: Vec<String>,
    needle: String,
    line_num: usize,
    index: HashMap<String, &'a ASTNode>,
}

impl<'a> ASTVisitor<'a> {
    pub fn new(src: &str, needle: &str) -> Self {
        ASTVisitor {
            line_num: 0,
            index: HashMap::new(),
            needle: needle.to_string(),
            src: src.split("\n").map(|s| s.to_string()).collect(),
        }
    }

    fn grep(&mut self, from: usize, to: usize) {
        for line in from..to {
            if self.src[line].contains(&self.needle) {
                logger::info(format!("{}: {}", line + 1, self.src[line]).as_str());
            }
        }
    }

    fn index_node(&mut self, root: &'a ASTNode) {
        let program = match root {
            ASTNode::Program(prog) => prog,
            _ => return,
        };

        for node in program.lines.as_ref() {
            if let ASTNode::FunctionStatement(fs) = node {
                self.index.insert(fs.name.clone(), node);
            }
        }
    }

    pub fn search(&mut self, program: &'a ASTNode, func_start: Option<&str>) -> Result<(), String> {
        self.index_node(&program);

        match func_start {
            Some(func_start) => match program.find_func(func_start) {
                Ok(start) => Ok(self.visit_node(start)),
                Err(e) => return Err(e),
            },
            None => {
                self.line_num = program.get_start();
                return Ok(self.visit_node(&program));
            }
        }
    }

    fn visit_node(&mut self, node: &ASTNode) {
        self.line_num = node.get_start();

        match node {
            ASTNode::BlockStatement(bs) => self.visit_block_statement(&bs),
            ASTNode::CallExpression(ce) => self.visit_call_expression(ce),
            ASTNode::Identifier(_) => (),
            ASTNode::FunctionStatement(fs) => self.visit_function(&fs),
            ASTNode::Program(prog) => self.visit_prog(prog),
        };

        let end = node.get_end();
        self.grep(self.line_num, end);
        self.line_num = end;
    }

    fn visit_prog(&mut self, program: &Program) {
        for node in program.lines.as_ref() {
            match node {
                ASTNode::FunctionStatement(fs) => {
                    if fs.start > 0 {
                        self.grep(self.line_num, fs.start - 1);
                    }
                    self.line_num = fs.end + 1;
                }
                _ => self.visit_node(node),
            };
        }
    }

    fn visit_function(&mut self, func_statement: &FunctionStatement) {
        self.visit_node(&func_statement.body);
    }

    fn visit_block_statement(&mut self, block_statement: &BlockStatement) {
        for node in block_statement.body.as_ref() {
            self.visit_node(&node);
        }
    }

    fn visit_call_expression(&mut self, call_expr: &CallExpression) {
        match call_expr.base.as_ref() {
            ASTNode::Identifier(ident) => {
                if let Some(func) = self.index.get(&ident.name) {
                    self.visit_node(func);
                }
            }
            _ => (),
        };
    }
}
