use std::collections::VecDeque;

type Num=isize;

#[derive(Debug, Clone)]
struct State {
    fours_used: usize,
    path: VecDeque<Action>,
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
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Add,
    Sub,
    Mul,
    Div,
    SquareRoot,
    FourthRoot,
    Push4,
    Factorial,
    Power,
}

impl Action {
    const VALUES: [Self; 9] = [Self::Add, Self::Sub, Self::Mul, Self::Div, Self::Push4, Self::SquareRoot, Self::FourthRoot, Self::Factorial, Self::Power];

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
                    if right == 0 || left % right != 0 {
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
                    let r = (n as f64).sqrt() as Num;
                    if n > 1 && r * r == n {
                        Some(r)
                    } else {
                        s.stack.push_back(n);
                        None
                    }
                } else {
                    None
                }
            },
            Action::FourthRoot => {
                if s.stack.len() >= 1 {
                    let n = s.stack.pop_back().unwrap();
                    let r = (n as f64).sqrt().sqrt() as Num;
                    if n > 1 && r * r * r * r == n {
                        Some(r)
                    } else {
                        s.stack.push_back(n);
                        None
                    }
                } else {
                    None
                }
            }
            Action::Push4 => {
                if s.fours_used < 4 {
                    s.fours_used += 1;
                    Some(4)
                } else {
                    None
                }
            },
            Action::Factorial => {
                if s.stack.len() >= 1 {
                    let mut n = s.stack.pop_back().unwrap();
                    if n > 2 && n < 10 {
                        for i in 2..n {
                            n *= i;
                        }
                        Some(n)
                    } else {
                        s.stack.push_back(n);
                        None
                    }
                } else {
                    None
                }
            },
            Action::Power => {
                let ipower = |base: Num, power: Num| {
                    if power == 0 {
                        Some(1)
                    } else if power > 0 {
                        let mut p = 1 as Num;
                        for _ in 0..power {
                            if let Some(pp) = p.checked_mul(base) {
                                p = pp;
                            } else {
                                return None;
                            }
                        }
                        Some(p)
                    } else {
                        None
                    }
                };

                if s.stack.len() >= 2 {
                    let power = s.stack.pop_back().unwrap();
                    let base = s.stack.pop_back().unwrap();

                    if let Some(p) = ipower(base, power) {
                        Some(p)
                    } else {
                        s.stack.push_back(base);
                        s.stack.push_back(power);
                        None
                    }
                } else {
                    None
                }
            },
        }
    }
}

fn explore(s: &mut State, found: &mut Vec<Option<State>>, depth: usize) {

    assert!(depth < 50);

    let prev_state = s.clone();

    for a in &Action::VALUES {
        if let Some(result) = a.apply(s) {

            // println!("{:?}", &s.path);
            s.stack.push_back(result);
            s.path.push_back(*a);

            // check to see if we are done
            if s.fours_used == 4 && s.stack.len() == 1 {
                if let Ok(result) = usize::try_from(result) {
                    if let Some(found) = found.get_mut(result) {
                        // println!("{:?} = {}", &s.path, result);
                        if found.is_none() {
                            // dbg!(result, &s.path);
                            println!("{:?} = {}", &s.path, result);
                            *found = Some(s.clone());
                        }
                    }
                }
            } else if result.abs() < 1_000_000 {
                // otherwise recurse
                explore(s, found, depth + 1)
            }

            *s = prev_state.clone();
        }
    }
}

fn main() {
    let mut s = State::new();
    let mut found = vec![None; 110];
    explore(&mut s, &mut found, 0);

    for (i, s) in found.iter().enumerate() {
        println!("{} = {:?}", i, s.as_ref().map(|s| &s.path));
    }

    println!("Found {} of [0,100]", (0..=100).filter(|i| found[*i].is_some()).count());
}
