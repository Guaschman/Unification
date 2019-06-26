use std::collections::{HashMap};

/**
 * A lookup table from integers in the union find algorithm to a term in the unification algorithm.
**/
struct SymbolTable(HashMap<usize, Term>);

/**
 * A rank based union find algorithm with path compression.
**/
#[derive(Debug, PartialEq)]
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n]
        }
    }

    fn find(&self, mut i: usize) -> usize {
        while (i != self.parent[i]) {
            // path compression
            i = self.find(self.parent[i]);
        }
        i
    }

    fn unite(&mut self, p: usize, q: usize) {
        let i = self.find(p);
        let j = self.find(q);
        if i == j {}
        else if self.rank[i] < self.rank[j] {
            // append smaller tree to bigger trees
            self.parent[i] = j;
        } else if self.rank[i] > self.rank[j] {
            self.parent[j] = i;
        } else {
            // chosen arbitraily
            self.parent[j] = i;
            self.rank[i] += 1;
        }
    }
}

/**
 * Represents a term in a unification algorithm.
**/
struct Term {
    symbol: Symbol,
    subnodes: Vec<Term>,
    class: u32
}

/**
 * Represents a symbol in a unification algorithm.
**/
#[derive(Debug, PartialEq)]
enum Symbol {
    Const(String),
    Var(String),
    Function(String, Vec<Symbol>)
}

/*
fn is_var(t: &Term) -> bool {
    match t {
        Term::Var(s) => true,
        _ => false
    }
}

fn var(t: (Term, Term)) -> (Term, Term) {
    match t {
        (Term::Var(s1), t1) => (Term::Var(s1), t1),
        (t1, Term::Var(s1)) => (Term::Var(s1), t1),
        _ => unreachable!()
    }
}

fn occur(t1: &Term, t2: &Term) -> bool {
    if let Term::Var(s) = t1 {
        match t2 {
            Term::Const(_) => false,
            Term::Var(s1) if s1 == s => true,
            Term::Function(lbl, v) => v.iter().any(|el| occur(&t1, &el)),
            _ => unreachable!()
        };
    }
    false
}*/

fn unify(t1: Term, t2: Term) -> (bool, ()) {
    let pairs_to_unify = vec!((t1,t2));

    (true, ())
}

fn main() {
    //let a = Term::Var("a".to_string());
    //let b = Term::Var("b".to_string());
    //let t = Term::Function("g".to_string(), vec!(a,b));
    //println!("Hello, world, {:?}", t);

    let mut uf = UnionFind::new(10);
    uf.unite(3,4);
    uf.unite(4,9);
    uf.unite(8,0);
    uf.unite(2,3);
    uf.unite(5,6);
    uf.unite(5,9);
    uf.unite(7,3);
    uf.unite(4,8);
    uf.unite(6,1);
    println!("{:?}", uf.find(1));
}
