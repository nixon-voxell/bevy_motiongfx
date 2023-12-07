pub use bevy_vello_renderer;

use bevy_app::prelude::*;
use bevy_vello_renderer::{
    prelude::*,
    vello::{kurbo, peniko},
};
use motiongfx_core::prelude::*;

mod convert;
mod fill_style;
mod stroke_style;
mod vello_motion;
mod vello_vector;

pub mod prelude {
    pub use crate::{
        convert::*,
        fill_style::{FillStyle, FillStyleMotion},
        stroke_style::{StrokeStyle, StrokeStyleMotion},
        vello_motion::{
            circle_motion::{VelloCircleBundleMotion, VelloCircleMotion},
            line_motion::{VelloLineBundleMotion, VelloLineMotion},
            rect_motion::{VelloRectBundleMotion, VelloRectMotion},
            text_motion::{VelloTextSimpleBundleMotion, VelloTextSimpleMotion},
        },
        vello_vector::{
            circle::{VelloCircle, VelloCircleBundle},
            line::{VelloLine, VelloLineBundle},
            rect::{VelloRect, VelloRectBundle},
            text::{VelloTextSimple, VelloTextSimpleBundle},
        },
    };
    pub use bevy_vello_renderer::prelude::*;
}

pub struct MotionGfxVello;

impl Plugin for MotionGfxVello {
    fn build(&self, app: &mut App) {
        app.add_plugins(VelloRenderPlugin)
            .add_plugins((
                vello_motion::circle_motion::VelloCircleMotionPlugin,
                vello_motion::rect_motion::VelloRectMotionPlugin,
                vello_motion::line_motion::VelloLineMotionPlugin,
                vello_motion::text_motion::VelloTextSimpleMotionPlugin,
            ))
            .add_systems(
                PostUpdate,
                (
                    // Vector builders
                    vello_vector::vector_builder::<vello_vector::rect::VelloRect>,
                    vello_vector::vector_builder::<vello_vector::circle::VelloCircle>,
                    vello_vector::vector_builder::<vello_vector::line::VelloLine>,
                    vello_vector::vector_builder::<vello_vector::text::VelloTextSimple>,
                    // Sequences
                    sequence_player_system::<fill_style::FillStyle, peniko::Brush, EmptyRes>,
                    sequence_player_system::<stroke_style::StrokeStyle, peniko::Brush, EmptyRes>,
                    sequence_player_system::<stroke_style::StrokeStyle, kurbo::Stroke, EmptyRes>,
                ),
            );
    }
}
