use std::rc::Rc;

pub type Q = Vec<String>;
pub type Db = Rc<Vec<Q>>;

pub type Param<'a> = Vec<&'a str>;
pub type Params<'a> = Vec<Param<'a>>;