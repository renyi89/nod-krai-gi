use tracing_subscriber::EnvFilter;

pub const TRACE_LOG_PACKET: [&str; 12] = [
    "AbilityInvocationsNotify",
    "ClientAbilityInitFinishNotify",
    "ClientAbilitiesInitFinishCombineNotify",
    "ClientAbilityChangeNotify",
    "CombatInvocationsNotify",
    "PathfindingEnterSceneReq",
    "PathfindingEnterSceneRsp",
    "ToTheMoonEnterSceneReq",
    "ToTheMoonEnterSceneRsp",
    "QueryPathReq",
    "QueryPathRsp",
    "ObstacleModifyNotify",
];

pub fn init() {
    println!(
        r#" _      ____  ____        _  __ ____  ____  _        _____ _ 
/ \  /|/  _ \/  _ \      / |/ //  __\/  _ \/ \      /  __// \
| |\ ||| / \|| | \|_____ |   / |  \/|| / \|| |_____ | |  _| |
| | \||| \_/|| |_/|\____\|   \ |    /| |-||| |\____\| |_//| |
\_/  \|\____/\____/      \_|\_\\_/\_\\_/ \|\_/      \____\\_/
                                                             "#
    );

    let filter = EnvFilter::try_from_env("RUST_LOG").unwrap_or_else(|_| EnvFilter::new("debug"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .without_time()
        .with_target(false)
        .init();
}
