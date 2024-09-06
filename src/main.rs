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

struct Predicate {
    name: String,
    id: usize
}

impl Predicate {
    fn new(name: String) -> Self {
        Self {
            name,
            id: next_id(),
        }
    }
}

struct Not {
    val: Statement,
}

struct And {
    left: Statement,
    right: Statement,
}

struct Or {
    left: Statement,
    right: Statement,
}

struct Xor {
    left: Statement,
    right: Statement,
}

struct Implication {
    condition: Statement,
    claim: Statement,
}

struct IfAndOnlyIf {
    left: Statement,
    right: Statement,
}

struct Deduction {
    conditions: Vec<Statement>,
    claim: Statement,
}

enum Statement {
    PRED(Predicate),
    NOT(Not),
    AND(And),
    OR(Or),
    XOR(Xor),
    IMPL(Implication),
    IFOIF(IfAndOnlyIf),
    DEDUC(Deduction),
}


struct Engine {
    grid: HashMap<Statement, Vec<bool>>,
}

impl Engine {
    fn new(statements: Vec<Statement>) -> Self {
        let map: HashMap<Statement, Vec<bool>> = HashMap::new();
        for s in statements {
            s
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