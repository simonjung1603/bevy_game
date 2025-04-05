use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use super::{assets::FontAssets, TEXT_COLOR};
use crate::GameState;

const FONT_SIZE: f32 = 33.0;
const DYNAMIC_VALUE_COLOR: Color = Color::srgb(0.5, 1.0, 0.5);
const STATIC_VALUE_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

pub fn plugin(app: &mut App) {
    app.insert_resource(Health {
        current: 100,
        max: 100,
    })
    .insert_resource(Experience {
        current: 0,
        target: 10,
    })
    .add_systems(OnEnter(GameState::Game), spawn_overlay)
    .add_systems(
        Update,
        update_ui::<Health, HealthUi>
            .run_if(in_state(GameState::Game).and(resource_changed::<Health>)),
    )
    .add_systems(
        Update,
        update_ui::<Experience, XpUi>
            .run_if(in_state(GameState::Game).and(resource_changed::<Experience>)),
    )
    .add_systems(
        FixedUpdate,
        update_xp.run_if(in_state(GameState::Game).and(on_timer(Duration::from_secs(1)))),
    );
}

fn update_xp(mut xp: ResMut<Experience>) {
    xp.current += 1;
}

fn update_ui<R: UiWritable + Resource, C: Component>(
    mut writer: TextUiWriter,
    res: Res<R>,
    ui: Single<Entity, (With<C>, With<Text>)>,
) {
    res.ui_updates()
        .into_iter()
        .for_each(|update| *writer.text(*ui, update.0) = update.1);
}

struct UiUpdate(usize, String);
trait UiWritable {
    fn ui_updates(&self) -> Vec<UiUpdate>;
}

impl UiWritable for Health {
    fn ui_updates(&self) -> Vec<UiUpdate> {
        vec![
            UiUpdate(1, format!("{:3}", self.current)),
            UiUpdate(3, self.max.to_string()),
        ]
    }
}

impl UiWritable for Experience {
    fn ui_updates(&self) -> Vec<UiUpdate> {
        vec![
            UiUpdate(1, format!("{:3}", self.current)),
            UiUpdate(3, self.target.to_string()),
        ]
    }
}

#[derive(Resource)]
struct Health {
    current: usize,
    max: usize,
}

#[derive(Resource)]
struct Experience {
    current: usize,
    target: usize,
}

#[derive(Component)]
struct TimeUi;
#[derive(Component)]
struct HealthUi;
#[derive(Component)]
struct XpUi;

fn spawn_overlay(mut commands: Commands, fonts: Res<FontAssets>) {
    let font = TextFont {
        font: fonts.normal.clone(),
        font_size: FONT_SIZE,
        ..default()
    };

    commands
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    HealthUi,
                    Text("HP:  ".to_string()),
                    font.clone(),
                    TextColor(TEXT_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextSpan("100".to_string()),
                        font.clone(),
                        TextColor(DYNAMIC_VALUE_COLOR),
                    ));
                    parent.spawn((TextSpan(" / ".to_string()), font.clone()));
                    parent.spawn((
                        TextSpan("100".to_string()),
                        font.clone(),
                        TextColor(STATIC_VALUE_COLOR),
                    ));
                });
            parent
                .spawn((
                    XpUi,
                    Text("XP:  ".to_string()),
                    font.clone(),
                    TextColor(TEXT_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextSpan("  0".to_string()),
                        font.clone(),
                        TextColor(DYNAMIC_VALUE_COLOR),
                    ));
                    parent.spawn((TextSpan(" / ".to_string()), font.clone()));
                    parent.spawn((
                        TextSpan("10".to_string()),
                        font.clone(),
                        TextColor(STATIC_VALUE_COLOR),
                    ));
                });
            parent
                .spawn((
                    TimeUi,
                    Text("Time: ".to_string()),
                    font.clone(),
                    TextColor(TEXT_COLOR),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextSpan("00".to_string()),
                        font.clone(),
                        TextColor(STATIC_VALUE_COLOR),
                    ));
                    parent.spawn((TextSpan(" : ".to_string()), font.clone()));
                    parent.spawn((
                        TextSpan("00".to_string()),
                        font.clone(),
                        TextColor(STATIC_VALUE_COLOR),
                    ));
                });
        });
}
