use anyhow::{anyhow, Ok, Result};
use sqlparser::ast::{
    BinaryOperator as SqlBinaryOperator, Expr as SqlExpr, Offset as SqlOffset, OrderByExpr, Select,
    SelectItem, SetExpr, Statement, TableFactor, TableWithJoins, Value as SqlValue,
};

use polars::prelude;

// 定义数据结构: elements of a sql sentence: like select condition limit order_by 等
// `SELECT * FROM customers WHERE last_name = 'Smith';`
pub struct Sql<'a> {
    pub(crate) selection: Vec<prelude::Expr>,
    pub(crate) condition: Option<prelude::Expr>,
    pub(crate) source: &'a str,
    pub(crate) order_by: Vec<(String, bool)>,
    pub(crate) offset: Option<i64>,
    pub(crate) limit: Option<usize>,
}

// 自定义的类型要使用第三方库中类型的方法或者关联函数有两种方法：
// 一是为其实现 trait
// 二是把自自定义的类型转换为第三方库的类型。关键 trait TryFrom 和 From
// 具体就是实现 From 或者 TryFrom trait, 一般约束是源类型，rust中的约束真的很强啊

// tryFrom 的本质 从匹配到取引用，再到将引用内部几个字段赋值给几个变量

impl<'a> TryFrom<&'a Statement> for Sql<'a> {
    type Error = anyhow::Error;

    fn try_from(sql: &'a Statement) -> Result<Self, Self::Error> {
        match sql {
            Statement::Query(q) => {
                let offset = q.offset.as_ref();
                let limit = q.limit.as_ref();
                let orders = &q.order_by;

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

                let source = Source(table_with_joins).try_into()?;
                let condition = match where_clause {
                    Some(expr) => Some(Expression(Box::new(expr.to_owned())).try_into()?),
                    None => None,
                };

                let mut selection = Vec::with_capacity(8);

                for p in projection {
                    let expr = Projection(p).try_into()?;
                    selection.push(expr);
                }

                let mut order_by = Vec::new();

                for expr in orders {
                    order_by.push(Order(expr).try_into()?);
                }
                let offset = offset.map(|v| Offset(v).into());

                let limit = limit.map(|v| Limit(v).into());

                // 拿出来构造自定义类型
                Ok(Sql {
                    selection,
                    condition,
                    source,
                    order_by,
                    offset,
                    limit,
                })
            }
            _ => Err(anyhow!("We only support Query at the moment")),
        }
    }
}

// wrapped types
pub struct Expression(pub(crate) Box<SqlExpr>);
pub struct Operation(pub(crate) SqlBinaryOperator);
pub struct Projection<'a>(pub(crate) &'a SelectItem);
pub struct Source<'a>(pub(crate) &'a [TableWithJoins]);
pub struct Order<'a>(pub(crate) &'a OrderByExpr);
pub struct Offset<'a>(pub(crate) &'a SqlOffset);
pub struct Limit<'a>(pub(crate) &'a SqlExpr);
pub struct Value(pub(crate) SqlValue);

// 把 sqlparser 中的 SqlExpr 转换为 polars中的 Expr

impl TryFrom<Expression> for prelude::Expr {
    type Error = anyhow::Error;

    fn try_from(expr: Expression) -> Result<Self, Self::Error> {
        match *expr.0 {
            SqlExpr::BinaryOp { left, op, right } => Ok(prelude::Expr::BinaryExpr {
                left: Box::new(Expression(left).try_into()?),
                op: Operation(op).try_into()?,
                right: Box::new(Expression(right).try_into()?),
            }),
            SqlExpr::Wildcard => Ok(Self::Wildcard),
            SqlExpr::IsNull(expr) => Ok(Self::IsNull(Box::new(Expression(expr).try_into()?))),
            SqlExpr::IsNotNull(expr) => Ok(Self::IsNotNull(Box::new(Expression(expr).try_into()?))),
            SqlExpr::Identifier(id) => Ok(Self::Column(prelude::Arc::new(id.value))),
            SqlExpr::Value(v) => Ok(Self::Literal(Value(v).try_into()?)),
            v => Err(anyhow!("expr {:#?} is not supported", v)),
        }
    }
}

// 把 sqlparser 中的 SqlExpr 转换为 polars中的 Expr

impl TryFrom<Operation> for prelude::Operator {
    type Error = anyhow::Error;

    fn try_from(op: Operation) -> Result<Self, Self::Error> {
        match op.0 {
            SqlBinaryOperator::Plus => Ok(Self::Plus),
            SqlBinaryOperator::Minus => Ok(Self::Minus),
            SqlBinaryOperator::Multiply => Ok(Self::Multiply),
            SqlBinaryOperator::Divide => Ok(Self::Divide),
            SqlBinaryOperator::Modulo => Ok(Self::Modulus),
            SqlBinaryOperator::Gt => Ok(Self::Gt),
            SqlBinaryOperator::Lt => Ok(Self::Lt),
            SqlBinaryOperator::GtEq => Ok(Self::GtEq),
            SqlBinaryOperator::LtEq => Ok(Self::LtEq),
            SqlBinaryOperator::Eq => Ok(Self::Eq),
            SqlBinaryOperator::NotEq => Ok(Self::NotEq),
            SqlBinaryOperator::And => Ok(Self::And),
            SqlBinaryOperator::Or => Ok(Self::Or),
            v => Err(anyhow!("Operator {} is not supported", v)),
        }
    }
}

// 同上

impl<'a> TryFrom<Projection<'a>> for prelude::Expr {
    type Error = anyhow::Error;
    fn try_from(p: Projection<'a>) -> Result<Self, Self::Error> {
        match p.0 {
            SelectItem::UnnamedExpr(SqlExpr::Identifier(id)) => Ok(prelude::col(&id.to_string())),
            SelectItem::ExprWithAlias {
                expr: SqlExpr::Identifier(id),
                alias,
            } => Ok(prelude::Expr::Alias(
                Box::new(prelude::Expr::Column(prelude::Arc::new(id.to_string()))),
                prelude::Arc::new(alias.to_string()),
            )),

            SelectItem::QualifiedWildcard(v) => Ok(prelude::col(&v.to_string())),
            SelectItem::Wildcard => Ok(prelude::col("*")),
            item => Err(anyhow!("projection {} not supported", item)),
        }
    }
}

impl<'a> TryFrom<Source<'a>> for &'a str {
    type Error = anyhow::Error;
    fn try_from(s: Source<'a>) -> Result<Self, Self::Error> {
        if s.0.len() != 1 {
            return Err(anyhow!("We only support single data source at the moment"));
        }
        let table = &s.0[0];
        if !table.joins.is_empty() {
            return Err(anyhow!("We do not  support join data source at the moment"));
        }

        match &table.relation {
            TableFactor::Table { name, .. } => Ok(&name.0.first().unwrap().value),
            _ => return Err(anyhow!("We only support table")),
        }
    }
}

impl<'a> TryFrom<Order<'a>> for (String, bool) {
    type Error = anyhow::Error;
    fn try_from(o: Order) -> Result<Self, Self::Error> {
        let name = match &o.0.expr {
            SqlExpr::Identifier(id) => id.to_string(),
            expr => {
                return Err(anyhow!(
                    "We only support identifier for order by, got {}",
                    expr
                ))
            }
        };

        Ok((name, !o.0.asc.unwrap_or(true)))
    }
}

// new type model, wrapped a type

// impl From trait, convert one type to another type !!!
// before that, we always endow a type power via impl some trait, but no meaning of convert directly
impl<'a> From<Offset<'a>> for i64 {
    fn from(offset: Offset) -> Self {
        match offset.0 {
            SqlOffset {
                value: SqlExpr::Value(SqlValue::Number(v, _b)),
                ..
            } => v.parse().unwrap_or(0),
            _ => 0,
        }
    }
}

impl<'a> From<Limit<'a>> for usize {
    fn from(l: Limit<'a>) -> Self {
        match l.0 {
            SqlExpr::Value(SqlValue::Number(v, _b)) => v.parse().unwrap_or(usize::MAX),
            _ => usize::MAX,
        }
    }
}

impl TryFrom<Value> for prelude::LiteralValue {
    type Error = anyhow::Error;
    fn try_from(v: Value) -> Result<Self, Self::Error> {
        match v.0 {
            SqlValue::Number(v, _) => Ok(prelude::LiteralValue::Float64(v.parse().unwrap())),
            SqlValue::Boolean(v) => Ok(prelude::LiteralValue::Boolean(v)),
            SqlValue::Null => Ok(prelude::LiteralValue::Null),
            v => return Err(anyhow!("Value {} is not supported", v)),
        }
    }
}

// impl<'a> From<Str<'a>> for bool {
//     fn from(s: Str<'a>) -> Self {
//         let b = s.0;
//         true
//     }
// }

#[cfg(test)]

mod tests {
    use super::*;
    use crate::dialect::TyrDialect;

    use sqlparser::parser::Parser;

    #[test]
    fn parse_sql_works() {
        let url = "http://abc.xyz/abc?a=1&b=2";
        let sql = format!(
            "select a, b, c from {} where a=1 order by c desc limit 5 offset 10",
            url
        );

        let statement = &Parser::parse_sql(&TyrDialect::default(), sql.as_ref()).unwrap()[0];
        let sql: Sql = statement.try_into().unwrap();

        assert_eq!(sql.source, url);
        assert_eq!(sql.limit, Some(5));
        assert_eq!(sql.offset, Some(10));
        assert_eq!(sql.order_by, vec![("c".into(), true)]);
        assert_eq!(
            sql.selection,
            vec![prelude::col("a"), prelude::col("b"), prelude::col("c")]
        );
    }
}
