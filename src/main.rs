use std::cell::RefCell;
use std::fmt::{write, Debug, Formatter};
use std::rc::Rc;

struct Node<T: Debug> {
    _self: Option<Rc<RefCell<Node<T>>>>,
    _pre: Option<Rc<RefCell<Node<T>>>>,
    _next: Option<Rc<RefCell<Node<T>>>>,
    value: Rc<T>,
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<T: Debug> Node<T> {
    fn new<K: Debug>(value: K) -> Rc<RefCell<Node<K>>> {
        let new_node = Rc::new(RefCell::new(Node {
            _self: None,
            _pre: None,
            _next: None,
            value: Rc::new(value),
        }));
        new_node.clone().borrow_mut()._self = Some(new_node.clone());
        new_node
    }

    fn add_last(&mut self, value: Rc<RefCell<Node<T>>>) {
        self.set_next(Some(value.clone()));
        value.borrow_mut().set_prev(self._self.clone())
    }

    fn set_next(&mut self, value: Option<Rc<RefCell<Node<T>>>>) {
        self._next = value;
    }

    fn set_prev(&mut self, value: Option<Rc<RefCell<Node<T>>>>) {
        self._pre = value;
    }
}

struct LinkedList<T: Debug> {
    _head: Option<Rc<RefCell<Node<T>>>>,
    _tail: Option<Rc<RefCell<Node<T>>>>,
    _size: u32,
}

impl<T: Debug> LinkedList<T> {
    fn new<K: Debug>() -> LinkedList<K> {
        LinkedList {
            _head: None,
            _tail: None,
            _size: 0,
        }
    }

    fn add_last(&mut self, value: T) {
        let new_node = Node::<T>::new(value);
        let tail = &self._tail;
        match tail {
            None => {
                self.set_head(Some(new_node.clone()));
                self.set_tail(Some(new_node.clone()));
            }
            Some(tail_node) => {
                tail_node.borrow_mut().add_last(new_node.clone());
                self.set_tail(Some(new_node.clone()));
            }
        }
        self._size += 1;
    }

    fn add(&mut self, value: T) {
        self.add_last(value);
    }

    fn set_head(&mut self, value: Option<Rc<RefCell<Node<T>>>>) {
        self._head = value;
    }

    fn set_tail(&mut self, value: Option<Rc<RefCell<Node<T>>>>) {
        self._tail = value;
    }

    fn size(&self) -> u32 {
        self._size
    }

    fn for_each<K>(&self, callback: K)
    where
        K: Fn(Rc<T>),
    {
        let mut optional_data = self._head.clone();
        while optional_data.is_some() {
            if let Some(node) = optional_data {
                let x = node.clone();
                let y = node.borrow();
                callback(y.value.clone());
                optional_data = y._next.clone();
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::<String>::new();
    list.add("Hello ");
    list.add("I ");
    list.add("am ");
    list.add("Janus ");
    list.add("Lin!");

    list.for_each(|v| {
        print!("{}", v);
    });
    // Hello I am Janus Lin!

    println!("\nsize={}", list.size());
    // size=5
}
