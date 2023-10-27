
pub struct Val {
    value: i32,
}

pub struct AddOp<T1: HasValue + Copy, T2: HasValue + Copy> {
    lhs: T1,
    rhs: T2,
}

pub struct MulOp<T1: HasValue + Copy, T2: HasValue + Copy> {
    lhs: T1,
    rhs: T2,
}

pub struct Calculus<T> {
    calc: T,
}
