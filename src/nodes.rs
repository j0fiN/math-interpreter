use std::fmt;


#[derive(Debug)]
pub struct NumberNode {
    pub val: isize
}
impl fmt::Display for NumberNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[derive(Debug)]
pub struct AddNode {
    pub node_a: isize,
    pub node_b: isize
}
impl fmt::Display for AddNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}+{})", self.node_a, self.node_b)
    }
}

#[derive(Debug)]
pub struct SubtractNode {
    pub node_a: isize,
    pub node_b: isize
}
impl fmt::Display for SubtractNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}-{})", self.node_a, self.node_b)
    }
}

#[derive(Debug)]
pub struct MultiplyNode {
    pub node_a: isize,
    pub node_b: isize
}
impl fmt::Display for MultiplyNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}*{})", self.node_a, self.node_b)
    }
}

#[derive(Debug)]
pub struct DivideNode {
    pub node_a: isize,
    pub node_b: isize
}
impl fmt::Display for DivideNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}/{})", self.node_a, self.node_b)
    }
}