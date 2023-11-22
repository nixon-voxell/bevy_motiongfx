use bevy_ecs::prelude::*;
use bevy_vello_renderer::vello::{kurbo, peniko};
use motiongfx_core::prelude::*;

use crate::convert::*;
use crate::vello_vector::VelloBuilder;

#[derive(Component, Clone)]
pub struct StrokeStyle {
    pub(crate) style: kurbo::Stroke,
    pub(crate) brush: peniko::Brush,
    should_build: bool,
}

impl StrokeStyle {
    #[inline]
    pub fn from_brush(brush: impl Into<PenikoBrush>) -> Self {
        Self::default().with_brush(brush)
    }

    #[inline]
    pub fn with_style(mut self, style: kurbo::Stroke) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_brush(mut self, brush: impl Into<PenikoBrush>) -> Self {
        self.brush = brush.into().0;
        self
    }
}

impl VelloBuilder for StrokeStyle {
    #[inline]
    fn should_build(&self) -> bool {
        self.should_build
    }

    #[inline]
    fn set_should_build(&mut self, should_build: bool) {
        self.should_build = should_build
    }
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            style: kurbo::Stroke::default(),
            brush: peniko::Brush::Solid(peniko::Color::WHITE_SMOKE),
            should_build: false,
        }
    }
}

pub struct StrokeStyleMotion {
    target_id: Entity,
    stroke: StrokeStyle,
}

impl StrokeStyleMotion {
    pub fn new(target_id: Entity, stroke: StrokeStyle) -> Self {
        Self { target_id, stroke }
    }
    // =====================
    // Stroke brush
    // =====================
    pub fn brush_to(
        &mut self,
        new_brush: impl Into<PenikoBrush>,
    ) -> Action<StrokeStyle, peniko::Brush, EmptyRes> {
        let new_brush: peniko::Brush = new_brush.into().0;

        let action: Action<StrokeStyle, peniko::Brush, EmptyRes> = Action::new(
            self.target_id,
            self.stroke.brush.clone(),
            new_brush.clone(),
            Self::brush_interp,
        );

        self.stroke.brush = new_brush;

        action
    }

    fn brush_interp(
        stroke: &mut StrokeStyle,
        begin: &peniko::Brush,
        end: &peniko::Brush,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        stroke.brush = peniko::Brush::lerp(begin, end, t);
        stroke.set_should_build(true);
    }

    // =====================
    // Stroke style
    // =====================
    pub fn style_to(
        &mut self,
        new_style: impl Into<KurboStroke>,
    ) -> Action<StrokeStyle, kurbo::Stroke, EmptyRes> {
        let new_style: kurbo::Stroke = new_style.into().0;

        let action: Action<StrokeStyle, kurbo::Stroke, EmptyRes> = Action::new(
            self.target_id,
            self.stroke.style.clone(),
            new_style.clone(),
            Self::style_interp,
        );

        self.stroke.style = new_style;

        action
    }

    fn style_interp(
        stroke: &mut StrokeStyle,
        begin: &kurbo::Stroke,
        end: &kurbo::Stroke,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        stroke.style = kurbo::Stroke::lerp(begin, end, t);
        stroke.set_should_build(true);
    }
}
