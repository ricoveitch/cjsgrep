use std::collections::HashSet;

use crate::{
    ast::{ASTNode, BlockStatement, CallExpression, FunctionStatement, Program},
    logger,
    parser::Parser,
};

struct File {
    ast: ASTNode,
    lines: Vec<String>,
}

impl File {
    fn new(src: &str) -> Self {
        let ast: ASTNode = Parser::new(src).parse();

        Self {
            ast,
            lines: src.split("\n").map(|s| s.to_string()).collect(),
        }
    }
}

pub fn find_func_in_lines<'a>(lines: &'a Vec<ASTNode>, name: &str) -> Option<&'a ASTNode> {
    lines.iter().find(|node| {
        if let ASTNode::FunctionStatement(fs) = node {
            return &fs.name == name;
        }
        false
    })
}

pub fn find_func<'a>(node: &'a ASTNode, name: &str, mut scope: Vec<String>) -> Option<&'a ASTNode> {
    let lines = match node {
        ASTNode::Program(prog) => prog.lines.as_ref(),
        ASTNode::FunctionStatement(fs) => match fs.body.as_ref() {
            ASTNode::BlockStatement(bs) => bs.body.as_ref(),
            _ => return None,
        },
        _ => return None,
    };

    if let Some(target) = scope.first() {
        let func = match find_func_in_lines(lines, target) {
            Some(f) => f,
            None => return None,
        };

        scope.pop();
        let res = find_func(func, name, scope);
        if res.is_some() {
            return res;
        }
    }

    find_func_in_lines(lines, name)
}

pub struct ASTVisitor {
    needle: String,
    line_num: usize,
    stack_trace: Vec<String>,
    lines_recorded: HashSet<usize>,
}

impl ASTVisitor {
    pub fn new(needle: &str) -> Self {
        ASTVisitor {
            line_num: 0,
            needle: String::from(needle),
            stack_trace: Vec::new(),
            lines_recorded: HashSet::new(),
        }
    }

    fn grep(&mut self, file: &File, from: usize, until: usize) {
        for line in from..=until {
            if !self.lines_recorded.contains(&line) && file.lines[line].contains(&self.needle) {
                logger::info(format!("{}: {}", line + 1, file.lines[line].trim()).as_str());
                self.lines_recorded.insert(line);
            }
        }
    }

    pub fn search(&mut self, src: &str, func_start: Option<&str>) -> Result<(), String> {
        let file = File::new(src);

        let node_start = match func_start {
            Some(func_start) => match find_func(&file.ast, func_start, self.stack_trace.clone()) {
                Some(start) => start,
                None => return Err(format!("function start {} not found", func_start)),
            },
            None => &file.ast,
        };

        self.line_num = node_start.get_start();
        Ok(self.visit_node(node_start, &file))
    }

    fn visit_node(&mut self, node: &ASTNode, file: &File) {
        let start = node.get_start();
        self.grep(file, self.line_num, start);
        self.line_num = start;

        match node {
            ASTNode::BlockStatement(bs) => self.visit_block_statement(&bs, &file),
            ASTNode::CallExpression(ce) => self.visit_call_expression(ce, &file),
            ASTNode::Identifier(_) => (),
            ASTNode::FunctionStatement(fs) => self.visit_function(&fs, &file),
            ASTNode::Program(prog) => self.visit_prog(prog, &file),
        };

        let end = node.get_end();
        self.grep(file, self.line_num, end);
        self.line_num = end;
    }

    fn visit_scope_lines(&mut self, lines: &Vec<ASTNode>, file: &File) {
        for node in lines {
            match node {
                ASTNode::FunctionStatement(fs) => {
                    self.grep(&file, self.line_num, fs.start);
                    self.line_num = fs.end + 1;
                }
                _ => self.visit_node(node, &file),
            };
        }
    }

    fn visit_prog(&mut self, program: &Program, file: &File) {
        self.visit_scope_lines(program.lines.as_ref(), file);
    }

    fn visit_function(&mut self, func_statement: &FunctionStatement, file: &File) {
        self.stack_trace.push(func_statement.name.clone());
        self.visit_node(&func_statement.body, &file);
        self.stack_trace.pop();
    }

    fn visit_block_statement(&mut self, block_statement: &BlockStatement, file: &File) {
        self.visit_scope_lines(block_statement.body.as_ref(), file);
    }

    fn visit_call_expression(&mut self, call_expr: &CallExpression, file: &File) {
        let ident = match call_expr.base.as_ref() {
            ASTNode::Identifier(ident) => ident,
            _ => return,
        };

        if let Some(func) = find_func(&file.ast, &ident.name, self.stack_trace.clone()) {
            self.line_num = func.get_start();
            self.visit_node(func, &file);
            self.line_num = call_expr.start;
        }
    }
}
