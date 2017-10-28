// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_SEAWEED_SEND_HEARTBEAT: ::grpcio::Method<super::zergling::Heartbeat, super::zergling::HeartbeatResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/pb.Seaweed/SendHeartbeat",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct SeaweedClient {
    client: ::grpcio::Client,
}

impl SeaweedClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        SeaweedClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn send_heartbeat_opt(&self, opt: ::grpcio::CallOption) -> (::grpcio::ClientDuplexSender<super::zergling::Heartbeat>, ::grpcio::ClientDuplexReceiver<super::zergling::HeartbeatResponse>) {
        self.client.duplex_streaming(&METHOD_SEAWEED_SEND_HEARTBEAT, opt)
    }

    pub fn send_heartbeat(&self) -> (::grpcio::ClientDuplexSender<super::zergling::Heartbeat>, ::grpcio::ClientDuplexReceiver<super::zergling::HeartbeatResponse>) {
        self.send_heartbeat_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Seaweed {
    fn send_heartbeat(&self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::zergling::Heartbeat>, sink: ::grpcio::DuplexSink<super::zergling::HeartbeatResponse>);
}

pub fn create_seaweed<S: Seaweed + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_SEAWEED_SEND_HEARTBEAT, move |ctx, req, resp| {
        instance.send_heartbeat(ctx, req, resp)
    });
    builder.build()
}
