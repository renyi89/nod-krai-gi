use crate::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use nod_krai_gi_encryption::rsa::RsaKeyPair;
use serde::Deserialize;
use serde_json::json;

pub fn routes() -> Router<&'static AppState> {
    Router::new()
        .route("/query_region_list", get(query_region_list))
        .route("/query_cur_region", get(query_cur_region))
}

#[derive(Deserialize, Debug, Default)]
struct QueryRegionListParam {
    pub version: String,
    #[expect(dead_code)]
    pub lang: u32,
    #[expect(dead_code)]
    pub platform: u32,
    #[expect(dead_code)]
    pub binary: u8,
    #[expect(dead_code)]
    pub time: u64,
    #[expect(dead_code)]
    pub channel_id: u16,
    #[expect(dead_code)]
    pub sub_channel_id: u16,
}

#[derive(Deserialize, Debug, Default)]
struct QueryCurrRegionParam {
    pub version: String,
    #[expect(dead_code)]
    pub lang: u32,
    #[expect(dead_code)]
    pub platform: u32,
    #[expect(dead_code)]
    pub binary: u8,
    #[expect(dead_code)]
    pub time: u64,
    pub channel_id: u16,
    pub sub_channel_id: u16,
    #[serde(rename = "dispatchSeed")]
    pub dispatch_seed: String,
    pub key_id: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrRegionHttpRsp {
    #[prost(bytes = "vec", tag = "13")]
    pub client_region_custom_config_encrypted: Vec<u8>,
    #[prost(string, tag = "2")]
    pub msg: String,
    #[prost(bytes = "vec", tag = "12")]
    pub region_custom_config_encrypted: Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub region_info: Option<RegionInfo>,
    #[prost(bytes = "vec", tag = "11")]
    pub client_secret_key: Vec<u8>,
    #[prost(int32, tag = "1")]
    pub retcode: i32,
    #[prost(oneof = "query_curr_region_http_rsp::Detail", tags = "4, 5")]
    pub detail: Option<query_curr_region_http_rsp::Detail>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionInfo {
    #[prost(string, tag = "30")]
    pub user_center_url: String,
    #[prost(string, tag = "27")]
    pub client_silence_version_suffix: String,
    #[prost(string, tag = "8")]
    pub resource_url: String,
    #[prost(string, tag = "26")]
    pub client_version_suffix: String,
    #[prost(string, tag = "36")]
    pub game_biz: String,
    #[prost(string, tag = "34")]
    pub next_resource_url: String,
    #[prost(string, tag = "11")]
    pub bulletin_url: String,
    #[prost(string, tag = "12")]
    pub resource_url_bak: String,
    #[prost(string, tag = "29")]
    pub gateserver_domain_name: String,
    #[prost(string, tag = "19")]
    pub client_data_md5: String,
    #[prost(string, tag = "9")]
    pub data_url: String,
    #[prost(string, tag = "13")]
    pub data_url_bak: String,
    #[prost(string, tag = "10")]
    pub feedback_url: String,
    #[prost(message, optional, tag = "22")]
    pub res_version_config: Option<ResVersionConfig>,
    #[prost(string, tag = "20")]
    pub client_silence_data_md5: String,
    #[prost(string, tag = "3")]
    pub pay_callback_url: String,
    #[prost(string, tag = "24")]
    pub official_community_url: String,
    #[prost(bytes = "vec", tag = "23")]
    pub secret_key: Vec<u8>,
    #[prost(string, tag = "31")]
    pub account_bind_url: String,
    #[prost(string, tag = "1")]
    pub gateserver_ip: String,
    #[prost(string, tag = "33")]
    pub privacy_policy_url: String,
    #[prost(string, tag = "32")]
    pub cdkey_url: String,
    #[prost(string, tag = "7")]
    pub area_type: String,
    #[prost(message, optional, tag = "35")]
    pub next_res_version_config: Option<ResVersionConfig>,
    #[prost(string, tag = "16")]
    pub handbook_url: String,
    #[prost(uint32, tag = "14")]
    pub client_data_version: u32,
    #[prost(uint32, tag = "18")]
    pub client_silence_data_version: u32,
    #[prost(uint32, tag = "2")]
    pub gateserver_port: u32,
    #[prost(bool, tag = "28")]
    pub use_gateserver_domain_name: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResVersionConfig {
    #[prost(string, tag = "4")]
    pub release_total_size: String,
    #[prost(string, tag = "5")]
    pub version_suffix: String,
    #[prost(string, tag = "7")]
    pub next_script_version: String,
    #[prost(string, tag = "6")]
    pub branch: String,
    #[prost(string, tag = "3")]
    pub md5: String,
    #[prost(uint32, tag = "1")]
    pub version: u32,
    #[prost(bool, tag = "2")]
    pub relogin: bool,
}

/// Nested message and enum types in `QueryCurrRegionHttpRsp`.
pub mod query_curr_region_http_rsp {
    #[derive(Clone, PartialEq, prost::Oneof)]
    pub enum Detail {
        #[prost(message, tag = "4")]
        ForceUpdate(super::ForceUpdateInfo),
        #[prost(message, tag = "5")]
        StopServer(super::StopServerInfo),
    }
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForceUpdateInfo {
    #[prost(string, tag = "1")]
    pub force_update_url: String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopServerInfo {
    #[prost(string, tag = "4")]
    pub content_msg: String,
    #[prost(string, tag = "3")]
    pub url: String,
    #[prost(uint32, tag = "1")]
    pub stop_begin_time: u32,
    #[prost(uint32, tag = "2")]
    pub stop_end_time: u32,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRegionListHttpRsp {
    #[prost(message, repeated, tag = "2")]
    pub region_list: Vec<RegionSimpleInfo>,
    #[prost(bytes = "vec", tag = "5")]
    pub client_secret_key: Vec<u8>,
    #[prost(bytes = "vec", tag = "6")]
    pub client_custom_config_encrypted: Vec<u8>,
    #[prost(int32, tag = "1")]
    pub retcode: i32,
    #[prost(bool, tag = "7")]
    pub enable_login_pc: bool,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionSimpleInfo {
    #[prost(string, tag = "2")]
    pub title: String,
    #[prost(string, tag = "3")]
    pub r#type: String,
    #[prost(string, tag = "4")]
    pub dispatch_url: String,
    #[prost(string, tag = "1")]
    pub name: String,
}

enum ProtobufData<T> {
    Plain(T),
    Encrypted(T, &'static RsaKeyPair),
}

impl<T> IntoResponse for ProtobufData<T>
where
    T: prost::Message,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Plain(proto) => (
                StatusCode::OK,
                base64_simd::STANDARD.encode_to_string(&proto.encode_to_vec()),
            )
                .into_response(),
            Self::Encrypted(proto, keys) => {
                let plain = proto.encode_to_vec();
                let content = keys.client_encrypt(&plain);
                let sign = keys.sign(&plain);

                (
                    StatusCode::OK,
                    json! ({
                        "content": base64_simd::STANDARD.encode_to_string(&content),
                        "sign": base64_simd::STANDARD.encode_to_string(&sign),
                    })
                    .to_string(),
                )
                    .into_response()
            }
        }
    }
}

async fn query_cur_region(
    Query(param): Query<QueryCurrRegionParam>,
    State(state): State<&'static AppState>,
) -> ProtobufData<QueryCurrRegionHttpRsp> {
    tracing::debug!("query_cur_region: {param:?}");

    let Some(cur_region_name) = state.config.region.cur_region_name.as_ref() else {
        tracing::debug!("query_cur_region requested, but this dispatch doesn't have bound region");
        return ProtobufData::Plain(QueryCurrRegionHttpRsp {
            retcode: 1,
            msg: "Not Found version config".to_string(),
            ..Default::default()
        });
    };

    let region_config = state
        .region_list
        .iter()
        .find(|r| &r.name == cur_region_name)
        .unwrap();

    if !region_config.allowed_key_id_list.contains(&param.key_id) {
        tracing::debug!(
            "query_cur_region: region {cur_region_name} doesn't allow key_id {}",
            param.key_id
        );
        return ProtobufData::Plain(QueryCurrRegionHttpRsp {
            retcode: 1,
            msg: "Not Found key_id config".to_string(),
            ..Default::default()
        });
    }

    let keys = state.key_pair_map.get(&param.key_id).unwrap();

    if !region_config.bind_version_list.contains(&param.version) {
        tracing::debug!(
            "Unsupported version (v={}, c={}, s={})",
            &param.version,
            param.channel_id,
            param.sub_channel_id
        );
        return ProtobufData::Encrypted(
            QueryCurrRegionHttpRsp {
                retcode: -1,
                msg: "Not Found version config".to_string(),
                ..Default::default()
            },
            keys,
        );
    }

    tracing::debug!("dispatch seed, client: {}", param.dispatch_seed);

    let mut region_info = RegionInfo {
        gateserver_ip: region_config.gateserver_ip.clone(),
        gateserver_port: region_config.gateserver_port as u32,
        ..Default::default()
    };

    match region_config.hot_fix_data.get(&param.version) {
        None => {}
        Some(hot_fix_data) => {
            region_info.resource_url = hot_fix_data.resource_url.clone();
            region_info.data_url = hot_fix_data.data_url.clone();
            region_info.client_data_md5 = hot_fix_data.client_data_md5.clone();
            region_info.client_silence_data_md5 = hot_fix_data.client_silence_data_md5.clone();
            region_info.client_data_version = hot_fix_data.client_data_version.clone();
            region_info.client_silence_data_version =
                hot_fix_data.client_silence_data_version.clone();
            region_info.client_version_suffix = hot_fix_data.client_version_suffix.clone();
            region_info.client_silence_version_suffix =
                hot_fix_data.client_silence_version_suffix.clone();

            region_info.res_version_config = Some(ResVersionConfig {
                version: hot_fix_data.res_version_config.version.clone(),
                md5: hot_fix_data.res_version_config.md5.clone(),
                release_total_size: hot_fix_data.res_version_config.release_total_size.clone(),
                version_suffix: hot_fix_data.res_version_config.version_suffix.clone(),
                branch: hot_fix_data.res_version_config.branch.clone(),
                ..Default::default()
            });
        }
    }

    ProtobufData::Encrypted(
        QueryCurrRegionHttpRsp {
            retcode: 0,
            client_secret_key: state
                .cur_region_secret_key_ec2b
                .as_ref()
                .map(|k| k.to_vec())
                .unwrap_or_default(),
            region_info: Some(region_info),
            ..Default::default()
        },
        keys,
    )
}

async fn query_region_list(
    Query(param): Query<QueryRegionListParam>,
    State(state): State<&'static AppState>,
) -> ProtobufData<QueryRegionListHttpRsp> {
    tracing::debug!("query_region_list: {param:?}");

    if state.config.forbid_first_dispatch {
        tracing::debug!("query_region_list is forbidden in this dispatch");

        ProtobufData::Plain(QueryRegionListHttpRsp {
            retcode: 8,
            ..Default::default()
        })
    } else {
        ProtobufData::Plain(QueryRegionListHttpRsp {
            enable_login_pc: state.config.region.enable_login_pc,
            client_secret_key: state.global_secret_key_ec2b.to_vec(),
            client_custom_config_encrypted: state.client_custom_config_encrypted.to_vec(),
            region_list: state
                .region_list
                .iter()
                .filter(|r| r.bind_version_list.contains(&param.version))
                .map(|r| RegionSimpleInfo {
                    name: r.name.clone(),
                    title: r.title.clone(),
                    r#type: r.r#type.clone(),
                    dispatch_url: r.dispatch_url.clone(),
                })
                .collect(),
            ..Default::default()
        })
    }
}
