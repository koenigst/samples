#[derive(Copy, Clone)]
pub struct Location(pub usize, pub usize);

impl std::ops::Add<Translation> for Location {
    type Output = Option<Location>;
    
    fn add(self, rhs: Translation) -> Self::Output {
        Option::zip(
            try_add(self.0, rhs.0),
            try_add(self.1, rhs.1))
            .map(|(l, c)| Location(l, c))
    }
}

#[derive(Copy, Clone)]
pub struct Direction(pub i32, pub i32);

impl Direction {
    pub fn invert(self) -> Self {
        Self(-self.0, -self.1)
    }
}

impl Into<Translation> for Direction {    
    fn into(self) -> Translation {
        Translation(self.0, self.1)
    }
}

impl std::ops::Mul<usize> for Direction {
    type Output = Option<Translation>;
    
    fn mul(self, rhs: usize) -> Self::Output {
        Option::zip(
            try_mul(rhs, self.0),
            try_mul(rhs, self.1))
            .map(|(l, c)| Translation(l, c))
    }
}

#[derive(Copy, Clone)]
pub struct Translation(i32, i32);

fn try_add<L, R, O>(left: L, right: R) -> Option<O> 
where 
    L: TryInto<R>,
    R: TryInto<O> + std::ops::Add<R, Output = R>,
{
    try_op(left, right, |l, r| l + r)
}

fn try_mul<L, R, O>(left: L, right: R) -> Option<O> 
where 
    L: TryInto<R>,
    R: TryInto<O> + std::ops::Mul<R, Output = R>,
{
    try_op(left, right, |l, r| l * r)
}

fn try_op<L, R, RRR, O>(left: L, right: R, op: RRR) -> Option<O> 
where 
    L: TryInto<R>,
    R: TryInto<O>,
    RRR: FnOnce(R, R) -> R
{
    left
        .try_into()
        .ok()
        .map(|l| op(l, right))
        .and_then(|o| o.try_into().ok())
}
