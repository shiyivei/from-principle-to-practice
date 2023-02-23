#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSpec {
    #[prost(message, repeated, tag = "1")]
    pub specs: ::prost::alloc::vec::Vec<Spec>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resize {
    #[prost(uint32, tag = "1")]
    pub width: u32,
    #[prost(uint32, tag = "2")]
    pub height: u32,
    #[prost(enumeration = "resize::ResizeType", tag = "3")]
    pub rtype: i32,
    #[prost(enumeration = "resize::SampleFilter", tag = "4")]
    pub filter: i32,
}
/// Nested message and enum types in `Resize`.
pub mod resize {
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
    pub enum ResizeType {
        Normal = 0,
        SeamCarve = 1,
    }
    impl ResizeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResizeType::Normal => "NORMAL",
                ResizeType::SeamCarve => "SEAM_CARVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "NORMAL" => Some(Self::Normal),
                "SEAM_CARVE" => Some(Self::SeamCarve),
                _ => None,
            }
        }
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
    pub enum SampleFilter {
        Undefined = 0,
        Nearest = 1,
        Triangle = 2,
        CatmullRom = 3,
        Gaussian = 4,
        Lanczos3 = 5,
    }
    impl SampleFilter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SampleFilter::Undefined => "UNDEFINED",
                SampleFilter::Nearest => "NEAREST",
                SampleFilter::Triangle => "TRIANGLE",
                SampleFilter::CatmullRom => "CATMULL_ROM",
                SampleFilter::Gaussian => "GAUSSIAN",
                SampleFilter::Lanczos3 => "LANCZOS3",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNDEFINED" => Some(Self::Undefined),
                "NEAREST" => Some(Self::Nearest),
                "TRIANGLE" => Some(Self::Triangle),
                "CATMULL_ROM" => Some(Self::CatmullRom),
                "GAUSSIAN" => Some(Self::Gaussian),
                "LANCZOS3" => Some(Self::Lanczos3),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Crop {
    #[prost(uint32, tag = "1")]
    pub x1: u32,
    #[prost(uint32, tag = "2")]
    pub y1: u32,
    #[prost(uint32, tag = "3")]
    pub x2: u32,
    #[prost(uint32, tag = "4")]
    pub y2: u32,
}
/// 处理水平翻转
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fliph {}
/// 处理垂直翻转
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flipv {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Contrast {
    #[prost(float, tag = "1")]
    pub contrast: f32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    #[prost(enumeration = "filter::Filter", tag = "1")]
    pub filter: i32,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
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
    pub enum Filter {
        Unspecified = 0,
        Oceanic = 1,
        Islands = 2,
        Marine = 3,
    }
    impl Filter {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Filter::Unspecified => "UNSPECIFIED",
                Filter::Oceanic => "OCEANIC",
                Filter::Islands => "ISLANDS",
                Filter::Marine => "MARINE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNSPECIFIED" => Some(Self::Unspecified),
                "OCEANIC" => Some(Self::Oceanic),
                "ISLANDS" => Some(Self::Islands),
                "MARINE" => Some(Self::Marine),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Watermark {
    #[prost(uint32, tag = "1")]
    pub x: u32,
    #[prost(uint32, tag = "2")]
    pub y: u32,
}
/// 一个spec可以包含上述处理方法
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Spec {
    #[prost(oneof = "spec::Data", tags = "1, 2, 3, 4, 5, 6, 7")]
    pub data: ::core::option::Option<spec::Data>,
}
/// Nested message and enum types in `Spec`.
pub mod spec {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        Resize(super::Resize),
        #[prost(message, tag = "2")]
        Crop(super::Crop),
        #[prost(message, tag = "3")]
        Flipv(super::Flipv),
        #[prost(message, tag = "4")]
        Fliph(super::Fliph),
        #[prost(message, tag = "5")]
        Contrast(super::Contrast),
        #[prost(message, tag = "6")]
        Filter(super::Filter),
        #[prost(message, tag = "7")]
        Watermark(super::Watermark),
    }
}
