#![warn(
    clippy::pedantic,
    clippy::correctness,
    clippy::suspicious,
    clippy::complexity,
    clippy::perf,
    clippy::style
)]
#![allow(clippy::missing_panics_doc, clippy::missing_errors_doc)]
//!
//! Collision detection for 2D irregular cutting and packing algorithms.
//!
//! [`geometry`] provides polygons and transformations. [`io`] imports item and container
//! geometry and exports placements. A [`Layout`](entities::Layout) owns placements and
//! keeps its [`CDEngine`](collision_detection::CDEngine) synchronized as items are added or removed.
//! Use the CDE to query candidate placements before adding them to a layout.
//! Optional `spp` and `bpp` features provide strip- and bin-packing models in [`probs`].
//!
//! # Import, query, place and restore
//!
//! ```
//! use jagua_rs::collision_detection::{CDEConfig, hazards::filter::NoFilter};
//! use jagua_rs::entities::Layout;
//! use jagua_rs::geometry::{DTransformation, fail_fast::SPSurrogateConfig, geo_traits::Transformable};
//! use jagua_rs::io::{ext_repr::{ExtContainer, ExtItem, ExtOrientation, ExtRotation, ExtShape}, import::Importer};
//!
//! # fn main() -> anyhow::Result<()> {
//! let importer = Importer::new(CDEConfig {
//!     quadtree_depth: 5,
//!     cd_threshold: 96,
//!     item_surrogate_config: SPSurrogateConfig::none(),
//! }, None, None);
//! let container = importer.import_container(&ExtContainer {
//!     id: 0,
//!     shape: ExtShape::Rectangle { x_min: 0.0, y_min: 0.0, width: 20.0, height: 10.0 },
//!     zones: vec![],
//! })?;
//! let item = importer.import_item(&ExtItem {
//!     id: 42,
//!     shape: ExtShape::Rectangle { x_min: 0.0, y_min: 0.0, width: 2.0, height: 2.0 },
//!     orientation: ExtOrientation {
//!         rotation: ExtRotation::Discrete { angles: vec![0.0] },
//!         reflection_axes: vec![],
//!     },
//!     min_quality: None,
//! }, 0)?;
//! let mut layout = Layout::new(container);
//! let empty = layout.save();
//!
//! // Imported item geometry is centered at the origin; angles here are in radians.
//! let pose = DTransformation::new(0.0, (5.0, 5.0));
//! let candidate = item.shape_cd().transform_clone(&pose.compose());
//! if !layout.cde().detect_poly_collision(&candidate, &NoFilter) {
//!     // Placement itself does not validate collisions or orientation permissions.
//!     layout.place_item(&item, pose);
//! }
//! assert_eq!(layout.placed_items().len(), 1);
//! assert!(layout.is_collision_free());
//! layout.restore(&empty)?;
//! assert!(layout.placed_items().is_empty());
//! # Ok(())
//! # }
//! ```

#![doc = document_features::document_features!()]

/// Everything related to the Collision Detection Engine
pub mod collision_detection;

/// Entities to model 2D Irregular Cutting and Packing Problems
pub mod entities;

/// Geometric primitives and base algorithms
pub mod geometry;

/// Importing problem instances into and exporting solutions out of this library
pub mod io;

/// Helper functions which do not belong to any specific module
pub mod util;

/// Enabled variants of the 2D irregular Cutting and Packing Problem.
pub mod probs;

/// Export the `web_time` crate's `Instant` type for compatibility with web environments.
pub type Instant = web_time::Instant;
