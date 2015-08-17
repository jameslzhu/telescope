use std::fmt;

#[derive(Copy, Clone, Debug)]
enum Sym {
    Add,
    Sub,
    Mul,
    Div,
}


impl fmt::Display for Sym {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let symbol = match self {
            &Sym::Add => "+",
            &Sym::Sub => "-",
            &Sym::Mul => "*",
            &Sym::Div => "/"
        };

        write!(f, "{}", symbol)
    }
}


// #[derive(Debug)]
// pub enum Node<T> {
//     Nil,
//     Sym(Sym),
//     Num(T),
//     List(Vec<List<T>>),
// }


// #[derive(Debug)]
// pub struct List<T> {
//     elem : Node<T>
// }

#[derive(Debug)]
pub enum Node<T> {
    Num(T),
    Sym(Sym),
    List(List<T>),
}

impl<T> Clone for Node<T> where T : Clone {
    fn clone(&self) -> Self {
        match self {
            &Node::Num(t) => Node::Num(t.clone()),
            &Node::Sym(t) => Node::Sym(t.clone()),
            &Node::List(t) => Node::List(t.clone())
        }
    }
}

#[derive(Debug)]
pub struct List<T> {
    elems : Vec<Node<T>>
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List::<T> {
            elems : Vec::<_>::new()
        }
    }
}

impl<T> Clone for List<T> where T : Clone {
    fn clone(&self) -> Self {
        List::<T> {
            elems : self.elems.clone()
        }
    }
}

impl<T> fmt::Display for List<T> where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( ");

        for e in &self.elems {
            match e {
                Node::Num(t) => write!(f, "{} ", t),
                Node::Sym(t) => write!(f, "{} ", t),
                Node::List(l) => write!(f, "( {} )", l)
            };
        }

        write!(f, ")")
    }
}


// impl<T> Iterator for List<T> {
//     type Item = Node<T>;
//
//     fn next(&mut self) -> Option<Node<T>> {
//         unimplemented!();
//     }
// }
