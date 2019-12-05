use std::cmp;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::stdin;

#[derive(Debug, PartialEq, Clone)]
pub struct Node<T: Ord + Debug + Copy> {
    value: T,
    height: u32,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}


impl<T: Ord + Debug + Copy> Node<T> {
    pub fn new(value: T) -> Self {
        Node {value: value, height: 1, left: None, right: None}
    }

    pub fn insert(mut root: Box<Node<T>>, value: T) -> Box<Node<T>>{

        match (*root).value.cmp(&value) {
            Ordering::Equal => { return root },
            Ordering::Less =>  { 
                root.right = Node::insert_recursive(root.right.take(), value);
            },
            Ordering::Greater => {
                root.left = Node::insert_recursive(root.left.take(), value);
            },
        }
        
        root.update_height();
        Node::reconstruction(root)
    }

    fn insert_recursive(root : Option<Box<Node<T>>>, value: T) -> Option<Box<Node<T>>>{
        match root {
            Some(node) => {
                Some(Node::insert(node, value))
            },
            None => Some(Box::new(Node::new(value)))
        }
    }

    fn reconstruction(mut root: Box<Node<T>>) -> Box<Node<T>>{
        let diff = Node::height(&root.left) as i32 - Node::height(&root.right) as i32;
        if diff.abs() <= 1{
            return root;
        }
        else if diff == -2 {
            let right = root.right.take().expect("miss right child");
            if Node::height(&right.left) > Node::height(&right.right) {
                root.right = Some(Node::rotate_right(right));
                root.update_height();
            }
            else {
                root.right = Some(right);
            }
            Node::rotate_left(root)
        }
        else if diff == 2 {
            let left = root.left.take().expect("miss left child");
            if Node::height(&left.left) < Node::height(&left.right) {
                root.left = Some(Node::rotate_left(left));
                root.update_height();
            }
            else {
                root.left = Some(left);
            }
            Node::rotate_right(root)
        }
        else{
            panic!("Wrong diff");
        }
    }

    fn rotate_left(mut root: Box<Node<T>>) -> Box<Node<T>> {
        let mut right = root.right.take().expect("miss right child!");
        root.right = right.left.take();
        root.update_height();
        right.left = Some(root);
        right.update_height();
        right
    }

    fn rotate_right(mut root: Box<Node<T>>) -> Box<Node<T>>{
        let mut left = root.left.take().expect("miss left child!");
        root.left = left.right.take();
        root.update_height();
        left.right = Some(root);
        left.update_height();
        left
    }


    fn update_height(&mut self) {
        self.height = cmp::max(Node::height(&self.left), Node::height(&self.right)) + 1;
    }

    pub fn height(root: &Option<Box<Node<T>>>) -> u32 {
        root.as_ref().map_or(0, |x| x.height)
    }

    pub fn delete(mut root: Box<Node<T>>, value: T) -> Option<Box<Node<T>>> {
        match root.value.cmp(&value){
            Ordering::Equal =>  return Node::delete_node(root),
            Ordering::Less => {
                if let Some(node) = root.right.take() {
                    root.right = Node::delete(node, value);
                }
            },
            Ordering::Greater => {
                if let Some(node) = root.left.take() {
                    root.left = Node::delete(node, value);
                }
            }
        }
        root.update_height();
        Some(Node::reconstruction(root))
    }

    fn delete_node(mut root: Box<Node<T>>) -> Option<Box<Node<T>>>{
        match (root.left.take(), root.right.take()) {
            (None, None) => None,
            (Some(left), None) => Some(left),
            (None, Some(right)) => Some(right),
            (Some(left), Some(right)) => Some(Node::reorder_children(left,right))
        } 
    }

    fn reorder_children(left: Box<Node<T>>, right: Box<Node<T>>) -> Box<Node<T>>{
        let (remaining_tree, min) = Node::drop_min(right);
        let mut new_root = min;
        new_root.left = Some(left);
        new_root.right = remaining_tree;
        new_root.update_height();
        Node::reconstruction(new_root)
    }

    fn drop_and_get_min(mut root : Box<Node<T>>, left: Box<Node<T>>) -> (Option<Box<Node<T>>>,Box<Node<T>>) {
        let (new_left, min) =  Node::drop_min(left);
        root.left = new_left;
        root.update_height();
        (Some(Node::reconstruction(root)),min)
    }

    fn drop_min(mut root: Box<Node<T>>) -> (Option<Box<Node<T>>>, Box<Node<T>>) {
        match root.left.take() {
            Some(left) => Node::drop_and_get_min(root, left),
            None => (root.right.take(), root)
        }
    }

    pub fn count_leaves(&self) -> u32 {
        match (self.left.as_ref(), self.right.as_ref()) {
            (None, None) => 1,
            (Some(left), None) => left.count_leaves(),
            (None, Some(right)) => right.count_leaves(),
            (Some(left), Some(right)) => left.count_leaves() + right.count_leaves()
        }
    }

    pub fn inorder_traversal(&self, res: &mut Vec<T>) {
        if let Some(ref left) = self.left {
            left.inorder_traversal(res);
        }
        //print!("{:?} ", self.value);
        res.push(self.value);
        if let Some(ref right) = self.right {
            right.inorder_traversal(res);
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
struct AVLTree<T: Ord + Debug + Copy> {
    root: Option<Box<Node<T>>>,
}

impl <T:Ord + Debug + Copy> AVLTree<T>{
    pub fn new() -> AVLTree<T> {
        AVLTree { root: None }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn insert(&mut self, val : T) {
        match self.root.take() {
            Some(node) => {
                self.root = Some(Node::insert(node,val));
            },
            None => {
                self.root = Some(Box::new(Node::new(val)));
            }
        }
    }

    pub fn delete(&mut self, value: T){
        match self.root.take() {
            Some(node) => self.root = Node::delete(node, value),
            None => return
        }
    }

    pub fn count_leaves(&self) -> u32 {
        match self.root.as_ref() {
            None => 0,
            Some(node) => {
                node.count_leaves()
            }
        }
    }
    
    pub fn height(&self) -> u32{
        match self.root.as_ref() {
            None => 0,
            Some(node) => {
                node.height
            }
        }
    }

    pub fn inorder_traversal(&self) -> Vec<T>{
        let mut res = Vec::new();
        match self.root.as_ref() {
            None => return res,
            Some(node) => {
                node.inorder_traversal(&mut res);
            }
        }
        return res;
    }
}

fn main(){
    println!("Welcome to use AVL-Tree, Guideline:");
    println!("The type of data in the test AVL-Tree shuold be i32");
    println!("\'insert\' to insert a value to the tree, i.e: insert 1");
    println!("\'delete\' to delte a value from the tree, i.e: delete 1");
    println!("\'height\' to get the height of the tree, i.e: height");
    println!("\'count\' to get the  the number of leaves in the tree, i.e: count");
    println!("\'empty\' to check if the tree is empty., i.e: empty");
    println!("\'inorder\' to get the in-order travesal result of the tree, i.e: inorder");
    println!("\'show\' to get the structure of the tree, i.e: structure");
    println!("\'quit\' to stop the program, i.e: quit");
    println!("Start:");
    

    let mut line = String::new();
    let mut t = AVLTree::<i32>::new();
    while let _ = std::io::stdin().read_line(&mut line).unwrap(){
        line = line[0..line.len()-1].to_string();
        let options: Vec<&str> = line.split(' ').collect();

        match options[0] {
            "quit" => break,
            "insert" => t.insert(options[1].parse().unwrap()),
            "delete" => t.delete(options[1].parse().unwrap()),
            "height" => println!("height is {}", t.height()),
            "count" => println!("number of leaves nodes is {}", t.count_leaves()),
            "empty" => println!("{}", t.is_empty()),
            "inorder" => println!("{:?}", t.inorder_traversal()),
            "show" => println!("{:#?}", t),
            _ => panic!("input error")
        }
        line = String::new();
    }

    println!("Thank you for use!");
}
