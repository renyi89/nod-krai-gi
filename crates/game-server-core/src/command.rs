use crossbeam_queue::SegQueue;
use nod_krai_gi_message::output::ClientOutput;
use nod_krai_gi_proto::packet_head::PacketHead;
use std::sync::LazyLock;
use nod_krai_gi_proto::server_only::PlayerDataBin;

pub static LOGIC_COMMAND_QUEUE: LazyLock<SegQueue<(u32, LogicCommand)>> =
    LazyLock::new(|| SegQueue::new());

pub enum LogicCommand {
    CreateWorld {
        player_information: PlayerDataBin,
        output: ClientOutput,
    },
    ClientInput {
        head: PacketHead,
        cmd_id: u16,
        data: Box<[u8]>,
        immediate_mode: bool,
    },
    WorldUpdate(),
    Offline(),
}

impl LogicCommand {
    pub fn push(self, uid: u32) {
        LOGIC_COMMAND_QUEUE.push((uid, self));
    }
}
