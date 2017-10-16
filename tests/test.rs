#![feature(prelude_import)]
#![no_std]
#![no_std]
#![feature(alloc)]
#![feature(proc_macro)]
#[prelude_import]
use core::prelude::v1::*;
#[macro_use]
extern crate core as core;
//#![cfg(test)]

#[cfg(not(test))]
extern crate alloc;

#[cfg(not(test))]
use alloc::vec::Vec;

#[cfg(not(test))]
use alloc::borrow::Cow;

extern crate pwasm_abi;
extern crate pwasm_abi_derive;

use pwasm_abi_derive::eth_dispatch;

type U256 = [u8; 32];

pub trait TestContract {
    fn ctor(&mut self);
    fn baz(&mut self, _p1: u32, _p2: bool);
    fn boo(&mut self, _arg: u32)
    -> u32;
    fn sam(&mut self, _p1: Vec<u8>, _p2: bool, _p3: Vec<U256>);



















}
pub struct Endpoint<T: TestContract> {
    inner: T,
    table: &'static ::pwasm_abi::eth::Table,
}
impl <T: TestContract> Endpoint<T> {
    pub fn new(inner: T) -> Self {
        Endpoint{inner: inner,
                 table:
                     {
                         const TABLE: &'static ::pwasm_abi::eth::Table =
                             &::pwasm_abi::eth::Table{inner:
                                                          Cow::Borrowed(&[::pwasm_abi::eth::HashSignature{hash:
                                                                                                              1177829425u32,
                                                                                                          signature:
                                                                                                              ::pwasm_abi::eth::Signature{params:
                                                                                                                                              Cow::Borrowed(&[]),
                                                                                                                                          result:
                                                                                                                                              None,},},
                                                                          ::pwasm_abi::eth::HashSignature{hash:
                                                                                                              3452794816u32,
                                                                                                          signature:
                                                                                                              ::pwasm_abi::eth::Signature{params:
                                                                                                                                              Cow::Borrowed(&[::pwasm_abi::eth::ParamType::U32,
                                                                                                                                                              ::pwasm_abi::eth::ParamType::Bool]),
                                                                                                                                          result:
                                                                                                                                              None,},},
                                                                          ::pwasm_abi::eth::HashSignature{hash:
                                                                                                              1574614228u32,
                                                                                                          signature:
                                                                                                              ::pwasm_abi::eth::Signature{params:
                                                                                                                                              Cow::Borrowed(&[::pwasm_abi::eth::ParamType::U32]),
                                                                                                                                          result:
                                                                                                                                              None,},},
                                                                          ::pwasm_abi::eth::HashSignature{hash:
                                                                                                              2774809586u32,
                                                                                                          signature:
                                                                                                              ::pwasm_abi::eth::Signature{params:
                                                                                                                                              Cow::Borrowed(&[::pwasm_abi::eth::ParamType::Bytes,
                                                                                                                                                              ::pwasm_abi::eth::ParamType::Bool,
                                                                                                                                                              ::pwasm_abi::eth::ParamType::Array(::pwasm_abi::eth::ArrayRef::Static(&::pwasm_abi::eth::ParamType::U256))]),
                                                                                                                                          result:
                                                                                                                                              None,},}]),
                                                      fallback: None,};
                         TABLE
                     },}
    }
    pub fn dispatch(&mut self, payload: &[u8]) -> Vec<u8> {
        let inner = &mut self.inner;
        self.table.dispatch(payload,
                            |method_id, args|
                                {
                                    let mut args = args.into_iter();
                                    match method_id {
                                        1177829425u32 => {
                                            inner.ctor();
                                            None
                                        }
                                        3452794816u32 => {
                                            inner.baz(args.next().expect("Failed to fetch next argument").into(),
                                                      args.next().expect("Failed to fetch next argument").into());
                                            None
                                        }
                                        1574614228u32 => {
                                            inner.boo(args.next().expect("Failed to fetch next argument").into());
                                            None
                                        }
                                        2774809586u32 => {
                                            inner.sam(args.next().expect("Failed to fetch next argument").into(),
                                                      args.next().expect("Failed to fetch next argument").into(),
                                                      args.next().expect("Failed to fetch next argument").into());
                                            None
                                        }
                                        _ => {
                                            ::panicking::panic(&("Invalid method signature",
                                                                 "src\\lib.rs",
                                                                 24u32, 0u32))
                                        }
                                    }
                                }).expect("Failed abi dispatch")
    }
    #[allow(unused_variables)]
    pub fn dispatch_ctor(&mut self, payload: &[u8]) {
        let inner = &mut self.inner;
        self.table.fallback_dispatch(payload,
                                     |args|
                                         {
                                             inner.ctor();
                                         }).expect("Failed fallback abi dispatch");
    }
}
const PAYLOAD_SAMPLE_1: &[u8] =
    &[205, 205, 119, 192, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
const PAYLOAD_SAMPLE_2: &[u8] =
    &[165, 100, 59, 242, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 100, 97, 118, 101, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 3];
const PAYLOAD_SAMPLE_3: &[u8] =
    &[93, 218, 180, 212, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 69];
