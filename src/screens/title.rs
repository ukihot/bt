use bevy::input_focus::AutoFocus;
use bevy::prelude::*;
use bevy::text::{EditableText, FontSize, TextCursorStyle};

use crate::app_state::AppState;
use crate::fonts::Fonts;
use crate::game_data::GameData;
use crate::theme::{DIM, FG};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Title), setup)
            .add_systems(Update, (handle_start, sync_name_cells).run_if(in_state(AppState::Title)))
            .add_systems(OnExit(AppState::Title), teardown);
    }
}

#[derive(Component)]
struct TitleRoot;

/// The real text-editing widget -- owns keyboard/IME input, the cursor, and
/// `max_characters`, but is never itself shown. Proportional fonts give
/// half-width and full-width glyphs different advance widths, so a single
/// flowing line of text can never line up with a fixed row of dashes
/// underneath it. Instead, `sync_name_cells` mirrors this widget's value,
/// one character at a time, into the fixed-width cells below -- that fixes
/// the correspondence by construction, regardless of what's typed.
#[derive(Component)]
struct NameInput;

/// One character's worth of `NAME_MAX_CHARS`, in the visible cell grid.
#[derive(Component)]
struct NameCell(usize);

/// Caps the recorded name at a fixed number of character cells so the
/// visible grid (see `setup`) can show exactly one dash per character.
const NAME_MAX_CHARS: usize = 7;
const NAME_FONT_SIZE: f32 = 20.0;
/// Wide enough for one full-width glyph at `NAME_FONT_SIZE` plus a little
/// breathing room. Every cell is this width regardless of what's typed --
/// that fixed width, not glyph metrics, is what keeps a half-width "a" and
/// a full-width "あ" both centered over their own single dash.
const NAME_CELL_WIDTH: f32 = 26.0;
const NAME_CELL_GAP: f32 = 4.0;

fn setup(mut commands: Commands, fonts: Res<Fonts>) {
    let font = fonts.normal();
    commands
        .spawn((
            TitleRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: px(16),
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Bakery Text"),
                TextFont { font: font.clone().into(), font_size: FontSize::Px(32.0), ..default() },
                TextColor(FG),
            ));
            parent.spawn((
                Text::new("文字だけのパン屋の記録"),
                TextFont { font: font.clone().into(), font_size: FontSize::Px(16.0), ..default() },
                TextColor(DIM),
            ));
            parent.spawn((
                Text::new("あなたの名を記帳してください"),
                TextFont { font: font.clone().into(), font_size: FontSize::Px(16.0), ..default() },
                TextColor(FG),
            ));
            parent.spawn((
                // Kept out of the visible layout entirely (zero-size,
                // absolutely positioned, hidden) -- it exists purely to own
                // keyboard/IME input and the real cursor. See `NameInput`.
                NameInput,
                EditableText {
                    max_characters: Some(NAME_MAX_CHARS),
                    allow_newlines: false,
                    ..default()
                },
                TextLayout::no_wrap(),
                TextFont {
                    font: font.clone().into(),
                    font_size: FontSize::Px(NAME_FONT_SIZE),
                    ..default()
                },
                TextColor(FG),
                TextCursorStyle::default(),
                AutoFocus,
                Node { position_type: PositionType::Absolute, ..default() },
                Visibility::Hidden,
            ));
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: px(NAME_CELL_GAP),
                    ..default()
                })
                .with_children(|parent| {
                    for i in 0..NAME_MAX_CHARS {
                        parent
                            .spawn(Node {
                                width: px(NAME_CELL_WIDTH),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                row_gap: px(2),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((
                                    NameCell(i),
                                    Text::new(""),
                                    TextFont {
                                        font: font.clone().into(),
                                        font_size: FontSize::Px(NAME_FONT_SIZE),
                                        ..default()
                                    },
                                    TextColor(FG),
                                ));
                                // A text-only stand-in for an input box's
                                // baseline -- no 罫線 (ruled line/border),
                                // just a dash under each cell, like a paper
                                // ledger's fill-in line.
                                parent.spawn((
                                    Text::new("―"),
                                    TextFont {
                                        font: font.clone().into(),
                                        font_size: FontSize::Px(NAME_FONT_SIZE),
                                        ..default()
                                    },
                                    TextColor(DIM),
                                ));
                            });
                    }
                });
            parent.spawn((
                Text::new("Enterで開店"),
                TextFont { font: font.clone().into(), font_size: FontSize::Px(14.0), ..default() },
                TextColor(DIM),
            ));
        });
}

fn handle_start(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut name_query: Query<&mut EditableText, With<NameInput>>,
    mut game_data: ResMut<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if !keyboard.just_pressed(KeyCode::Enter) {
        return;
    }
    let Ok(name_input) = name_query.single_mut() else {
        return;
    };
    let raw = name_input.value().to_string();
    let name = raw.trim();
    let name = if name.is_empty() { "名無し" } else { name }.to_string();
    *game_data = GameData::fresh();
    game_data.player_name = name;
    next_state.set(AppState::Playing);
}

/// Mirrors `NameInput`'s current value into the visible cell grid, one
/// character per `NameCell`, every frame -- the source of truth stays the
/// hidden `EditableText`; this just re-derives the display from it, the
/// same pattern `screens::playing::render::sync_log_display` uses for the
/// log.
fn sync_name_cells(
    name_query: Query<&EditableText, With<NameInput>>,
    mut cells: Query<(&NameCell, &mut Text)>,
) {
    let Ok(name_input) = name_query.single() else {
        return;
    };
    let typed: Vec<char> = name_input.value().chars().collect();
    for (cell, mut text) in &mut cells {
        text.0 = typed.get(cell.0).map(|c| c.to_string()).unwrap_or_default();
    }
}

fn teardown(mut commands: Commands, query: Query<Entity, With<TitleRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
