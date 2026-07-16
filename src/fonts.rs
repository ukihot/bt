use bevy::prelude::*;

use crate::domain::LineFont;

const NORMAL_PATH: &str = "fonts/Noto_Sans_JP/static/NotoSansJP-Regular.ttf";
/// 二人称の障り専用。
const MISTAKE_PATH: &str = "fonts/Potta_One/PottaOne-Regular.ttf";
/// 呼ばれる / Lost 画面の侵食ログ専用。
const CALL_PATH: &str = "fonts/Yuji_Syuku/YujiSyuku-Regular.ttf";

#[derive(Resource)]
pub struct Fonts {
    normal: Handle<Font>,
    mistake: Handle<Font>,
    call: Handle<Font>,
}

impl Fonts {
    pub fn load(asset_server: &AssetServer) -> Self {
        Self {
            normal: asset_server.load(NORMAL_PATH),
            mistake: asset_server.load(MISTAKE_PATH),
            call: asset_server.load(CALL_PATH),
        }
    }

    pub fn normal(&self) -> Handle<Font> {
        self.normal.clone()
    }

    pub fn for_line(&self, font: LineFont) -> Handle<Font> {
        match font {
            LineFont::Normal => self.normal.clone(),
            LineFont::Mistake => self.mistake.clone(),
            LineFont::Call => self.call.clone(),
        }
    }
}
