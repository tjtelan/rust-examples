// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_DINER_EAT: ::grpcio::Method<super::diner::Order, super::diner::Check> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/example.Diner/Eat",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DinerClient {
    client: ::grpcio::Client,
}

impl DinerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DinerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn eat_opt(&self, req: &super::diner::Order, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::diner::Check> {
        self.client.unary_call(&METHOD_DINER_EAT, req, opt)
    }

    pub fn eat(&self, req: &super::diner::Order) -> ::grpcio::Result<super::diner::Check> {
        self.eat_opt(req, ::grpcio::CallOption::default())
    }

    pub fn eat_async_opt(&self, req: &super::diner::Order, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::diner::Check>> {
        self.client.unary_call_async(&METHOD_DINER_EAT, req, opt)
    }

    pub fn eat_async(&self, req: &super::diner::Order) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::diner::Check>> {
        self.eat_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Diner {
    fn eat(&mut self, ctx: ::grpcio::RpcContext, req: super::diner::Order, sink: ::grpcio::UnarySink<super::diner::Check>);
}

pub fn create_diner<S: Diner + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_DINER_EAT, move |ctx, req, resp| {
        instance.eat(ctx, req, resp)
    });
    builder.build()
}
