//! Tracing Subscriber configuration for Day02 of Advent of Code 2024.

use std::io::Stderr;

use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, Registry, layer::Layered, prelude::*};
use tracing_tree::HierarchicalLayer;

// workaround to hairy return type
type SpecificLayered =
        Layered<EnvFilter, Layered<HierarchicalLayer<fn() -> Stderr, tracing_tree::time::Uptime>, Registry>>;

/// Generates a tracing_subcsriber.  (Convenience function.)
pub fn generate_tracing_subscriber() -> SpecificLayered {
        let tree_layer = tracing_tree::HierarchicalLayer::new(3)
                .with_timer(tracing_tree::time::Uptime::default())
                // .with_span_modes(true)
                .with_indent_lines(true);
        let envfilter_layer = EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy();
        Registry::default().with(tree_layer).with(envfilter_layer)
}
