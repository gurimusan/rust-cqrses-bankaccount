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

const METHOD_BANK_ACCOUNT_SERVICE_OPEN: ::grpcio::Method<super::bank_account::OpenBankAccountRequest, super::bank_account::OpenBankAccountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/BankAccountService/open",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BANK_ACCOUNT_SERVICE_UPDATE: ::grpcio::Method<super::bank_account::UpdateBankAccountRequest, super::bank_account::UpdateBankAccountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/BankAccountService/update",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BANK_ACCOUNT_SERVICE_DEPOSIT: ::grpcio::Method<super::bank_account::DepositBankAccountRequest, super::bank_account::DepositBankAccountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/BankAccountService/deposit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BANK_ACCOUNT_SERVICE_WITHDRAW: ::grpcio::Method<super::bank_account::WithdrawBankAccountRequest, super::bank_account::WithdrawBankAccountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/BankAccountService/withdraw",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_BANK_ACCOUNT_SERVICE_CLOSE: ::grpcio::Method<super::bank_account::CloseBankAccountRequest, super::bank_account::CloseBankAccountResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/BankAccountService/close",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct BankAccountServiceClient {
    client: ::grpcio::Client,
}

impl BankAccountServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        BankAccountServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn open_opt(&self, req: &super::bank_account::OpenBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::bank_account::OpenBankAccountResponse> {
        self.client.unary_call(&METHOD_BANK_ACCOUNT_SERVICE_OPEN, req, opt)
    }

    pub fn open(&self, req: &super::bank_account::OpenBankAccountRequest) -> ::grpcio::Result<super::bank_account::OpenBankAccountResponse> {
        self.open_opt(req, ::grpcio::CallOption::default())
    }

    pub fn open_async_opt(&self, req: &super::bank_account::OpenBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::OpenBankAccountResponse>> {
        self.client.unary_call_async(&METHOD_BANK_ACCOUNT_SERVICE_OPEN, req, opt)
    }

    pub fn open_async(&self, req: &super::bank_account::OpenBankAccountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::OpenBankAccountResponse>> {
        self.open_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_opt(&self, req: &super::bank_account::UpdateBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::bank_account::UpdateBankAccountResponse> {
        self.client.unary_call(&METHOD_BANK_ACCOUNT_SERVICE_UPDATE, req, opt)
    }

    pub fn update(&self, req: &super::bank_account::UpdateBankAccountRequest) -> ::grpcio::Result<super::bank_account::UpdateBankAccountResponse> {
        self.update_opt(req, ::grpcio::CallOption::default())
    }

    pub fn update_async_opt(&self, req: &super::bank_account::UpdateBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::UpdateBankAccountResponse>> {
        self.client.unary_call_async(&METHOD_BANK_ACCOUNT_SERVICE_UPDATE, req, opt)
    }

    pub fn update_async(&self, req: &super::bank_account::UpdateBankAccountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::UpdateBankAccountResponse>> {
        self.update_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn deposit_opt(&self, req: &super::bank_account::DepositBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::bank_account::DepositBankAccountResponse> {
        self.client.unary_call(&METHOD_BANK_ACCOUNT_SERVICE_DEPOSIT, req, opt)
    }

    pub fn deposit(&self, req: &super::bank_account::DepositBankAccountRequest) -> ::grpcio::Result<super::bank_account::DepositBankAccountResponse> {
        self.deposit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn deposit_async_opt(&self, req: &super::bank_account::DepositBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::DepositBankAccountResponse>> {
        self.client.unary_call_async(&METHOD_BANK_ACCOUNT_SERVICE_DEPOSIT, req, opt)
    }

    pub fn deposit_async(&self, req: &super::bank_account::DepositBankAccountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::DepositBankAccountResponse>> {
        self.deposit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn withdraw_opt(&self, req: &super::bank_account::WithdrawBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::bank_account::WithdrawBankAccountResponse> {
        self.client.unary_call(&METHOD_BANK_ACCOUNT_SERVICE_WITHDRAW, req, opt)
    }

    pub fn withdraw(&self, req: &super::bank_account::WithdrawBankAccountRequest) -> ::grpcio::Result<super::bank_account::WithdrawBankAccountResponse> {
        self.withdraw_opt(req, ::grpcio::CallOption::default())
    }

    pub fn withdraw_async_opt(&self, req: &super::bank_account::WithdrawBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::WithdrawBankAccountResponse>> {
        self.client.unary_call_async(&METHOD_BANK_ACCOUNT_SERVICE_WITHDRAW, req, opt)
    }

    pub fn withdraw_async(&self, req: &super::bank_account::WithdrawBankAccountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::WithdrawBankAccountResponse>> {
        self.withdraw_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn close_opt(&self, req: &super::bank_account::CloseBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::bank_account::CloseBankAccountResponse> {
        self.client.unary_call(&METHOD_BANK_ACCOUNT_SERVICE_CLOSE, req, opt)
    }

    pub fn close(&self, req: &super::bank_account::CloseBankAccountRequest) -> ::grpcio::Result<super::bank_account::CloseBankAccountResponse> {
        self.close_opt(req, ::grpcio::CallOption::default())
    }

    pub fn close_async_opt(&self, req: &super::bank_account::CloseBankAccountRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::CloseBankAccountResponse>> {
        self.client.unary_call_async(&METHOD_BANK_ACCOUNT_SERVICE_CLOSE, req, opt)
    }

    pub fn close_async(&self, req: &super::bank_account::CloseBankAccountRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::bank_account::CloseBankAccountResponse>> {
        self.close_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait BankAccountService {
    fn open(&mut self, ctx: ::grpcio::RpcContext, req: super::bank_account::OpenBankAccountRequest, sink: ::grpcio::UnarySink<super::bank_account::OpenBankAccountResponse>);
    fn update(&mut self, ctx: ::grpcio::RpcContext, req: super::bank_account::UpdateBankAccountRequest, sink: ::grpcio::UnarySink<super::bank_account::UpdateBankAccountResponse>);
    fn deposit(&mut self, ctx: ::grpcio::RpcContext, req: super::bank_account::DepositBankAccountRequest, sink: ::grpcio::UnarySink<super::bank_account::DepositBankAccountResponse>);
    fn withdraw(&mut self, ctx: ::grpcio::RpcContext, req: super::bank_account::WithdrawBankAccountRequest, sink: ::grpcio::UnarySink<super::bank_account::WithdrawBankAccountResponse>);
    fn close(&mut self, ctx: ::grpcio::RpcContext, req: super::bank_account::CloseBankAccountRequest, sink: ::grpcio::UnarySink<super::bank_account::CloseBankAccountResponse>);
}

pub fn create_bank_account_service<S: BankAccountService + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BANK_ACCOUNT_SERVICE_OPEN, move |ctx, req, resp| {
        instance.open(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BANK_ACCOUNT_SERVICE_UPDATE, move |ctx, req, resp| {
        instance.update(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BANK_ACCOUNT_SERVICE_DEPOSIT, move |ctx, req, resp| {
        instance.deposit(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_BANK_ACCOUNT_SERVICE_WITHDRAW, move |ctx, req, resp| {
        instance.withdraw(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_BANK_ACCOUNT_SERVICE_CLOSE, move |ctx, req, resp| {
        instance.close(ctx, req, resp)
    });
    builder.build()
}
