use std::{collections::VecDeque, fmt::Display};

type Num=f64;

#[derive(Debug, Clone)]
struct State {
    fours_used: usize,
    path: VecDeque<(Action, Num)>,
    stack: VecDeque<Num>
}

impl State {
    fn new() -> Self {
        State {
            fours_used: 0,
            path: VecDeque::new(),
            stack: VecDeque::new(),
        }
    }

    fn fmt(f: &mut std::fmt::Formatter<'_>, actions: &mut VecDeque<Action>) -> std::fmt::Result {
        match actions.pop_back() {
            Some(Action::Abs) => {
                write!(f, "|")?;
                Self::fmt(f, actions)?;
                write!(f, "|")?;
            },
            Some(Action::Add) => {
                write!(f, "(")?;
                Self::fmt(f, actions)?;
                write!(f, ")+(")?;
                Self::fmt(f, actions)?;
                write!(f, ")")?;
            }
            Some(_) => todo!(),
            None => todo!(),
        }
        Ok(())
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path : VecDeque<_> = self.path.iter().map(|(a, _)| *a).collect();
        State::fmt(f, &mut path)
    }
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Add,
    Sub,
    Mul,
    Div,
    SquareRoot,
    Root,
    Push4,
    Power,
    Floor,
    Ceiling,
    Abs
}

impl Action {
    const VALUES: [Self; 11] = [
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
                    let right = s.stack.pop_back().unwrap();
                    let left = s.stack.pop_back().unwrap();
                    Some(left - right)
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
                    let right = s.stack.pop_back().unwrap();
                    let left = s.stack.pop_back().unwrap();
                    if right == 0.0 {
                        s.stack.push_back(left);
                        s.stack.push_back(right);
                        None
                    } else {
                        Some(left / right)
                    }
                } else {
                    None
                }
            },
            Action::SquareRoot => {
                if s.stack.len() >= 1 {
                    let n = s.stack.pop_back().unwrap();
                    Some(n.sqrt())
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
                    let power = s.stack.pop_back().unwrap();
                    let base = s.stack.pop_back().unwrap();
                    Some(base.powf(power))
                } else {
                    None
                }
            },
            Action::Root => {
                if s.stack.len() >= 2 {
                    let power = s.stack.pop_back().unwrap();
                    let base = s.stack.pop_back().unwrap();
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

fn explore(s: &mut State, found: &mut Vec<Option<State>>, depth: usize) -> bool {

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
                if closest > 0.0 && (result - closest).abs() < 0.0000001 {
                    if let Some(slot) = found.get_mut(closest as usize) {
                        // println!("{:?} = {}", &s.path, result);
                        if slot.is_none() || slot.as_ref().unwrap().path.len() > s.path.len() {
                            // dbg!(result, &s.path);

                            *slot = Some(s.clone());

                            let total = found.len();
                            let found_already = found.iter().filter(|x| x.is_some()).count();

                            println!("{:?} = {} [{} of {} found]", &s.path, result, found_already, total);

                            if found_already == total {
                                return true;
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
        println!("{} = {:?}", i, s.as_ref().map(|s| &s.path));
    }

    println!("Found {} of [0,100]", (0..=100).filter(|i| found[*i].is_some()).count());
}
