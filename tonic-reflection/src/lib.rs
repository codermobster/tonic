//! A `tonic` based gRPC Server Reflection implementation.

#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![doc(
    html_logo_url = "https://github.com/hyperium/tonic/raw/master/.github/assets/tonic-docs.png"
)]
#![doc(html_root_url = "https://docs.rs/tonic-reflection/0.1.0")]
#![doc(issue_tracker_base_url = "https://github.com/hyperium/tonic/issues/")]
#![doc(test(no_crate_inject, attr(deny(rust_2018_idioms))))]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod proto {
    #![allow(unreachable_pub)]
    include!("proto/grpc.reflection.v1alpha.rs");
    // tonic::include_proto!("");
}

/// Implementation of the server component of gRPC Server Reflection.
pub mod server;