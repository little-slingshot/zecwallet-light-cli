// This file is generated by rust-protobuf 2.10.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
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
//! Generated file from `compact_formats.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_10_2;

#[derive(PartialEq,Clone,Default)]
pub struct CompactBlock {
    // message fields
    pub protoVersion: u32,
    pub height: u64,
    pub hash: ::std::vec::Vec<u8>,
    pub prevHash: ::std::vec::Vec<u8>,
    pub time: u32,
    pub header: ::std::vec::Vec<u8>,
    pub vtx: ::protobuf::RepeatedField<CompactTx>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CompactBlock {
    fn default() -> &'a CompactBlock {
        <CompactBlock as ::protobuf::Message>::default_instance()
    }
}

impl CompactBlock {
    pub fn new() -> CompactBlock {
        ::std::default::Default::default()
    }

    // uint32 protoVersion = 1;


    pub fn get_protoVersion(&self) -> u32 {
        self.protoVersion
    }
    pub fn clear_protoVersion(&mut self) {
        self.protoVersion = 0;
    }

    // Param is passed by value, moved
    pub fn set_protoVersion(&mut self, v: u32) {
        self.protoVersion = v;
    }

    // uint64 height = 2;


    pub fn get_height(&self) -> u64 {
        self.height
    }
    pub fn clear_height(&mut self) {
        self.height = 0;
    }

    // Param is passed by value, moved
    pub fn set_height(&mut self, v: u64) {
        self.height = v;
    }

    // bytes hash = 3;


    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }
    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    // bytes prevHash = 4;


    pub fn get_prevHash(&self) -> &[u8] {
        &self.prevHash
    }
    pub fn clear_prevHash(&mut self) {
        self.prevHash.clear();
    }

    // Param is passed by value, moved
    pub fn set_prevHash(&mut self, v: ::std::vec::Vec<u8>) {
        self.prevHash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prevHash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.prevHash
    }

    // Take field
    pub fn take_prevHash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.prevHash, ::std::vec::Vec::new())
    }

    // uint32 time = 5;


    pub fn get_time(&self) -> u32 {
        self.time
    }
    pub fn clear_time(&mut self) {
        self.time = 0;
    }

    // Param is passed by value, moved
    pub fn set_time(&mut self, v: u32) {
        self.time = v;
    }

    // bytes header = 6;


    pub fn get_header(&self) -> &[u8] {
        &self.header
    }
    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ::std::vec::Vec<u8>) {
        self.header = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.header
    }

    // Take field
    pub fn take_header(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.header, ::std::vec::Vec::new())
    }

    // repeated .cash.z.wallet.sdk.rpc.CompactTx vtx = 7;


    pub fn get_vtx(&self) -> &[CompactTx] {
        &self.vtx
    }
    pub fn clear_vtx(&mut self) {
        self.vtx.clear();
    }

    // Param is passed by value, moved
    pub fn set_vtx(&mut self, v: ::protobuf::RepeatedField<CompactTx>) {
        self.vtx = v;
    }

    // Mutable pointer to the field.
    pub fn mut_vtx(&mut self) -> &mut ::protobuf::RepeatedField<CompactTx> {
        &mut self.vtx
    }

    // Take field
    pub fn take_vtx(&mut self) -> ::protobuf::RepeatedField<CompactTx> {
        ::std::mem::replace(&mut self.vtx, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for CompactBlock {
    fn is_initialized(&self) -> bool {
        for v in &self.vtx {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.protoVersion = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.height = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.prevHash)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.time = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.header)?;
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.vtx)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.protoVersion != 0 {
            my_size += ::protobuf::rt::value_size(1, self.protoVersion, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.height != 0 {
            my_size += ::protobuf::rt::value_size(2, self.height, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.hash);
        }
        if !self.prevHash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.prevHash);
        }
        if self.time != 0 {
            my_size += ::protobuf::rt::value_size(5, self.time, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.header.is_empty() {
            my_size += ::protobuf::rt::bytes_size(6, &self.header);
        }
        for value in &self.vtx {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.protoVersion != 0 {
            os.write_uint32(1, self.protoVersion)?;
        }
        if self.height != 0 {
            os.write_uint64(2, self.height)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(3, &self.hash)?;
        }
        if !self.prevHash.is_empty() {
            os.write_bytes(4, &self.prevHash)?;
        }
        if self.time != 0 {
            os.write_uint32(5, self.time)?;
        }
        if !self.header.is_empty() {
            os.write_bytes(6, &self.header)?;
        }
        for v in &self.vtx {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CompactBlock {
        CompactBlock::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "protoVersion",
                    |m: &CompactBlock| { &m.protoVersion },
                    |m: &mut CompactBlock| { &mut m.protoVersion },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "height",
                    |m: &CompactBlock| { &m.height },
                    |m: &mut CompactBlock| { &mut m.height },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    |m: &CompactBlock| { &m.hash },
                    |m: &mut CompactBlock| { &mut m.hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "prevHash",
                    |m: &CompactBlock| { &m.prevHash },
                    |m: &mut CompactBlock| { &mut m.prevHash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "time",
                    |m: &CompactBlock| { &m.time },
                    |m: &mut CompactBlock| { &mut m.time },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "header",
                    |m: &CompactBlock| { &m.header },
                    |m: &mut CompactBlock| { &mut m.header },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CompactTx>>(
                    "vtx",
                    |m: &CompactBlock| { &m.vtx },
                    |m: &mut CompactBlock| { &mut m.vtx },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CompactBlock>(
                    "CompactBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CompactBlock {
        static mut instance: ::protobuf::lazy::Lazy<CompactBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CompactBlock,
        };
        unsafe {
            instance.get(CompactBlock::new)
        }
    }
}

impl ::protobuf::Clear for CompactBlock {
    fn clear(&mut self) {
        self.protoVersion = 0;
        self.height = 0;
        self.hash.clear();
        self.prevHash.clear();
        self.time = 0;
        self.header.clear();
        self.vtx.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompactTx {
    // message fields
    pub index: u64,
    pub hash: ::std::vec::Vec<u8>,
    pub fee: u32,
    pub spends: ::protobuf::RepeatedField<CompactSpend>,
    pub outputs: ::protobuf::RepeatedField<CompactOutput>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CompactTx {
    fn default() -> &'a CompactTx {
        <CompactTx as ::protobuf::Message>::default_instance()
    }
}

impl CompactTx {
    pub fn new() -> CompactTx {
        ::std::default::Default::default()
    }

    // uint64 index = 1;


    pub fn get_index(&self) -> u64 {
        self.index
    }
    pub fn clear_index(&mut self) {
        self.index = 0;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }

    // bytes hash = 2;


    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }
    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    // uint32 fee = 3;


    pub fn get_fee(&self) -> u32 {
        self.fee
    }
    pub fn clear_fee(&mut self) {
        self.fee = 0;
    }

    // Param is passed by value, moved
    pub fn set_fee(&mut self, v: u32) {
        self.fee = v;
    }

    // repeated .cash.z.wallet.sdk.rpc.CompactSpend spends = 4;


    pub fn get_spends(&self) -> &[CompactSpend] {
        &self.spends
    }
    pub fn clear_spends(&mut self) {
        self.spends.clear();
    }

    // Param is passed by value, moved
    pub fn set_spends(&mut self, v: ::protobuf::RepeatedField<CompactSpend>) {
        self.spends = v;
    }

    // Mutable pointer to the field.
    pub fn mut_spends(&mut self) -> &mut ::protobuf::RepeatedField<CompactSpend> {
        &mut self.spends
    }

    // Take field
    pub fn take_spends(&mut self) -> ::protobuf::RepeatedField<CompactSpend> {
        ::std::mem::replace(&mut self.spends, ::protobuf::RepeatedField::new())
    }

    // repeated .cash.z.wallet.sdk.rpc.CompactOutput outputs = 5;


    pub fn get_outputs(&self) -> &[CompactOutput] {
        &self.outputs
    }
    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    // Param is passed by value, moved
    pub fn set_outputs(&mut self, v: ::protobuf::RepeatedField<CompactOutput>) {
        self.outputs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_outputs(&mut self) -> &mut ::protobuf::RepeatedField<CompactOutput> {
        &mut self.outputs
    }

    // Take field
    pub fn take_outputs(&mut self) -> ::protobuf::RepeatedField<CompactOutput> {
        ::std::mem::replace(&mut self.outputs, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for CompactTx {
    fn is_initialized(&self) -> bool {
        for v in &self.spends {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.outputs {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.fee = tmp;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.spends)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.outputs)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(1, self.index, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.hash);
        }
        if self.fee != 0 {
            my_size += ::protobuf::rt::value_size(3, self.fee, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.spends {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.outputs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.index != 0 {
            os.write_uint64(1, self.index)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(2, &self.hash)?;
        }
        if self.fee != 0 {
            os.write_uint32(3, self.fee)?;
        }
        for v in &self.spends {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.outputs {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CompactTx {
        CompactTx::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    |m: &CompactTx| { &m.index },
                    |m: &mut CompactTx| { &mut m.index },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    |m: &CompactTx| { &m.hash },
                    |m: &mut CompactTx| { &mut m.hash },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "fee",
                    |m: &CompactTx| { &m.fee },
                    |m: &mut CompactTx| { &mut m.fee },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CompactSpend>>(
                    "spends",
                    |m: &CompactTx| { &m.spends },
                    |m: &mut CompactTx| { &mut m.spends },
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CompactOutput>>(
                    "outputs",
                    |m: &CompactTx| { &m.outputs },
                    |m: &mut CompactTx| { &mut m.outputs },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CompactTx>(
                    "CompactTx",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CompactTx {
        static mut instance: ::protobuf::lazy::Lazy<CompactTx> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CompactTx,
        };
        unsafe {
            instance.get(CompactTx::new)
        }
    }
}

impl ::protobuf::Clear for CompactTx {
    fn clear(&mut self) {
        self.index = 0;
        self.hash.clear();
        self.fee = 0;
        self.spends.clear();
        self.outputs.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactTx {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactTx {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompactSpend {
    // message fields
    pub nf: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CompactSpend {
    fn default() -> &'a CompactSpend {
        <CompactSpend as ::protobuf::Message>::default_instance()
    }
}

impl CompactSpend {
    pub fn new() -> CompactSpend {
        ::std::default::Default::default()
    }

    // bytes nf = 1;


    pub fn get_nf(&self) -> &[u8] {
        &self.nf
    }
    pub fn clear_nf(&mut self) {
        self.nf.clear();
    }

    // Param is passed by value, moved
    pub fn set_nf(&mut self, v: ::std::vec::Vec<u8>) {
        self.nf = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nf(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.nf
    }

    // Take field
    pub fn take_nf(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.nf, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CompactSpend {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.nf)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.nf.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.nf);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.nf.is_empty() {
            os.write_bytes(1, &self.nf)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CompactSpend {
        CompactSpend::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "nf",
                    |m: &CompactSpend| { &m.nf },
                    |m: &mut CompactSpend| { &mut m.nf },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CompactSpend>(
                    "CompactSpend",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CompactSpend {
        static mut instance: ::protobuf::lazy::Lazy<CompactSpend> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CompactSpend,
        };
        unsafe {
            instance.get(CompactSpend::new)
        }
    }
}

impl ::protobuf::Clear for CompactSpend {
    fn clear(&mut self) {
        self.nf.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactSpend {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactSpend {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompactOutput {
    // message fields
    pub cmu: ::std::vec::Vec<u8>,
    pub epk: ::std::vec::Vec<u8>,
    pub ciphertext: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CompactOutput {
    fn default() -> &'a CompactOutput {
        <CompactOutput as ::protobuf::Message>::default_instance()
    }
}

impl CompactOutput {
    pub fn new() -> CompactOutput {
        ::std::default::Default::default()
    }

    // bytes cmu = 1;


    pub fn get_cmu(&self) -> &[u8] {
        &self.cmu
    }
    pub fn clear_cmu(&mut self) {
        self.cmu.clear();
    }

    // Param is passed by value, moved
    pub fn set_cmu(&mut self, v: ::std::vec::Vec<u8>) {
        self.cmu = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmu(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.cmu
    }

    // Take field
    pub fn take_cmu(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.cmu, ::std::vec::Vec::new())
    }

    // bytes epk = 2;


    pub fn get_epk(&self) -> &[u8] {
        &self.epk
    }
    pub fn clear_epk(&mut self) {
        self.epk.clear();
    }

    // Param is passed by value, moved
    pub fn set_epk(&mut self, v: ::std::vec::Vec<u8>) {
        self.epk = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_epk(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.epk
    }

    // Take field
    pub fn take_epk(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.epk, ::std::vec::Vec::new())
    }

    // bytes ciphertext = 3;


    pub fn get_ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }
    pub fn clear_ciphertext(&mut self) {
        self.ciphertext.clear();
    }

    // Param is passed by value, moved
    pub fn set_ciphertext(&mut self, v: ::std::vec::Vec<u8>) {
        self.ciphertext = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ciphertext(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.ciphertext
    }

    // Take field
    pub fn take_ciphertext(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.ciphertext, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for CompactOutput {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.cmu)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.epk)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.ciphertext)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.cmu.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.cmu);
        }
        if !self.epk.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.epk);
        }
        if !self.ciphertext.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.ciphertext);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.cmu.is_empty() {
            os.write_bytes(1, &self.cmu)?;
        }
        if !self.epk.is_empty() {
            os.write_bytes(2, &self.epk)?;
        }
        if !self.ciphertext.is_empty() {
            os.write_bytes(3, &self.ciphertext)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CompactOutput {
        CompactOutput::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "cmu",
                    |m: &CompactOutput| { &m.cmu },
                    |m: &mut CompactOutput| { &mut m.cmu },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "epk",
                    |m: &CompactOutput| { &m.epk },
                    |m: &mut CompactOutput| { &mut m.epk },
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "ciphertext",
                    |m: &CompactOutput| { &m.ciphertext },
                    |m: &mut CompactOutput| { &mut m.ciphertext },
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CompactOutput>(
                    "CompactOutput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static CompactOutput {
        static mut instance: ::protobuf::lazy::Lazy<CompactOutput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CompactOutput,
        };
        unsafe {
            instance.get(CompactOutput::new)
        }
    }
}

impl ::protobuf::Clear for CompactOutput {
    fn clear(&mut self) {
        self.cmu.clear();
        self.epk.clear();
        self.ciphertext.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactOutput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15compact_formats.proto\x12\x15cash.z.wallet.sdk.rpc\"\xb1\x01\n\x0c\
    CompactBlock\x12\x16\n\x0cprotoVersion\x18\x01\x20\x01(\rB\0\x12\x10\n\
    \x06height\x18\x02\x20\x01(\x04B\0\x12\x0e\n\x04hash\x18\x03\x20\x01(\
    \x0cB\0\x12\x12\n\x08prevHash\x18\x04\x20\x01(\x0cB\0\x12\x0e\n\x04time\
    \x18\x05\x20\x01(\rB\0\x12\x10\n\x06header\x18\x06\x20\x01(\x0cB\0\x12/\
    \n\x03vtx\x18\x07\x20\x03(\x0b2\x20.cash.z.wallet.sdk.rpc.CompactTxB\0:\
    \0\"\xad\x01\n\tCompactTx\x12\x0f\n\x05index\x18\x01\x20\x01(\x04B\0\x12\
    \x0e\n\x04hash\x18\x02\x20\x01(\x0cB\0\x12\r\n\x03fee\x18\x03\x20\x01(\r\
    B\0\x125\n\x06spends\x18\x04\x20\x03(\x0b2#.cash.z.wallet.sdk.rpc.Compac\
    tSpendB\0\x127\n\x07outputs\x18\x05\x20\x03(\x0b2$.cash.z.wallet.sdk.rpc\
    .CompactOutputB\0:\0\"\x1e\n\x0cCompactSpend\x12\x0c\n\x02nf\x18\x01\x20\
    \x01(\x0cB\0:\0\"E\n\rCompactOutput\x12\r\n\x03cmu\x18\x01\x20\x01(\x0cB\
    \0\x12\r\n\x03epk\x18\x02\x20\x01(\x0cB\0\x12\x14\n\nciphertext\x18\x03\
    \x20\x01(\x0cB\0:\0B\0b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
