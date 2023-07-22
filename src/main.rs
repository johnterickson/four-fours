use std::{collections::VecDeque, fmt::Display, ops::Neg, sync::{Mutex, Arc}};

type Num=f64;

use smallvec::SmallVec;

type Stack=SmallVec<[Num; MAX_STACK_DEPTH]>;

const MAX_STACK_DEPTH: usize = 4;
const MAX_SEARCH_DEPTH: usize = 11;

#[derive(Debug, Clone, PartialEq)]
struct State {
    fours_used: usize,
    stack: Stack,
}

impl State {
    pub fn new() -> State {
        State {
            fours_used: 0,
            stack: Stack::new(),
        }
    }
}

#[derive(Debug, Clone)]
struct Search {
    path: VecDeque<(State, Action, Num)>,
}

impl Search {
    fn new() -> Self {
        Search {
            path: VecDeque::new(),
        }
    }

    fn make_expression(&self) -> Expression {
        let mut stack = VecDeque::new();
        // println!("make_expression {:?}", &self);
        for (_, a, _) in &self.path {
            // println!("{:?} {:?}", a, &stack);
            match a {
                Action::Add | Action::Sub | Action::Mul | Action::Div | Action::Power | Action::Modulo | Action::FourthRoot => {
                    let first = stack.pop_back().unwrap();
                    let second = stack.pop_back().unwrap();
                    stack.push_back(Expression { action: *a, args: vec![first, second]});
                }
                Action::SquareRoot | Action::Floor | Action::Ceiling | Action::Abs | Action::Negate | Action::Factorial => {
                    let first = stack.pop_back().unwrap();
                    stack.push_back(Expression { action: *a, args: vec![first]});
                }
                Action::Push4 => stack.push_back(Expression { action: *a, args: Vec::new() }),
            }
        }

        assert_eq!(1, stack.len());

        stack.pop_back().unwrap()
    }
}

#[derive(Debug)]
struct Expression {
    action: Action,
    args: Vec<Expression>
}

impl Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, has_parens: bool, parent_action: Option<Action>) -> std::fmt::Result {
        let needs_parens_around = if self.action.communative() && parent_action == Some(self.action) {
            false
        } else {
            self.action.needs_parens_around() & !has_parens
        };

        if needs_parens_around {
            write!(f, "(")?;
        }

        match self.action {
            Action::Push4 => {
                write!(f, "4")?;
            }
            Action::Negate => {
                write!(f, "-")?;
                self.args[0].fmt(f, false, Some(self.action))?;
            }
            Action::Factorial => {
                self.args[0].fmt(f, false, Some(self.action))?;
                write!(f, "!")?;
            }
            Action::Abs => {
                write!(f, "|")?;
                self.args[0].fmt(f, true, Some(self.action))?;
                write!(f, "|")?;
            }
            Action::FourthRoot => {
                write!(f, "∜(")?;
                self.args[0].fmt(f, true, Some(self.action))?;
                write!(f, ")")?;
            }
            Action::SquareRoot => {
                write!(f, "√(")?;
                self.args[0].fmt(f, true, Some(self.action))?;
                write!(f, ")")?;
            }
            Action::Floor => {
                write!(f, "⌊")?;
                self.args[0].fmt(f, true, Some(self.action))?;
                write!(f, "⌋")?;
            }
            Action::Ceiling => {
                write!(f, "⌈")?;
                self.args[0].fmt(f, true, Some(self.action))?;
                write!(f, "⌉")?;
            }
            Action::Add | Action::Sub | Action::Mul | Action::Div | Action::Power | Action::Modulo => {                
                self.args[0].fmt(f, false, Some(self.action))?;
                write!(f, "{}",
                    match self.action {
                        Action::Add => "+",
                        Action::Sub => "-",
                        Action::Mul => "*",
                        Action::Div => "/",
                        Action::Power => "^",
                        Action::Modulo => "mod",
                        _ => panic!(),
                    })?;
                self.args[1].fmt(f, false, Some(self.action))?;
            }
        }

        if needs_parens_around {
            write!(f, ")")?;
        }

        Ok(())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, true, None)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Action {
    Add,
    Sub,
    Mul,
    Div,
    Modulo,
    SquareRoot,
    FourthRoot,
    Push4,
    Power,
    Floor,
    Ceiling,
    Abs,
    Negate,
    Factorial,
}

impl Action {
    const VALUES: [Self; 14] = [
        Self::Push4,
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Div,
        Self::Modulo,
        Self::Power,
        Self::Floor,
        Self::Ceiling,
        Self::Abs,
        Self::Negate,
        Self::SquareRoot,
        Self::FourthRoot,
        Self::Factorial
    ];

    fn communative(&self) -> bool {
        match self {
            Action::Add | Action::Mul => true,
            _ => false
        }
    }

    fn needs_parens_around(&self) -> bool {
        match self {
            Action::Add | Action::Sub | Action::Mul | Action::Div | Action::Modulo | Action::Factorial | Action::Power | Action::Negate
                => true,
            Action::SquareRoot | Action::FourthRoot | Action::Push4 | Action::Floor | Action::Ceiling | Action::Abs
                => false,
        }
    }

    fn apply(&self, s: &State) -> Option<(State, Action, Num)> {
        match self {
            Action::Add | Action::Sub | Action::Mul | Action::Power => {
                if s.stack.len() >= 2 {
                    let mut s = s.clone();
                    let a = s.stack.pop().unwrap();
                    let b = s.stack.pop().unwrap();
                    Some((s, *self, match self {
                        Action::Add => a + b,
                        Action::Sub => a - b,
                        Action::Mul => a * b,
                        Action::Power => a.powf(b),
                        _ => unreachable!()
                    }))
                } else {
                    None
                }
            }
            Action::Div | Action::Modulo => {
                if s.stack.len() >= 2 {
                    let mut s = s.clone();
                    let a = s.stack.pop().unwrap();
                    let b = s.stack.pop().unwrap();
                    if b == 0.0 {
                        None
                    } else {
                        Some((s, *self, match self {
                            Action::Div => a / b,
                            Action::Modulo => a % b,
                            _ => unreachable!()
                        }))
                    }
                } else {
                    None
                }
            },
            Action::SquareRoot => {
                if s.stack.len() >= 1 && s.stack.last().unwrap() > &0.0 {
                    let mut s = s.clone();
                    let a = s.stack.pop().unwrap();
                    Some((s, *self, a.sqrt()))
                } else {
                    None
                }
            },
            Action::FourthRoot => {
                if s.stack.len() >= 2 && s.stack[s.stack.len() - 2] == 4.0 {
                    let mut s = s.clone();
                    let a = s.stack.pop().unwrap();
                    let b = s.stack.pop().unwrap();
                    Some((s, *self, a.powf(1.0 / b)))
                } else {
                    None
                }
            }
            Action::Push4 => {
                if s.fours_used < 4 {
                    let mut s = s.clone();
                    s.fours_used += 1;
                    Some((s, *self, 4.0))
                } else {
                    None
                }
            },
            Action::Floor | Action::Ceiling => {
                if s.stack.len() >= 1 {
                    let mut s = s.clone();
                    let n = s.stack.pop().unwrap();
                    let closest = n.round();
                    if (closest - n).abs() < 0.001 {
                        None
                    } else {
                        Some((s, *self, match self {
                            Action::Floor => n.floor(),
                            Action::Ceiling => n.ceil(),
                            _ => unreachable!()
                        }))
                    }
                } else {
                    None
                }
            },
            Action::Abs | Action::Negate =>  {
                if s.stack.len() >= 1 {
                    if self == &Action::Abs && s.stack.last().unwrap() >= &0.0 {
                        None
                    } else {
                        let mut s = s.clone();
                        let n = s.stack.pop().unwrap();
                        Some((s, *self, match self {
                            Action::Abs => n.abs(),
                            Action::Negate => n.neg(),
                            _ => unreachable!()
                        }))
                    }
                } else {
                    None
                }
            },
            Action::Factorial =>  {
                if s.stack.len() >= 1 {
                    let n = *s.stack.last().unwrap();
                    if let Some(i) = strict_f64_to_i64(n) {
                        if let Some(f) = factorial(i) {
                            if strict_f64_to_i64(f as f64) == Some(f) {
                                let mut s = s.clone();
                                let _ = s.stack.pop();
                                Some((s, *self, f as f64))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        }
    }
}

pub fn strict_f64_to_i64(x: f64) -> Option<i64> {
    // Check if fractional component is 0 and that it can map to an integer in the f64
    // Using fract() is equivalent to using `as u64 as f64` and checking it matches
    let i = x as i64;
    if x.fract() == 0.0 && x >= i64::MIN as f64 && x <= i64::MAX as f64 && x == i as f64{
        return Some(i)
    }

    None
}

fn factorial(n: i64) -> Option<i64> {
    if n < 0 {
        return None;
    }

    let mut f: i64 = 1;
    for i in 2..=n {
        if let Some(p) = f.checked_mul(i) {
            f = p;
        } else {
            return None;
        }
    }

    Some(f)
}

fn explore(search: &mut Search, found: &Mutex<Vec<Option<(Search,String)>>>) {

    assert!(search.path.len() <= MAX_SEARCH_DEPTH);

    // dbg!(&s);

    let new_state = State::new();

    for a in &Action::VALUES {
        let (state, last_action) = if search.path.len() == 0 {
            (&new_state, None)
        } else {
            let last = search.path.back().unwrap();
            (&last.0, Some(last.1))
        };

        if let Some(last_action) = last_action {
            if match (last_action, a) {
                // no-ops
                (Action::Abs, Action::Abs) |
                (Action::SquareRoot, Action::Abs) |
                (Action::FourthRoot, Action::Abs) |
                (Action::Floor, Action::Floor) |
                (Action::Floor, Action::Ceiling) |
                (Action::Ceiling, Action::Floor) |
                (Action::Ceiling, Action::Ceiling) |
                // undos
                (Action::Negate, Action::Abs) |
                (Action::Abs, Action::Negate)
                    => true,
                _ => false,
            } {
                continue;
            }
        }

        if let Some((mut next_state, a, result)) = a.apply(state) {

            if !result.is_finite() {
                continue;
            }
            
            // no change
            if state == &next_state {
                dbg!((state, a));
                continue;
            }

            next_state.stack.push(result);

            let done = next_state.fours_used == 4 && next_state.stack.len() == 1;

            // println!("{:?} = {}", &s.path, result);
            search.path.push_back((next_state, a, result));

            // check to see if we are done
            if done {
                let closest = result.round();
                if closest >= 0.0 && (result - closest).abs() < 0.0000001 {
                    let closest = closest as usize;

                    let mut found = found.lock().unwrap();

                    if let Some(slot) = found.get_mut(closest) {
                        // println!("{:?} = {}", &s.path, result);

                        let existing_path_len = slot.as_ref().map(|(s,_)| s.path.len()).unwrap_or(usize::MAX);

                        if search.path.len() <= existing_path_len {
                            // dbg!(result, &s.path);

                            let e = format!("{}", search.make_expression());

                            let existing_expression_len = slot.as_ref().map(|(_,e)| e.chars().count()).unwrap_or(usize::MAX);
                            
                            if search.path.len() < existing_path_len || (search.path.len() == existing_path_len && e.chars().count() < existing_expression_len) {

                                *slot = Some((search.clone(), e.clone()));

                                let total = found.len();
                                let found_already = found.iter().filter(|x| x.is_some()).count();
                                let longest = found.iter()
                                    .filter_map(|s| s.as_ref())
                                    .map(|(s,_)| s.path.len())
                                    .max(); 

                                println!("{:?} = {} = {} [{} of {} found] Longest:{:?}", 
                                    &search.path.iter().map(|p| p.1).collect::<Vec<_>>(),
                                    &e, closest, found_already, total, longest);
                            }
                        }
                    }
                }
            } else if search.path.len() < MAX_SEARCH_DEPTH {
                explore(search, found);
            }

            let _ = search.path.pop_back();
        }
    }
}

fn main() {
    let found = Arc::new(Mutex::new(vec![None; 101]));

    // let cpus = 1;
    // let threads: Vec<_> = (0..cpus)
    //     .map(|i| {
    //         let found = found.clone();
    //         thread::spawn(move || {
    //             let mut s = State::new();
                
    //             eprintln!("Start {:?} complete.", i);
    //         })
    //     }).collect();

    // for t in threads {
    //     t.join().unwrap();
    // }
    let mut search = Search::new();
    explore(&mut search, &found);
    
    let found = found.lock().unwrap();

    for (i, s) in found.iter().enumerate() {
        if let Some(s) = s.as_ref() {
            println!("{} = {}", i, s.0.make_expression());
        } else {
            println!("{} = ?", i);
        }
    }

    println!("Found {} of [0,100]", (0..=100).filter(|i| found[*i].is_some()).count());
}
