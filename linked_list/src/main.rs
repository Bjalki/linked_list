use core::panic;

fn main() {
    let a = Node::new(32);
    //let b = ; 
    let b:Node<i32> = NodeCont::unbox(a);
    println!("{}", b.data);
 //   let c = NodeCont::unbox(*b.next);//Should panic


}

enum NodeCont<T>{
    Live(Node<T>),
    Null(Null),
}

impl<T> NodeCont<T>{
    fn new_e()->NodeCont<T>{
        return NodeCont::Null(Null::new());
    }

    fn new(node: Node<T>) ->NodeCont<T>{
        return NodeCont::Live(node);
    }

    fn unbox(container: Self)->Node<T>{
        let b:Node<T> = match container {
            NodeCont::Live(node) => node,
            NodeCont::Null(_) => panic!("Null Pointer Error"),
        };
        return b;
    }
}

struct Node<T>{
    next: Box<NodeCont<T>>,
    data: T,
}

impl<T> Node<T>{
    fn new(data: T)->NodeCont<T>{
        let node:Node<T> = Node{ //create a node with a null next and initiate data
            next: Box::new(NodeCont::new_e()),
            data,
        };
        let cont: NodeCont<T> = NodeCont::new(node);
        return cont;
    }
}

struct Null{

}

impl Null{ //is this neccesary? probably not 
    fn new()->Null{
        Null{

        }
    }

    fn null()->bool{
        return true;
    }
}