// Code generated by Prisma Client Rust. DO NOT EDIT

#![allow(warnings, unused)]
use prisma_client_rust::{
    bigdecimal::{self, FromPrimitive},
    chrono,
    datamodel::parse_configuration,
    operator::Operator,
    prisma_models::{InternalDataModelBuilder, PrismaValue},
    queries::{QueryContext, QueryInfo, Result as QueryResult},
    query_core::{
        executor, schema_builder, BuildMode, CoreError, InterpreterError, QueryExecutor,
        QueryGraphBuilderError, QuerySchema, QueryValue, Selection,
    },
    serde_json, transform_equals, BatchResult, Direction, ManyArgs, SerializedWhere,
    SerializedWhereValue, UniqueArgs,
};
pub use prisma_client_rust::{queries::Error as QueryError, NewClientError};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
static DATAMODEL_STR : & 'static str = "datasource db {\n  provider = \"mysql\"\n  url      = env(\"DATABASE_URL\")\n}\n\ngenerator client {\n  provider = \"cargo prisma\"\n  output   = \"../src/prisma.rs\"\n}\n\nmodel User {\n  id       Int     @id @default(autoincrement())\n  username String\n  password String\n  admin    Boolean\n}\n" ;
static DATABASE_STR: &'static str = "mysql";
pub async fn new_client() -> Result<_prisma::PrismaClient, NewClientError> {
    let config = parse_configuration(DATAMODEL_STR)?.subject;
    let source = config
        .datasources
        .first()
        .expect("Pleasy supply a datasource in your schema.prisma file");
    let url = if let Some(url) = source.load_shadow_database_url()? {
        url
    } else {
        source.load_url(|key| std::env::var(key).ok())?
    };
    let url = if url.starts_with("file:") {
        let path = url.split(":").nth(1).unwrap();
        if Path::new("./schema.prisma").exists() {
            url
        } else if Path::new("./prisma/schema.prisma").exists() {
            format!("file:./prisma/{}", path)
        } else {
            url
        }
    } else {
        url
    };
    new_client_with_url(&url).await
}
pub async fn new_client_with_url(url: &str) -> Result<_prisma::PrismaClient, NewClientError> {
    let config = parse_configuration(DATAMODEL_STR)?.subject;
    let source = config
        .datasources
        .first()
        .expect("Pleasy supply a datasource in your schema.prisma file");
    let (db_name, executor) = executor::load(&source, &[], &url).await?;
    let internal_model = InternalDataModelBuilder::new(DATAMODEL_STR).build(db_name);
    let query_schema = Arc::new(schema_builder::build(
        internal_model,
        BuildMode::Modern,
        true,
        source.capabilities(),
        vec![],
        source.referential_integrity(),
    ));
    executor.primary_connector().get_connection().await?;
    Ok(PrismaClient::_new(executor, query_schema))
}
pub mod user {
    use super::_prisma::*;
    use super::*;
    pub mod id {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: i32) -> T {
            Set(value).into()
        }
        pub fn equals<T: From<UniqueWhereParam>>(value: i32) -> T {
            UniqueWhereParam::IdEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Id(direction)
        }
        pub fn cursor(cursor: i32) -> Cursor {
            Cursor::Id(cursor)
        }
        pub fn in_vec(value: Vec<i32>) -> WhereParam {
            WhereParam::IdInVec(value)
        }
        pub fn not_in_vec(value: Vec<i32>) -> WhereParam {
            WhereParam::IdNotInVec(value)
        }
        pub fn lt(value: i32) -> WhereParam {
            WhereParam::IdLt(value)
        }
        pub fn lte(value: i32) -> WhereParam {
            WhereParam::IdLte(value)
        }
        pub fn gt(value: i32) -> WhereParam {
            WhereParam::IdGt(value)
        }
        pub fn gte(value: i32) -> WhereParam {
            WhereParam::IdGte(value)
        }
        pub fn not(value: i32) -> WhereParam {
            WhereParam::IdNot(value)
        }
        pub fn increment(value: i32) -> SetParam {
            SetParam::IncrementId(value)
        }
        pub fn decrement(value: i32) -> SetParam {
            SetParam::DecrementId(value)
        }
        pub fn multiply(value: i32) -> SetParam {
            SetParam::MultiplyId(value)
        }
        pub fn divide(value: i32) -> SetParam {
            SetParam::DivideId(value)
        }
        pub struct Set(i32);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetId(value.0)
            }
        }
    }
    pub mod username {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn equals(value: String) -> WhereParam {
            WhereParam::UsernameEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Username(direction)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::UsernameInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::UsernameNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::UsernameLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::UsernameLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::UsernameGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::UsernameGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::UsernameContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::UsernameStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::UsernameEndsWith(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::UsernameNot(value)
        }
        pub struct Set(String);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetUsername(value.0)
            }
        }
    }
    pub mod password {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: String) -> T {
            Set(value).into()
        }
        pub fn equals(value: String) -> WhereParam {
            WhereParam::PasswordEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Password(direction)
        }
        pub fn in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::PasswordInVec(value)
        }
        pub fn not_in_vec(value: Vec<String>) -> WhereParam {
            WhereParam::PasswordNotInVec(value)
        }
        pub fn lt(value: String) -> WhereParam {
            WhereParam::PasswordLt(value)
        }
        pub fn lte(value: String) -> WhereParam {
            WhereParam::PasswordLte(value)
        }
        pub fn gt(value: String) -> WhereParam {
            WhereParam::PasswordGt(value)
        }
        pub fn gte(value: String) -> WhereParam {
            WhereParam::PasswordGte(value)
        }
        pub fn contains(value: String) -> WhereParam {
            WhereParam::PasswordContains(value)
        }
        pub fn starts_with(value: String) -> WhereParam {
            WhereParam::PasswordStartsWith(value)
        }
        pub fn ends_with(value: String) -> WhereParam {
            WhereParam::PasswordEndsWith(value)
        }
        pub fn not(value: String) -> WhereParam {
            WhereParam::PasswordNot(value)
        }
        pub struct Set(String);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetPassword(value.0)
            }
        }
    }
    pub mod admin {
        use super::super::*;
        use super::_prisma::*;
        use super::{Cursor, OrderByParam, SetParam, UniqueWhereParam, WhereParam, WithParam};
        pub fn set<T: From<Set>>(value: bool) -> T {
            Set(value).into()
        }
        pub fn equals(value: bool) -> WhereParam {
            WhereParam::AdminEquals(value).into()
        }
        pub fn order(direction: Direction) -> OrderByParam {
            OrderByParam::Admin(direction)
        }
        pub struct Set(bool);
        impl From<Set> for SetParam {
            fn from(value: Set) -> Self {
                Self::SetAdmin(value.0)
            }
        }
    }
    pub fn _outputs() -> Vec<Selection> {
        ["id", "username", "password", "admin"]
            .into_iter()
            .map(|o| {
                let builder = Selection::builder(o);
                builder.build()
            })
            .collect()
    }
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Data {
        #[serde(rename = "id")]
        pub id: i32,
        #[serde(rename = "username")]
        pub username: String,
        #[serde(rename = "password")]
        pub password: String,
        #[serde(rename = "admin")]
        pub admin: bool,
    }
    impl Data {}
    #[derive(Clone)]
    pub enum WithParam {}
    impl Into<Selection> for WithParam {
        fn into(self) -> Selection {
            match self {}
        }
    }
    #[derive(Clone)]
    pub enum SetParam {
        SetId(i32),
        IncrementId(i32),
        DecrementId(i32),
        MultiplyId(i32),
        DivideId(i32),
        SetUsername(String),
        SetPassword(String),
        SetAdmin(bool),
    }
    impl Into<(String, PrismaValue)> for SetParam {
        fn into(self) -> (String, PrismaValue) {
            match self {
                SetParam::SetId(value) => ("id".to_string(), PrismaValue::Int(value as i64)),
                SetParam::IncrementId(value) => (
                    "id".to_string(),
                    PrismaValue::Object(vec![(
                        "increment".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DecrementId(value) => (
                    "id".to_string(),
                    PrismaValue::Object(vec![(
                        "decrement".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::MultiplyId(value) => (
                    "id".to_string(),
                    PrismaValue::Object(vec![(
                        "multiply".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::DivideId(value) => (
                    "id".to_string(),
                    PrismaValue::Object(vec![(
                        "divide".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                SetParam::SetUsername(value) => {
                    ("username".to_string(), PrismaValue::String(value))
                }
                SetParam::SetPassword(value) => {
                    ("password".to_string(), PrismaValue::String(value))
                }
                SetParam::SetAdmin(value) => ("admin".to_string(), PrismaValue::Boolean(value)),
            }
        }
    }
    #[derive(Clone)]
    pub enum OrderByParam {
        Id(Direction),
        Username(Direction),
        Password(Direction),
        Admin(Direction),
    }
    impl Into<(String, PrismaValue)> for OrderByParam {
        fn into(self) -> (String, PrismaValue) {
            match self {
                Self::Id(direction) => {
                    ("id".to_string(), PrismaValue::String(direction.to_string()))
                }
                Self::Username(direction) => (
                    "username".to_string(),
                    PrismaValue::String(direction.to_string()),
                ),
                Self::Password(direction) => (
                    "password".to_string(),
                    PrismaValue::String(direction.to_string()),
                ),
                Self::Admin(direction) => (
                    "admin".to_string(),
                    PrismaValue::String(direction.to_string()),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum Cursor {
        Id(i32),
    }
    impl Into<(String, PrismaValue)> for Cursor {
        fn into(self) -> (String, PrismaValue) {
            match self {
                Self::Id(cursor) => ("id".to_string(), PrismaValue::Int(cursor as i64)),
            }
        }
    }
    #[derive(Clone)]
    pub enum WhereParam {
        Not(Vec<WhereParam>),
        Or(Vec<WhereParam>),
        And(Vec<WhereParam>),
        IdEquals(i32),
        IdInVec(Vec<i32>),
        IdNotInVec(Vec<i32>),
        IdLt(i32),
        IdLte(i32),
        IdGt(i32),
        IdGte(i32),
        IdNot(i32),
        UsernameEquals(String),
        UsernameInVec(Vec<String>),
        UsernameNotInVec(Vec<String>),
        UsernameLt(String),
        UsernameLte(String),
        UsernameGt(String),
        UsernameGte(String),
        UsernameContains(String),
        UsernameStartsWith(String),
        UsernameEndsWith(String),
        UsernameNot(String),
        PasswordEquals(String),
        PasswordInVec(Vec<String>),
        PasswordNotInVec(Vec<String>),
        PasswordLt(String),
        PasswordLte(String),
        PasswordGt(String),
        PasswordGte(String),
        PasswordContains(String),
        PasswordStartsWith(String),
        PasswordEndsWith(String),
        PasswordNot(String),
        AdminEquals(bool),
    }
    impl Into<SerializedWhere> for WhereParam {
        fn into(self) -> SerializedWhere {
            match self {
                Self::Not(value) => (
                    "NOT".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| PrismaValue::Object(transform_equals(vec![v].into_iter())))
                            .collect(),
                    ),
                ),
                Self::Or(value) => (
                    "OR".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| PrismaValue::Object(transform_equals(vec![v].into_iter())))
                            .collect(),
                    ),
                ),
                Self::And(value) => (
                    "AND".to_string(),
                    SerializedWhereValue::List(
                        value
                            .into_iter()
                            .map(|v| PrismaValue::Object(transform_equals(vec![v].into_iter())))
                            .collect(),
                    ),
                ),
                Self::IdEquals(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                Self::IdInVec(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "in".to_string(),
                        PrismaValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::Int(v as i64))
                                .collect(),
                        ),
                    )]),
                ),
                Self::IdNotInVec(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "notIn".to_string(),
                        PrismaValue::List(
                            value
                                .into_iter()
                                .map(|v| PrismaValue::Int(v as i64))
                                .collect(),
                        ),
                    )]),
                ),
                Self::IdLt(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                Self::IdLte(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                Self::IdGt(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                Self::IdGte(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                Self::IdNot(value) => (
                    "id".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        PrismaValue::Int(value as i64),
                    )]),
                ),
                Self::UsernameEquals(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::UsernameInVec(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "in".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::UsernameNotInVec(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "notIn".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::UsernameLt(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::UsernameLte(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::UsernameGt(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::UsernameGte(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::UsernameContains(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "contains".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::UsernameStartsWith(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "startsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::UsernameEndsWith(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "endsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::UsernameNot(value) => (
                    "username".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordEquals(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordInVec(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "in".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::PasswordNotInVec(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "notIn".to_string(),
                        PrismaValue::List(
                            value.into_iter().map(|v| PrismaValue::String(v)).collect(),
                        ),
                    )]),
                ),
                Self::PasswordLt(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordLte(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "lte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordGt(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gt".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordGte(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "gte".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordContains(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "contains".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordStartsWith(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "startsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordEndsWith(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "endsWith".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::PasswordNot(value) => (
                    "password".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "not".to_string(),
                        PrismaValue::String(value),
                    )]),
                ),
                Self::AdminEquals(value) => (
                    "admin".to_string(),
                    SerializedWhereValue::Object(vec![(
                        "equals".to_string(),
                        PrismaValue::Boolean(value),
                    )]),
                ),
            }
        }
    }
    #[derive(Clone)]
    pub enum UniqueWhereParam {
        IdEquals(i32),
    }
    impl From<UniqueWhereParam> for WhereParam {
        fn from(value: UniqueWhereParam) -> Self {
            match value {
                UniqueWhereParam::IdEquals(value) => Self::IdEquals(value),
            }
        }
    }
    impl From<Operator<Self>> for WhereParam {
        fn from(op: Operator<Self>) -> Self {
            match op {
                Operator::Not(value) => Self::Not(value),
                Operator::And(value) => Self::And(value),
                Operator::Or(value) => Self::Or(value),
            }
        }
    }
    pub type UniqueArgs = prisma_client_rust::UniqueArgs<WithParam>;
    pub type ManyArgs = prisma_client_rust::ManyArgs<WhereParam, WithParam, OrderByParam, Cursor>;
    pub type Create<'a> = prisma_client_rust::Create<'a, SetParam, WithParam, Data>;
    pub type FindUnique<'a> =
        prisma_client_rust::FindUnique<'a, WhereParam, WithParam, SetParam, Data>;
    pub type FindMany<'a> = prisma_client_rust::FindMany<
        'a,
        WhereParam,
        WithParam,
        OrderByParam,
        Cursor,
        SetParam,
        Data,
    >;
    pub type FindFirst<'a> =
        prisma_client_rust::FindFirst<'a, WhereParam, WithParam, OrderByParam, Cursor, Data>;
    pub type Update<'a> = prisma_client_rust::Update<'a, WhereParam, SetParam, WithParam, Data>;
    pub type UpdateMany<'a> = prisma_client_rust::UpdateMany<'a, WhereParam, SetParam>;
    pub type Upsert<'a> = prisma_client_rust::Upsert<'a, WhereParam, SetParam, WithParam, Data>;
    pub type Delete<'a> = prisma_client_rust::Delete<'a, WhereParam, WithParam, Data>;
    pub type DeleteMany<'a> = prisma_client_rust::DeleteMany<'a, WhereParam>;
    pub struct Actions<'a> {
        pub client: &'a PrismaClient,
    }
    impl<'a> Actions<'a> {
        pub fn create(
            self,
            username: username::Set,
            password: password::Set,
            admin: admin::Set,
            mut _params: Vec<SetParam>,
        ) -> Create<'a> {
            _params.push(username.into());
            _params.push(password.into());
            _params.push(admin.into());
            Create::new(
                self.client._new_query_context(),
                QueryInfo::new("User", _outputs()),
                _params,
            )
        }
        pub fn find_unique(self, param: UniqueWhereParam) -> FindUnique<'a> {
            FindUnique::new(
                self.client._new_query_context(),
                QueryInfo::new("User", _outputs()),
                param.into(),
            )
        }
        pub fn find_first(self, params: Vec<WhereParam>) -> FindFirst<'a> {
            FindFirst::new(
                self.client._new_query_context(),
                QueryInfo::new("User", _outputs()),
                params,
            )
        }
        pub fn find_many(self, params: Vec<WhereParam>) -> FindMany<'a> {
            FindMany::new(
                self.client._new_query_context(),
                QueryInfo::new("User", _outputs()),
                params,
            )
        }
        pub fn upsert(
            self,
            _where: UniqueWhereParam,
            _create: (username::Set, password::Set, admin::Set, Vec<SetParam>),
            _update: Vec<SetParam>,
        ) -> Upsert<'a> {
            let (username, password, admin, mut _params) = _create;
            _params.push(username.into());
            _params.push(password.into());
            _params.push(admin.into());
            Upsert::new(
                self.client._new_query_context(),
                QueryInfo::new("User", _outputs()),
                _where.into(),
                _params,
                _update,
            )
        }
    }
}
pub mod _prisma {
    use super::*;
    use prisma_client_rust::{
        queries::QueryContext,
        query_core::{QueryExecutor, QuerySchema},
        raw, ExecuteRaw, QueryRaw,
    };
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::sync::Arc;
    pub struct PrismaClient {
        executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
        query_schema: Arc<QuerySchema>,
    }
    impl fmt::Debug for PrismaClient {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("PrismaClient").finish()
        }
    }
    impl PrismaClient {
        pub(super) fn _new_query_context(&self) -> QueryContext {
            QueryContext::new(&self.executor, self.query_schema.clone())
        }
        pub(super) fn _new(
            executor: Box<dyn QueryExecutor + Send + Sync + 'static>,
            query_schema: Arc<QuerySchema>,
        ) -> Self {
            Self {
                executor,
                query_schema,
            }
        }
        pub async fn _query_raw<T: serde::de::DeserializeOwned>(
            &self,
            query: raw::Raw,
        ) -> QueryResult<Vec<T>> {
            QueryRaw::new(
                QueryContext::new(&self.executor, self.query_schema.clone()),
                query,
                DATABASE_STR,
            )
            .exec()
            .await
        }
        pub async fn _execute_raw(&self, query: raw::Raw) -> QueryResult<i64> {
            ExecuteRaw::new(
                QueryContext::new(&self.executor, self.query_schema.clone()),
                query,
                DATABASE_STR,
            )
            .exec()
            .await
        }
        pub fn user(&self) -> user::Actions {
            user::Actions { client: &self }
        }
    }
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum UserScalarFieldEnum {
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "username")]
        Username,
        #[serde(rename = "password")]
        Password,
        #[serde(rename = "admin")]
        Admin,
    }
    impl ToString for UserScalarFieldEnum {
        fn to_string(&self) -> String {
            match self {
                Self::Id => "id".to_string(),
                Self::Username => "username".to_string(),
                Self::Password => "password".to_string(),
                Self::Admin => "admin".to_string(),
            }
        }
    }
    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    pub enum SortOrder {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl ToString for SortOrder {
        fn to_string(&self) -> String {
            match self {
                Self::Asc => "asc".to_string(),
                Self::Desc => "desc".to_string(),
            }
        }
    }
}
pub use _prisma::PrismaClient;
