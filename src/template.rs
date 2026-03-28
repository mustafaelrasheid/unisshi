use serde::Deserialize;

#[derive(Clone)]
#[derive(Deserialize)]
pub struct Check {
    pub prompt: String,
    pub item: String,
    pub expected: bool,
}

#[derive(Deserialize)]
pub struct Query {
    pub prompt: String,
    pub item: String,
}

#[derive(Deserialize)]
pub struct Template {
    pub checks: Option<Vec<Check>>,
    pub queries: Option<Vec<Query>>,
}
