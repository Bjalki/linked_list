use arraylist::arl::ArrayList;
use arraylist::arraylist;

fn main(){
   let mut tree = Bst::new();
   Bst::add(10, &mut tree);
   Bst::add(5, &mut tree);
   Bst::add(15, &mut tree);
   //println!("{}", Node::contains(5, tree.list));
   Bst::add(4, &mut tree);
   
   //Bst::remove(5, &mut tree);
   println!("At least it compiled");
}

struct Bst{
    list: ArrayList<Node>,
}

impl Bst{
    fn new()->Bst{
        Bst{
            list: arraylist![],
        }
    }
    
    fn add(data: i32,  lst: &mut Self){
        Node::add(data, &mut lst.list);

    }

   // fn remove(data: i32, lst: &mut Self){
       // Node::delete(data, &mut lst.list);
   // }
}
#[derive(Debug, PartialEq, Clone, Copy)]
struct Node{
    left: i32,
    right: i32,
    data: i32,
    is_valid: bool,
    parent: i32
}

impl Node{
    fn new(data: i32, p: i32)->Node{
        Node{
            left: -1,
            right:  -1,
            data,
            is_valid: true,
            parent: p,
        }
    }

    fn add(data: i32, tree: &mut ArrayList<Node>){
        //let lst = &tree.list;
        let lst = tree;
        if lst.is_empty(){
            lst.add(Node::new(data, -1));
        }else{
            Self::in_add(data, 0, lst);
        }
    }

    fn in_add(data: i32, ref_point: i32, tree: &mut ArrayList<Node>)/*-> [&mut Node; 2]*/{
        //let lst = &mut tree.list;
        let lst = tree;
        let h:Option<Node> = lst.get(ref_point.try_into().unwrap());
        let head: &mut Node =&mut h.unwrap();
        if data < head.data {
            if head.left!=-1 {
                return Self::in_add(data, head.left, lst);
            }else{
                let n = Node::new(data, lst.index_of(*head).unwrap() as i32);
                lst.add(n);
                //return [&mut head, &mut n];
                let unref = &*lst;
                let un_num = unref.index_of(n).unwrap() as i32;
                let _num = lst.index_of(n).unwrap() as i32;//This line does nothing but it breaks when I remove it
                let mut a_node = lst.get(n.parent.try_into().unwrap()).unwrap();
                a_node.left = un_num;
                lst.replace(n.parent.try_into().unwrap(), a_node);
                //head.left = un_num;
            }
        }else{
            if head.right!=-1 {
                return Self::in_add(data, head.right, lst);
            }else{
                let n = Node::new(data, lst.index_of(*head).unwrap() as i32);
                lst.add(n);
                //return [head, &mut n];
                let unref = &*lst;
                let un_num = unref.index_of(n).unwrap() as i32;
                let _num = lst.index_of(n).unwrap() as i32;
                let mut a_node = lst.get(n.parent.try_into().unwrap()).unwrap();
                a_node.right = un_num;
                lst.replace(n.parent.try_into().unwrap(), a_node);
                //head.right = un_num;
                //lst.get(n.parent.try_into().unwrap()).unwrap().right = un_num;
            }
        }
    }
/* 
    //returns -1 if false index of node if true
    fn contains(data: i32, tree: ArrayList<Node>){

    }

    fn in_cont(data: i32, ref_point: i32, tree: ArrayList<Node>){
 
    }

    fn delete(data: i32, tree: &mut ArrayList<Node>){
       
    }*/
}