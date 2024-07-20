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
    MemberExpression(MemberExpression),
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

impl ObjectPattern {
    pub fn get_value(&self, key: &str) -> Option<&String> {
        for prop in &self.properties {
            if prop.key == key {
                return Some(&prop.value);
            }
        }
        None
    }
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

#[derive(Debug, Clone)]
pub struct MemberExpression {
    pub base: Box<ASTNode>,
    pub property: String,
    pub start: Line,
    pub end: Line,
}

impl MemberExpression {
    pub fn get_base(&self) -> &Identifier {
        match self.base.as_ref() {
            ASTNode::MemberExpression(me) => return me.get_base(),
            ASTNode::Identifier(ident) => return ident,
            _ => panic!("invalid membership expression"),
        }
    }
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
            ASTNode::MemberExpression(me) => me.start,
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
            ASTNode::MemberExpression(me) => me.end,
            ASTNode::ObjectPattern(op) | ASTNode::ExportStatement(op) => op.end,
        }
    }

    pub fn find_function(&self, name: &str) -> Option<&ASTNode> {
        let prog_lines = match self {
            ASTNode::Program(prog) => &prog.lines,
            _ => return None,
        };

        for node in prog_lines.as_ref() {
            match node {
                ASTNode::FunctionStatement(fs) if fs.name == name => return Some(node),
                _ => (),
            };
        }

        None
    }

    pub fn try_get_require_file(&self) -> Option<(String, &VariableExpression)> {
        let ve = match self {
            ASTNode::VariableExpression(ve) => ve,
            _ => return None,
        };

        let ce = match ve.rhs.as_ref() {
            ASTNode::CallExpression(ce) => ce,
            _ => return None,
        };

        match &ce.base.as_ref() {
            ASTNode::Identifier(ident) if ident.name == "require" => (),
            _ => return None,
        }

        let require_file = if let Some(param) = &ce.param {
            param
        } else {
            return None;
        };

        if !require_file.starts_with("./") && !require_file.starts_with("../") {
            return None;
        }

        return Some((require_file.clone(), ve));
    }

    pub fn try_export_extract(&self) -> Option<(String, &ASTNode)> {
        match self.try_get_require_file() {
            Some(r) => Some((r.0, &r.1.lhs)),
            None => None,
        }
    }

    fn find_node(&self, pred: impl Fn(&ASTNode) -> bool) -> Option<&ASTNode> {
        let prog_lines = match self {
            ASTNode::Program(prog) => &prog.lines,
            _ => return None,
        };

        for node in prog_lines.as_ref() {
            if pred(node) {
                return Some(node);
            }
        }

        return None;
    }

    pub fn find_export_statement(&self) -> Option<&ObjectPattern> {
        if let Some(ASTNode::ExportStatement(es)) = self.find_node(|node| match node {
            ASTNode::ExportStatement(_) => true,
            _ => false,
        }) {
            return Some(es);
        }

        None
    }

    pub fn find_exported_func(&self, target: &str) -> Option<&ASTNode> {
        if let Some(es) = self.find_export_statement() {
            if let Some(val) = es.get_value(target) {
                return self.find_function(val);
            }
        }

        None
    }
}
