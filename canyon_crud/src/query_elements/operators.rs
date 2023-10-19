pub trait Operator {
    fn as_str(&self, placeholder_counter: usize) -> String;
}

/// Enumerated type for represent the comparison operations
/// in SQL sentences
pub enum Comp {
    /// Operator "=" equals
    Eq,
    /// Operator "!=" not equals
    Neq,
    /// Operator ">" greater than value
    Gt,
    /// Operator ">=" greater or equals than value
    GtEq,
    /// Operator "<" less than value
    Lt,
    /// Operator "=<" less or equals than value
    LtEq,
}

impl Operator for Comp {
    fn as_str(&self, placeholder_counter: usize) -> String {
        match *self {
            Self::Eq => format!(" = ${placeholder_counter}"),
            Self::Neq => format!(" <> ${placeholder_counter}"),
            Self::Gt => format!(" > ${placeholder_counter}"),
            Self::GtEq => format!(" >= ${placeholder_counter}"),
            Self::Lt => format!(" < ${placeholder_counter}"),
            Self::LtEq => format!(" <= ${placeholder_counter}"),
        }
    }
}

// This String argument is temporal solution to change datatypes to cast in other database
pub enum Like {
    /// Operator "LIKE"  as '%pattern%'
    Full(String),
    /// Operator "LIKE"  as '%pattern'
    Left(String),
    /// Operator "LIKE"  as 'pattern%'
    Right(String),
}

impl Operator for Like {
    fn as_str(&self, placeholder_counter: usize) -> String {
        match self {
            Like::Full(datasource_name) => {
                format!(" LIKE CONCAT('%', CAST(${placeholder_counter} AS VARCHAR) ,'%')")
            }
            Like::Left(datasource_name) => {
                format!(" LIKE CONCAT('%', CAST(${placeholder_counter} AS VARCHAR))")
            }
            Like::Right(datasource_name) => {
                format!(" LIKE CONCAT(CAST(${placeholder_counter} AS VARCHAR) ,'%')")
            }
        }
    }
}

fn get_datatype_require_to_cast_as_str_by_database_type(datasource_name: &str) -> String {
    "".to_string()
}
