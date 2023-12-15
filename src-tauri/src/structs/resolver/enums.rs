use serde::{Deserialize, Serialize};

use super::entities::{Atom, ObjectLike};

#[derive(PartialEq, Debug)]
pub enum JsonType {
    Object,
    Array,
    Atom,
    Unknown,
}

#[derive(PartialEq, Debug, Deserialize, Serialize , Clone)]
pub enum FormType {
    Form,
    Json,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ObjectType {
    Object,
    Array,
}

#[derive(PartialEq, Debug)]
pub enum WebType {
    Request,
    Response,
}

#[derive(Clone, Debug)]
pub enum JsonValue {
    ObjectLike(ObjectLike),
    Atom(Atom),
    Null
}
