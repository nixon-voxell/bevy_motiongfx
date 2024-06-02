use bevy::prelude::*;
use prelude::{update_asset, update_component};
use sequence::{sequence_controller, sequence_player};
use slide::slide_controller;

pub mod action;
pub mod color_palette;
pub mod ease;
pub mod f32lerp;
pub mod motion;
pub mod sequence;
pub mod slide;

pub mod prelude {
    pub use crate::{
        action::{act, Action, SequenceBuilderExt},
        color_palette::{ColorKey, ColorPalette},
        ease,
        f32lerp::*,
        motion::{pbr_motion::BuildPbrMotionExt, transform_motion::TransformMotion, GetId},
        sequence::{
            all, any, chain, delay, flow, update_asset, update_component, MultiSequenceOrdering,
            Sequence, SequenceBundle, SequenceController, SequencePlayer, SequencePlayerBundle,
            SingleSequenceOrdering,
        },
        slide::{create_slide, SlideBundle, SlideController, SlideCurrState, SlideTargetState},
        MotionGfxPlugin, UpdateSequenceSet,
    };
}

pub struct MotionGfxPlugin;

impl Plugin for MotionGfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (sequence_player, slide_controller).before(UpdateSequenceSet),
        )
        .add_systems(
            Update,
            (
                update_component::<Transform, Vec3>,
                update_component::<Transform, Quat>,
                update_component::<Transform, f32>,
                update_component::<Sprite, Color>,
                update_component::<Sprite, f32>,
                update_asset::<StandardMaterial, Color>,
                update_asset::<StandardMaterial, f32>,
                update_asset::<ColorMaterial, Color>,
                update_asset::<ColorMaterial, f32>,
            )
                .in_set(UpdateSequenceSet),
        )
        .add_systems(Update, sequence_controller.after(UpdateSequenceSet));
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct UpdateSequenceSet;
