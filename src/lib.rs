#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![cfg_attr(
feature = "document-features",
cfg_attr(doc, doc = ::document_features::document_features!())
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod fmt;
pub mod prelude;
