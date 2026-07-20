use bevy::prelude::*;
use bevy::text::FontSize;

use super::day_indicator::spawn_day_indicator;
use super::glitch::GlitchTimer;
use super::intrusion::{ActiveIntrusion, IntrusionSlot};
use super::pane::{ActivePane, PaneRuntime};
use super::pause::{PauseRoot, PauseState};
use super::pending::Pending;
use super::render::{CURSOR_BLANK, PaneHeader, container_height_px, shrinkable};
use super::spawn::LogUi;
use crate::domain::{LineFont, Pane, Phase};
use crate::fonts::Fonts;
use crate::game_data::GameData;
use crate::theme::{DIM, FG, MONITOR_BG};

#[derive(Component)]
pub(super) struct PlayingRoot;

/// Builds one pane's box (header + clipped log container) and attaches the
/// components that make it a pane rather than plain UI: `PaneRuntime`
/// (clock, own history, own scripted queue), `Pending` (cursor/window), and
/// `LogUi` (where spawned lines get parented). `node` is the caller's own
/// layout for the box itself, since `Kiln` (alone, full width) and
/// `Outside`/`Floor` (sharing a row) size themselves differently.
fn spawn_pane(
    commands: &mut Commands,
    fonts: &Fonts,
    pane: Pane,
    phase: Phase,
    mut node: Node,
) -> Entity {
    let capacity = pane.capacity();
    // Padding lives on the pane box itself (not a gap between boxes) so the
    // three monitors can still sit frame-to-frame with zero seam (第3.1節).
    // The box grows past its own `padding` via `flex_grow` (set by the
    // caller) to fill whatever space the window has beyond `capacity`
    // lines' worth of content -- that leftover space stays inside the
    // colored monitor box (`MONITOR_BG`) instead of becoming blank void
    // around the grid, so a maximized window doesn't read as mostly empty.
    // `SpaceBetween` sends all of that grown space into the single gap
    // between `header` and `container`, pinning the header to the box's
    // top edge and the log window to its bottom edge -- lines still stack
    // up from the bottom of their own capacity window, not from the
    // vertical middle of the box.
    node.padding = px(14).all();
    node.justify_content = JustifyContent::SpaceBetween;

    let header = commands
        .spawn((
            PaneHeader(pane),
            Text::new(format!("{CURSOR_BLANK}{}", pane.label())),
            TextFont { font: fonts.normal().into(), font_size: FontSize::Px(14.0), ..default() },
            TextColor(DIM),
        ))
        .id();

    let container = commands
        .spawn(shrinkable(Node {
            width: Val::Percent(100.0),
            height: Val::Px(container_height_px(capacity)),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            overflow: Overflow::clip(),
            ..default()
        }))
        .id();

    let entity = commands
        .spawn((
            PaneRuntime::new(pane, phase),
            Pending::new(capacity),
            LogUi { container, spawned: Default::default() },
            BackgroundColor(MONITOR_BG),
            node,
        ))
        .id();
    commands.entity(entity).add_children(&[header, container]);
    entity
}

fn split_pane_node() -> Node {
    shrinkable(Node {
        // A zero basis forces `Outside`/`Floor` to split their row exactly
        // in half regardless of how much text either happens to hold --
        // otherwise a longer line could nudge the grid off its 2x2 shape.
        // That intent only holds if `shrinkable` also strips the box's own
        // content-based minimum width, though -- otherwise an unbreakable
        // long line still wins over the equal-split flex-basis (see
        // `render::shrinkable`).
        flex_grow: 1.0,
        flex_basis: Val::Percent(0.0),
        flex_direction: FlexDirection::Column,
        row_gap: px(4),
        ..default()
    })
}

pub(super) fn setup(mut commands: Commands, fonts: Res<Fonts>, game_data: Res<GameData>) {
    let root = commands
        .spawn((
            PlayingRoot,
            Visibility::Inherited,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: px(6).all(),
                ..default()
            },
        ))
        .id();

    // `content` claims the window's entire leftover height above the TIPS
    // legend and hands it straight to `grid` via `flex_grow` -- there is no
    // separate breathing margin anymore. Any space beyond what `capacity`
    // lines need (第3.1節) ends up growing the pane boxes themselves
    // (see `spawn_pane`), so it reads as monitor, not void.
    let content = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(10),
            ..default()
        })
        .id();

    // 隣接モニタ間はベゼル幅ぶんの `BG` だけを挟む(色付きの罫線ではなく
    // 地の色そのもの)。同じ `MONITOR_BG` を隙間なく並べると3画面が一枚の
    // 塊に見えてしまうため、物理モニタを壁掛けしたときの筐体の継ぎ目に
    // 相当する最小限の seam を残し、「枠は接するが画面は別」を保つ。
    let bezel = px(3.0);

    // 第3.1節: 上段(横いっぱい)が焼成室、下段左右が外・売り場の 2x2 グリッド。
    // 縦の配分は Kiln:(Outside/Floor) = capacity 比(8:5, φ 近似)に揃えて
    // いる(domain::pane::Pane::capacity 参照)ので、行数で決まる「主に見る
    // 画面」の重みが、画面上の面積比とも一致する。上段自体は日数インジケータ
    // (`day_indicator`)と焼成室ログの横並び——インジケータは固定幅の小さな
    // 計器で、焼成室ログとの間で幅を競う対象ではないため `flex_grow` は
    // 使わず(`day_indicator::INDICATOR_WIDTH`)、余った幅はすべて焼成室ログ
    // 側の `flex_grow: 1.0` に流れる。
    let day_indicator = spawn_day_indicator(&mut commands, game_data.day);
    let kiln = spawn_pane(
        &mut commands,
        &fonts,
        Pane::Kiln,
        game_data.phase,
        shrinkable(Node {
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            row_gap: px(4),
            ..default()
        }),
    );
    let top_row = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_grow: Pane::Kiln.capacity() as f32,
            flex_direction: FlexDirection::Row,
            column_gap: bezel,
            ..default()
        })
        .id();
    commands.entity(top_row).add_children(&[day_indicator, kiln]);

    let outside =
        spawn_pane(&mut commands, &fonts, Pane::Outside, game_data.phase, split_pane_node());
    let floor = spawn_pane(&mut commands, &fonts, Pane::Floor, game_data.phase, split_pane_node());

    let bottom_row = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_grow: Pane::Outside.capacity() as f32,
            flex_direction: FlexDirection::Row,
            column_gap: bezel,
            ..default()
        })
        .id();
    commands.entity(bottom_row).add_children(&[outside, floor]);

    let grid = commands
        .spawn(Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            flex_direction: FlexDirection::Column,
            row_gap: bezel,
            ..default()
        })
        .id();
    commands.entity(grid).add_children(&[top_row, bottom_row]);

    // 呼ばれる(第7節)専用の枠。3画面のどの `Node` 木にも属さない、独立した
    // 兄弟要素であることそのものが「不可触」を実装として保証する。
    let intrusion = commands
        .spawn((
            IntrusionSlot,
            Text::new(""),
            TextFont {
                font: fonts.for_line(LineFont::Call).into(),
                font_size: FontSize::Px(18.0),
                ..default()
            },
            TextColor(FG),
        ))
        .id();

    let legend = commands
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: px(24),
            padding: px(12).bottom(),
            ..default()
        })
        .with_children(|parent| {
            for label in ["削除 Z", "検印 X", "移動 J/K", "画面切替 H/L", "一時停止 ESC"]
            {
                parent.spawn((
                    Text::new(label),
                    TextFont {
                        font: fonts.normal().into(),
                        font_size: FontSize::Px(14.0),
                        ..default()
                    },
                    TextColor(DIM),
                ));
            }
        })
        .id();

    commands.entity(content).add_children(&[grid, intrusion]);
    commands.entity(root).add_children(&[content, legend]);
    commands.insert_resource(PauseState::default());
    commands.insert_resource(GlitchTimer::default());
    commands.insert_resource(ActivePane::default());
    commands.insert_resource(ActiveIntrusion::default());
}

pub(super) fn teardown(
    mut commands: Commands,
    playing_root: Query<Entity, With<PlayingRoot>>,
    pause_root: Query<Entity, With<PauseRoot>>,
) {
    for entity in &playing_root {
        commands.entity(entity).despawn();
    }
    for entity in &pause_root {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<PauseState>();
    commands.remove_resource::<GlitchTimer>();
    commands.remove_resource::<ActivePane>();
    commands.remove_resource::<ActiveIntrusion>();
}
