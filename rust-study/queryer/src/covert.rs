use anyhow::anyhow;
use sqlparser::ast::{Expr, Statement};

pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<f64>,
    pub(crate) limit: Option<usize>,
}

impl <'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
        match sql {
            Statement::Query(q) => {
                Ok(Sql {
                    selection,
                    condition,
                    source,
                    order_by,
                    offset,
                    limit,
                })
            },
            _ => Err(anyhow!("We only support Query at the moment")),
        }
    }
}