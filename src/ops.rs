pub trait Dot {
    type Output;

    fn dot(self, rhs: Self) -> Self::Output;
}

pub trait Sum {
    type Output;

    fn sum(self) -> Self::Output;
}
