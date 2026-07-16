use bevy::prelude::*;

use crate::app_state::AppState;
use crate::game_data::GameData;

pub(super) fn corruption_check(
    game_data: Res<GameData>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if game_data.corruption >= 100.0 {
        next_state.set(AppState::Lost);
    }
}
