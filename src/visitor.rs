use std::collections::{HashMap, HashSet};

use crate::{
    ast::{ASTNode, BlockStatement, CallExpression, FunctionStatement, ObjectPattern, Program},
    file_scope::{ProgramScope, Symbol},
    logger,
    parser::Parser,
    utils,
};

#[derive(Clone)]
struct File {
    path: String,
    ast: ASTNode,
    lines: Vec<String>,
    lines_recorded: HashSet<usize>,
}

impl File {
    fn new(path: &str) -> Self {
        let src = utils::read_file(path);
        let ast: ASTNode = Parser::new(&src).parse();

        Self {
            path: String::from(path),
            ast,
            lines: src.split("\n").map(|s| s.to_string()).collect(),
            lines_recorded: HashSet::new(),
        }
    }
}

pub struct ASTVisitor {
    needle: String,
    line_num: usize,
    files: HashMap<String, File>,
    scope: ProgramScope,
}

impl ASTVisitor {
    pub fn new(needle: &str) -> Self {
        ASTVisitor {
            line_num: 0,
            needle: String::from(needle),
            files: HashMap::new(),
            scope: ProgramScope::new(),
        }
    }

    fn grep(&mut self, from: usize, until: usize) {
        let file_path = match self.scope.current() {
            Some(s) => &s.file_path,
            None => return,
        };

        let file = self.files.get_mut(file_path).unwrap();

        for line in from..=until {
            if !file.lines_recorded.contains(&line) && file.lines[line].contains(&self.needle) {
                logger::info(format!("{}: {}", line + 1, file.lines[line].trim()).as_str());
                file.lines_recorded.insert(line);
            }
        }
    }

    fn push_file_scope(&mut self, file_path: &str) -> bool {
        self.scope.push_file(file_path);
        let ast = match self.files.get(file_path) {
            Some(f) => f.ast.clone(),
            None => return false,
        };

        match &ast {
            ASTNode::Program(prog) => self.index_block(&prog.lines),
            _ => return false,
        };

        return true;
    }

    fn init_visit(&mut self, node: &ASTNode) {
        self.line_num = node.get_start();
        self.visit_node(node)
    }

    pub fn init_search(&mut self, path: &str) -> Result<ASTNode, String> {
        let path = match utils::get_absolute_path(path) {
            Ok(pb) => pb,
            Err(e) => return Err(e.to_string()),
        };

        let mut files = HashMap::new();
        files.insert(path.clone(), File::new(&path));
        self.files = files;

        self.scope.push_file(&path);
        Ok(self.files.get(&path).unwrap().ast.clone())
    }

    pub fn search(&mut self, path: &str, func_start: Option<&str>) -> Result<(), String> {
        let file_ast = self.init_search(path)?;

        match func_start {
            Some(func_start_name) => {
                match &file_ast {
                    ASTNode::Program(prog) => self.index_block(&prog.lines),
                    _ => return Err(format!("program not found")),
                };

                match file_ast.find_function(func_start_name) {
                    Some(start) => self.init_visit(start),
                    None => return Err(format!("function start {} not found", func_start_name)),
                };
            }
            None => self.init_visit(&file_ast),
        };

        Ok(())
    }

    fn visit_node(&mut self, node: &ASTNode) {
        let start = node.get_start();
        self.grep(self.line_num, start);
        self.line_num = start;

        match node {
            ASTNode::BlockStatement(bs) => self.visit_block_statement(&bs),
            ASTNode::CallExpression(ce) => self.visit_call_expression(ce),
            ASTNode::Identifier(_) => (),
            ASTNode::FunctionStatement(fs) => self.visit_function(&fs),
            ASTNode::Program(prog) => self.visit_prog(prog),

            ASTNode::ExportStatement(_) => (),
            ASTNode::VariableExpression(_) => (),
            ASTNode::ObjectPattern(_) => (),
        };

        let end = node.get_end();
        self.grep(self.line_num, end);
        self.line_num = end;

        match node {
            ASTNode::BlockStatement(_) => self.scope.pop(),
            _ => (),
        };
    }

    fn index_export(&mut self, required_file: &str, op: &ObjectPattern) {
        let file = match self.files.get(required_file) {
            Some(file) => &file,
            None => {
                self.files
                    .insert(String::from(required_file), File::new(required_file));
                self.files.get(required_file).unwrap()
            }
        };

        for prop in &op.properties {
            if let Some(func) = file.ast.find_function(&prop.key) {
                self.scope.insert_symbol(
                    &prop.value,
                    Symbol {
                        node: func.clone(),
                        file_path: file.path.clone(),
                    },
                )
            }
        }
    }

    fn index_block(&mut self, lines: &Vec<ASTNode>) {
        let current_file = self.scope.current().unwrap().file_path.clone();
        for node in lines {
            if let Some((required_file, op)) = node.try_export_extract() {
                if let Some(full_path) = utils::join_path(&current_file, &required_file) {
                    self.index_export(&full_path, op);
                }
            }

            match node {
                ASTNode::FunctionStatement(fs) => self.scope.insert_symbol(
                    &fs.name,
                    Symbol {
                        node: ASTNode::FunctionStatement(fs.clone()),
                        file_path: current_file.clone(),
                    },
                ),
                _ => (),
            }
        }
    }

    fn visit_block(&mut self, lines: &Vec<ASTNode>) {
        self.index_block(lines);

        for node in lines {
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

    fn visit_prog(&mut self, program: &Program) {
        self.visit_block(program.lines.as_ref());
    }

    fn visit_function(&mut self, func_statement: &FunctionStatement) {
        self.visit_node(&func_statement.body);
    }

    fn visit_block_statement(&mut self, block_statement: &BlockStatement) {
        self.scope.push_block();
        self.visit_block(block_statement.body.as_ref());
    }

    fn visit_call_expression(&mut self, call_expr: &CallExpression) {
        let ident = match call_expr.base.as_ref() {
            ASTNode::Identifier(ident) => ident,
            _ => return,
        };

        if let Some(symbol) = self.scope.find_symbol(&ident.name).map(|s| s.clone()) {
            if symbol.file_path != self.scope.current().unwrap().file_path {
                if !self.push_file_scope(&symbol.file_path) {
                    return;
                }
            }

            self.line_num = symbol.node.get_start();
            self.visit_node(&symbol.node);
            self.line_num = call_expr.start;
        }
    }
}
