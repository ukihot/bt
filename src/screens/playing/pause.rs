use bevy::prelude::*;
use bevy::text::FontSize;

use super::render::{CURSOR_BLANK, CURSOR_MARK};
use super::setup::PlayingRoot;
use crate::app_state::AppState;
use crate::fonts::Fonts;
use crate::theme::FG;

/// Whether ESC-pause is in effect. Deliberately not an `AppState` variant:
/// pausing must not trigger `OnExit(Playing)` / `OnEnter(Playing)`, since
/// that would tear down and respawn the whole log instead of just freezing
/// it in place.
#[derive(Resource, Default)]
pub(super) struct PauseState {
    paused: bool,
    selection: PauseOption,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum PauseOption {
    #[default]
    Resume,
    ReturnToTitle,
}

impl PauseOption {
    fn label(self) -> &'static str {
        match self {
            PauseOption::Resume => "続ける",
            PauseOption::ReturnToTitle => "タイトルに戻る",
        }
    }

    fn toggled(self) -> Self {
        match self {
            PauseOption::Resume => PauseOption::ReturnToTitle,
            PauseOption::ReturnToTitle => PauseOption::Resume,
        }
    }
}

#[derive(Component)]
pub(super) struct PauseRoot;

#[derive(Component)]
pub(super) struct PauseOptionText(PauseOption);

pub(super) fn not_paused(pause: Res<PauseState>) -> bool {
    !pause.paused
}

fn pause_option_line(option: PauseOption, selection: PauseOption) -> String {
    let mark = if option == selection { CURSOR_MARK } else { CURSOR_BLANK };
    format!("{mark}{}", option.label())
}

fn spawn_pause_overlay(commands: &mut Commands, fonts: &Fonts, selection: PauseOption) {
    let font = fonts.normal();
    commands
        .spawn((
            PauseRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(12),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("一時停止"),
                TextFont { font: font.clone().into(), font_size: FontSize::Px(20.0), ..default() },
                TextColor(FG),
            ));
            for option in [PauseOption::Resume, PauseOption::ReturnToTitle] {
                parent.spawn((
                    PauseOptionText(option),
                    Text::new(pause_option_line(option, selection)),
                    TextFont {
                        font: font.clone().into(),
                        font_size: FontSize::Px(16.0),
                        ..default()
                    },
                    TextColor(FG),
                ));
            }
        });
}

#[allow(clippy::too_many_arguments)]
pub(super) fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut pause: ResMut<PauseState>,
    mut next_state: ResMut<NextState<AppState>>,
    fonts: Res<Fonts>,
    pause_root: Query<Entity, With<PauseRoot>>,
    mut playing_visibility: Query<&mut Visibility, With<PlayingRoot>>,
    mut option_texts: Query<(&PauseOptionText, &mut Text)>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        pause.paused = !pause.paused;
        if let Ok(mut visibility) = playing_visibility.single_mut() {
            *visibility = if pause.paused { Visibility::Hidden } else { Visibility::Inherited };
        }
        if pause.paused {
            pause.selection = PauseOption::default();
            spawn_pause_overlay(&mut commands, &fonts, pause.selection);
        } else {
            for entity in &pause_root {
                commands.entity(entity).despawn();
            }
        }
        return;
    }

    if !pause.paused {
        return;
    }

    if keyboard.just_pressed(KeyCode::KeyJ) || keyboard.just_pressed(KeyCode::KeyK) {
        pause.selection = pause.selection.toggled();
        for (marker, mut text) in &mut option_texts {
            text.0 = pause_option_line(marker.0, pause.selection);
        }
    }

    if keyboard.just_pressed(KeyCode::Enter) {
        pause.paused = false;
        if let Ok(mut visibility) = playing_visibility.single_mut() {
            *visibility = Visibility::Inherited;
        }
        for entity in &pause_root {
            commands.entity(entity).despawn();
        }
        if pause.selection == PauseOption::ReturnToTitle {
            next_state.set(AppState::Title);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggled_swaps_between_the_two_options() {
        assert_eq!(PauseOption::Resume.toggled(), PauseOption::ReturnToTitle);
        assert_eq!(PauseOption::ReturnToTitle.toggled(), PauseOption::Resume);
    }

    #[test]
    fn pause_option_line_marks_only_the_selected_option() {
        let selected = pause_option_line(PauseOption::Resume, PauseOption::Resume);
        let other = pause_option_line(PauseOption::ReturnToTitle, PauseOption::Resume);
        assert!(selected.starts_with(CURSOR_MARK));
        assert!(other.starts_with(CURSOR_BLANK));
    }
}
