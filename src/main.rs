#[derive(Debug)]
enum Operand {
    Not,
    And,
    Or,
    Xor,
    If,
    Iff,
}
fn op_order(op: &Operand) -> i32 {
    match op {
        Operand::Not => 1,
        Operand::And => 2,
        Operand::Or => 3,
        Operand::Xor => 4,
        Operand::If => 5,
        Operand::Iff => 6,
    }
}

#[derive(Debug)]
struct Proposition {
    name: String,
    truth: Option<bool>,
    operand: Option<Operand>,
}
impl Proposition {
    fn new(name: String, truth: Option<bool>) -> Self {
        Self {
            name,
            truth,
            operand: None,
        }
    }
    fn name_fmt(&self, op: &Operand) -> String {
        if self.operand.is_some() && op_order(self.operand.as_ref().unwrap()) > op_order(op) {
            format!("({})", self.name.clone())
        } else {
            self.name.clone()
        }
    }
    fn and(&self, other: &Proposition) -> Self {
        Self {
            name: format!(
                "{}^{}",
                self.name_fmt(&Operand::And),
                other.name_fmt(&Operand::And)
            ),
            truth: match self.truth {
                Some(true) => match other.truth {
                    Some(true) => Some(true),
                    Some(false) => Some(false),
                    None => other.truth,
                },
                Some(false) => Some(false),
                None => match other.truth {
                    Some(true) => self.truth,
                    Some(false) => Some(false),
                    None => None,
                },
            },
            operand: Some(Operand::And),
        }
    }
    fn or(&self, other: &Proposition) -> Self {
        Self {
            name: format!(
                "{}v{}",
                self.name_fmt(&Operand::Or),
                other.name_fmt(&Operand::Or)
            ),
            truth: match &self.truth {
                Some(true) => Some(true),
                Some(false) => match other.truth {
                    Some(true) => Some(true),
                    Some(false) => Some(false),
                    None => other.truth,
                },
                None => match other.truth {
                    Some(true) => Some(true),
                    Some(false) => self.truth,
                    None => None,
                },
            },
            operand: Some(Operand::Or),
        }
    }
    fn not(&self) -> Self {
        Self {
            name: format!("~{}", self.name_fmt(&Operand::Not)),
            truth: match self.truth {
                Some(true) => Some(false),
                Some(false) => Some(true),
                None => None,
            },
            operand: Some(Operand::Not),
        }
    }
    fn xor(&self, other: &Proposition) -> Self {
        Self {
            name: format!(
                "{}(+){}",
                self.name_fmt(&Operand::Xor),
                other.name_fmt(&Operand::Xor)
            ),
            // (self or other) and not (self and other)
            truth: (self.or(other).and(&self.and(other).not())).truth,
            operand: Some(Operand::Xor),
        }
    }
    fn cond(&self, other: &Proposition) -> Self {
        Self {
            name: format!(
                "{}->{}",
                &self.name_fmt(&Operand::If),
                &other.name_fmt(&Operand::If)
            ),
            // self or not other
            truth: self.or(&other.not()).truth,
            operand: Some(Operand::If),
        }
    }
    fn bicond(&self, other: &Proposition) -> Self {
        Self {
            name: format!(
                "({})<->({})",
                &self.name_fmt(&Operand::Iff),
                &other.name_fmt(&Operand::Iff)
            ),
            // (self and other) or (not self and not other)
            truth: (self.and(other).or(&self.not().and(&other.not()))).truth,
            operand: Some(Operand::Iff),
        }
    }
    fn is_axiom(&self) -> bool {
        self.operand.is_none()
    }
    fn is_tautology(&self) -> bool {
        self.truth == Some(true)
    }
    fn is_contradiction(&self) -> bool {
        self.truth == Some(false)
    }
}

fn main() {
    let p = Proposition::new("p".to_string(), None);
    let q = Proposition::new("q".to_string(), Some(true));
    let p_or_q = Proposition::or(&p, &q);
    let if_p_then_q = Proposition::cond(&p, &(q.not()));
    let taut = Proposition::bicond(&p_or_q, &if_p_then_q);
    println!("{:?}, {:?}", p_or_q, if_p_then_q);
    println!(
        "{:?}\n{}",
        &taut,
        taut.is_tautology()
    );
}
