mod block_chooser;
mod jump;
mod parkour;
mod port;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::PathBuf;

use valence::network::{async_trait, HandshakeData, ServerListPing};
use valence::prelude::*;
use valence::{MINECRAFT_VERSION, PROTOCOL_VERSION};

use clap::Parser;
use parkour::{ParkourCourse, PlayerOnCourse, PlayerStateUpdate};
use valence::anvil::{AnvilLevel, ChunkLoadEvent, ChunkLoadStatus};
use valence::command::handler::CommandResultEvent;
use valence::command::scopes::CommandScopes;
use valence::command::{AddCommand, CommandScopeRegistry};
use valence::command_macros::Command;
use valence::message::SendMessage;
use valence::player_list::DisplayName;
use valence::protocol::sound::SoundCategory;
use valence::protocol::Sound;

#[derive(Parser, Resource)]
#[clap(author, version, about)]
struct Cli {
    /// The path to a Minecraft world save containing a `region` subdirectory.
    path: PathBuf,
}

#[derive(Component)]
struct GameState {
    course: ParkourCourse,
    player_states: HashMap<Uuid, PlayerOnCourse>,
}

impl GameState {
    fn update_player_state(
        &mut self,
        player_id: Uuid,
        pos: &mut Position,
        look: &mut Look,
    ) -> PlayerStateUpdate {
        let player_state = self
            .player_states
            .entry(player_id)
            .or_insert_with(|| PlayerOnCourse::new(self.course.get_start().into()));

        player_state.update_player_state(&self.course, pos, look)
    }

    fn current_platform(&self, player_id: &Uuid) -> i32 {
        self.player_states
            .get(&player_id)
            .map(|state| state.current_platform())
            .unwrap_or(0)
    }

    fn spawn_platform(&mut self, layer: &mut ChunkLayer) -> bool {
        self.course.spawn_platform(layer)
    }

    fn platforms_left(&self, player_id: &Uuid) -> i32 {
        self.player_states
            .get(player_id)
            .map(|state| state.platforms_left(&self.course))
            .unwrap_or(0)
    }

    fn done(&self) -> bool {
        self.course.done()
    }

    fn reset_player(&mut self, player_id: Uuid, pos: &mut Position) {
        let spawn = self.course.get_start();

        self.player_states
            .insert(player_id, PlayerOnCourse::new(spawn.into()));

        pos.set(spawn);
    }

    fn to_last_checkpoint(&mut self, player_id: Uuid, pos: &mut Position, look: &mut Look) {
        self.player_states
            .get_mut(&player_id)
            .map(|state| state.to_last_checkpoint(pos, look));
    }

    fn pause(&mut self, player_id: Uuid) {
        self.player_states
            .get_mut(&player_id)
            .map(|state| state.pause());
    }

    fn resume(&mut self, player_id: Uuid) {
        self.player_states
            .get_mut(&player_id)
            .map(|state| state.resume());
    }
}

#[derive(Command, Debug, Clone)]
#[paths("help", "h")]
#[scopes("parkour.command.help")]
struct Help;

#[derive(Command, Debug, Clone)]
#[paths("info", "i")]
#[scopes("parkour.command.info")]
struct Info;

#[derive(Command, Debug, Clone)]
#[paths("reset")]
#[scopes("parkour.command.reset")]
struct Reset;

#[derive(Command, Debug, Clone)]
#[paths("kill")]
#[scopes("parkour.command.kill")]
struct Kill;

#[derive(Command, Debug, Clone)]
#[paths("pause")]
#[scopes("parkour.command.pause")]
struct Pause;

#[derive(Command, Debug, Clone)]
#[paths("resume")]
#[scopes("parkour.command.resume")]
struct Resume;

#[derive(Command, Debug, Clone)]
#[paths("gamemode", "gm")]
#[scopes("parkour.command.gamemode")]
enum GamemodeCommand {
    #[paths("creative", "{/} gmc")]
    Creative,
    #[paths("adventure", "{/} gma")]
    Adventure,
    #[paths("spectator", "{/} gms")]
    Spectator,
}

fn handle_help_command(
    mut events: EventReader<CommandResultEvent<Help>>,
    mut clients: Query<(Entity, &mut Client)>,
) {
    for event in events.read() {
        let author = &mut clients.get_mut(event.executor).unwrap().1;

        author.send_chat_message(
            "Welcome to the tini parkour server :3\n\n"
                .color(Color::YELLOW)
                .bold()
                + "- ".color(Color::WHITE).not_bold()
                + "/help"
                    .on_click_run_command("/help")
                    .on_hover_show_text("Click to run.")
                    .color(Color::AQUA)
                    .bold()
                + " - see this message again.\n- "
                    .color(Color::WHITE)
                    .not_bold()
                + "/info"
                    .on_click_run_command("/info")
                    .on_hover_show_text("Click to run.")
                    .color(Color::AQUA)
                    .bold()
                + " - get the course status, i.e. the number of platforms left.\n- "
                    .color(Color::WHITE)
                    .not_bold()
                + "/reset"
                    .on_click_run_command("/reset")
                    .on_hover_show_text("Click to run.")
                    .color(Color::AQUA)
                    .bold()
                + " - reset your position to the start of the course.\n- "
                    .color(Color::WHITE)
                    .not_bold()
                + "/kill"
                    .on_click_run_command("/kill")
                    .on_hover_show_text("Click to run.")
                    .color(Color::AQUA)
                    .bold()
                + "- reset your position to the last checkpoint.\n- "
                    .color(Color::WHITE)
                    .not_bold()
                + "/pause"
                    .on_click_run_command("/pause")
                    .on_hover_show_text("Click to run.")
                    .color(Color::AQUA)
                    .bold()
                + "- pause the game, letting you explore the world.\n- "
                    .color(Color::WHITE)
                    .not_bold()
                + "/resume"
                    .on_click_run_command("/resume")
                    .on_hover_show_text("Click to run.")
                    .color(Color::AQUA)
                    .bold()
                + "- resume the game.\n- ".color(Color::WHITE).not_bold()
                + "/gamemode"
                    .on_click_run_command("/gamemode")
                    .on_hover_show_text("Click to run.")
                    .color(Color::AQUA)
                    .bold()
                + " - change your gamemode. Automatically pauses the game."
                    .color(Color::WHITE)
                    .not_bold(),
        );
    }
}

fn handle_info_command(
    mut events: EventReader<CommandResultEvent<Info>>,
    mut clients: Query<(Entity, &mut Client)>,
    mut players: Query<(Entity, &mut UniqueId)>,
    state: Query<&GameState>,
) {
    let state = state.single();

    for event in events.read() {
        let author = &players.get_mut(event.executor).unwrap().1;
        let client = &mut clients.get_mut(event.executor).unwrap().1;

        client.send_chat_message(
            "Course status:\n\n".color(Color::YELLOW).bold()
                + "Platforms generated ahead: ".color(Color::WHITE).not_bold()
                + (state.platforms_left(&author.0) - 1)
                    .color(Color::LIGHT_PURPLE)
                    .bold()
                + "\nCurrent platform: ".color(Color::WHITE).not_bold()
                + state
                    .current_platform(&author.0)
                    .color(Color::LIGHT_PURPLE)
                    .bold()
                + "\nTotal platform count: ".color(Color::WHITE).not_bold()
                + (state.course.len() - 1).color(Color::LIGHT_PURPLE).bold()
                + if state.done() {
                    "\n\nThe course has finished generating."
                        .color(Color::GREEN)
                        .not_bold()
                } else {
                    "\n\nThe course is still generating. There will be more platforms."
                        .color(Color::YELLOW)
                        .not_bold()
                },
        );
    }
}

fn handle_reset_command(
    mut events: EventReader<CommandResultEvent<Reset>>,
    mut clients: Query<(Entity, &mut Position, &UniqueId)>,
    mut state: Query<&mut GameState>,
) {
    let mut state = state.single_mut();

    for event in events.read() {
        let player_id = clients.get(event.executor).unwrap().2 .0;
        let mut author_position = clients.get_mut(event.executor).unwrap().1;

        state.reset_player(player_id, author_position.as_mut());
    }
}

fn handle_kill_command(
    mut events: EventReader<CommandResultEvent<Kill>>,
    mut clients: Query<(Entity, &mut Position, &mut Look, &UniqueId)>,
    mut state: Query<&mut GameState>,
) {
    let mut state = state.single_mut();

    for event in events.read() {
        let player_id = clients.get(event.executor).unwrap().3 .0;
        let author = clients.get_mut(event.executor).unwrap();
        let mut author_position = author.1;
        let mut author_look = author.2;

        state.to_last_checkpoint(player_id, author_position.as_mut(), author_look.as_mut());
    }
}

fn handle_pause_command(
    mut events: EventReader<CommandResultEvent<Pause>>,
    clients: Query<&UniqueId>,
    mut state: Query<&mut GameState>,
) {
    let mut state = state.single_mut();

    for event in events.read() {
        let player_id = clients.get(event.executor).unwrap();

        state.pause(player_id.0);
    }
}

fn handle_resume_command(
    mut events: EventReader<CommandResultEvent<Resume>>,
    mut clients: Query<(&mut GameMode, &UniqueId, &mut Position, &mut Look, Entity)>,
    mut state: Query<&mut GameState>,
) {
    let mut state = state.single_mut();

    for event in events.read() {
        let (mut gamemode, player_id, mut pos, mut look, ..) =
            clients.get_mut(event.executor).unwrap();

        *gamemode = GameMode::Adventure;

        state.resume(player_id.0);
        state.to_last_checkpoint(player_id.0, pos.as_mut(), look.as_mut());
    }
}

fn handle_gamemode_command(
    mut events: EventReader<CommandResultEvent<GamemodeCommand>>,
    mut clients: Query<(&mut GameMode, &UniqueId, Entity)>,
    mut state: Query<&mut GameState>,
) {
    let mut state = state.single_mut();

    for event in events.read() {
        let (mut gamemode, player_id, ..) = clients.get_mut(event.executor).unwrap();

        let game_mode_to_set = match &event.result {
            GamemodeCommand::Creative => GameMode::Creative,
            GamemodeCommand::Adventure => GameMode::Adventure,
            GamemodeCommand::Spectator => GameMode::Spectator,
        };

        match &event.result {
            GamemodeCommand::Adventure => (),
            _ => state.pause(player_id.0),
        }

        *gamemode = game_mode_to_set;
    }
}

pub fn main() {
    let cli = Cli::parse();

    if !cli.path.exists() {
        eprintln!(
            "Directory `{}` does not exist. Exiting.",
            cli.path.display()
        );
        return;
    }

    if !cli.path.is_dir() {
        eprintln!("`{}` is not a directory. Exiting.", cli.path.display());
        return;
    }

    App::new()
        .insert_resource(NetworkSettings {
            callbacks: ServerListPingCallback.into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_command::<Help>()
        .add_command::<Info>()
        .add_command::<Reset>()
        .add_command::<Kill>()
        .add_command::<Pause>()
        .add_command::<Resume>()
        .add_command::<GamemodeCommand>()
        .insert_resource(cli)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                despawn_disconnected_clients,
                override_display_name_to_platform_no,
                manage_course,
                (init_clients, handle_chunk_loads).chain(),
                handle_help_command,
                handle_info_command,
                handle_reset_command,
                handle_kill_command,
                handle_pause_command,
                handle_resume_command,
                handle_gamemode_command,
            ),
        )
        .run();
}

struct ServerListPingCallback;

#[async_trait]
impl NetworkCallbacks for ServerListPingCallback {
    async fn server_list_ping(
        &self,
        _shared: &SharedNetworkState,
        _remote_addr: SocketAddr,
        _handshake_data: &HandshakeData,
    ) -> ServerListPing {
        ServerListPing::Respond {
            // TODO: Implement a real player count
            online_players: 0,
            max_players: 1,
            player_sample: vec![],
            description: "A tini parkour server".into_text(),
            favicon_png: include_bytes!("logo.png"),
            version_name: ("Valence ".color(Color::GOLD) + MINECRAFT_VERSION.color(Color::RED))
                .to_legacy_lossy(),
            protocol: PROTOCOL_VERSION,
        }
    }
}

fn setup(
    mut commands: Commands,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
    server: Res<Server>,
    mut command_scopes: ResMut<CommandScopeRegistry>,
    cli: Res<Cli>,
) {
    let layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);
    let mut level = AnvilLevel::new(&cli.path, &biomes);

    for z in -16..16 {
        for x in -16..16 {
            let pos = ChunkPos::new(x, z);

            level.ignored_chunks.insert(pos);
            level.force_chunk_load(pos);
        }
    }

    let course = ParkourCourse::new(BlockPos::new(0, 128, 0), &layer.chunk);

    command_scopes.link("parkour.actor", "parkour.command");

    commands.spawn((
        layer,
        level,
        GameState {
            course,
            player_states: HashMap::new(),
        },
    ));
}

fn init_clients(
    mut clients: Query<
        (
            &mut UniqueId,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            &mut CommandScopes,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, With<ChunkLayer>>,
    mut course: Query<&mut GameState>,
) {
    let mut course = course.single_mut();

    for (
        player_uuid,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        mut command_scope,
    ) in &mut clients
    {
        let e_layer = layers.single();

        let spawn = course.course.get_start();

        layer_id.0 = e_layer;
        visible_chunk_layer.0 = e_layer;
        visible_entity_layers.0.insert(e_layer);
        pos.set(spawn);
        *game_mode = GameMode::Adventure;

        command_scope.add("parkour.actor");

        // Set the game state
        course
            .player_states
            .entry(player_uuid.0)
            .or_insert_with(|| PlayerOnCourse::new(spawn.into()));
    }
}

fn manage_course(
    mut clients: Query<(&mut Client, &mut Position, &mut Look, &UniqueId)>,
    mut layers: Query<&mut ChunkLayer, With<AnvilLevel>>,
    mut courses: Query<&mut GameState>,
) {
    let mut layer = layers.single_mut();
    let parkour = &mut courses.single_mut();

    for (mut client, mut pos, mut look, player_id) in &mut clients {
        // Get the player's current state
        let player_update = parkour.update_player_state(player_id.0, pos.as_mut(), look.as_mut());

        match player_update {
            PlayerStateUpdate::Paused => {
                client.send_action_bar_message(
                    "Game paused. Use ".color(Color::WHITE).not_bold()
                        + "/resume".color(Color::AQUA).bold()
                        + " to continue.".color(Color::WHITE).not_bold(),
                );
            }
            PlayerStateUpdate::Skipped => {
                client.play_sound(
                    Sound::EntityExperienceOrbPickup,
                    SoundCategory::Block,
                    pos.0,
                    1.0,
                    1.0,
                );

                client.send_action_bar_message(
                    "Platform: ".into_text()
                        + parkour.current_platform(&player_id.0).color(Color::GOLD),
                );
            }
            PlayerStateUpdate::OnCourse => {
                client.send_action_bar_message(
                    "Platform: ".into_text()
                        + parkour
                            .current_platform(&player_id.0)
                            .color(Color::LIGHT_PURPLE),
                );
            }
            PlayerStateUpdate::OnPastPlatform => {
                client.send_action_bar_message(
                    "You are currently going backwards!".color(Color::RED),
                );
            }
            _ => {}
        }

        // Spawn the platforms
        while parkour.platforms_left(&player_id.0) < 1500 {
            if !parkour.spawn_platform(&mut layer) {
                break;
            }
        }
    }
}

fn handle_chunk_loads(
    mut events: EventReader<ChunkLoadEvent>,
    mut layers: Query<&mut ChunkLayer, With<AnvilLevel>>,
    mut state: Query<&mut GameState>,
) {
    let mut layer = layers.single_mut();

    for event in events.read() {
        match &event.status {
            ChunkLoadStatus::Success { .. } => {
                // The chunk was inserted into the world. Nothing for us to do.
            }
            ChunkLoadStatus::Empty => {
                // There's no chunk here so let's insert an empty chunk. If we were doing
                // terrain generation we would prepare that here.
                layer.insert_chunk(event.pos, UnloadedChunk::new());
            }
            ChunkLoadStatus::Failed(e) => {
                // Something went wrong.
                let errmsg = format!(
                    "failed to load chunk at ({}, {}): {e:#}",
                    event.pos.x, event.pos.z
                );

                eprintln!("{errmsg}");
                // layer.send_chat_message(errmsg.color(Color::RED));

                layer.insert_chunk(event.pos, UnloadedChunk::new());
            }
        };

        for mut state in &mut state.iter_mut() {
            let course = &mut state.course;
            course.respawn_course(&mut layer);
        }
    }
}

fn override_display_name_to_platform_no(
    mut clients: Query<(&mut DisplayName, &Username, &UniqueId)>,
    state: Query<&GameState>,
) {
    let state = state.single();

    for (mut display_name, username, player_id) in &mut clients {
        display_name.0 = Some(
            username.0.clone().into_text()
                + " "
                + state
                    .current_platform(&player_id.0)
                    .color(Color::LIGHT_PURPLE),
        );
    }
}
