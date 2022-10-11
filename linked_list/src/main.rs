use core::panic;

fn main() {
    let a = Node::new(32);
    //let b = ; 
    let b:Node<i32> = NodeCont::unbox(a);
    println!("{}", b.data);
 //   let c = NodeCont::unbox(*b.next);//Should panic

    let mut lst:LinkedList<i32> = LinkedList::new();
    LinkedList::add(32,  lst);
    println!("{}", b.data);

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

    //returns the unboxed node or throws a null pointer error
    fn unbox(container: Self)->Node<T>{
        let b:Node<T> = match container {
            NodeCont::Live(node) => node,
            NodeCont::Null(_) => panic!("Null Pointer Error"),
        };
        return b;
    }

    fn is_null(container: Self)->bool{
        return match container {
            NodeCont::Live(_) => false,
            NodeCont::Null(_) => true,
        };
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


struct LinkedList<T>{
    head: NodeCont<T>,
    last: NodeCont<T>,
}

impl<T> LinkedList<T>{
    fn new()->LinkedList<T>{
        //let mut e =NodeCont::new_e();
        return LinkedList { head: NodeCont::new_e(), last: NodeCont::new_e()};
    }

    fn add(data: T, list: Self){
        if NodeCont::is_null(list.head){
            LinkedList::set_head(list, Node::new(data));
            //let mut n = NodeCont::unbox(list.head);
            //list.last = *n.next;
        }
    }

    fn head(mut me: Self, data: NodeCont<T>) ->&mut {
        me.head = data;
    }


}