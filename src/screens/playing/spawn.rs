use std::collections::VecDeque;

use bevy::prelude::*;
use bevy::text::FontSize;

use super::pane::PaneRuntime;
use super::pending::{Pending, PendingLine};
use super::render::{CURSOR_BLANK, LOG_FONT_SIZE, MARK_BLANK, render_resolved_line, shrinkable};
use crate::domain::{self, Classification, LogLine, Pane};
use crate::fonts::Fonts;
use crate::game_data::GameData;
use crate::theme::FG;

const MAX_SPAWNED: usize = 400;

#[derive(Component)]
pub(super) struct LogUi {
    pub(super) container: Entity,
    pub(super) spawned: VecDeque<Entity>,
}

pub(super) fn spawn_line_ui(
    commands: &mut Commands,
    log_ui: &mut LogUi,
    fonts: &Fonts,
    line: &LogLine,
) -> Entity {
    let entity = commands
        .spawn((
            Text::new(format!("{CURSOR_BLANK}{MARK_BLANK}{}", line.text)),
            // `Outside`/`Floor` are narrow enough that a long line would
            // otherwise wrap onto a second visual row, which desyncs what's
            // on screen from `Pending`'s one-row-per-line bookkeeping (its
            // capacity counts *lines*, not wrapped rows). Clipping the tail
            // instead (via the container's own `Overflow::clip`) keeps that
            // invariant exact regardless of pane width.
            TextLayout::no_wrap(),
            TextFont {
                font: fonts.for_line(line.font).into(),
                font_size: FontSize::Px(LOG_FONT_SIZE),
                ..default()
            },
            TextColor(FG),
            // Every spawned line gets its own width-safe `Node` too, not
            // just its ancestors -- see `render::shrinkable`.
            shrinkable(Node { width: Val::Percent(100.0), ..default() }),
        ))
        .id();
    commands.entity(log_ui.container).add_child(entity);
    log_ui.spawned.push_back(entity);
    if log_ui.spawned.len() > MAX_SPAWNED
        && let Some(old) = log_ui.spawned.pop_front()
    {
        commands.entity(old).despawn();
    }
    entity
}

/// Ticks the one shared `DayClock` (CLAUDE.md §3.7) and re-derives `phase`
/// from it every frame, rather than running phase transitions off their own
/// independent timer -- this is the single source of "what time it is" that
/// every pane's line-stamping and weighting reads (`domain::generate`).
pub(super) fn phase_tick(
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
    mut panes: Query<&mut PaneRuntime>,
) {
    let wraps = game_data.clock.advance(time.delta_secs());
    if wraps > 0 {
        let mut rng = rand::rng();
        for _ in 0..wraps {
            game_data.day += 1;
            game_data.zone = game_data.zone.next();
            let day = game_data.day;
            // ルールの効果は日をまたぐたびに白紙に戻る(CLAUDE.md §3.4) --
            // Cast(誰が何を言うか)はそのまま、聞いた噂の履歴だけが消える。
            game_data.rule_ledger.reset_day();
            // 日替わりの記帳は、店主の記録である焼成室にだけ流れる(第5節)。
            if let Some(mut kiln) = panes.iter_mut().find(|p| p.pane == Pane::Kiln) {
                kiln.pending_scripted.push_back(domain::day_marker(day));
            }
            // 「昨日までの制約が明けた」ことをほのめかす一言は、ルール変更の
            // 発信源である売り場に流す(第3.4節)。
            if let Some(mut floor) = panes.iter_mut().find(|p| p.pane == Pane::Floor) {
                let notice = domain::rule_reset_notice(game_data.clock, &mut rng);
                floor.pending_scripted.push_back(notice);
            }
        }
    }

    let next_phase = domain::Phase::for_hour(game_data.clock.hour());
    if next_phase != game_data.phase {
        game_data.phase = next_phase;
        for mut runtime in &mut panes {
            runtime.retime(next_phase);
        }
    }
    game_data.maybe_queue_name_call();
}

/// Drives all three panes' log spawning from one system rather than three
/// near-identical copies -- each pane ticks its own `spawn_timer` and, on
/// its own schedule, generates and resolves its own lines exactly as the
/// single log used to.
pub(super) fn line_spawn(
    time: Res<Time>,
    mut commands: Commands,
    mut game_data: ResMut<GameData>,
    mut panes: Query<(&mut PaneRuntime, &mut LogUi, &mut Pending)>,
    fonts: Res<Fonts>,
    mut texts: Query<&mut Text>,
) {
    let mut rng = rand::rng();

    for (mut runtime, mut log_ui, mut pending) in &mut panes {
        runtime.spawn_timer.tick(time.delta());
        if !runtime.spawn_timer.just_finished() {
            continue;
        }

        let line = if let Some(scripted) = runtime.pending_scripted.pop_front() {
            scripted
        } else {
            let last_normal = runtime.last_normal_line.clone();
            let clock = game_data.clock;
            let zone = game_data.zone;
            let day = game_data.day;
            let corruption = game_data.corruption;
            domain::generate(
                runtime.pane,
                clock,
                zone,
                day,
                last_normal.as_deref(),
                &mut game_data.rule_ledger,
                corruption,
                &mut rng,
            )
        };

        if line.classification == Classification::Normal {
            runtime.last_normal_line = Some(line.text.clone());
        }

        let entity = spawn_line_ui(&mut commands, &mut log_ui, &fonts, &line);

        // Scripted beats (日替わり, 二人称の障り, ...) are never reachable --
        // they don't enter the cursor's pending window at all. 呼ばれる is
        // scripted too, but never reaches even this far: see `GameData`.
        if !line.scripted {
            let pending_line = PendingLine {
                entity,
                text: line.text.clone(),
                classification: line.classification,
                relief: line.relief,
                mark: None,
                delete_wipe: 0.0,
            };
            for expired in pending.push(pending_line) {
                if let Ok(mut text) = texts.get_mut(expired.entity) {
                    text.0 = render_resolved_line(&expired);
                }
                let outcome = domain::resolve(expired.classification, expired.mark, expired.relief);
                let was_mistake = outcome.corruption > 0.0;
                apply_outcome(&mut game_data, outcome);
                if let Some(verb) = expired.mark
                    && was_mistake
                    && !game_data.first_mistake_done
                {
                    game_data.first_mistake_done = true;
                    let beat = domain::mistake_beat(game_data.clock, verb);
                    runtime.pending_scripted.push_back(beat);
                }
            }
        }
    }

    game_data.maybe_queue_name_call();
}

pub(super) fn apply_outcome(game_data: &mut GameData, outcome: domain::Outcome) {
    game_data.corruption = (game_data.corruption + outcome.corruption).clamp(0.0, 100.0);
    game_data.income += outcome.income;
    if outcome.zone_bump {
        game_data.zone = game_data.zone.next();
    }
}
