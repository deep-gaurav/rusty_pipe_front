pub struct Suggestions;
pub mod suggestions {
    #![allow(dead_code)]
    pub const OPERATION_NAME: &'static str = "Suggestions";
    pub const QUERY: &'static str =
        "query Suggestions($query:String!) {\n  search(query:$query){\n    suggestion\n  }\n}";
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Debug, Deserialize)]
    pub struct SuggestionsSearch {
        pub suggestion: Vec<String>,
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub query: String,
    }
    impl Variables {}
    #[derive(Debug, Deserialize)]
    pub struct ResponseData {
        pub search: SuggestionsSearch,
    }
}
impl graphql_client::GraphQLQuery for Suggestions {
    type Variables = suggestions::Variables;
    type ResponseData = suggestions::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: suggestions::QUERY,
            operation_name: suggestions::OPERATION_NAME,
        }
    }
}
