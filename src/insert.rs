use crate::begin_batch::BeginBatch;
use crate::common::{FQName, Identifier, Operand, TtlTimestamp};
use itertools::Itertools;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

/// the data for insert statements.
#[derive(PartialEq, Debug, Clone)]
pub struct Insert {
    /// if set the statement starts with `BEGIN BATCH`
    pub begin_batch: Option<BeginBatch>,
    /// the table name
    pub table_name: FQName,
    /// an the list of of column names to insert into.
    pub columns: Vec<Identifier>,
    /// the `VALUES` to insert
    pub values: InsertValues,
    /// if set the timestamp for `USING TTL`
    pub using_ttl: Option<TtlTimestamp>,
    /// if true then `IF NOT EXISTS` is added to the statement
    pub if_not_exists: bool,
}

impl Insert {
    /// return a sorted map of column names to Operands.
    pub fn get_value_map(&self) -> BTreeMap<Identifier, &Operand> {
        let mut result = BTreeMap::new();
        match &self.values {
            InsertValues::Values(operands) => {
                // if there is a column mismatch we have a problem so
                // return an empty list
                if self.columns.len() == operands.len() {
                    for (i, operand) in operands.iter().enumerate() {
                        result.insert(self.columns[i].clone(), operand);
                    }
                }
            }
            InsertValues::Json(_) => {}
        }
        result
    }
}
impl Display for Insert {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}INSERT INTO {} ({}) {}{}{}",
            self.begin_batch
                .as_ref()
                .map_or("".to_string(), |x| x.to_string()),
            self.table_name,
            self.columns.iter().map(|c| c.to_string()).join(", "),
            self.values,
            if self.if_not_exists {
                " IF NOT EXISTS"
            } else {
                ""
            },
            self.using_ttl
                .as_ref()
                .map_or("".to_string(), |x| x.to_string()),
        )
    }
}

/// The structure that describs the values to insert.
#[derive(PartialEq, Debug, Clone)]
pub enum InsertValues {
    /// this is the standard list of values.
    Values(Vec<Operand>),
    /// this option allows JSON string to define the values.
    Json(String),
}

impl Display for InsertValues {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertValues::Values(columns) => {
                write!(f, "VALUES ({})", columns.iter().join(", "))
            }
            InsertValues::Json(text) => {
                write!(f, "JSON {}", text)
            }
        }
    }
}
