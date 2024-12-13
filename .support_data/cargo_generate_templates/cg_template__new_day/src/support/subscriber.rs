//! Tracing Subscriber configuration for {{ project-name | title_case }} of Advent of Code 2024.
//!
//! `generate_tracing_subscriber()` is a convenience function designed to be used with `tracint::subscriber::set_global_default(_)`
//! Unfortunately, the return type created by composing Layers is fragile.
//! And the desired trait (Subscriber) is not Sized and therefore not amenable to use of the `--> dyn _` syntax.
//! Similarly, this makes dynamic choice difficult.
//!
//! A prefer solution may be to simple set the global default subscriber *in* the convenience function as a side-effect.
//! This would allow various branches and customizations.
//!
//! For now, this is workable.
//!
//! ## Caution
//! - Tracing is poorly documented and methods poorly named.  One can easily use, e.g., `::fmt()` instead of `::fmt` and be greeted with cryptic or even misdirecting errors.
//!   - I have no solution for this.  *Just be careful!*  It is very easy to lose a lot of time chain one's tail, on seemingly trivial configuration.

use std::io::Stderr;

use tracing::level_filters::LevelFilter;
use tracing_error::ErrorLayer;
use tracing_subscriber::{EnvFilter, Registry, layer::Layered, prelude::*};
use tracing_tree::HierarchicalLayer;

// workaround to hairy return type
type VerboseLayeredCompositeType = Layered<
        ErrorLayer<
                Layered<HierarchicalLayer<fn() -> Stderr, tracing_tree::time::Uptime>, Layered<EnvFilter, Registry>>,
        >,
        Layered<HierarchicalLayer<fn() -> Stderr, tracing_tree::time::Uptime>, Layered<EnvFilter, Registry>>,
>;

/// Generates a tracing_subcsriber.  (Convenience function.)
///
/// # Use:
/// ```text
/// fn main() -> Result<()> {
///     let subscriber = generate_tracing_subscriber();
///     tracing::subscriber::set_global_default(subscriber)?;
///    // ...
///    Ok(())
/// }
/// ```
pub fn generate_tracing_subscriber() -> VerboseLayeredCompositeType {
        let envfilter_layer = EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy();

        let tree_layer = tracing_tree::HierarchicalLayer::new(2)
                .with_span_modes(true)
                .with_indent_lines(true)
                .with_timer(tracing_tree::time::Uptime::default());

        let error_layer = ErrorLayer::default();

        Registry::default()
                .with(envfilter_layer)
                .with(tree_layer)
                .with(error_layer)
}

// /// Some example for printing to screen and to two files.
// fn example_file_writing_subscriber_generator() -> Result<RidiculousLayeredSubscriberName> {
//         let err_file = std::fs::OpenOptions::new()
//                 .append(true)
//                 .create(true)
//                 .open("log-error.log")?;

//         let debug_file = std::fs::OpenOptions::new()
//                 .append(true)
//                 .create(true)
//                 .open("log-debug.log")?;

//         let subscriber = Registry::default()
//                 .with(tracing_subscriber::fmt::layer().compact().with_ansi(true))
//                 .with(tracing_subscriber::fmt::layer()
//                         .json()
//                         .with_writer(err_file)
//                         .with_filter(LevelFilter::from_level(Level::WARN)))
//                 .with(tracing_subscriber::fmt::layer()
//                         .json()
//                         .with_writer(debug_file)
//                         .with_filter(LevelFilter::from_level(Level::TRACE)));
//         Ok(subscriber)
// }
// type RidiculousLayeredSubscriberName = Layered<
//         Filtered<
//                 tracing_subscriber::fmt::Layer<
//                         Layered<
//                                 Filtered<
//                                         tracing_subscriber::fmt::Layer<
//                                                 Layered<
//                                                         tracing_subscriber::fmt::Layer<
//                                                                 Registry,
//                                                                 tracing_subscriber::fmt::format::DefaultFields,
//                                                                 tracing_subscriber::fmt::format::Format<
//                                                                         tracing_subscriber::fmt::format::Compact,
//                                                                 >,
//                                                         >,
//                                                         Registry,
//                                                 >,
//                                                 tracing_subscriber::fmt::format::JsonFields,
//                                                 tracing_subscriber::fmt::format::Format<
//                                                         tracing_subscriber::fmt::format::Json,
//                                                 >,
//                                                 std::fs::File,
//                                         >,
//                                         LevelFilter,
//                                         Layered<
//                                                 tracing_subscriber::fmt::Layer<
//                                                         Registry,
//                                                         tracing_subscriber::fmt::format::DefaultFields,
//                                                         tracing_subscriber::fmt::format::Format<
//                                                                 tracing_subscriber::fmt::format::Compact,
//                                                         >,
//                                                 >,
//                                                 Registry,
//                                         >,
//                                 >,
//                                 Layered<
//                                         tracing_subscriber::fmt::Layer<
//                                                 Registry,
//                                                 tracing_subscriber::fmt::format::DefaultFields,
//                                                 tracing_subscriber::fmt::format::Format<
//                                                         tracing_subscriber::fmt::format::Compact,
//                                                 >,
//                                         >,
//                                         Registry,
//                                 >,
//                         >,
//                         tracing_subscriber::fmt::format::JsonFields,
//                         tracing_subscriber::fmt::format::Format<tracing_subscriber::fmt::format::Json>,
//                         std::fs::File,
//                 >,
//                 LevelFilter,
//                 Layered<
//                         Filtered<
//                                 tracing_subscriber::fmt::Layer<
//                                         Layered<
//                                                 tracing_subscriber::fmt::Layer<
//                                                         Registry,
//                                                         tracing_subscriber::fmt::format::DefaultFields,
//                                                         tracing_subscriber::fmt::format::Format<
//                                                                 tracing_subscriber::fmt::format::Compact,
//                                                         >,
//                                                 >,
//                                                 Registry,
//                                         >,
//                                         tracing_subscriber::fmt::format::JsonFields,
//                                         tracing_subscriber::fmt::format::Format<tracing_subscriber::fmt::format::Json>,
//                                         std::fs::File,
//                                 >,
//                                 LevelFilter,
//                                 Layered<
//                                         tracing_subscriber::fmt::Layer<
//                                                 Registry,
//                                                 tracing_subscriber::fmt::format::DefaultFields,
//                                                 tracing_subscriber::fmt::format::Format<
//                                                         tracing_subscriber::fmt::format::Compact,
//                                                 >,
//                                         >,
//                                         Registry,
//                                 >,
//                         >,
//                         Layered<
//                                 tracing_subscriber::fmt::Layer<
//                                         Registry,
//                                         tracing_subscriber::fmt::format::DefaultFields,
//                                         tracing_subscriber::fmt::format::Format<
//                                                 tracing_subscriber::fmt::format::Compact,
//                                         >,
//                                 >,
//                                 Registry,
//                         >,
//                 >,
//         >,
//         Layered<
//                 Filtered<
//                         tracing_subscriber::fmt::Layer<
//                                 Layered<
//                                         tracing_subscriber::fmt::Layer<
//                                                 Registry,
//                                                 tracing_subscriber::fmt::format::DefaultFields,
//                                                 tracing_subscriber::fmt::format::Format<
//                                                         tracing_subscriber::fmt::format::Compact,
//                                                 >,
//                                         >,
//                                         Registry,
//                                 >,
//                                 tracing_subscriber::fmt::format::JsonFields,
//                                 tracing_subscriber::fmt::format::Format<tracing_subscriber::fmt::format::Json>,
//                                 std::fs::File,
//                         >,
//                         LevelFilter,
//                         Layered<
//                                 tracing_subscriber::fmt::Layer<
//                                         Registry,
//                                         tracing_subscriber::fmt::format::DefaultFields,
//                                         tracing_subscriber::fmt::format::Format<
//                                                 tracing_subscriber::fmt::format::Compact,
//                                         >,
//                                 >,
//                                 Registry,
//                         >,
//                 >,
//                 Layered<
//                         tracing_subscriber::fmt::Layer<
//                                 Registry,
//                                 tracing_subscriber::fmt::format::DefaultFields,
//                                 tracing_subscriber::fmt::format::Format<tracing_subscriber::fmt::format::Compact>,
//                         >,
//                         Registry,
//                 >,
//         >,
// >;
