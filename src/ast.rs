#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Program),
    FunctionStatement(FunctionStatement),
    BlockStatement(BlockStatement),
    CallExpression(CallExpression),
    VariableExpression(VariableExpression),
    ObjectPattern(ObjectPattern),
    ExportStatement(ObjectPattern),
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
pub struct VariableExpression {
    pub lhs: Box<ASTNode>,
    pub rhs: Box<ASTNode>,
    pub start: Line,
    pub end: Line,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct ObjectPattern {
    pub properties: Vec<Property>,
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
    pub param: Option<String>,
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
            ASTNode::VariableExpression(ve) => ve.start,
            ASTNode::ObjectPattern(op) | ASTNode::ExportStatement(op) => op.start,
        }
    }

    pub fn get_end(&self) -> usize {
        match self {
            ASTNode::BlockStatement(bs) => bs.end,
            ASTNode::CallExpression(ce) => ce.end,
            ASTNode::Identifier(ident) => ident.end,
            ASTNode::FunctionStatement(fs) => fs.end,
            ASTNode::Program(p) => p.end,
            ASTNode::VariableExpression(ve) => ve.end,
            ASTNode::ObjectPattern(op) | ASTNode::ExportStatement(op) => op.end,
        }
    }
}
