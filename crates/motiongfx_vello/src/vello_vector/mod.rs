use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_vello_renderer::{prelude::*, vello};

use crate::fill_style::FillStyle;
use crate::stroke_style::StrokeStyle;

pub mod circle;
pub mod line;
pub mod rect;

pub(crate) trait VelloVector {
    fn build_fill(&self, fill: &FillStyle, builder: &mut vello::SceneBuilder) {}

    fn build_stroke(&self, stroke: &StrokeStyle, builder: &mut vello::SceneBuilder) {}
}

pub(crate) trait VelloBuilder {
    fn should_build(&self) -> bool;

    fn set_should_build(&mut self, should_build: bool);
}

pub(crate) fn vector_builder<Vector: VelloVector + VelloBuilder + Component>(
    mut q_fill_only_vectors: Query<
        (&mut Vector, &mut FillStyle, &Handle<VelloFragment>),
        Without<StrokeStyle>,
    >,
    mut q_stroke_only_vectors: Query<
        (&mut Vector, &mut StrokeStyle, &Handle<VelloFragment>),
        Without<FillStyle>,
    >,
    mut q_fill_and_stroke_vectors: Query<(
        &mut Vector,
        &mut FillStyle,
        &mut StrokeStyle,
        &Handle<VelloFragment>,
    )>,
    mut fragments: ResMut<Assets<VelloFragment>>,
) {
    for (mut vector, mut fill, fragment_handle) in q_fill_only_vectors.iter_mut() {
        if let Some(fragment) = fragments.get_mut(fragment_handle.id()) {
            let mut frag: vello::SceneFragment = vello::SceneFragment::new();
            let mut builder: vello::SceneBuilder = vello::SceneBuilder::for_fragment(&mut frag);

            if vector.should_build() == false && fill.should_build() == false {
                continue;
            }

            // Build the vector to the VelloFragment
            vector.build_fill(&fill, &mut builder);

            // Set it to false after building
            fill.set_should_build(false);
            vector.set_should_build(false);

            // Replace with new fragment
            fragment.fragment = frag.into();
        }
    }

    for (mut vector, mut stroke, fragment_handle) in q_stroke_only_vectors.iter_mut() {
        if let Some(fragment) = fragments.get_mut(fragment_handle.id()) {
            let mut frag: vello::SceneFragment = vello::SceneFragment::new();
            let mut builder: vello::SceneBuilder = vello::SceneBuilder::for_fragment(&mut frag);

            if vector.should_build() == false && stroke.should_build() == false {
                continue;
            }

            // Build the vector to the VelloFragment
            vector.build_stroke(&stroke, &mut builder);

            // Set it to false after building
            stroke.set_should_build(false);
            vector.set_should_build(false);

            // Replace with new fragment
            fragment.fragment = frag.into();
        }
    }

    for (mut vector, mut fill, mut stroke, fragment_handle) in q_fill_and_stroke_vectors.iter_mut()
    {
        if let Some(fragment) = fragments.get_mut(fragment_handle.id()) {
            let mut frag: vello::SceneFragment = vello::SceneFragment::new();
            let mut builder: vello::SceneBuilder = vello::SceneBuilder::for_fragment(&mut frag);

            if vector.should_build() == false
                && fill.should_build() == false
                && stroke.should_build() == false
            {
                continue;
            }

            // Build the vector to the VelloFragment
            vector.build_fill(&fill, &mut builder);
            vector.build_stroke(&stroke, &mut builder);

            // Set it to false after building
            fill.set_should_build(false);
            stroke.set_should_build(false);
            vector.set_should_build(false);

            // Replace with new fragment
            fragment.fragment = frag.into();
        }
    }
}
