use std::marker::PhantomData;
use std::str::FromStr;

use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use clap::{ArgAction, Parser};
use derive_more::derive::Display;

use super::character::player::Player;
use super::prototype::{PrototypeId, PrototypeInstance, PrototypeRegistry};

pub struct DebugConsolePlugin<CharacterId: PrototypeId, ItemId: PrototypeId> {
    _character_id: PhantomData<CharacterId>,
    _item_id: PhantomData<ItemId>,
}

impl<CharacterId: PrototypeId, ItemId: PrototypeId> Default
    for DebugConsolePlugin<CharacterId, ItemId>
{
    fn default() -> Self {
        Self {
            _character_id: default(),
            _item_id: default(),
        }
    }
}

impl<CharacterId: PrototypeId, ItemId: PrototypeId> Plugin
    for DebugConsolePlugin<CharacterId, ItemId>
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ConsolePlugin);
        app.insert_resource(ConsoleConfiguration::default());
        app.add_console_command::<ListCharactersCommand, _>(list_characters::<CharacterId>);
        app.add_console_command::<SpawnCharacterCommand, _>(spawn_character::<CharacterId>);
        app.add_console_command::<DespawnCharactersCommand, _>(despawn_characters::<CharacterId>);
        app.add_console_command::<ListItemsCommand, _>(list_items::<ItemId>);
        app.add_console_command::<SpawnItemCommand, _>(spawn_item::<ItemId>);
        app.add_console_command::<DespawnItemsCommand, _>(despawn_items::<ItemId>);
    }
}

#[derive(Clone, Default, Display)]
enum Position {
    Custom(Vec3),
    #[default]
    Player,
}

impl FromStr for Position {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        const ERROR_SUFFIX: &str = "Supported values are (@p), (x,z), (x,y,z).";

        let input = input.trim().trim_matches(['(', ')']).to_lowercase();

        if input == "@p" {
            return Ok(Self::Player);
        }

        let mut split = input.split_terminator(',');
        let Some(x) = split.next().and_then(|x| f32::from_str(x).ok()) else {
            return Err(format!("Cannot parse X coordinate. {}", ERROR_SUFFIX));
        };

        let Some(y) = split.next().and_then(|x| f32::from_str(x).ok()) else {
            return Err(format!("Cannot parse Y/Z coordinate. {}", ERROR_SUFFIX));
        };

        match split.next() {
            Some(z) => match f32::from_str(z) {
                Ok(z) => Ok(Self::Custom(Vec3::new(x, y, z))),
                Err(_) => Err(format!("Cannot parse Z coordinate. {}", ERROR_SUFFIX)),
            },
            None => Ok(Self::Custom(Vec3::new(x, 0.0, y))),
        }
    }
}

macro_rules! generate_registry_commands {
    (
        $spawn_name:literal, $spawn_about:literal, $spawn_command:ident, $spawn_system:ident,
        $despawn_name:literal, $despawn_about:literal, $despawn_command:ident, $despawn_system:ident,
        $list_name:literal, $list_about:literal, $list_command:ident, $list_system:ident
    ) => {
        #[derive(Parser, ConsoleCommand)]
        #[command(name = $list_name, about = $list_about)]
        struct $list_command {
            #[arg(default_value = None)]
            id: Option<String>,
        }

        #[derive(Parser, ConsoleCommand)]
        #[command(name = $spawn_name, about = $spawn_about)]
        struct $spawn_command {
            id: String,
            #[arg(value_parser = clap::value_parser!(Position))]
            position: Position,
        }

        #[derive(Parser, ConsoleCommand)]
        #[command(name = $despawn_name, about = $despawn_about)]
        struct $despawn_command {
            id: String,
            #[arg(action = ArgAction::Set, default_value_t = false)]
            all: bool,
        }

        fn $list_system<T: PrototypeId>(
            mut command: ConsoleCommand<$list_command>,
            query: Populated<(Entity, Option<&Name>, &PrototypeInstance<T>)>,
        ) {
            let Some(Ok($list_command { id })) = command.take() else {
                return;
            };

            match id {
                None => {
                    for (entity, name, _) in query.iter() {
                        command.reply(format!(
                            "{} - {}",
                            entity,
                            name.map(|n| n.as_str()).unwrap_or("<None>")
                        ));
                    }
                }
                Some(id) => {
                    let Ok(id) = T::from_str(&id) else {
                        command.reply(format!("Cannot parse id '{}'", id));
                        return;
                    };

                    for (entity, name, prototype) in query.iter() {
                        if prototype.id() == id {
                            command.reply(format!(
                                "{} - {}",
                                entity,
                                name.map(|n| n.as_str()).unwrap_or("<None>")
                            ));
                        }
                    }
                }
            }
        }

        fn $spawn_system<T: PrototypeId>(
            mut command: ConsoleCommand<$spawn_command>,
            mut commands: Commands,
            registry: Res<PrototypeRegistry<T>>,
            player: Single<&GlobalTransform, With<Player>>,
        ) {
            let Some(Ok($spawn_command { id, position })) = command.take() else {
                return;
            };

            let Ok(id) = T::from_str(&id) else {
                command.reply(format!("Cannot parse id '{}'", id));
                return;
            };

            let transform = Transform::from_translation(match position {
                Position::Custom(vec) => vec,
                Position::Player => player.translation(),
            });

            registry.spawn_at(id, transform, &mut commands);
            command.reply(format!(
                "{} has been successfully spawned at {}",
                id, transform.translation
            ));
        }

        fn $despawn_system<T: PrototypeId>(
            mut command: ConsoleCommand<$despawn_command>,
            mut commands: Commands,
            query: Populated<(Entity, &PrototypeInstance<T>)>,
        ) {
            let Some(Ok($despawn_command { id, all })) = command.take() else {
                return;
            };

            let Ok(id) = T::from_str(&id) else {
                command.reply(format!("Cannot parse id '{}'", id));
                return;
            };

            for (entity, prototype) in query.iter() {
                if prototype.id() == id {
                    commands.entity(entity).despawn_recursive();
                    command.reply(format!("{} has been successfully despawned", entity));

                    if !all {
                        return;
                    }
                }
            }
        }
    };
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "despawn-entity", about = "Despawns entity from a world")]
struct DespawnEntityCommand {
    id: String,
}

generate_registry_commands!(
    "spawn-character",
    "Spawns new instance of a character in world",
    SpawnCharacterCommand,
    spawn_character,
    "despawn-characters",
    "Despawns instances of a character from a world",
    DespawnCharactersCommand,
    despawn_characters,
    "list-characters",
    "Lists instances of a character in a world",
    ListCharactersCommand,
    list_characters
);

generate_registry_commands!(
    "spawn-item",
    "Spawns new instance of an item in world",
    SpawnItemCommand,
    spawn_item,
    "despawn-items",
    "Despawns instances of an item from a world",
    DespawnItemsCommand,
    despawn_items,
    "list-items",
    "Lists instances of an item in a world",
    ListItemsCommand,
    list_items
);
