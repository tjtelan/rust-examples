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

const METHOD_REFINERY_STATUS: ::grpcio::Method<super::refinery::OrderID, super::refinery::OrderRecord> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/refinery.Refinery/Status",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_REFINERY_ORDER: ::grpcio::Method<super::refinery::OrderForm, super::refinery::OrderStatus> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/refinery.Refinery/Order",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct RefineryClient {
    client: ::grpcio::Client,
}

impl RefineryClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        RefineryClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn status_opt(&self, req: &super::refinery::OrderID, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::refinery::OrderRecord> {
        self.client.unary_call(&METHOD_REFINERY_STATUS, req, opt)
    }

    pub fn status(&self, req: &super::refinery::OrderID) -> ::grpcio::Result<super::refinery::OrderRecord> {
        self.status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn status_async_opt(&self, req: &super::refinery::OrderID, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::refinery::OrderRecord>> {
        self.client.unary_call_async(&METHOD_REFINERY_STATUS, req, opt)
    }

    pub fn status_async(&self, req: &super::refinery::OrderID) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::refinery::OrderRecord>> {
        self.status_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn order_opt(&self, req: &super::refinery::OrderForm, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::refinery::OrderStatus> {
        self.client.unary_call(&METHOD_REFINERY_ORDER, req, opt)
    }

    pub fn order(&self, req: &super::refinery::OrderForm) -> ::grpcio::Result<super::refinery::OrderStatus> {
        self.order_opt(req, ::grpcio::CallOption::default())
    }

    pub fn order_async_opt(&self, req: &super::refinery::OrderForm, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::refinery::OrderStatus>> {
        self.client.unary_call_async(&METHOD_REFINERY_ORDER, req, opt)
    }

    pub fn order_async(&self, req: &super::refinery::OrderForm) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::refinery::OrderStatus>> {
        self.order_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Refinery {
    fn status(&mut self, ctx: ::grpcio::RpcContext, req: super::refinery::OrderID, sink: ::grpcio::UnarySink<super::refinery::OrderRecord>);
    fn order(&mut self, ctx: ::grpcio::RpcContext, req: super::refinery::OrderForm, sink: ::grpcio::UnarySink<super::refinery::OrderStatus>);
}

pub fn create_refinery<S: Refinery + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_REFINERY_STATUS, move |ctx, req, resp| {
        instance.status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_REFINERY_ORDER, move |ctx, req, resp| {
        instance.order(ctx, req, resp)
    });
    builder.build()
}
