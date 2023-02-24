use sqlparser::{dialect::GenericDialect, parser::Parser};

fn main() {
    tracing_subscriber::fmt::init();

    let sql = "SELECT a a1, b, 123, myfunc(b), *  FROM data_source  WHERE a > b AND b < 100 AND c BETWEEN 10 AND 20  ORDER BY a DESC, b  LIMIT 50 OFFSET 10";

    // 将一个sql查询语句转换为一棵抽象语法树
    let ast = Parser::parse_sql(&GenericDialect::default(), sql);

    println!("{:#?}", ast);
}
