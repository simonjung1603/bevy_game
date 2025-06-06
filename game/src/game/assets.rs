use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kenney_assets::KenneySpriteSheetAsset;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/sfx_laser1.ogg")]
    pub laser: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/bg_black.png")]
    pub background: Handle<Image>,
    #[asset(path = "images/sheet.xml")]
    pub main_space_sheet: Handle<KenneySpriteSheetAsset>,
    #[asset(path = "images/spaceShooter2_spritesheet.xml")]
    pub _extended_space_sheet: Handle<KenneySpriteSheetAsset>,
}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/kenvector_future_thin.ttf")]
    pub _thin: Handle<Font>,
    #[asset(path = "fonts/kenvector_future.ttf")]
    pub normal: Handle<Font>,
}

// Include generated spritesheed indices
pub mod indices {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/generated/spritesheet_asset_indices.rs"
    ));
}
