pub enum Neighborhood {
    Moore(i32),
}

impl Neighborhood {
    pub fn neighbors(&self) -> impl Iterator<Item = (i32, i32)> {
        match self {
            Neighborhood::Moore(n) => Self::moore(*n*2 + 1)
        }
    }
    fn moore(n: i32) -> impl Iterator<Item = (i32, i32)> {
        (0..n*n).map(move |i|(i%n-(n-1)/2, i/n-(n-1)/2)).filter(|(i, j)|!(*i==0 && *j==0))
    }
}
