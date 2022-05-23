use crate::common::{ColumnDefinition, FQName, Identifier, WithItem};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

/// data for the `AlterTable` command
#[derive(PartialEq, Debug, Clone)]
pub struct AlterTable {
    /// the name of the table.
    pub name: FQName,
    /// the table alteration operation.
    pub operation: AlterTableOperation,
}

/// table alteration operations
#[derive(PartialEq, Debug, Clone)]
pub enum AlterTableOperation {
    /// add columns to the table.
    Add(Vec<ColumnDefinition>),
    /// drop columns from the table.
    DropColumns(Vec<Identifier>),
    /// drop the "compact storage"
    DropCompactStorage,
    /// rename columns `(from, to)`
    Rename((Identifier, Identifier)),
    /// add with element options.
    With(Vec<WithItem>),
}

impl Display for AlterTableOperation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AlterTableOperation::Add(columns) => write!(
                f,
                "ADD {}",
                columns.iter().map(|x| x.to_string()).join(", ")
            ),
            AlterTableOperation::DropColumns(columns) => write!(
                f,
                "DROP {}",
                columns.iter().map(|c| c.to_string()).join(", ")
            ),
            AlterTableOperation::DropCompactStorage => write!(f, "DROP COMPACT STORAGE"),
            AlterTableOperation::Rename((from, to)) => write!(f, "RENAME {} TO {}", from, to),
            AlterTableOperation::With(with_element) => write!(
                f,
                "WITH {}",
                with_element.iter().map(|x| x.to_string()).join(" AND ")
            ),
        }
    }
}
