use std::fmt::Display;
use std::ops::Shl;

pub struct Cout;
impl<T: Display> Shl<T> for Cout {
    type Output = Cout;
    fn shl(self, data: T) -> Cout {
        print!("{}", data);
        self
    }
}
pub struct Endl;
impl Display for Endl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n")
    }
}
