use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use clap::{arg, Parser};

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file = File::open(args.file)?;

    Ok(())
}

fn next_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

trait Logic {
    fn eval(&self) -> bool;
}

struct Predicate {
    name: String,
    val: bool,
    id: usize,
}

impl Predicate {
    fn new(name: String, value: bool) -> Self {
        Self {
            name,
            val: value,
            id: next_id(),
        }
    }
}

impl Logic for Predicate {
    fn eval(&self) -> bool {
        self.val
    }
}

struct Not {
    val: dyn Logic,
}

impl Logic for Not {
    fn eval(&self) -> bool {
        -self.val.eval()
    }
}

struct And {
    left: dyn Logic,
    right: dyn Logic,
}

impl Logic for And {
    fn eval(&self) -> bool {
        self.left.eval() && self.right.eval()
    }
}

struct Or {
    left: dyn Logic,
    right: dyn Logic,
}

impl Logic for Or {
    fn eval(&self) -> bool {
        self.left.eval() || self.right.eval()
    }
}

struct Xor {
    left: dyn Logic,
    right: dyn Logic,
}

impl Logic for Xor {
    fn eval(&self) -> bool {
        self.left.eval() ^ self.right.eval()
    }
}

struct Implication {
    condition: dyn Logic,
    claim: dyn Logic,
}

impl Logic for Implication {
    fn eval(&self) -> bool {
        self.condition.eval() || !self.claim.eval()
    }
}

struct IfAndOnlyIf {
    left: dyn Logic,
    right: dyn Logic,
}

impl Logic for IfAndOnlyIf {
    fn eval(&self) -> bool {
        self.left.eval() == self.right.eval()
    }
}

struct Deduction {
    conditions: Vec<dyn Logic>,
    claim: dyn Logic,
}

impl Logic for Deduction {
    fn eval(&self) -> bool {
        for c in self.conditions {
            if c.eval() && !self.claim.eval() {
                return false;
            }
        }
        true
    }
}

/*
enum {
    PRED(Predicate),
    NOT(Not),
    AND(And),
    OR(Or),
    XOR(Xor),
    IMPL(Implication),
    IFOIF(IfAndOnlyIf),
    DEDUC(Deduction),
}
*/


struct Engine {
    grid: HashMap<dyn Logic, Vec<bool>>,
}

impl Engine {
    fn new(statements: Vec<dyn Logic>) -> Self {
        let map: HashMap<dyn Logic, Vec<bool>> = HashMap::new();
        for s in statements {
            todo!()
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    file: String,

    #[arg(short, long)]
    verbose: bool,
}