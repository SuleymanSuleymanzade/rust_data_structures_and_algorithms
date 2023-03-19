#[derive(Debug)]
struct linkedlist{
    head: pointer,
}

#[derive(Debug)]
struct Node{
    element:i32,
    next: pointer,
}

type pointer = Option<Box<Node>>

fn main() {
    let node1: Node = Node{element:1, next:None};
    let node2: Node = Node{element:3, next:Some(Box::new(
        Node{element:54, next:Some(Box::new(
            Node{element:54, next: None}
        ))}
    ))};

    let list_a = linkedlist(head: Node{element:1, next:None});
    

println!("{:?}", node2);
}
