use sqlite::Sqlite;
use types::{self, FromSql, FromSqlRow, HasSqlType, Numeric, Float};
use std::error::Error;
use sqlite::connection::SqliteValue;
use query_source::Queryable;
use row::Row;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum SqliteNumeric {
    Positive {
        number: String
    },
    Negative {
        number: String
    },
    NaN
}

impl FromSql<types::Numeric, Sqlite> for SqliteNumeric {
    fn from_sql(bytes: Option<&SqliteValue>) -> Result<Self, Box<Error>> {
        let bytes = not_none!(bytes);
        let str_num = bytes.read_text();
        let number_result = str_num.parse::<f64>();

        match number_result {
            Ok(n) => {
                if n.is_sign_positive() {
                    Ok(SqliteNumeric::Positive {number: str_num.to_string()})
                } else if n.is_sign_negative() {
                    Ok(SqliteNumeric::Negative {number: str_num.to_string()})
                } else {
                    Ok(SqliteNumeric::NaN)
                }
            },
            Err(_) => Ok(SqliteNumeric::NaN),
        }
    }
}

impl HasSqlType<Numeric> for Sqlite {
    fn metadata() -> Self::TypeMetadata {
        <Sqlite as HasSqlType<Float>>::metadata()
    }
}

impl Queryable<Numeric, Sqlite> for SqliteNumeric {
    type Row = Self;

    fn build(row: Self) -> Self {
        row
    }
}

impl FromSqlRow<Numeric, Sqlite> for SqliteNumeric {
    fn build_from_row<R:  Row<Sqlite>>(row: &mut R) -> Result<Self, Box<Error>> {
        FromSql::<Numeric, Sqlite>::from_sql(row.take())
    }
}
