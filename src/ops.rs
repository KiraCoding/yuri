pub trait Dot {
    type Output;

    fn dot(self) -> Self::Output;
}

pub trait Sum {
    type Output;

    fn sum(self) -> Self::Output;
}