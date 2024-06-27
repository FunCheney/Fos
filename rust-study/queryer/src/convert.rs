use anyhow::anyhow;
use sqlparser::ast::{Expr, Select, SetExpr, Statement, Value};

pub struct Sql<'a> {
    pub(crate) selection: Vec<Expr>,
    pub(crate) condition: Option<Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<f64>,
    pub(crate) limit: Option<usize>,
}

use sqlparser::ast::Offset as SqlOffset;
pub struct Offset<'a> (pub(crate) &'a SqlOffset);

impl <'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
        match sql {
            Statement::Query(q) => {
                let Select {
                    from: table_with_joins,
                    selection: where_clause,
                    projection,

                    group_by: _,
                    ..
                } = match &q.body {
                    SetExpr::Select(statement) => statement.as_ref(),
                    _ => return Err(anyhow!("We only support Select Query at the moment")),
                };
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

/// 把 SqlParser 的 offset expr 转化为 i64
impl <'a> From<Offset<'a>> for i64 {
    fn from(offset: Offset<'a>) -> Self {
        match offset.0 {
            SqlOffset {
                value: Expr::Value(Value::Number(v, _b)),
                ..
            } => v.parse().unwrap_or(0),
            _ => 0,
        }
    }
}