use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use nod_krai_gi_message::{event::ClientMessageEvent, output::MessageOutput};
use nod_krai_gi_proto::{
    query_path_rsp::PathStatusType, retcode::Retcode, QueryPathReq,
    QueryPathRsp,
};

pub struct PathfindingPlugin;

impl Plugin for PathfindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, pathfinding_packet_processor);
    }
}

fn pathfinding_packet_processor(
    mut events: MessageReader<ClientMessageEvent>,
    message_output: Res<MessageOutput>,
) {
    for message in events.read() {
        match message.message_name() {
            "PathfindingEnterSceneReq" => {
                message_output.send_none(message.sender_uid(), "PathfindingEnterSceneRsp")
                // if let Some(request) = message.decode::<PathfindingEnterSceneReq>() {
                //     tracing::debug!("PathfindingEnterScene: {request:?}");
                //     message_output.send_none(message.sender_uid(), "PathfindingEnterSceneRsp")
                // }
            }
            "ToTheMoonEnterSceneReq" => {
                message_output.send_none(message.sender_uid(), "ToTheMoonEnterSceneRsp")
                // if let Some(request) = message.decode::<ToTheMoonEnterSceneReq>() {
                //     tracing::debug!("PathfindingEnterScene: {request:?}");
                //     message_output.send_none(message.sender_uid(), "ToTheMoonEnterSceneRsp")
                // }
            }
            "QueryPathReq" => {
                if let Some(request) = message.decode::<QueryPathReq>() {
                    // tracing::debug!("QueryPath: {request:?}");

                    let mut corners = Vec::with_capacity(2);

                    if let Some(source_pos) = request.source_pos {
                        corners.push(source_pos);
                    }

                    if let Some(destination) = request.destination_pos.first() {
                        corners.push(*destination);
                    }

                    message_output.send(
                        message.sender_uid(),
                        "QueryPathRsp",
                        QueryPathRsp {
                            retcode: Retcode::RetSucc.into(),
                            query_status: PathStatusType::StatusSucc.into(),
                            query_id: request.query_id,
                            corners,
                        },
                    )
                }
            }
            &_ => {}
        }
    }
}
