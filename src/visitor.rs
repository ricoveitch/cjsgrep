use std::collections::HashSet;

use crate::{
    ast::{ASTNode, BlockStatement, CallExpression, FunctionStatement, Program},
    logger,
    parser::Parser,
    symbol_table::SymbolTable,
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

pub struct ASTVisitor {
    needle: String,
    line_num: usize,
    lines_recorded: HashSet<usize>,
    symbol_table: SymbolTable,
}

impl ASTVisitor {
    pub fn new(needle: &str) -> Self {
        ASTVisitor {
            line_num: 0,
            needle: String::from(needle),
            lines_recorded: HashSet::new(),
            symbol_table: SymbolTable::new(),
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

    fn init_search(&mut self, node: &ASTNode, file: &File) {
        self.line_num = node.get_start();
        self.visit_node(node, &file)
    }

    pub fn search(&mut self, src: &str, func_start: Option<&str>) -> Result<(), String> {
        let file = File::new(src);

        match func_start {
            Some(func_start_name) => {
                match &file.ast {
                    ASTNode::Program(prog) => self.index_block(&prog.lines),
                    _ => return Err(format!("program not found")),
                };

                match self.symbol_table.get(func_start_name) {
                    Some(start) => self.init_search(&start.clone(), &file),
                    None => return Err(format!("function start {} not found", func_start_name)),
                }
            }
            None => self.init_search(&file.ast, &file),
        };

        Ok(())
    }

    fn visit_node<'a>(&mut self, node: &ASTNode, file: &File) {
        let start = node.get_start();
        self.grep(file, self.line_num, start);
        self.line_num = start;

        match node {
            ASTNode::BlockStatement(bs) => self.visit_block_statement(&bs, &file),
            ASTNode::CallExpression(ce) => self.visit_call_expression(ce, &file),
            ASTNode::Identifier(_) => (),
            ASTNode::FunctionStatement(fs) => self.visit_function(&fs, &file),
            ASTNode::Program(prog) => self.visit_prog(prog, &file),

            ASTNode::ExportStatement(_) => (),
            ASTNode::VariableExpression(_) => (),
            ASTNode::ObjectPattern(_) => (),
        };

        let end = node.get_end();
        self.grep(file, self.line_num, end);
        self.line_num = end;
    }

    fn index_block(&mut self, lines: &Vec<ASTNode>) {
        for node in lines {
            if let ASTNode::FunctionStatement(fs) = node {
                self.symbol_table
                    .set(&fs.name, ASTNode::FunctionStatement(fs.clone()))
            }
        }
    }

    fn visit_block(&mut self, lines: &Vec<ASTNode>, file: &File) {
        self.index_block(lines);

        for node in lines {
            match node {
                ASTNode::FunctionStatement(fs) => {
                    if fs.start > 0 {
                        self.grep(&file, self.line_num, fs.start - 1);
                    }
                    self.line_num = fs.end + 1;
                }
                _ => self.visit_node(node, &file),
            };
        }
    }

    fn visit_prog(&mut self, program: &Program, file: &File) {
        self.visit_block(program.lines.as_ref(), file);
    }

    fn visit_function(&mut self, func_statement: &FunctionStatement, file: &File) {
        self.visit_node(&func_statement.body, &file);
    }

    fn visit_block_statement(&mut self, block_statement: &BlockStatement, file: &File) {
        self.symbol_table.push_scope();
        self.visit_block(block_statement.body.as_ref(), file);
        self.symbol_table.pop_scope();
    }

    fn visit_call_expression(&mut self, call_expr: &CallExpression, file: &File) {
        let ident = match call_expr.base.as_ref() {
            ASTNode::Identifier(ident) => ident,
            _ => return,
        };

        if let Some(func) = self.symbol_table.get(&ident.name) {
            self.line_num = func.get_start();
            self.visit_node(&func.clone(), &file);
            self.line_num = call_expr.start;
        }
    }
}
