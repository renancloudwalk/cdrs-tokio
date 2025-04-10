pub mod batch_query_builder;
pub mod prepared_query;
pub mod query_flags;
pub mod query_params;
pub mod query_params_builder;
pub mod query_values;
pub mod utils;

pub use crate::query::batch_query_builder::{BatchQueryBuilder, QueryBatch};
pub use crate::query::prepared_query::PreparedQuery;
pub use crate::query::query_flags::QueryFlags;
pub use crate::query::query_params::QueryParams;
pub use crate::query::query_params_builder::QueryParamsBuilder;
pub use crate::query::query_values::QueryValues;

/// Structure that represents CQL query and parameters which will be applied during
/// its execution
#[derive(Debug, Default)]
pub struct Query {
    pub query: String,
    pub params: QueryParams,
}
