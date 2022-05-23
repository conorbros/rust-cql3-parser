use crate::common::{FQName, Identifier};
use std::fmt::{Display, Formatter};

/// data to for the create index statement.
#[derive(PartialEq, Debug, Clone)]
pub struct CreateIndex {
    /// only if not exists.
    pub if_not_exists: bool,
    /// optional name of the index.
    pub name: Option<Identifier>,
    /// the table the index is on.
    pub table: FQName,
    /// the index column type.
    pub column: IndexColumnType,
}

impl Display for CreateIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = if let Some(name) = &self.name {
            format!("{} ", name)
        } else {
            "".to_string()
        };
        let exists = if self.if_not_exists {
            "IF NOT EXISTS "
        } else {
            ""
        };

        write!(
            f,
            "CREATE INDEX {}{}ON {}( {} )",
            exists, name, self.table, self.column
        )
    }
}

/// The definition of an index column type
#[derive(PartialEq, Debug, Clone)]
pub enum IndexColumnType {
    /// column is a column
    Column(Identifier),
    /// use the keys from the column
    Keys(Identifier),
    /// use the entries from the column
    Entries(Identifier),
    /// use the full column entry.
    Full(Identifier),
}

impl Display for IndexColumnType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexColumnType::Column(name) => write!(f, "{}", name),
            IndexColumnType::Keys(name) => write!(f, "KEYS( {} )", name),
            IndexColumnType::Entries(name) => write!(f, "ENTRIES( {} )", name),
            IndexColumnType::Full(name) => write!(f, "FULL( {} )", name),
        }
    }
}
