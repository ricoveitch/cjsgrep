#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Program),
    FunctionStatement(FunctionStatement),
    BlockStatement(BlockStatement),
    CallExpression(CallExpression),
    Identifier(Identifier),
}

type Line = usize;

#[derive(Debug, Clone)]
pub struct Program {
    pub lines: Box<Vec<ASTNode>>,
    pub start: Line,
    pub end: Line,
}

#[derive(Debug, Clone)]
pub struct FunctionStatement {
    pub name: String,
    pub body: Box<ASTNode>,
    pub start: Line,
    pub end: Line,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub body: Box<Vec<ASTNode>>,
    pub start: Line,
    pub end: Line,
}

#[derive(Debug, Clone)]
pub struct CallExpression {
    pub base: Box<ASTNode>,
    pub start: Line,
    pub end: Line,
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub start: Line,
    pub end: Line,
}

impl ASTNode {
    pub fn get_start(&self) -> usize {
        match self {
            ASTNode::BlockStatement(bs) => bs.start,
            ASTNode::CallExpression(ce) => ce.start,
            ASTNode::Identifier(ident) => ident.start,
            ASTNode::FunctionStatement(fs) => fs.start,
            ASTNode::Program(p) => p.start,
        }
    }

    pub fn get_end(&self) -> usize {
        match self {
            ASTNode::BlockStatement(bs) => bs.end,
            ASTNode::CallExpression(ce) => ce.end,
            ASTNode::Identifier(ident) => ident.end,
            ASTNode::FunctionStatement(fs) => fs.end,
            ASTNode::Program(p) => p.end,
        }
    }

    pub fn find_func(&self, name: &str) -> Result<&ASTNode, String> {
        let prog_node = match self {
            ASTNode::Program(prog) => prog,
            _ => return Err("expected program".to_string()),
        };

        let start = prog_node.lines.iter().find(|node| {
            if let ASTNode::FunctionStatement(fs) = node {
                return &fs.name == name;
            }
            false
        });

        if let Some(start) = start {
            return Ok(start);
        }

        return Err("function start not found".to_string());
    }
}
