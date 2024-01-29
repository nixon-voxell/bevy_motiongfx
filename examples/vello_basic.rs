use bevy::{
    math::{DVec2, DVec4},
    prelude::*,
};
use bevy_motiongfx::prelude::*;

fn main() {
    App::new()
        // Bevy plugins
        .add_plugins(DefaultPlugins)
        // Custom plugins
        .add_plugins((MotionGfx, MotionGfxBevy, MotionGfxVello))
        .add_systems(Startup, (setup_system, vello_basic_system))
        .add_systems(Update, timeline_movement_system)
        .run();
}

fn vello_basic_system(mut commands: Commands, mut fragments: ResMut<Assets<VelloFragment>>) {
    // Color palette
    let palette: ColorPalette<ColorKey> = ColorPalette::default();

    // Spawning entities
    let rect_bundle: VelloRectBundle = VelloRectBundle {
        rect: VelloRect::anchor_center(DVec2::new(100.0, 100.0), DVec4::splat(10.0)),
        fill: FillStyle::from_brush(*palette.get_or_default(&ColorKey::Blue)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::Blue) * 1.5)
            .with_style(4.0),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(Transform::from_xyz(-200.0, 0.0, 0.0)),
            ..default()
        },
    };

    let circ_bundle: VelloCircleBundle = VelloCircleBundle {
        circle: VelloCircle::from_radius(50.0),
        fill: FillStyle::from_brush(*palette.get_or_default(&ColorKey::Purple)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::Purple) * 1.5)
            .with_style(4.0),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(Transform::from_xyz(200.0, 0.0, 0.0)),
            ..default()
        },
    };

    let line_bundle: VelloLineBundle = VelloLineBundle {
        line: VelloLine::from_points(DVec2::new(-300.0, 0.0), DVec2::new(300.0, 0.0)),
        stroke: StrokeStyle::from_brush(*palette.get_or_default(&ColorKey::Base8)),
        fragment_bundle: VelloFragmentBundle {
            fragment: fragments.add(VelloFragment::default()),
            transform: TransformBundle::from_transform(Transform::from_xyz(0.0, -100.0, 0.0)),
            ..default()
        },
    };

    let rect_id: Entity = commands.spawn(rect_bundle.clone()).id();
    let circ_id: Entity = commands.spawn(circ_bundle.clone()).id();
    let line_id: Entity = commands.spawn(line_bundle.clone()).id();

    // Motions
    let mut rect_motion: VelloRectBundleMotion = VelloRectBundleMotion::new(rect_id, rect_bundle);
    let mut circ_motion: VelloCircleBundleMotion =
        VelloCircleBundleMotion::new(circ_id, circ_bundle);
    let mut line_motion: VelloLineBundleMotion = VelloLineBundleMotion::new(line_id, line_bundle);

    // Sequence
    let sequence: Sequence = flow(
        0.5,
        &[
            // Line animation
            chain(&[
                all(&[
                    commands.play(
                        line_motion
                            .transform
                            .translate_add(Vec3::new(0.0, -100.0, 0.0)),
                        1.5,
                    ),
                    commands.play(line_motion.line.extend(100.0), 1.0),
                    commands.play(line_motion.stroke.style_to(10.0), 1.0),
                ]),
                all(&[
                    commands.play(
                        line_motion
                            .transform
                            .translate_add(Vec3::new(0.0, 100.0, 0.0)),
                        1.5,
                    ),
                    commands.play(line_motion.line.extend(-100.0), 1.0),
                    commands.play(line_motion.stroke.style_to(1.0), 1.0),
                ]),
            ]),
            // Rect animation
            chain(&[
                all(&[
                    commands.play(rect_motion.rect.inflate(DVec2::splat(50.0)), 1.0),
                    commands.play(
                        rect_motion.transform.rotate_to(Quat::from_euler(
                            EulerRot::XYZ,
                            0.0,
                            0.0,
                            std::f32::consts::PI,
                        )),
                        1.0,
                    ),
                    commands.play(rect_motion.stroke.style_to(20.0), 1.0),
                ]),
                all(&[
                    commands.play(rect_motion.rect.inflate(-DVec2::splat(50.0)), 1.0),
                    commands.play(
                        rect_motion.transform.rotate_to(Quat::from_euler(
                            EulerRot::XYZ,
                            0.0,
                            0.0,
                            std::f32::consts::TAU,
                        )),
                        1.0,
                    ),
                    commands.play(rect_motion.stroke.style_to(4.0), 1.0),
                ]),
            ]),
            // Circle animation
            chain(&[
                all(&[
                    commands.play(circ_motion.circle.inflate(50.0), 1.0),
                    commands.play(circ_motion.stroke.style_to(20.0), 1.0),
                ]),
                all(&[
                    commands.play(circ_motion.circle.inflate(-50.0), 1.0),
                    commands.play(circ_motion.stroke.style_to(4.0), 1.0),
                ]),
            ]),
        ],
    )
    .with_ease(ease::cubic::ease_in_out);

    commands.spawn(SequencePlayerBundle {
        sequence,
        ..default()
    });
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn timeline_movement_system(
    mut q_timelines: Query<(&mut SequencePlayer, &mut SequenceTime)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut sequence_player, mut sequence_time) in q_timelines.iter_mut() {
        if keys.pressed(KeyCode::D) {
            sequence_time.target_time += time.delta_seconds();
        }

        if keys.pressed(KeyCode::A) {
            sequence_time.target_time -= time.delta_seconds();
        }

        if keys.pressed(KeyCode::Space) && keys.pressed(KeyCode::ShiftLeft) {
            sequence_player.time_scale = -1.0;
        } else if keys.pressed(KeyCode::Space) {
            sequence_player.time_scale = 1.0;
        }
    }
}
