pub use bevy_vello_renderer;

use bevy_app::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{kurbo, peniko},
};
use motiongfx_core::prelude::*;

pub mod convert;
pub mod vector_style;
pub mod vello_vector;

pub struct MotionGfxVello;

impl Plugin for MotionGfxVello {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloRenderPlugin)
            // .add_systems(PostStartup)
            .add_systems(
                PostUpdate,
                (
                    // Vector builders
                    vello_vector::vector_builder::<vello_vector::rect::VelloRect>,
                    vello_vector::vector_builder::<vello_vector::circ::VelloCircle>,
                    // Sequences
                    sequence_player_system::<vello_vector::rect::VelloRect, kurbo::Rect, EmptyRes>,
                    sequence_player_system::<vector_style::FillStyle, peniko::Brush, EmptyRes>,
                    sequence_player_system::<vector_style::StrokeStyle, peniko::Brush, EmptyRes>,
                    sequence_player_system::<vector_style::StrokeStyle, kurbo::Stroke, EmptyRes>,
                    sequence_player_system::<
                        vello_vector::rect::VelloRect,
                        kurbo::RoundedRectRadii,
                        EmptyRes,
                    >,
                ),
            );
    }
}
