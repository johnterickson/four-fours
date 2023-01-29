use std::{collections::VecDeque, fmt::Display};

type Num=f64;

#[derive(Debug, Clone)]
struct State {
    fours_used: usize,
    path: VecDeque<(Action, Num)>,
    stack: VecDeque<Num>
}

#[derive(Debug)]
struct Expression {
    action: Action,
    args: Vec<Expression>
}

impl Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, has_parens: bool, parent_action: Option<Action>) -> std::fmt::Result {
        match self.action {
            Action::Push4 => {
                write!(f, "4")?;
            }
            Action::Abs => {
                write!(f, "|")?;
                self.args[0].fmt(f, true, Some(self.action))?;
                write!(f, "|")?;
            },
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
            Action::Add | Action::Sub | Action::Mul | Action::Div | Action::Power => {
                let needs_parens = parent_action.is_some() && parent_action != Some(self.action);
                if needs_parens && !has_parens {
                    write!(f, "(")?;
                }
                self.args[0].fmt(f, false, Some(self.action))?;
                write!(f, "{}",
                    match self.action {
                        Action::Add => '+',
                        Action::Sub => '-',
                        Action::Mul => '*',
                        Action::Div => '/',
                        Action::Power => '^',
                        _ => panic!(),
                    })?;
                self.args[1].fmt(f, false, Some(self.action))?;
                if needs_parens && !has_parens {
                    write!(f, ")")?;
                }
            }
            Action::Root => {
                if !has_parens {
                    write!(f, "(")?;
                }
                write!(f, "(")?;
                self.args[0].fmt(f, true, Some(self.action))?;
                write!(f, ")^(")?;
                self.args[1].fmt(f, true, Some(self.action))?;
                write!(f, ")")?;
                if !has_parens {
                    write!(f, ")")?;
                }
            }
        }
        Ok(())
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, true, None)
    }
}

impl State {
    fn new() -> Self {
        State {
            fours_used: 0,
            path: VecDeque::new(),
            stack: VecDeque::new(),
        }
    }

    fn make_expression(&self) -> Expression {
        let mut stack = VecDeque::new();
        // println!("make_expression {:?}", &self);
        for (a, _) in &self.path {
            // println!("{:?} {:?}", a, &stack);
            match a {
                Action::Add | Action::Sub | Action::Mul | Action::Div | Action::Power | Action::Root => {
                    let first = stack.pop_back().unwrap();
                    let second = stack.pop_back().unwrap();
                    stack.push_back(Expression { action: *a, args: vec![first, second]});
                }
                Action::SquareRoot | Action::FourthRoot | Action::Floor | Action::Ceiling | Action::Abs => {
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    // use super::*;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Action {
    Add,
    Sub,
    Mul,
    Div,
    SquareRoot,
    FourthRoot,
    Root,
    Push4,
    Power,
    Floor,
    Ceiling,
    Abs
}

impl Action {
    const VALUES: [Self; 12] = [
        Self::Push4,
        Self::Add,
        Self::Sub,
        Self::Mul,
        Self::Div,
        Self::Power,
        Self::Floor,
        Self::Ceiling,
        Self::Abs,
        Self::SquareRoot,
        Self::FourthRoot,
        Self::Root
    ];

    fn apply(&self, s: &mut State) -> Option<Num> {
        match self {
            Action::Add => {
                if s.stack.len() >= 2 {
                    Some(s.stack.pop_back().unwrap() + s.stack.pop_back().unwrap())
                } else {
                    None
                }
            }
            Action::Sub => {
                if s.stack.len() >= 2 {
                    let first = s.stack.pop_back().unwrap();
                    let second = s.stack.pop_back().unwrap();
                    Some(first - second)
                } else {
                    None
                }
            },
            Action::Mul => {
                if s.stack.len() >= 2 {
                    Some(s.stack.pop_back().unwrap() * s.stack.pop_back().unwrap())
                } else {
                    None
                }
            }
            Action::Div => {
                if s.stack.len() >= 2 {
                    let first = s.stack.pop_back().unwrap();
                    let second = s.stack.pop_back().unwrap();
                    if second == 0.0 {
                        s.stack.push_back(second);
                        s.stack.push_back(first);
                        None
                    } else {
                        Some(first / second)
                    }
                } else {
                    None
                }
            },
            Action::SquareRoot => {
                if s.stack.len() >= 1 {
                    Some(s.stack.pop_back().unwrap().sqrt())
                } else {
                    None
                }
            },
            Action::FourthRoot => {
                if s.fours_used < 4 && s.stack.len() >= 1 {
                    s.fours_used += 1;
                    Some(s.stack.pop_back().unwrap().sqrt().sqrt())
                } else {
                    None
                }
            },
            Action::Push4 => {
                if s.fours_used < 4 {
                    s.fours_used += 1;
                    Some(4.0)
                } else {
                    None
                }
            },
            Action::Power => {
                if s.stack.len() >= 2 {
                    let base = s.stack.pop_back().unwrap();
                    let power = s.stack.pop_back().unwrap();
                    Some(base.powf(power))
                } else {
                    None
                }
            },
            Action::Root => {
                if s.stack.len() >= 2 {
                    let base = s.stack.pop_back().unwrap();
                    let power = s.stack.pop_back().unwrap();
                    Some(base.powf(1.0/power))
                } else {
                    None
                }
            },
            Action::Floor => {
                if s.stack.len() >= 1 {
                    let n = s.stack.pop_back().unwrap();
                    let closest = n.round();
                    if (closest - n).abs() < 0.001 {
                        s.stack.push_back(n);
                        None
                    } else {
                        Some(n.floor())
                    }
                } else {
                    None
                }
            },
            Action::Ceiling => {
                if s.stack.len() >= 1 {
                    let n = s.stack.pop_back().unwrap();
                    let closest = n.round();
                    if (closest - n).abs() < 0.001 {
                        s.stack.push_back(n);
                        None
                    } else {
                        Some(n.floor())
                    }
                } else {
                    None
                }
            },
            Action::Abs =>  {
                if s.stack.len() >= 1 {
                    let n = s.stack.pop_back().unwrap();
                    if n >= 0.0 {
                        s.stack.push_back(n);
                        None
                    } else {
                        Some(n.abs())
                    }
                } else {
                    None
                }
            },
        }
    }
}

fn explore(s: &mut State, found: &mut Vec<Option<(State,usize)>>, depth: usize) -> bool {

    assert!(depth < 50);

    let prev_state = s.clone();
    // let prev_result = prev_state.stack.back();

    for a in &Action::VALUES {
        if let Some(result) = a.apply(s) {

            if !result.is_finite() {
                *s = prev_state.clone();
                continue;
            }

            // println!("{:?} = {}", &s.path, result);
            s.path.push_back((*a, /* prev_state.stack.clone(), */ result));
            s.stack.push_back(result);

            // check to see if we are done
            if s.fours_used == 4 && s.stack.len() == 1 {
                let closest = result.round();
                if closest >= 0.0 && (result - closest).abs() < 0.0000001 {
                    let closest = closest as usize;
                    if let Some(slot) = found.get_mut(closest) {
                        // println!("{:?} = {}", &s.path, result);

                        if slot.is_none() || slot.as_ref().unwrap().0.path.len() >= s.path.len() {
                            // dbg!(result, &s.path);

                            let e = format!("{}", s.make_expression());

                            if slot.is_none() || e.len() < slot.as_ref().unwrap().1 {

                                *slot = Some((s.clone(), e.len()));

                                let total = found.len();
                                let found_already = found.iter().filter(|x| x.is_some()).count();

                                println!("{:?} = {} = {} [{} of {} found]", &s.path, &e, closest, found_already, total);

                                if found_already == total {
                                    return true;
                                }
                            }
                        }
                    }
                }
            } else if /*prev_result.is_none() ||  (prev_result.unwrap() - result).abs() > 0.01 &&*/ depth < 25 {
            // } else if depth < 20 {
                // otherwise recurse
                if explore(s, found, depth + 1) {
                    return true;
                }
            }

            *s = prev_state.clone();
        }
    }

    false
}

fn main() {
    let mut s = State::new();
    let mut found = vec![None; 101];
    explore(&mut s, &mut found, 0);

    for (i, s) in found.iter().enumerate() {
        if let Some(s) = s.as_ref() {
            println!("{} = {}", i, s.0.make_expression());
        } else {
            println!("{} = ?", i);
        }
    }

    println!("Found {} of [0,100]", (0..=100).filter(|i| found[*i].is_some()).count());
}
