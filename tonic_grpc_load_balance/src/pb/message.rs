// This file is @generated by prost-build.
/// 原始消息
#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Msg {
    /// 发送者id
    #[prost(string, tag = "1")]
    pub send_id: ::prost::alloc::string::String,
    /// 接收者id
    #[prost(string, tag = "2")]
    pub recv_id: ::prost::alloc::string::String,
    /// 本地消息id
    #[prost(string, tag = "3")]
    pub local_id: ::prost::alloc::string::String,
    /// 服务端消息id
    #[prost(string, tag = "4")]
    pub server_id: ::prost::alloc::string::String,
    /// 消息创建时间
    #[prost(int64, tag = "5")]
    pub create_time: i64,
    /// 消息发送时间
    #[prost(int64, tag = "6")]
    pub send_time: i64,
    /// 接收方消息序列号
    #[prost(int64, tag = "7")]
    pub seq: i64,
    /// 消息类型(单聊or群聊)
    #[prost(enumeration = "MsgType", tag = "8")]
    pub msg_type: i32,
    /// 内容类型(文本or视频or图片)
    #[prost(enumeration = "ContentType", tag = "9")]
    pub content_type: i32,
    /// 根据content_type来解码
    #[prost(bytes = "vec", tag = "10")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    /// 是否已读
    #[prost(bool, tag = "11")]
    pub is_read: bool,
    /// 群id
    #[prost(string, tag = "12")]
    pub group_id: ::prost::alloc::string::String,
    /// 平台类型
    #[prost(enumeration = "PlatformType", tag = "13")]
    pub platform: i32,
    /// 用户头像
    #[prost(string, tag = "14")]
    pub avatar: ::prost::alloc::string::String,
    /// 用户昵称
    #[prost(string, tag = "15")]
    pub nickname: ::prost::alloc::string::String,
    /// 关联消息id(回复/引用功能)
    #[prost(string, optional, tag = "16")]
    pub related_msg_id: ::core::option::Option<::prost::alloc::string::String>,
    /// 发送方消息序列号(用于ACK确认)
    #[prost(int64, tag = "17")]
    pub send_seq: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendMsgRequest {
    #[prost(message, optional, tag = "1")]
    pub message: ::core::option::Option<Msg>,
}
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SendMsgResponse {}
/// 消息内容类型(文本,图片,视频,语音,文件,表情,视频通话,音频通话等)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    Default = 0,
    Text = 1,
    Image = 2,
    Video = 3,
    Audio = 4,
    File = 5,
    Emoji = 6,
    VideoCall = 7,
    AudioCall = 8,
    Error = 9,
}
impl ContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Text => "Text",
            Self::Image => "Image",
            Self::Video => "Video",
            Self::Audio => "Audio",
            Self::File => "File",
            Self::Emoji => "Emoji",
            Self::VideoCall => "VideoCall",
            Self::AudioCall => "AudioCall",
            Self::Error => "Error",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Default" => Some(Self::Default),
            "Text" => Some(Self::Text),
            "Image" => Some(Self::Image),
            "Video" => Some(Self::Video),
            "Audio" => Some(Self::Audio),
            "File" => Some(Self::File),
            "Emoji" => Some(Self::Emoji),
            "VideoCall" => Some(Self::VideoCall),
            "AudioCall" => Some(Self::AudioCall),
            "Error" => Some(Self::Error),
            _ => None,
        }
    }
}
/// 消息类型(单聊、群聊等)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MsgType {
    /// 单聊消息
    SingleMsg = 0,
    /// 群聊消息
    GroupMsg = 1,
    /// 群邀请
    GroupInvitation = 2,
    /// 群邀请新成员
    GroupInviteNew = 3,
    /// 群成员退出
    GroupMemberExit = 4,
    /// 群成员移除
    GroupRemoveMember = 5,
    /// 群解散
    GroupDismiss = 6,
    /// 群解散或成员退出接收
    GroupDismissOrExitReceived = 7,
    /// 群邀请接收
    GroupInvitationReceived = 8,
    /// 群信息更新
    GroupUpdate = 9,
    /// 好友申请
    FriendApplyReq = 10,
    /// 好友申请响应
    FriendApplyResp = 11,
    /// 好友拉黑
    FriendBlack = 12,
    /// 好友删除
    FriendDelete = 13,
    /// 单聊呼叫邀请
    SingleCallInvite = 14,
    /// 单聊呼叫拒绝
    RejectSingleCall = 15,
    /// 单聊呼叫同意
    AgreeSingleCall = 16,
    /// 单聊呼叫邀请未接
    SingleCallInviteNotAnswer = 17,
    /// 单聊呼叫邀请取消
    SingleCallInviteCancel = 18,
    /// 单聊呼叫offer 在点对点通话建立过程中，发起方发送 Offer 包含媒体配置信息
    SingleCallOffer = 19,
    /// 单聊挂断
    Hangup = 20,
    /// 单聊呼叫连接
    ConnectSingleCall = 21,
    /// 单聊呼叫候选人
    Candidate = 22,
    /// 已读回执
    Read = 23,
    /// 消息接收响应
    MsgRecResp = 24,
    /// 通知消息
    Notification = 25,
    /// 服务消息
    Service = 26,
    /// 好友关系接收
    FriendshipReceived = 27,
}
impl MsgType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::SingleMsg => "MsgTypeSingleMsg",
            Self::GroupMsg => "MsgTypeGroupMsg",
            Self::GroupInvitation => "MsgTypeGroupInvitation",
            Self::GroupInviteNew => "MsgTypeGroupInviteNew",
            Self::GroupMemberExit => "MsgTypeGroupMemberExit",
            Self::GroupRemoveMember => "MsgTypeGroupRemoveMember",
            Self::GroupDismiss => "MsgTypeGroupDismiss",
            Self::GroupDismissOrExitReceived => "MsgTypeGroupDismissOrExitReceived",
            Self::GroupInvitationReceived => "MsgTypeGroupInvitationReceived",
            Self::GroupUpdate => "MsgTypeGroupUpdate",
            Self::FriendApplyReq => "MsgTypeFriendApplyReq",
            Self::FriendApplyResp => "MsgTypeFriendApplyResp",
            Self::FriendBlack => "MsgTypeFriendBlack",
            Self::FriendDelete => "MsgTypeFriendDelete",
            Self::SingleCallInvite => "MsgTypeSingleCallInvite",
            Self::RejectSingleCall => "MsgTypeRejectSingleCall",
            Self::AgreeSingleCall => "MsgTypeAgreeSingleCall",
            Self::SingleCallInviteNotAnswer => "MsgTypeSingleCallInviteNotAnswer",
            Self::SingleCallInviteCancel => "MsgTypeSingleCallInviteCancel",
            Self::SingleCallOffer => "MsgTypeSingleCallOffer",
            Self::Hangup => "MsgTypeHangup",
            Self::ConnectSingleCall => "MsgTypeConnectSingleCall",
            Self::Candidate => "MsgTypeCandidate",
            Self::Read => "MsgTypeRead",
            Self::MsgRecResp => "MsgTypeMsgRecResp",
            Self::Notification => "MsgTypeNotification",
            Self::Service => "MsgTypeService",
            Self::FriendshipReceived => "MsgTypeFriendshipReceived",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MsgTypeSingleMsg" => Some(Self::SingleMsg),
            "MsgTypeGroupMsg" => Some(Self::GroupMsg),
            "MsgTypeGroupInvitation" => Some(Self::GroupInvitation),
            "MsgTypeGroupInviteNew" => Some(Self::GroupInviteNew),
            "MsgTypeGroupMemberExit" => Some(Self::GroupMemberExit),
            "MsgTypeGroupRemoveMember" => Some(Self::GroupRemoveMember),
            "MsgTypeGroupDismiss" => Some(Self::GroupDismiss),
            "MsgTypeGroupDismissOrExitReceived" => Some(Self::GroupDismissOrExitReceived),
            "MsgTypeGroupInvitationReceived" => Some(Self::GroupInvitationReceived),
            "MsgTypeGroupUpdate" => Some(Self::GroupUpdate),
            "MsgTypeFriendApplyReq" => Some(Self::FriendApplyReq),
            "MsgTypeFriendApplyResp" => Some(Self::FriendApplyResp),
            "MsgTypeFriendBlack" => Some(Self::FriendBlack),
            "MsgTypeFriendDelete" => Some(Self::FriendDelete),
            "MsgTypeSingleCallInvite" => Some(Self::SingleCallInvite),
            "MsgTypeRejectSingleCall" => Some(Self::RejectSingleCall),
            "MsgTypeAgreeSingleCall" => Some(Self::AgreeSingleCall),
            "MsgTypeSingleCallInviteNotAnswer" => Some(Self::SingleCallInviteNotAnswer),
            "MsgTypeSingleCallInviteCancel" => Some(Self::SingleCallInviteCancel),
            "MsgTypeSingleCallOffer" => Some(Self::SingleCallOffer),
            "MsgTypeHangup" => Some(Self::Hangup),
            "MsgTypeConnectSingleCall" => Some(Self::ConnectSingleCall),
            "MsgTypeCandidate" => Some(Self::Candidate),
            "MsgTypeRead" => Some(Self::Read),
            "MsgTypeMsgRecResp" => Some(Self::MsgRecResp),
            "MsgTypeNotification" => Some(Self::Notification),
            "MsgTypeService" => Some(Self::Service),
            "MsgTypeFriendshipReceived" => Some(Self::FriendshipReceived),
            _ => None,
        }
    }
}
/// 平台类型
#[derive(
    serde::Serialize,
    serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[repr(i32)]
pub enum PlatformType {
    Desktop = 0,
    Mobile = 1,
}
impl PlatformType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Desktop => "Desktop",
            Self::Mobile => "Mobile",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Desktop" => Some(Self::Desktop),
            "Mobile" => Some(Self::Mobile),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod msg_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MsgServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MsgServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MsgServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            MsgServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// send message through rpc
        pub async fn send_message(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.MsgService/SendMessage");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.MsgService", "SendMessage"));
            self.inner.unary(req, path, codec).await
        }
        /// send single message to user by websocket
        pub async fn send_msg_to_user(
            &mut self,
            request: impl tonic::IntoRequest<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/message.MsgService/SendMsgToUser");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("message.MsgService", "SendMsgToUser"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod msg_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServiceServer.
    #[async_trait]
    pub trait MsgService: std::marker::Send + std::marker::Sync + 'static {
        /// send message through rpc
        async fn send_message(
            &self,
            request: tonic::Request<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status>;
        /// send single message to user by websocket
        async fn send_msg_to_user(
            &self,
            request: tonic::Request<super::SendMsgRequest>,
        ) -> std::result::Result<tonic::Response<super::SendMsgResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MsgServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> MsgServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServiceServer<T>
    where
        T: MsgService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/message.MsgService/SendMessage" => {
                    #[allow(non_camel_case_types)]
                    struct SendMessageSvc<T: MsgService>(pub Arc<T>);
                    impl<T: MsgService> tonic::server::UnaryService<super::SendMsgRequest> for SendMessageSvc<T> {
                        type Response = super::SendMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MsgService>::send_message(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SendMessageSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/message.MsgService/SendMsgToUser" => {
                    #[allow(non_camel_case_types)]
                    struct SendMsgToUserSvc<T: MsgService>(pub Arc<T>);
                    impl<T: MsgService> tonic::server::UnaryService<super::SendMsgRequest> for SendMsgToUserSvc<T> {
                        type Response = super::SendMsgResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendMsgRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as MsgService>::send_msg_to_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = SendMsgToUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    let mut response = http::Response::new(empty_body());
                    let headers = response.headers_mut();
                    headers.insert(
                        tonic::Status::GRPC_STATUS,
                        (tonic::Code::Unimplemented as i32).into(),
                    );
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    );
                    Ok(response)
                }),
            }
        }
    }
    impl<T> Clone for MsgServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "message.MsgService";
    impl<T> tonic::server::NamedService for MsgServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
