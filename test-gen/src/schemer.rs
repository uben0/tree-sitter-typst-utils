use std::io::Write;

pub struct Writter<W: Write> {
    color: bool,
    writter: W,
    indent: usize,
}
impl<W: Write> Writter<W> {
    const NAME: &str = "\x1b[94m";
    // const LABEL: &str = "\x1b[95m";
    const RESET: &str = "\x1b[0m";
    pub fn root(writter: W, root: impl Fn(Self) -> Self, color: bool) {
        root(Self {
            writter,
            indent: 0,
            color,
        });
    }
    pub fn colored(&self) -> bool {
        self.color
    }
    pub fn param(mut self, param: impl Writtable) -> Self {
        write!(self.writter, " ").unwrap();
        param.write(self)
    }
    // pub fn label(mut self, label: &str) -> Self {
    //     if self.color {
    //         write!(self.writter, "{}{}{}", Self::LABEL, label, Self::RESET).unwrap();
    //     } else {
    //         write!(self.writter, "{}", label).unwrap();
    //     }
    //     self
    // }
    pub fn line(mut self) -> Self {
        writeln!(self.writter).unwrap();
        for _ in 0..self.indent {
            write!(self.writter, "    ").unwrap();
        }
        self
    }
    pub fn node(mut self, name: &str, inside: impl Fn(Self) -> Self) -> Self {
        self.indent += 1;
        write!(self.writter, "(").unwrap();
        if self.color {
            write!(self.writter, "{}{}{}", Self::NAME, name, Self::RESET).unwrap();
        } else {
            write!(self.writter, "{}", name).unwrap();
        }
        self = inside(self);
        write!(self.writter, ")").unwrap();
        self.indent -= 1;
        self
    }
    pub fn fold<T>(self, iter: impl IntoIterator<Item = T>, map: impl Fn(Self, T) -> Self) -> Self {
        iter.into_iter().fold(self, map)
    }
}

pub trait Writtable {
    fn write<W: Write>(self, writter: Writter<W>) -> Writter<W>;
}
impl Writtable for u32 {
    fn write<W: Write>(self, mut writter: Writter<W>) -> Writter<W> {
        write!(writter.writter, "{}", self).unwrap();
        writter
    }
}
impl Writtable for bool {
    fn write<W: Write>(self, mut writter: Writter<W>) -> Writter<W> {
        write!(writter.writter, "{}", self).unwrap();
        writter
    }
}
impl Writtable for usize {
    fn write<W: Write>(self, mut writter: Writter<W>) -> Writter<W> {
        write!(writter.writter, "{}", self).unwrap();
        writter
    }
}
impl Writtable for i32 {
    fn write<W: Write>(self, mut writter: Writter<W>) -> Writter<W> {
        write!(writter.writter, "{:+}", self).unwrap();
        writter
    }
}
impl Writtable for f32 {
    fn write<W: Write>(self, mut writter: Writter<W>) -> Writter<W> {
        write!(writter.writter, "{:+}", self).unwrap();
        writter
    }
}
impl Writtable for &str {
    fn write<W: Write>(self, mut writter: Writter<W>) -> Writter<W> {
        write!(writter.writter, "{:?}", self).unwrap();
        writter
    }
}
