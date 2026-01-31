use nod_krai_gi_message::output::ClientOutput;
use nod_krai_gi_persistence::player_information::PlayerInformation;
use nod_krai_gi_proto::packet_head::PacketHead;

pub enum LogicCommand {
    CreateWorld {
        player_information: PlayerInformation,
        output: ClientOutput,
    },
    ClientInput {
        head: PacketHead,
        cmd_id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    },
    WorldUpdate(u32),
}
