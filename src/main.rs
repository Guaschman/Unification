use std::collections::{HashMap};

/**
 * A lookup table from integers in the union find algorithm to a term in the unification algorithm.
**/
struct SymbolTable(HashMap<usize, Symbol>);

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
        while i != self.parent[i] {
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
 * Represents a symbol in a unification algorithm.
 **/
#[derive(Debug, PartialEq, Clone)]
enum Symbol {
    Var(String),
    Function(String, Vec<Symbol>)
}

/*
fn var(t: (Term, Term)) -> (Term, Term) {
    match t {
        (Term::Var(s1), t1) => (Term::Var(s1), t1),
        (t1, Term::Var(s1)) => (Term::Var(s1), t1),
        _ => unreachable!()
    }
}
*/

#[derive(Debug)]
struct Substitution(HashMap<String, Symbol>);

impl Substitution {
    fn new() -> Substitution {
        Substitution(HashMap::new())
    }

    fn substitute(&self, t: Symbol) -> Symbol {
        match t {
            Symbol::Var(s) => match self.0.get(&s) {
                // to_owned can incur a heavy copy penalty for big symbols
                Some(t1) => t1.to_owned(),
                None => Symbol::Var(s)
            }
            Symbol::Function(n, v) => Symbol::Function(n,
                v.iter()
                // is there not a better way to do this map?
                .map(|sym| self.substitute(sym.to_owned()))
                .collect())
        }
    }

    fn compose(&mut self, _s: Substitution) -> Substitution {
        unimplemented!()
    }

    fn add (&mut self, s: Symbol, t: Symbol) {
        match s {
            Symbol::Var(st) => self.0.insert(st, t),
            _ => None
        };
    }
}

fn is_var(t: &Symbol) -> bool {
    match t {
        Symbol::Var(_) => true,
        _ => false
    }
}


fn occur(t1: &Symbol, t2: &Symbol) -> bool {
    if let Symbol::Var(s) = t1 {
        match t2 {
            Symbol::Var(s1) if s1 == s => true,
            Symbol::Function(_, v) => v.iter().any(|el| occur(&t1, &el)),
            _ => false
        };
    }
    false
}

/**
 * A unification algorithm based on the original unification algorithm described by Robinson 1965.
 **/
fn unify_naive(mut t1: Symbol, mut t2: Symbol, subs: &mut Substitution) -> &Substitution {
    if let Symbol::Var(s) = t1 {
        t1 = subs.substitute(Symbol::Var(s));
        println!("t1: {:?}", t1);
    }
    if let Symbol::Var(s) = t2 {
        t2 = subs.substitute(Symbol::Var(s));
    }
    if is_var(&t1) && t1 == t2 {
            //do nothing
    } else if let Symbol::Function(n1, v1) = t1 {
        if let Symbol::Function(n2, v2) = t2 {
            if n1 == n2 {
                let it = v1.iter().zip(v2.iter());
                for (a,b) in it {
                    unify_naive(a.clone(), b.clone(), subs);
                }
            } else {
                println!("://");
            }
        } else {
            println!(":/");
        }
    } else if !is_var(&t1) {
        unify_naive(t1, t2, subs);
    } else if occur(&t1, &t2) {
        // exit
    } else {
        // subst = subst with s -> t
        subs.add(t1, t2);
    }

    subs
}

fn main() {
    let mut subs = Substitution::new();
    let s1 = Symbol::Var("a".to_string());
    let s2 = Symbol::Var("b".to_string());
    let s3 = Symbol::Var("c".to_string());
    let f1 = Symbol::Function("g".to_string(), vec!(s1));
    let f2 = Symbol::Function("f".to_string(), vec!(s2, s3));
    let f3 = Symbol::Function("g".to_string(), vec!(f2));
    println!("{:?}", unify_naive(f1, f3, &mut subs));
}
