use std::ops::{Add, Mul};
use crate::custom_info::{Val, AddOp, MulOp, Calculus};

pub trait HasValue {
    fn get_value(&self) -> i32;
}

impl HasValue for Val {
    fn get_value(&self) -> i32 {
        self.value
    }
}

impl<T1: HasValue + Copy, T2: HasValue + Copy> HasValue for AddOp<T1, T2> {
    fn get_value(&self) -> i32 {
        self.lhs.get_value() + self.rhs.get_value()
    }
}

impl<T: HasValue + Copy, O: HasValue + Copy> Add<O> for T {
    type Output = AddOp<T, O>;

    fn add(&self, other: &O) -> AddOp<T, O> {
        AddOp {
            lhs: *self,
            rhs: *other,
        }
    }
}

impl<T1: HasValue + Copy, T2: HasValue + Copy> HasValue for MulOp<T1, T2> {
    fn get_value(&self) -> i32 {
        self.lhs.get_value() * self.rhs.get_value()
    }
}

impl<T: HasValue + Copy, O: HasValue + Copy> Mul<O> for T {
    type Output = MulOp<T, O>;

    fn mul(&self, other: &O) -> MulOp<T, O> {
        MulOp {
            lhs: *self,
            rhs: *other,
        }
    }
}

impl<T: HasValue + Copy> HasValue for Calculus<T> {
    fn get_value(&self) -> i32 {
        self.calc.get_value()
    }
}

impl<T, O> Add<Calculus<O>> for Calculus<T>
where 
    T: HasValue + Copy,
    O: HasValue + Copy,
{
    type Output = Calculus<AddOp<T, O>>;

    fn add(self, other: Calculus<O>) -> Calculus<AddOp<T, O>> {
        Calculus {
            calc: AddOp {
                lhs: self.calc,
                rhs: other.calc,
            },
        }
    }
}

impl<T, O> Mul<Calculus<O>> for Calculus<T>
where
    T: HasValue + Copy,
    O: HasValue + Copy,
{
    type Output = Calculus<MulOp<T, O>>;

    fn mul(self, other: Calculus<O>) -> Calculus<MulOp<T, O>> {
        Calculus {
            calc: MulOp {
                lhs: self.calc,
                rhs: other. calc,
            },
        }
    }
}

impl Val {
    fn new(n: i32) -> Calculus<Val> {
        Calculus {
            calc: Val { value: n },
        }
    }
}