pub struct SyntaxTreeNode {
    pub node: Box<dyn SyntaxTreeNodeTrait>,
}

pub trait SyntaxTreeNodeTrait {
    fn to_syntax_tree_node(self) -> SyntaxTreeNode;
    fn evaluate(&self) -> i32;
}

pub struct AddOperatorNode {
    left: SyntaxTreeNode,
    right: SyntaxTreeNode,
}

impl SyntaxTreeNodeTrait for AddOperatorNode {
    fn to_syntax_tree_node(self) -> SyntaxTreeNode {
        todo!()
    }

    fn evaluate(&self) -> i32 {
        todo!()
    }
}

pub struct MulOperatorNode {
    left: SyntaxTreeNode,
    right: SyntaxTreeNode,
}

impl SyntaxTreeNodeTrait for MulOperatorNode {
    fn to_syntax_tree_node(self) -> SyntaxTreeNode {
        todo!()
    }

    fn evaluate(&self) -> i32 {
        todo!()
    }
}

pub struct IntLiteralNode {
    pub value: i32,
}

impl SyntaxTreeNodeTrait for IntLiteralNode {
    fn to_syntax_tree_node(self) -> SyntaxTreeNode {
        SyntaxTreeNode { node: Box::new(self) as Box<dyn SyntaxTreeNodeTrait> }
    }

    fn evaluate(&self) -> i32 {
        self.value
    }
}

pub struct BrokenNode {
}

impl SyntaxTreeNodeTrait for BrokenNode {
    fn to_syntax_tree_node(self) -> SyntaxTreeNode {
        todo!()
    }

    fn evaluate(&self) -> i32 {
        todo!()
    }
}