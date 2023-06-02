extern crate pest;
use std::collections::VecDeque;
use pest::{
    iterators::Pair,
    Parser
};

// Stores individual text or tag node
#[derive(Debug)]
pub enum Node {
    TagNode(Tag),
    TextNode(String)
}

// For storing attribute info
#[derive(Debug)]
pub struct Attr {
    name: String,
    value: String
}

// Represents a html tag
#[derive(Debug)]
pub struct Tag {
    typ: TagType,
    name: String,
    attrs: Vec<Attr>
}

#[derive(Debug, PartialEq, Eq)]
pub enum TagType {
    Start,
    End,
    Simple
}

// Represents a document tree
#[derive(Debug)]
pub struct NodeTree {
    root: Node,
    children: Vec<NodeTree>
}

impl NodeTree {
    fn new(node: Node) -> Self {
        Self {
            root: node,
            children: Vec::new()
        }
    }
    fn _print_indent(indent: u8) {
        for _ in 0..indent {
            print!(" ");
        }
    }
    fn _print_node(node: &Node) {
        match node {
            Node::TagNode(tag) => {
                print!("- {}", tag.name);
            }
            Node::TextNode(s) => {
                print!("- text({})", s);
            }
        }
        print!("\n");
    }
    fn _print(tree: &Vec<Self>, indent: u8) {
        for node in tree {
            NodeTree::_print_indent(indent);
            NodeTree::_print_node(&node.root);
            NodeTree::_print(&node.children, indent + 2);
        }
    }
    pub fn print_tree(&self) {
        NodeTree::_print_node(&self.root);
        NodeTree::_print(&self.children, 2);
    }
}

// Parse a tag to convert from Pest to our representation
pub fn parse_tag(pair: Pair<Rule>) -> Tag {
    // as tag will always have on inner, because the way we have defined the rule
    let tag = pair.into_inner().next().unwrap();

    // map the rule to tag type
    let tag_type = match tag.as_rule() {
        Rule::start_tag => TagType::Start,
        Rule::end_tag => TagType::End,
        Rule::simple => TagType::Simple,
        _ => unreachable!()
    };

    let mut tag_data = tag.into_inner();
    let name = tag_data.next().unwrap().as_str().to_owned();

    let mut attributes = Vec::new();

    // parse all attributes on a tag
    for attr in tag_data {
        if !matches!(attr.as_rule(), Rule::attr) {
            unreachable!("as per syntax, tag name is followed by zero or more attr only")
        }
        let mut tmp = attr.into_inner();
        // as per the parsing rule, attr has first the name, then the value
        let attr_name = tmp.next().unwrap().as_str();
        let attr_val = tmp.next().unwrap().as_str();
        attributes.push(Attr {
            name: attr_name.to_owned(),
            value: attr_val.to_owned()
        });
    }

    Tag {
        typ: tag_type,
        name,
        attrs: attributes
    }
}

// parses the list of tags into a document tree
pub fn parse(nodes: Vec<Node>) -> NodeTree {
    let mut stack: VecDeque<NodeTree> = VecDeque::new();
    let mut active: Option<NodeTree> = None;

    for node in nodes {
        match node {
            Node::TextNode(_) => {
                active.as_mut().unwrap().children.push(NodeTree::new(node));
            }
            Node::TagNode(ref tag) => match tag.typ {
                // if tag is starting, then push current active tag into stack
                TagType::Start => {
                    if active.is_some() {
                        stack.push_back(active.unwrap());
                    }
                    active = Some(NodeTree::new(node));
                }
                // if tag is self closing, add it as child of current active node
                TagType::Simple => {
                    active.as_mut().unwrap().children.push(NodeTree::new(node));
                }
                // check that it is valid
                TagType::End => {
                    let active_tag_name = match active.as_ref().unwrap().root {
                        Node::TagNode(ref tag) => tag.name.clone(),
                        _ => unreachable!()
                    };
                }
            }
        }
    }
    active.unwrap()
}

