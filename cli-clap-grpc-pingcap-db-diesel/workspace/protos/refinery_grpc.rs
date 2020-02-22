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

const METHOD_REFINERY_ORDER: ::grpcio::Method<super::refinery::OrderForm, super::refinery::OrderStatus> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/refinery.Refinery/Order",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_REFINERY_GET_ALL_RECORDS: ::grpcio::Method<super::empty::Empty, super::refinery::OrderRecordList> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/refinery.Refinery/GetAllRecords",
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

    pub fn get_all_records_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::refinery::OrderRecordList> {
        self.client.unary_call(&METHOD_REFINERY_GET_ALL_RECORDS, req, opt)
    }

    pub fn get_all_records(&self, req: &super::empty::Empty) -> ::grpcio::Result<super::refinery::OrderRecordList> {
        self.get_all_records_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_all_records_async_opt(&self, req: &super::empty::Empty, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::refinery::OrderRecordList>> {
        self.client.unary_call_async(&METHOD_REFINERY_GET_ALL_RECORDS, req, opt)
    }

    pub fn get_all_records_async(&self, req: &super::empty::Empty) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::refinery::OrderRecordList>> {
        self.get_all_records_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Refinery {
    fn order(&mut self, ctx: ::grpcio::RpcContext, req: super::refinery::OrderForm, sink: ::grpcio::UnarySink<super::refinery::OrderStatus>);
    fn get_all_records(&mut self, ctx: ::grpcio::RpcContext, req: super::empty::Empty, sink: ::grpcio::UnarySink<super::refinery::OrderRecordList>);
}

pub fn create_refinery<S: Refinery + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_REFINERY_ORDER, move |ctx, req, resp| {
        instance.order(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_REFINERY_GET_ALL_RECORDS, move |ctx, req, resp| {
        instance.get_all_records(ctx, req, resp)
    });
    builder.build()
}
