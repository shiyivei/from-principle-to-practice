/// 定义结构体
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Student {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, tag = "2")]
    pub age: u32,
    #[prost(bool, tag = "3")]
    pub is_male: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// 引用嵌套的消息定义
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<search_response::Result>,
}
/// Nested message and enum types in `SearchResponse`.
pub mod search_response {
    /// 嵌套消息定义
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Result {
        #[prost(string, tag = "1")]
        pub url: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub title: ::prost::alloc::string::String,
        #[prost(string, repeated, tag = "3")]
        pub snippets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// 商品名
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// 定义一个k/v类型，key是string类型，value也是string类型
    ///
    /// 商品属性，键值对
    #[prost(map = "string, string", tag = "2")]
    pub attrs: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Type {
    #[prost(enumeration = "r#type::Color", tag = "1")]
    pub color: i32,
    #[prost(message, optional, tag = "2")]
    pub point: ::core::option::Option<r#type::Point>,
    #[prost(message, optional, tag = "3")]
    pub points: ::core::option::Option<r#type::Points>,
}
/// Nested message and enum types in `Type`.
pub mod r#type {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Point {
        #[prost(uint32, tag = "1")]
        pub x: u32,
        #[prost(uint32, tag = "2")]
        pub y: u32,
        #[prost(uint32, tag = "3")]
        pub z: u32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Points {
        #[prost(message, repeated, tag = "1")]
        pub points: ::prost::alloc::vec::Vec<Point>,
    }
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Color {
        White = 0,
        Orange = 1,
        Green = 2,
    }
    impl Color {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Color::White => "WHITE",
                Color::Orange => "ORANGE",
                Color::Green => "GREEN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "WHITE" => Some(Self::White),
                "ORANGE" => Some(Self::Orange),
                "GREEN" => Some(Self::Green),
                _ => None,
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Subject {
    Math = 0,
    English = 1,
    Chinese = 2,
    Physical = 3,
    Chemical = 4,
}
impl Subject {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Subject::Math => "MATH",
            Subject::English => "ENGLISH",
            Subject::Chinese => "CHINESE",
            Subject::Physical => "PHYSICAL",
            Subject::Chemical => "CHEMICAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MATH" => Some(Self::Math),
            "ENGLISH" => Some(Self::English),
            "CHINESE" => Some(Self::Chinese),
            "PHYSICAL" => Some(Self::Physical),
            "CHEMICAL" => Some(Self::Chemical),
            _ => None,
        }
    }
}
