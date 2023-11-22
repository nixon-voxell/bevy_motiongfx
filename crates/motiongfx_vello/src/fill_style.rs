use bevy_ecs::prelude::*;
use bevy_render::prelude::*;
use bevy_utils::prelude::*;
use bevy_vello_renderer::vello::peniko;
use motiongfx_core::prelude::*;

use crate::convert::*;
use crate::vello_vector::VelloBuilder;

#[derive(Component, Clone)]
pub struct FillStyle {
    pub(crate) style: peniko::Fill,
    pub(crate) brush: peniko::Brush,
    should_build: bool,
}

impl FillStyle {
    #[inline]
    pub fn from_brush(brush: impl Into<PenikoBrush>) -> Self {
        Self::default().with_brush(brush)
    }

    #[inline]
    pub fn with_style(mut self, style: peniko::Fill) -> Self {
        self.style = style;
        self
    }

    #[inline]
    pub fn with_brush(mut self, brush: impl Into<PenikoBrush>) -> Self {
        self.brush = brush.into().0;
        self
    }
}

impl VelloBuilder for FillStyle {
    #[inline]
    fn should_build(&self) -> bool {
        self.should_build
    }

    #[inline]
    fn set_should_build(&mut self, should_build: bool) {
        self.should_build = should_build
    }
}

impl Default for FillStyle {
    fn default() -> Self {
        Self {
            style: peniko::Fill::NonZero,
            brush: peniko::Brush::Solid(peniko::Color::WHITE_SMOKE),
            should_build: false,
        }
    }
}

impl From<Color> for FillStyle {
    fn from(value: Color) -> Self {
        Self {
            brush: peniko::Brush::Solid(peniko::Color::rgba(
                value.r() as f64,
                value.g() as f64,
                value.b() as f64,
                value.a() as f64,
            )),
            ..default()
        }
    }
}

pub struct FillStyleMotion {
    target_id: Entity,
    fill: FillStyle,
}

impl FillStyleMotion {
    pub fn new(target_id: Entity, fill: FillStyle) -> Self {
        Self { target_id, fill }
    }

    pub fn brush_to(
        &mut self,
        new_brush: impl Into<PenikoBrush>,
    ) -> Action<FillStyle, peniko::Brush, EmptyRes> {
        let new_brush: peniko::Brush = new_brush.into().0;

        let action: Action<FillStyle, peniko::Brush, EmptyRes> = Action::new(
            self.target_id,
            self.fill.brush.clone(),
            new_brush.clone(),
            Self::brush_interp,
        );

        self.fill.brush = new_brush;

        action
    }

    fn brush_interp(
        fill: &mut FillStyle,
        begin: &peniko::Brush,
        end: &peniko::Brush,
        t: f32,
        _: &mut ResMut<EmptyRes>,
    ) {
        fill.brush = peniko::Brush::lerp(begin, end, t);
        fill.set_should_build(true);
    }
}
