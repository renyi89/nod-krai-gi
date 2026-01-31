use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use nod_krai_gi_message::output::MessageOutput;
use nod_krai_gi_proto::WindSeedType1Notify;

pub struct LuaShellPlugin;

#[derive(Resource, Default)]
pub struct LuaShellSettings {
    pub startup_payloads: Vec<Box<[u8]>>,
}

impl Plugin for LuaShellPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LuaShellSettings::default())
            .add_message::<LuaShellEvent>()
            .add_systems(Update, send_shell_payload);
    }
}

#[derive(Message)]
pub struct LuaShellEvent();

fn send_shell_payload(
    mut lua_shell_events: MessageReader<LuaShellEvent>,
    message_output: Res<MessageOutput>,
    settings: Res<LuaShellSettings>,
) {
    for _ in lua_shell_events.read() {
        settings.startup_payloads.iter().for_each(|data| {
            message_output.send_to_all(
                "WindSeedType1Notify",
                Some(WindSeedType1Notify {
                    payload: data.to_vec(),
                    ..Default::default()
                }),
            )
        });
    }
}
