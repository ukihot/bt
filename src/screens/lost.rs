use bevy::prelude::*;
use bevy::text::FontSize;

use crate::app_state::AppState;
use crate::domain;
use crate::fonts::Fonts;
use crate::theme::FG;

pub struct LostPlugin;

impl Plugin for LostPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Lost), setup)
            .add_systems(Update, tick.run_if(in_state(AppState::Lost)))
            .add_systems(OnExit(AppState::Lost), teardown);
    }
}

const RETURN_TO_TITLE_SECS: f32 = 9.0;
const CORRUPTED_LINE_INTERVAL_SECS: f32 = 1.4;

#[derive(Component)]
struct LostRoot;

#[derive(Resource)]
struct LostState {
    container: Entity,
    return_timer: Timer,
    line_timer: Timer,
}

fn setup(mut commands: Commands) {
    let root = commands
        .spawn((
            LostRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .id();

    let container = commands
        .spawn(Node {
            width: Val::Percent(70.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .id();

    commands.entity(root).add_child(container);
    commands.insert_resource(LostState {
        container,
        return_timer: Timer::from_seconds(RETURN_TO_TITLE_SECS, TimerMode::Once),
        line_timer: Timer::from_seconds(CORRUPTED_LINE_INTERVAL_SECS, TimerMode::Repeating),
    });
}

fn tick(
    time: Res<Time>,
    mut commands: Commands,
    mut lost_state: ResMut<LostState>,
    mut next_state: ResMut<NextState<AppState>>,
    fonts: Res<Fonts>,
) {
    lost_state.line_timer.tick(time.delta());
    if lost_state.line_timer.just_finished() {
        let mut rng = rand::rng();
        let line = domain::corrupted_line(&mut rng);
        let entity = commands
            .spawn((
                Text::new(line.text),
                TextFont {
                    font: fonts.for_line(line.font).into(),
                    font_size: FontSize::Px(16.0),
                    ..default()
                },
                TextColor(FG),
            ))
            .id();
        commands.entity(lost_state.container).add_child(entity);
    }

    lost_state.return_timer.tick(time.delta());
    if lost_state.return_timer.is_finished() {
        next_state.set(AppState::Title);
    }
}

fn teardown(mut commands: Commands, query: Query<Entity, With<LostRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<LostState>();
}
