#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GmTalkByMuipReq {
    #[prost(uint32, tag = "1")]
    pub uuid: u32,
    #[prost(uint32, tag = "2")]
    pub player_uid: u32,
    #[prost(uint32, tag = "3")]
    pub token_level: u32,
    #[prost(string, tag = "4")]
    pub verify_code: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub msg: ::prost::alloc::string::String,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GmTalkByMuipRsp {
    #[prost(uint32, tag = "1")]
    pub uuid: u32,
    #[prost(int32, tag = "2")]
    pub retcode: i32,
    #[prost(string, tag = "3")]
    pub msg: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub param_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
