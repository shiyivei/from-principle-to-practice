use sqlparser::dialect::Dialect;

// 创建自己的 Dialect，仍然是从数据结构先开始

#[derive(Debug, Default)]
pub struct TyrDialect;

impl Dialect for TyrDialect {
    // 标识符中是否包含下列字符集
    fn is_identifier_part(&self, ch: char) -> bool {
        // 大小写字母字符 52个
        ('a'..='z').contains(&ch)
            || ('A'..='Z').contains(&ch)
            // 10个罗马数字
            || ('0'..='9').contains(&ch)
            // 8 个特殊符号
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }

    // 标识符是否以大小写字母或者下划线开头
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..='Z').contains(&ch) || '_' == ch
    }
}
/// 测试辅助函数
#[allow(dead_code)]
pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );

    sql
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;

    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&TyrDialect::default(), &example_sql()).is_ok());
    }
}
