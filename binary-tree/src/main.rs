#![allow(dead_code)]
struct Tree<T> {
    root: Option<Node<T>>,
}

impl Tree<i32> {
    pub fn parse(node: &Box<Node<i32>>) -> i32 {
        let mut left: Option<i32> = None;
        let mut right: Option<i32> = None;

        if let Some(left_child) = &node.left {
            left = Some(Tree::parse(left_child));
        }

        if let Some(right_child) = &&node.right {
            right = Some(Tree::parse(right_child));
        }

        let left = if let Some(x) = left { x } else { 0 };
        let right = if let Some(x) = right { x } else { 0 };

        match node.operation {
            Operation::Add => left + right,
            Operation::Sub => left - right,
            Operation::Mul => left * right,
            Operation::Div => {
                if right == 0 {
                    panic!("Division by zero happened")
                }
                left / right
            }
            Operation::Value(x) => x,
        }
    }
}

struct Node<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    operation: Operation<T>,
}

type ChildNode<T> = Option<Box<Node<T>>>;

impl<T> Node<T> {
    pub fn new(operation: Operation<T>, left: Option<Node<T>>, right: Option<Node<T>>) -> Self {
        Node::<T> {
            left: match left {
                Some(node) => Some(Box::new(node)),
                _ => None,
            },
            right: match right {
                Some(node) => Some(Box::new(node)),
                _ => None,
            },
            operation,
        }
    }
}

enum Operation<T> {
    Add,
    Sub,
    Mul,
    Div,
    Value(T),
}

fn add<T>(left: Node<T>, right: Node<T>) -> Node<T> {
    Node::new(Operation::Add, Some(left), Some(right))
}

fn sub<T>(left: Node<T>, right: Node<T>) -> Node<T> {
    Node::new(Operation::Sub, Some(left), Some(right))
}

fn mul<T>(left: Node<T>, right: Node<T>) -> Node<T> {
    Node::new(Operation::Mul, Some(left), Some(right))
}

fn div<T>(left: Node<T>, right: Node<T>) -> Node<T> {
    Node::new(Operation::Div, Some(left), Some(right))
}

fn value<T>(value: T) -> Node<T> {
    Node::new(Operation::<T>::Value(value), None, None)
}

fn main() {
    let mut tree = Tree::<i32> { root: None };

    tree.root = Some(add::<i32>(mul(value(10), value(3)), value(3)));

    println!(
        "{}",
        Tree::<i32>::parse(&Box::new(tree.root.expect("No root")))
    )
}
