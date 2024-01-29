use bevy_ecs::prelude::*;
use bevy_utils::prelude::*;

use crate::{
    action::{Action, ActionMeta},
    ease::EaseFn,
    lerp::*,
    EmptyRes,
};

#[derive(Bundle, Default)]
pub struct SequenceBundle {
    pub sequence: Sequence,
    pub sequence_time: SequenceTime,
}

impl SequenceBundle {
    pub fn new(sequence: Sequence) -> Self {
        Self {
            sequence,
            ..default()
        }
    }
}

/// Plays the [`Sequence`] component attached to this entity through `target_time` manipulation.
#[derive(Component, Default)]
pub struct SequenceTime {
    pub(crate) curr_time: f32,
    pub target_time: f32,
}

/// A group of actions in chronological order.
#[derive(Component, Default)]
pub struct Sequence {
    duration: f32,
    pub(crate) action_metas: Vec<ActionMeta>,
}

impl Sequence {
    pub(crate) fn single(action_meta: ActionMeta) -> Self {
        let duration: f32 = action_meta.duration;
        Self {
            action_metas: vec![action_meta],
            duration,
        }
    }

    pub(crate) fn empty(duration: f32) -> Self {
        Self {
            duration,
            ..default()
        }
    }

    /// Add easing to all the [`Action`]s within this [`Sequence`].
    pub fn with_ease(mut self, ease_fn: EaseFn) -> Self {
        for action_meta in &mut self.action_metas {
            action_meta.ease_fn = ease_fn;
        }

        self
    }

    #[inline]
    pub fn duration(&self) -> f32 {
        self.duration
    }
}

/// Interpolation for [`SequenceTime`].
pub(crate) fn sequence_time_interp(
    player: &mut SequenceTime,
    begin: &f32,
    end: &f32,
    t: f32,
    _: &mut ResMut<EmptyRes>,
) {
    player.target_time = f32::lerp(begin, end, t);
}

// ANIMATION FLOW FUNCTIONS

/// Run one [`Sequence`] after another.
pub fn chain(sequences: &[Sequence]) -> Sequence {
    let mut final_sequence: Sequence = Sequence::default();
    let mut chain_duration: f32 = 0.0;

    for sequence in sequences {
        for action_meta in &sequence.action_metas {
            let mut action_meta: ActionMeta = action_meta.clone();

            action_meta.start_time += chain_duration;
            final_sequence.action_metas.push(action_meta);
        }

        chain_duration += sequence.duration;
    }

    final_sequence.duration = chain_duration;
    final_sequence
}

/// Run all [`Sequence`]s concurrently and wait for all of them to finish.
pub fn all(sequences: &[Sequence]) -> Sequence {
    let mut final_sequence: Sequence = Sequence::default();
    let mut max_duration: f32 = 0.0;

    for sequence in sequences {
        for action_meta in &sequence.action_metas {
            final_sequence.action_metas.push(action_meta.clone());
        }

        max_duration = f32::max(max_duration, sequence.duration);
    }

    final_sequence.duration = max_duration;
    final_sequence
}

/// Run all [`Sequence`]s concurrently and wait for any of them to finish.
pub fn any(sequences: &[Sequence]) -> Sequence {
    let mut final_sequence: Sequence = Sequence::default();
    let mut min_duration: f32 = 0.0;

    for action_grp in sequences {
        for action_meta in &action_grp.action_metas {
            final_sequence.action_metas.push(action_meta.clone());
        }

        min_duration = f32::min(min_duration, action_grp.duration);
    }

    final_sequence.duration = min_duration;
    final_sequence
}

/// Run one [`Sequence`] after another with a fixed delay time.
pub fn flow(delay: f32, sequences: &[Sequence]) -> Sequence {
    let mut final_sequence: Sequence = Sequence::default();
    let mut flow_duration: f32 = 0.0;
    let mut final_duration: f32 = 0.0;

    for sequence in sequences {
        for action_meta in &sequence.action_metas {
            let mut action_meta: ActionMeta = action_meta.clone();

            action_meta.start_time += flow_duration;
            final_sequence.action_metas.push(action_meta);
        }

        flow_duration += delay;
        final_duration = f32::max(final_duration, flow_duration + sequence.duration);
    }

    final_sequence.duration = final_duration;
    final_sequence
}

/// Run an [`Sequence`] after a fixed delay time.
pub fn delay(delay: f32, sequence: &Sequence) -> Sequence {
    let mut final_sequence: Sequence = Sequence::default();

    for action_meta in &sequence.action_metas {
        let mut action_meta: ActionMeta = action_meta.clone();

        action_meta.start_time += delay;
        final_sequence.action_metas.push(action_meta);
    }

    final_sequence.duration = sequence.duration + delay;
    final_sequence
}

/// Safely update the `target_time` in [`SequenceTime`] after performing all the necessary actions.
pub(crate) fn sequence_time_update_system(mut q_sequences: Query<(&Sequence, &mut SequenceTime)>) {
    for (sequence, mut sequence_time) in q_sequences.iter_mut() {
        sequence_time.target_time = f32::clamp(sequence_time.target_time, 0.0, sequence.duration());
        sequence_time.curr_time = sequence_time.target_time;
    }
}

/// System for playing the [`Action`]s that are inside the [`Sequence`].
pub fn sequence_update_system<CompType, InterpType, ResType>(
    mut q_components: Query<&mut CompType>,
    q_actions: Query<&Action<CompType, InterpType, ResType>>,
    q_sequences: Query<(&Sequence, &SequenceTime)>,
    mut resource: ResMut<ResType>,
) where
    CompType: Component,
    InterpType: Send + Sync + 'static,
    ResType: Resource,
{
    for (sequence, sequence_time) in q_sequences.iter() {
        play_sequence(
            &mut q_components,
            &q_actions,
            sequence,
            sequence_time,
            &mut resource,
        );
    }
}

fn play_sequence<CompType, InterpType, ResType>(
    q_components: &mut Query<&mut CompType>,
    q_actions: &Query<&Action<CompType, InterpType, ResType>>,
    sequence: &Sequence,
    sequence_time: &SequenceTime,
    resource: &mut ResMut<ResType>,
) where
    CompType: Component,
    InterpType: Send + Sync + 'static,
    ResType: Resource,
{
    // Do not perform any actions if there are no changes to the timeline timings
    // or there are no actions at all.
    if sequence_time.curr_time == sequence_time.target_time || sequence.action_metas.is_empty() {
        return;
    }

    let direction: i32 = f32::signum(sequence_time.target_time - sequence_time.curr_time) as i32;

    let timeline_start: f32 = f32::min(sequence_time.curr_time, sequence_time.target_time);
    let timeline_end: f32 = f32::max(sequence_time.curr_time, sequence_time.target_time);

    let mut start_index: usize = 0;
    let mut end_index: usize = sequence.action_metas.len() - 1;

    // Swap direction if needed
    if direction == -1 {
        start_index = end_index;
        end_index = 0;
    }

    let mut action_index: usize = start_index;

    // Loop through `Action`s in the direction that the timeline is going towards.
    loop {
        if action_index == (end_index as i32 + direction) as usize {
            break;
        }

        let action_meta: &ActionMeta = &sequence.action_metas[action_index];
        let action_id: Entity = action_meta.id();

        action_index = (action_index as i32 + direction) as usize;

        // Ignore if `ActionMeta` not in range
        if !time_range_overlap(
            action_meta.start_time,
            action_meta.end_time(),
            timeline_start,
            timeline_end,
        ) {
            continue;
        }

        // Ignore if `Action` does not exists
        let Ok(action) = q_actions.get(action_id) else {
            continue;
        };

        // Get component to mutate based on action id
        if let Ok(mut component) = q_components.get_mut(action.target_id) {
            let mut unit_time: f32 =
                (sequence_time.target_time - action_meta.start_time) / action_meta.duration;

            // In case of division by 0.0
            if f32::is_nan(unit_time) {
                unit_time = 0.0;
            }

            unit_time = f32::clamp(unit_time, 0.0, 1.0);
            // Calculate unit time using ease function
            unit_time = (action_meta.ease_fn)(unit_time);

            // Mutate the component using interpolate function
            (action.interp_fn)(
                &mut component,
                &action.begin,
                &action.end,
                unit_time,
                resource,
            );
        }
    }
}

/// Calculate if 2 time range (in float) overlaps.
fn time_range_overlap(a_begin: f32, a_end: f32, b_begin: f32, b_end: f32) -> bool {
    a_begin <= b_end && b_begin <= a_end
}
