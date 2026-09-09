use crate::parsemath::parser::Node;

pub fn eval(expr: Node) -> Result<f64, Box<dyn std::error::Error>> {
    use self::Node::*;
    match expr {
        Number(i) => Ok(i),
        Add(l, r) => Ok(eval(*l)? + eval(*r)?),
        Subtract(l, r) => Ok(eval(*l)? - eval(*r)?),
        Multiply(l, r) => Ok(eval(*l)? * eval(*r)?),
        Devide(l, r) => Ok(eval(*l)? / eval(*r)?),
        Caret(b, e) => Ok(eval(*b)?.powf(eval(*e)?)),
        Negative(num) => Ok(-(eval(*num)?)),
    }
}
