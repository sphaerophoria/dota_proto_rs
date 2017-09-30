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

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct CDOTAClientHardwareSpecs {
    // message fields
    logical_processors: ::std::option::Option<u32>,
    cpu_cycles_per_second: ::std::option::Option<u64>,
    total_physical_memory: ::std::option::Option<u64>,
    is_64_bit_os: ::std::option::Option<bool>,
    upload_measurement: ::std::option::Option<u64>,
    prefer_not_host: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTAClientHardwareSpecs {}

impl CDOTAClientHardwareSpecs {
    pub fn new() -> CDOTAClientHardwareSpecs {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTAClientHardwareSpecs {
        static mut instance: ::protobuf::lazy::Lazy<CDOTAClientHardwareSpecs> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTAClientHardwareSpecs,
        };
        unsafe {
            instance.get(CDOTAClientHardwareSpecs::new)
        }
    }

    // optional uint32 logical_processors = 1;

    pub fn clear_logical_processors(&mut self) {
        self.logical_processors = ::std::option::Option::None;
    }

    pub fn has_logical_processors(&self) -> bool {
        self.logical_processors.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logical_processors(&mut self, v: u32) {
        self.logical_processors = ::std::option::Option::Some(v);
    }

    pub fn get_logical_processors(&self) -> u32 {
        self.logical_processors.unwrap_or(0)
    }

    fn get_logical_processors_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.logical_processors
    }

    fn mut_logical_processors_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.logical_processors
    }

    // optional fixed64 cpu_cycles_per_second = 2;

    pub fn clear_cpu_cycles_per_second(&mut self) {
        self.cpu_cycles_per_second = ::std::option::Option::None;
    }

    pub fn has_cpu_cycles_per_second(&self) -> bool {
        self.cpu_cycles_per_second.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cpu_cycles_per_second(&mut self, v: u64) {
        self.cpu_cycles_per_second = ::std::option::Option::Some(v);
    }

    pub fn get_cpu_cycles_per_second(&self) -> u64 {
        self.cpu_cycles_per_second.unwrap_or(0)
    }

    fn get_cpu_cycles_per_second_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.cpu_cycles_per_second
    }

    fn mut_cpu_cycles_per_second_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.cpu_cycles_per_second
    }

    // optional fixed64 total_physical_memory = 3;

    pub fn clear_total_physical_memory(&mut self) {
        self.total_physical_memory = ::std::option::Option::None;
    }

    pub fn has_total_physical_memory(&self) -> bool {
        self.total_physical_memory.is_some()
    }

    // Param is passed by value, moved
    pub fn set_total_physical_memory(&mut self, v: u64) {
        self.total_physical_memory = ::std::option::Option::Some(v);
    }

    pub fn get_total_physical_memory(&self) -> u64 {
        self.total_physical_memory.unwrap_or(0)
    }

    fn get_total_physical_memory_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.total_physical_memory
    }

    fn mut_total_physical_memory_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.total_physical_memory
    }

    // optional bool is_64_bit_os = 4;

    pub fn clear_is_64_bit_os(&mut self) {
        self.is_64_bit_os = ::std::option::Option::None;
    }

    pub fn has_is_64_bit_os(&self) -> bool {
        self.is_64_bit_os.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_64_bit_os(&mut self, v: bool) {
        self.is_64_bit_os = ::std::option::Option::Some(v);
    }

    pub fn get_is_64_bit_os(&self) -> bool {
        self.is_64_bit_os.unwrap_or(false)
    }

    fn get_is_64_bit_os_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.is_64_bit_os
    }

    fn mut_is_64_bit_os_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.is_64_bit_os
    }

    // optional uint64 upload_measurement = 5;

    pub fn clear_upload_measurement(&mut self) {
        self.upload_measurement = ::std::option::Option::None;
    }

    pub fn has_upload_measurement(&self) -> bool {
        self.upload_measurement.is_some()
    }

    // Param is passed by value, moved
    pub fn set_upload_measurement(&mut self, v: u64) {
        self.upload_measurement = ::std::option::Option::Some(v);
    }

    pub fn get_upload_measurement(&self) -> u64 {
        self.upload_measurement.unwrap_or(0)
    }

    fn get_upload_measurement_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.upload_measurement
    }

    fn mut_upload_measurement_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.upload_measurement
    }

    // optional bool prefer_not_host = 6;

    pub fn clear_prefer_not_host(&mut self) {
        self.prefer_not_host = ::std::option::Option::None;
    }

    pub fn has_prefer_not_host(&self) -> bool {
        self.prefer_not_host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prefer_not_host(&mut self, v: bool) {
        self.prefer_not_host = ::std::option::Option::Some(v);
    }

    pub fn get_prefer_not_host(&self) -> bool {
        self.prefer_not_host.unwrap_or(false)
    }

    fn get_prefer_not_host_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.prefer_not_host
    }

    fn mut_prefer_not_host_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.prefer_not_host
    }
}

impl ::protobuf::Message for CDOTAClientHardwareSpecs {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.logical_processors = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.cpu_cycles_per_second = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_fixed64()?;
                    self.total_physical_memory = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.is_64_bit_os = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.upload_measurement = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.prefer_not_host = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.logical_processors {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.cpu_cycles_per_second {
            my_size += 9;
        }
        if let Some(v) = self.total_physical_memory {
            my_size += 9;
        }
        if let Some(v) = self.is_64_bit_os {
            my_size += 2;
        }
        if let Some(v) = self.upload_measurement {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.prefer_not_host {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.logical_processors {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.cpu_cycles_per_second {
            os.write_fixed64(2, v)?;
        }
        if let Some(v) = self.total_physical_memory {
            os.write_fixed64(3, v)?;
        }
        if let Some(v) = self.is_64_bit_os {
            os.write_bool(4, v)?;
        }
        if let Some(v) = self.upload_measurement {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.prefer_not_host {
            os.write_bool(6, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTAClientHardwareSpecs {
    fn new() -> CDOTAClientHardwareSpecs {
        CDOTAClientHardwareSpecs::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTAClientHardwareSpecs>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "logical_processors",
                    CDOTAClientHardwareSpecs::get_logical_processors_for_reflect,
                    CDOTAClientHardwareSpecs::mut_logical_processors_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "cpu_cycles_per_second",
                    CDOTAClientHardwareSpecs::get_cpu_cycles_per_second_for_reflect,
                    CDOTAClientHardwareSpecs::mut_cpu_cycles_per_second_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFixed64>(
                    "total_physical_memory",
                    CDOTAClientHardwareSpecs::get_total_physical_memory_for_reflect,
                    CDOTAClientHardwareSpecs::mut_total_physical_memory_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_64_bit_os",
                    CDOTAClientHardwareSpecs::get_is_64_bit_os_for_reflect,
                    CDOTAClientHardwareSpecs::mut_is_64_bit_os_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "upload_measurement",
                    CDOTAClientHardwareSpecs::get_upload_measurement_for_reflect,
                    CDOTAClientHardwareSpecs::mut_upload_measurement_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "prefer_not_host",
                    CDOTAClientHardwareSpecs::get_prefer_not_host_for_reflect,
                    CDOTAClientHardwareSpecs::mut_prefer_not_host_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTAClientHardwareSpecs>(
                    "CDOTAClientHardwareSpecs",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTAClientHardwareSpecs {
    fn clear(&mut self) {
        self.clear_logical_processors();
        self.clear_cpu_cycles_per_second();
        self.clear_total_physical_memory();
        self.clear_is_64_bit_os();
        self.clear_upload_measurement();
        self.clear_prefer_not_host();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTAClientHardwareSpecs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTAClientHardwareSpecs {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTASaveGame {
    // message fields
    match_id: ::std::option::Option<u64>,
    save_time: ::std::option::Option<u32>,
    players: ::protobuf::RepeatedField<CDOTASaveGame_Player>,
    save_instances: ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTASaveGame {}

impl CDOTASaveGame {
    pub fn new() -> CDOTASaveGame {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTASaveGame {
        static mut instance: ::protobuf::lazy::Lazy<CDOTASaveGame> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTASaveGame,
        };
        unsafe {
            instance.get(CDOTASaveGame::new)
        }
    }

    // optional uint64 match_id = 5;

    pub fn clear_match_id(&mut self) {
        self.match_id = ::std::option::Option::None;
    }

    pub fn has_match_id(&self) -> bool {
        self.match_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_match_id(&mut self, v: u64) {
        self.match_id = ::std::option::Option::Some(v);
    }

    pub fn get_match_id(&self) -> u64 {
        self.match_id.unwrap_or(0)
    }

    fn get_match_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.match_id
    }

    fn mut_match_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.match_id
    }

    // optional uint32 save_time = 2;

    pub fn clear_save_time(&mut self) {
        self.save_time = ::std::option::Option::None;
    }

    pub fn has_save_time(&self) -> bool {
        self.save_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_time(&mut self, v: u32) {
        self.save_time = ::std::option::Option::Some(v);
    }

    pub fn get_save_time(&self) -> u32 {
        self.save_time.unwrap_or(0)
    }

    fn get_save_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.save_time
    }

    fn mut_save_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.save_time
    }

    // repeated .CDOTASaveGame.Player players = 3;

    pub fn clear_players(&mut self) {
        self.players.clear();
    }

    // Param is passed by value, moved
    pub fn set_players(&mut self, v: ::protobuf::RepeatedField<CDOTASaveGame_Player>) {
        self.players = v;
    }

    // Mutable pointer to the field.
    pub fn mut_players(&mut self) -> &mut ::protobuf::RepeatedField<CDOTASaveGame_Player> {
        &mut self.players
    }

    // Take field
    pub fn take_players(&mut self) -> ::protobuf::RepeatedField<CDOTASaveGame_Player> {
        ::std::mem::replace(&mut self.players, ::protobuf::RepeatedField::new())
    }

    pub fn get_players(&self) -> &[CDOTASaveGame_Player] {
        &self.players
    }

    fn get_players_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTASaveGame_Player> {
        &self.players
    }

    fn mut_players_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTASaveGame_Player> {
        &mut self.players
    }

    // repeated .CDOTASaveGame.SaveInstance save_instances = 4;

    pub fn clear_save_instances(&mut self) {
        self.save_instances.clear();
    }

    // Param is passed by value, moved
    pub fn set_save_instances(&mut self, v: ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance>) {
        self.save_instances = v;
    }

    // Mutable pointer to the field.
    pub fn mut_save_instances(&mut self) -> &mut ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance> {
        &mut self.save_instances
    }

    // Take field
    pub fn take_save_instances(&mut self) -> ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance> {
        ::std::mem::replace(&mut self.save_instances, ::protobuf::RepeatedField::new())
    }

    pub fn get_save_instances(&self) -> &[CDOTASaveGame_SaveInstance] {
        &self.save_instances
    }

    fn get_save_instances_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTASaveGame_SaveInstance> {
        &self.save_instances
    }

    fn mut_save_instances_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance> {
        &mut self.save_instances
    }
}

impl ::protobuf::Message for CDOTASaveGame {
    fn is_initialized(&self) -> bool {
        for v in &self.players {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.save_instances {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.match_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.save_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.players)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.save_instances)?;
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
        if let Some(v) = self.match_id {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.save_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.players {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.save_instances {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.match_id {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.save_time {
            os.write_uint32(2, v)?;
        }
        for v in &self.players {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.save_instances {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTASaveGame {
    fn new() -> CDOTASaveGame {
        CDOTASaveGame::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTASaveGame>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "match_id",
                    CDOTASaveGame::get_match_id_for_reflect,
                    CDOTASaveGame::mut_match_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "save_time",
                    CDOTASaveGame::get_save_time_for_reflect,
                    CDOTASaveGame::mut_save_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTASaveGame_Player>>(
                    "players",
                    CDOTASaveGame::get_players_for_reflect,
                    CDOTASaveGame::mut_players_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTASaveGame_SaveInstance>>(
                    "save_instances",
                    CDOTASaveGame::get_save_instances_for_reflect,
                    CDOTASaveGame::mut_save_instances_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTASaveGame>(
                    "CDOTASaveGame",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTASaveGame {
    fn clear(&mut self) {
        self.clear_match_id();
        self.clear_save_time();
        self.clear_players();
        self.clear_save_instances();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTASaveGame {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTASaveGame {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTASaveGame_Player {
    // message fields
    team: ::std::option::Option<DOTA_GC_TEAM>,
    name: ::protobuf::SingularField<::std::string::String>,
    hero: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTASaveGame_Player {}

impl CDOTASaveGame_Player {
    pub fn new() -> CDOTASaveGame_Player {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTASaveGame_Player {
        static mut instance: ::protobuf::lazy::Lazy<CDOTASaveGame_Player> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTASaveGame_Player,
        };
        unsafe {
            instance.get(CDOTASaveGame_Player::new)
        }
    }

    // optional .DOTA_GC_TEAM team = 1;

    pub fn clear_team(&mut self) {
        self.team = ::std::option::Option::None;
    }

    pub fn has_team(&self) -> bool {
        self.team.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team(&mut self, v: DOTA_GC_TEAM) {
        self.team = ::std::option::Option::Some(v);
    }

    pub fn get_team(&self) -> DOTA_GC_TEAM {
        self.team.unwrap_or(DOTA_GC_TEAM::DOTA_GC_TEAM_GOOD_GUYS)
    }

    fn get_team_for_reflect(&self) -> &::std::option::Option<DOTA_GC_TEAM> {
        &self.team
    }

    fn mut_team_for_reflect(&mut self) -> &mut ::std::option::Option<DOTA_GC_TEAM> {
        &mut self.team
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string hero = 3;

    pub fn clear_hero(&mut self) {
        self.hero.clear();
    }

    pub fn has_hero(&self) -> bool {
        self.hero.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hero(&mut self, v: ::std::string::String) {
        self.hero = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hero(&mut self) -> &mut ::std::string::String {
        if self.hero.is_none() {
            self.hero.set_default();
        }
        self.hero.as_mut().unwrap()
    }

    // Take field
    pub fn take_hero(&mut self) -> ::std::string::String {
        self.hero.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hero(&self) -> &str {
        match self.hero.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_hero_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.hero
    }

    fn mut_hero_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.hero
    }
}

impl ::protobuf::Message for CDOTASaveGame_Player {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_enum()?;
                    self.team = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.hero)?;
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
        if let Some(v) = self.team {
            my_size += ::protobuf::rt::enum_size(1, v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.hero.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.team {
            os.write_enum(1, v.value())?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.hero.as_ref() {
            os.write_string(3, &v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTASaveGame_Player {
    fn new() -> CDOTASaveGame_Player {
        CDOTASaveGame_Player::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTASaveGame_Player>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DOTA_GC_TEAM>>(
                    "team",
                    CDOTASaveGame_Player::get_team_for_reflect,
                    CDOTASaveGame_Player::mut_team_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CDOTASaveGame_Player::get_name_for_reflect,
                    CDOTASaveGame_Player::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hero",
                    CDOTASaveGame_Player::get_hero_for_reflect,
                    CDOTASaveGame_Player::mut_hero_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTASaveGame_Player>(
                    "CDOTASaveGame_Player",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTASaveGame_Player {
    fn clear(&mut self) {
        self.clear_team();
        self.clear_name();
        self.clear_hero();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTASaveGame_Player {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTASaveGame_Player {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTASaveGame_SaveInstance {
    // message fields
    game_time: ::std::option::Option<u32>,
    team1_score: ::std::option::Option<u32>,
    team2_score: ::std::option::Option<u32>,
    player_positions: ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance_PlayerPositions>,
    save_id: ::std::option::Option<u32>,
    save_time: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTASaveGame_SaveInstance {}

impl CDOTASaveGame_SaveInstance {
    pub fn new() -> CDOTASaveGame_SaveInstance {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTASaveGame_SaveInstance {
        static mut instance: ::protobuf::lazy::Lazy<CDOTASaveGame_SaveInstance> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTASaveGame_SaveInstance,
        };
        unsafe {
            instance.get(CDOTASaveGame_SaveInstance::new)
        }
    }

    // optional uint32 game_time = 2;

    pub fn clear_game_time(&mut self) {
        self.game_time = ::std::option::Option::None;
    }

    pub fn has_game_time(&self) -> bool {
        self.game_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_game_time(&mut self, v: u32) {
        self.game_time = ::std::option::Option::Some(v);
    }

    pub fn get_game_time(&self) -> u32 {
        self.game_time.unwrap_or(0)
    }

    fn get_game_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.game_time
    }

    fn mut_game_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.game_time
    }

    // optional uint32 team1_score = 3;

    pub fn clear_team1_score(&mut self) {
        self.team1_score = ::std::option::Option::None;
    }

    pub fn has_team1_score(&self) -> bool {
        self.team1_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team1_score(&mut self, v: u32) {
        self.team1_score = ::std::option::Option::Some(v);
    }

    pub fn get_team1_score(&self) -> u32 {
        self.team1_score.unwrap_or(0)
    }

    fn get_team1_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team1_score
    }

    fn mut_team1_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team1_score
    }

    // optional uint32 team2_score = 4;

    pub fn clear_team2_score(&mut self) {
        self.team2_score = ::std::option::Option::None;
    }

    pub fn has_team2_score(&self) -> bool {
        self.team2_score.is_some()
    }

    // Param is passed by value, moved
    pub fn set_team2_score(&mut self, v: u32) {
        self.team2_score = ::std::option::Option::Some(v);
    }

    pub fn get_team2_score(&self) -> u32 {
        self.team2_score.unwrap_or(0)
    }

    fn get_team2_score_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.team2_score
    }

    fn mut_team2_score_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.team2_score
    }

    // repeated .CDOTASaveGame.SaveInstance.PlayerPositions player_positions = 5;

    pub fn clear_player_positions(&mut self) {
        self.player_positions.clear();
    }

    // Param is passed by value, moved
    pub fn set_player_positions(&mut self, v: ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance_PlayerPositions>) {
        self.player_positions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_player_positions(&mut self) -> &mut ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance_PlayerPositions> {
        &mut self.player_positions
    }

    // Take field
    pub fn take_player_positions(&mut self) -> ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance_PlayerPositions> {
        ::std::mem::replace(&mut self.player_positions, ::protobuf::RepeatedField::new())
    }

    pub fn get_player_positions(&self) -> &[CDOTASaveGame_SaveInstance_PlayerPositions] {
        &self.player_positions
    }

    fn get_player_positions_for_reflect(&self) -> &::protobuf::RepeatedField<CDOTASaveGame_SaveInstance_PlayerPositions> {
        &self.player_positions
    }

    fn mut_player_positions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CDOTASaveGame_SaveInstance_PlayerPositions> {
        &mut self.player_positions
    }

    // optional uint32 save_id = 6;

    pub fn clear_save_id(&mut self) {
        self.save_id = ::std::option::Option::None;
    }

    pub fn has_save_id(&self) -> bool {
        self.save_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_id(&mut self, v: u32) {
        self.save_id = ::std::option::Option::Some(v);
    }

    pub fn get_save_id(&self) -> u32 {
        self.save_id.unwrap_or(0)
    }

    fn get_save_id_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.save_id
    }

    fn mut_save_id_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.save_id
    }

    // optional uint32 save_time = 7;

    pub fn clear_save_time(&mut self) {
        self.save_time = ::std::option::Option::None;
    }

    pub fn has_save_time(&self) -> bool {
        self.save_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_save_time(&mut self, v: u32) {
        self.save_time = ::std::option::Option::Some(v);
    }

    pub fn get_save_time(&self) -> u32 {
        self.save_time.unwrap_or(0)
    }

    fn get_save_time_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.save_time
    }

    fn mut_save_time_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.save_time
    }
}

impl ::protobuf::Message for CDOTASaveGame_SaveInstance {
    fn is_initialized(&self) -> bool {
        for v in &self.player_positions {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.game_time = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team1_score = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.team2_score = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.player_positions)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.save_id = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.save_time = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.game_time {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team1_score {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.team2_score {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.player_positions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.save_id {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.save_time {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.game_time {
            os.write_uint32(2, v)?;
        }
        if let Some(v) = self.team1_score {
            os.write_uint32(3, v)?;
        }
        if let Some(v) = self.team2_score {
            os.write_uint32(4, v)?;
        }
        for v in &self.player_positions {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.save_id {
            os.write_uint32(6, v)?;
        }
        if let Some(v) = self.save_time {
            os.write_uint32(7, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTASaveGame_SaveInstance {
    fn new() -> CDOTASaveGame_SaveInstance {
        CDOTASaveGame_SaveInstance::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTASaveGame_SaveInstance>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "game_time",
                    CDOTASaveGame_SaveInstance::get_game_time_for_reflect,
                    CDOTASaveGame_SaveInstance::mut_game_time_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team1_score",
                    CDOTASaveGame_SaveInstance::get_team1_score_for_reflect,
                    CDOTASaveGame_SaveInstance::mut_team1_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "team2_score",
                    CDOTASaveGame_SaveInstance::get_team2_score_for_reflect,
                    CDOTASaveGame_SaveInstance::mut_team2_score_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CDOTASaveGame_SaveInstance_PlayerPositions>>(
                    "player_positions",
                    CDOTASaveGame_SaveInstance::get_player_positions_for_reflect,
                    CDOTASaveGame_SaveInstance::mut_player_positions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "save_id",
                    CDOTASaveGame_SaveInstance::get_save_id_for_reflect,
                    CDOTASaveGame_SaveInstance::mut_save_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "save_time",
                    CDOTASaveGame_SaveInstance::get_save_time_for_reflect,
                    CDOTASaveGame_SaveInstance::mut_save_time_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTASaveGame_SaveInstance>(
                    "CDOTASaveGame_SaveInstance",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTASaveGame_SaveInstance {
    fn clear(&mut self) {
        self.clear_game_time();
        self.clear_team1_score();
        self.clear_team2_score();
        self.clear_player_positions();
        self.clear_save_id();
        self.clear_save_time();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTASaveGame_SaveInstance {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTASaveGame_SaveInstance {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CDOTASaveGame_SaveInstance_PlayerPositions {
    // message fields
    x: ::std::option::Option<f32>,
    y: ::std::option::Option<f32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CDOTASaveGame_SaveInstance_PlayerPositions {}

impl CDOTASaveGame_SaveInstance_PlayerPositions {
    pub fn new() -> CDOTASaveGame_SaveInstance_PlayerPositions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CDOTASaveGame_SaveInstance_PlayerPositions {
        static mut instance: ::protobuf::lazy::Lazy<CDOTASaveGame_SaveInstance_PlayerPositions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CDOTASaveGame_SaveInstance_PlayerPositions,
        };
        unsafe {
            instance.get(CDOTASaveGame_SaveInstance_PlayerPositions::new)
        }
    }

    // optional float x = 1;

    pub fn clear_x(&mut self) {
        self.x = ::std::option::Option::None;
    }

    pub fn has_x(&self) -> bool {
        self.x.is_some()
    }

    // Param is passed by value, moved
    pub fn set_x(&mut self, v: f32) {
        self.x = ::std::option::Option::Some(v);
    }

    pub fn get_x(&self) -> f32 {
        self.x.unwrap_or(0.)
    }

    fn get_x_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.x
    }

    fn mut_x_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.x
    }

    // optional float y = 2;

    pub fn clear_y(&mut self) {
        self.y = ::std::option::Option::None;
    }

    pub fn has_y(&self) -> bool {
        self.y.is_some()
    }

    // Param is passed by value, moved
    pub fn set_y(&mut self, v: f32) {
        self.y = ::std::option::Option::Some(v);
    }

    pub fn get_y(&self) -> f32 {
        self.y.unwrap_or(0.)
    }

    fn get_y_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.y
    }

    fn mut_y_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.y
    }
}

impl ::protobuf::Message for CDOTASaveGame_SaveInstance_PlayerPositions {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.x = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.y = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.x {
            my_size += 5;
        }
        if let Some(v) = self.y {
            my_size += 5;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.x {
            os.write_float(1, v)?;
        }
        if let Some(v) = self.y {
            os.write_float(2, v)?;
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CDOTASaveGame_SaveInstance_PlayerPositions {
    fn new() -> CDOTASaveGame_SaveInstance_PlayerPositions {
        CDOTASaveGame_SaveInstance_PlayerPositions::new()
    }

    fn descriptor_static(_: ::std::option::Option<CDOTASaveGame_SaveInstance_PlayerPositions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "x",
                    CDOTASaveGame_SaveInstance_PlayerPositions::get_x_for_reflect,
                    CDOTASaveGame_SaveInstance_PlayerPositions::mut_x_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "y",
                    CDOTASaveGame_SaveInstance_PlayerPositions::get_y_for_reflect,
                    CDOTASaveGame_SaveInstance_PlayerPositions::mut_y_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CDOTASaveGame_SaveInstance_PlayerPositions>(
                    "CDOTASaveGame_SaveInstance_PlayerPositions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CDOTASaveGame_SaveInstance_PlayerPositions {
    fn clear(&mut self) {
        self.clear_x();
        self.clear_y();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CDOTASaveGame_SaveInstance_PlayerPositions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDOTASaveGame_SaveInstance_PlayerPositions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_GameMode {
    DOTA_GAMEMODE_NONE = 0,
    DOTA_GAMEMODE_AP = 1,
    DOTA_GAMEMODE_CM = 2,
    DOTA_GAMEMODE_RD = 3,
    DOTA_GAMEMODE_SD = 4,
    DOTA_GAMEMODE_AR = 5,
    DOTA_GAMEMODE_INTRO = 6,
    DOTA_GAMEMODE_HW = 7,
    DOTA_GAMEMODE_REVERSE_CM = 8,
    DOTA_GAMEMODE_XMAS = 9,
    DOTA_GAMEMODE_TUTORIAL = 10,
    DOTA_GAMEMODE_MO = 11,
    DOTA_GAMEMODE_LP = 12,
    DOTA_GAMEMODE_POOL1 = 13,
    DOTA_GAMEMODE_FH = 14,
    DOTA_GAMEMODE_CUSTOM = 15,
    DOTA_GAMEMODE_CD = 16,
    DOTA_GAMEMODE_BD = 17,
    DOTA_GAMEMODE_ABILITY_DRAFT = 18,
    DOTA_GAMEMODE_EVENT = 19,
    DOTA_GAMEMODE_ARDM = 20,
    DOTA_GAMEMODE_1V1MID = 21,
    DOTA_GAMEMODE_ALL_DRAFT = 22,
}

impl ::protobuf::ProtobufEnum for DOTA_GameMode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_GameMode> {
        match value {
            0 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_NONE),
            1 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_AP),
            2 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_CM),
            3 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_RD),
            4 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_SD),
            5 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_AR),
            6 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_INTRO),
            7 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_HW),
            8 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_REVERSE_CM),
            9 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_XMAS),
            10 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_TUTORIAL),
            11 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_MO),
            12 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_LP),
            13 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_POOL1),
            14 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_FH),
            15 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_CUSTOM),
            16 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_CD),
            17 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_BD),
            18 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_ABILITY_DRAFT),
            19 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_EVENT),
            20 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_ARDM),
            21 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_1V1MID),
            22 => ::std::option::Option::Some(DOTA_GameMode::DOTA_GAMEMODE_ALL_DRAFT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_GameMode] = &[
            DOTA_GameMode::DOTA_GAMEMODE_NONE,
            DOTA_GameMode::DOTA_GAMEMODE_AP,
            DOTA_GameMode::DOTA_GAMEMODE_CM,
            DOTA_GameMode::DOTA_GAMEMODE_RD,
            DOTA_GameMode::DOTA_GAMEMODE_SD,
            DOTA_GameMode::DOTA_GAMEMODE_AR,
            DOTA_GameMode::DOTA_GAMEMODE_INTRO,
            DOTA_GameMode::DOTA_GAMEMODE_HW,
            DOTA_GameMode::DOTA_GAMEMODE_REVERSE_CM,
            DOTA_GameMode::DOTA_GAMEMODE_XMAS,
            DOTA_GameMode::DOTA_GAMEMODE_TUTORIAL,
            DOTA_GameMode::DOTA_GAMEMODE_MO,
            DOTA_GameMode::DOTA_GAMEMODE_LP,
            DOTA_GameMode::DOTA_GAMEMODE_POOL1,
            DOTA_GameMode::DOTA_GAMEMODE_FH,
            DOTA_GameMode::DOTA_GAMEMODE_CUSTOM,
            DOTA_GameMode::DOTA_GAMEMODE_CD,
            DOTA_GameMode::DOTA_GAMEMODE_BD,
            DOTA_GameMode::DOTA_GAMEMODE_ABILITY_DRAFT,
            DOTA_GameMode::DOTA_GAMEMODE_EVENT,
            DOTA_GameMode::DOTA_GAMEMODE_ARDM,
            DOTA_GameMode::DOTA_GAMEMODE_1V1MID,
            DOTA_GameMode::DOTA_GAMEMODE_ALL_DRAFT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTA_GameMode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_GameMode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_GameMode {
}

impl ::protobuf::reflect::ProtobufValue for DOTA_GameMode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_GameState {
    DOTA_GAMERULES_STATE_INIT = 0,
    DOTA_GAMERULES_STATE_WAIT_FOR_PLAYERS_TO_LOAD = 1,
    DOTA_GAMERULES_STATE_HERO_SELECTION = 2,
    DOTA_GAMERULES_STATE_STRATEGY_TIME = 3,
    DOTA_GAMERULES_STATE_PRE_GAME = 4,
    DOTA_GAMERULES_STATE_GAME_IN_PROGRESS = 5,
    DOTA_GAMERULES_STATE_POST_GAME = 6,
    DOTA_GAMERULES_STATE_DISCONNECT = 7,
    DOTA_GAMERULES_STATE_TEAM_SHOWCASE = 8,
    DOTA_GAMERULES_STATE_CUSTOM_GAME_SETUP = 9,
    DOTA_GAMERULES_STATE_WAIT_FOR_MAP_TO_LOAD = 10,
    DOTA_GAMERULES_STATE_LAST = 11,
}

impl ::protobuf::ProtobufEnum for DOTA_GameState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_GameState> {
        match value {
            0 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_INIT),
            1 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_WAIT_FOR_PLAYERS_TO_LOAD),
            2 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_HERO_SELECTION),
            3 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_STRATEGY_TIME),
            4 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_PRE_GAME),
            5 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_GAME_IN_PROGRESS),
            6 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_POST_GAME),
            7 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_DISCONNECT),
            8 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_TEAM_SHOWCASE),
            9 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_CUSTOM_GAME_SETUP),
            10 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_WAIT_FOR_MAP_TO_LOAD),
            11 => ::std::option::Option::Some(DOTA_GameState::DOTA_GAMERULES_STATE_LAST),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_GameState] = &[
            DOTA_GameState::DOTA_GAMERULES_STATE_INIT,
            DOTA_GameState::DOTA_GAMERULES_STATE_WAIT_FOR_PLAYERS_TO_LOAD,
            DOTA_GameState::DOTA_GAMERULES_STATE_HERO_SELECTION,
            DOTA_GameState::DOTA_GAMERULES_STATE_STRATEGY_TIME,
            DOTA_GameState::DOTA_GAMERULES_STATE_PRE_GAME,
            DOTA_GameState::DOTA_GAMERULES_STATE_GAME_IN_PROGRESS,
            DOTA_GameState::DOTA_GAMERULES_STATE_POST_GAME,
            DOTA_GameState::DOTA_GAMERULES_STATE_DISCONNECT,
            DOTA_GameState::DOTA_GAMERULES_STATE_TEAM_SHOWCASE,
            DOTA_GameState::DOTA_GAMERULES_STATE_CUSTOM_GAME_SETUP,
            DOTA_GameState::DOTA_GAMERULES_STATE_WAIT_FOR_MAP_TO_LOAD,
            DOTA_GameState::DOTA_GAMERULES_STATE_LAST,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTA_GameState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_GameState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_GameState {
}

impl ::protobuf::reflect::ProtobufValue for DOTA_GameState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_GC_TEAM {
    DOTA_GC_TEAM_GOOD_GUYS = 0,
    DOTA_GC_TEAM_BAD_GUYS = 1,
    DOTA_GC_TEAM_BROADCASTER = 2,
    DOTA_GC_TEAM_SPECTATOR = 3,
    DOTA_GC_TEAM_PLAYER_POOL = 4,
    DOTA_GC_TEAM_NOTEAM = 5,
}

impl ::protobuf::ProtobufEnum for DOTA_GC_TEAM {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_GC_TEAM> {
        match value {
            0 => ::std::option::Option::Some(DOTA_GC_TEAM::DOTA_GC_TEAM_GOOD_GUYS),
            1 => ::std::option::Option::Some(DOTA_GC_TEAM::DOTA_GC_TEAM_BAD_GUYS),
            2 => ::std::option::Option::Some(DOTA_GC_TEAM::DOTA_GC_TEAM_BROADCASTER),
            3 => ::std::option::Option::Some(DOTA_GC_TEAM::DOTA_GC_TEAM_SPECTATOR),
            4 => ::std::option::Option::Some(DOTA_GC_TEAM::DOTA_GC_TEAM_PLAYER_POOL),
            5 => ::std::option::Option::Some(DOTA_GC_TEAM::DOTA_GC_TEAM_NOTEAM),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_GC_TEAM] = &[
            DOTA_GC_TEAM::DOTA_GC_TEAM_GOOD_GUYS,
            DOTA_GC_TEAM::DOTA_GC_TEAM_BAD_GUYS,
            DOTA_GC_TEAM::DOTA_GC_TEAM_BROADCASTER,
            DOTA_GC_TEAM::DOTA_GC_TEAM_SPECTATOR,
            DOTA_GC_TEAM::DOTA_GC_TEAM_PLAYER_POOL,
            DOTA_GC_TEAM::DOTA_GC_TEAM_NOTEAM,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTA_GC_TEAM>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_GC_TEAM", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_GC_TEAM {
}

impl ::protobuf::reflect::ProtobufValue for DOTA_GC_TEAM {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EEvent {
    EVENT_ID_NONE = 0,
    EVENT_ID_DIRETIDE = 1,
    EVENT_ID_SPRING_FESTIVAL = 2,
    EVENT_ID_FROSTIVUS_2013 = 3,
    EVENT_ID_COMPENDIUM_2014 = 4,
    EVENT_ID_NEXON_PC_BANG = 5,
    EVENT_ID_PWRD_DAC_2015 = 6,
    EVENT_ID_NEW_BLOOM_2015 = 7,
    EVENT_ID_INTERNATIONAL_2015 = 8,
    EVENT_ID_FALL_MAJOR_2015 = 9,
    EVENT_ID_ORACLE_PA = 10,
    EVENT_ID_NEW_BLOOM_2015_PREBEAST = 11,
    EVENT_ID_FROSTIVUS = 12,
    EVENT_ID_WINTER_MAJOR_2016 = 13,
    EVENT_ID_INTERNATIONAL_2016 = 14,
    EVENT_ID_FALL_MAJOR_2016 = 15,
    EVENT_ID_WINTER_MAJOR_2017 = 16,
    EVENT_ID_NEW_BLOOM_2017 = 17,
    EVENT_ID_INTERNATIONAL_2017 = 18,
    EVENT_ID_COUNT = 19,
}

impl ::protobuf::ProtobufEnum for EEvent {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EEvent> {
        match value {
            0 => ::std::option::Option::Some(EEvent::EVENT_ID_NONE),
            1 => ::std::option::Option::Some(EEvent::EVENT_ID_DIRETIDE),
            2 => ::std::option::Option::Some(EEvent::EVENT_ID_SPRING_FESTIVAL),
            3 => ::std::option::Option::Some(EEvent::EVENT_ID_FROSTIVUS_2013),
            4 => ::std::option::Option::Some(EEvent::EVENT_ID_COMPENDIUM_2014),
            5 => ::std::option::Option::Some(EEvent::EVENT_ID_NEXON_PC_BANG),
            6 => ::std::option::Option::Some(EEvent::EVENT_ID_PWRD_DAC_2015),
            7 => ::std::option::Option::Some(EEvent::EVENT_ID_NEW_BLOOM_2015),
            8 => ::std::option::Option::Some(EEvent::EVENT_ID_INTERNATIONAL_2015),
            9 => ::std::option::Option::Some(EEvent::EVENT_ID_FALL_MAJOR_2015),
            10 => ::std::option::Option::Some(EEvent::EVENT_ID_ORACLE_PA),
            11 => ::std::option::Option::Some(EEvent::EVENT_ID_NEW_BLOOM_2015_PREBEAST),
            12 => ::std::option::Option::Some(EEvent::EVENT_ID_FROSTIVUS),
            13 => ::std::option::Option::Some(EEvent::EVENT_ID_WINTER_MAJOR_2016),
            14 => ::std::option::Option::Some(EEvent::EVENT_ID_INTERNATIONAL_2016),
            15 => ::std::option::Option::Some(EEvent::EVENT_ID_FALL_MAJOR_2016),
            16 => ::std::option::Option::Some(EEvent::EVENT_ID_WINTER_MAJOR_2017),
            17 => ::std::option::Option::Some(EEvent::EVENT_ID_NEW_BLOOM_2017),
            18 => ::std::option::Option::Some(EEvent::EVENT_ID_INTERNATIONAL_2017),
            19 => ::std::option::Option::Some(EEvent::EVENT_ID_COUNT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EEvent] = &[
            EEvent::EVENT_ID_NONE,
            EEvent::EVENT_ID_DIRETIDE,
            EEvent::EVENT_ID_SPRING_FESTIVAL,
            EEvent::EVENT_ID_FROSTIVUS_2013,
            EEvent::EVENT_ID_COMPENDIUM_2014,
            EEvent::EVENT_ID_NEXON_PC_BANG,
            EEvent::EVENT_ID_PWRD_DAC_2015,
            EEvent::EVENT_ID_NEW_BLOOM_2015,
            EEvent::EVENT_ID_INTERNATIONAL_2015,
            EEvent::EVENT_ID_FALL_MAJOR_2015,
            EEvent::EVENT_ID_ORACLE_PA,
            EEvent::EVENT_ID_NEW_BLOOM_2015_PREBEAST,
            EEvent::EVENT_ID_FROSTIVUS,
            EEvent::EVENT_ID_WINTER_MAJOR_2016,
            EEvent::EVENT_ID_INTERNATIONAL_2016,
            EEvent::EVENT_ID_FALL_MAJOR_2016,
            EEvent::EVENT_ID_WINTER_MAJOR_2017,
            EEvent::EVENT_ID_NEW_BLOOM_2017,
            EEvent::EVENT_ID_INTERNATIONAL_2017,
            EEvent::EVENT_ID_COUNT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EEvent>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EEvent", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EEvent {
}

impl ::protobuf::reflect::ProtobufValue for EEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTALeaverStatus_t {
    DOTA_LEAVER_NONE = 0,
    DOTA_LEAVER_DISCONNECTED = 1,
    DOTA_LEAVER_DISCONNECTED_TOO_LONG = 2,
    DOTA_LEAVER_ABANDONED = 3,
    DOTA_LEAVER_AFK = 4,
    DOTA_LEAVER_NEVER_CONNECTED = 5,
    DOTA_LEAVER_NEVER_CONNECTED_TOO_LONG = 6,
    DOTA_LEAVER_FAILED_TO_READY_UP = 7,
    DOTA_LEAVER_DECLINED = 8,
}

impl ::protobuf::ProtobufEnum for DOTALeaverStatus_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTALeaverStatus_t> {
        match value {
            0 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_NONE),
            1 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_DISCONNECTED),
            2 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_DISCONNECTED_TOO_LONG),
            3 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_ABANDONED),
            4 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_AFK),
            5 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_NEVER_CONNECTED),
            6 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_NEVER_CONNECTED_TOO_LONG),
            7 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_FAILED_TO_READY_UP),
            8 => ::std::option::Option::Some(DOTALeaverStatus_t::DOTA_LEAVER_DECLINED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTALeaverStatus_t] = &[
            DOTALeaverStatus_t::DOTA_LEAVER_NONE,
            DOTALeaverStatus_t::DOTA_LEAVER_DISCONNECTED,
            DOTALeaverStatus_t::DOTA_LEAVER_DISCONNECTED_TOO_LONG,
            DOTALeaverStatus_t::DOTA_LEAVER_ABANDONED,
            DOTALeaverStatus_t::DOTA_LEAVER_AFK,
            DOTALeaverStatus_t::DOTA_LEAVER_NEVER_CONNECTED,
            DOTALeaverStatus_t::DOTA_LEAVER_NEVER_CONNECTED_TOO_LONG,
            DOTALeaverStatus_t::DOTA_LEAVER_FAILED_TO_READY_UP,
            DOTALeaverStatus_t::DOTA_LEAVER_DECLINED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTALeaverStatus_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTALeaverStatus_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTALeaverStatus_t {
}

impl ::protobuf::reflect::ProtobufValue for DOTALeaverStatus_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTAConnectionState_t {
    DOTA_CONNECTION_STATE_UNKNOWN = 0,
    DOTA_CONNECTION_STATE_NOT_YET_CONNECTED = 1,
    DOTA_CONNECTION_STATE_CONNECTED = 2,
    DOTA_CONNECTION_STATE_DISCONNECTED = 3,
    DOTA_CONNECTION_STATE_ABANDONED = 4,
    DOTA_CONNECTION_STATE_LOADING = 5,
    DOTA_CONNECTION_STATE_FAILED = 6,
}

impl ::protobuf::ProtobufEnum for DOTAConnectionState_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTAConnectionState_t> {
        match value {
            0 => ::std::option::Option::Some(DOTAConnectionState_t::DOTA_CONNECTION_STATE_UNKNOWN),
            1 => ::std::option::Option::Some(DOTAConnectionState_t::DOTA_CONNECTION_STATE_NOT_YET_CONNECTED),
            2 => ::std::option::Option::Some(DOTAConnectionState_t::DOTA_CONNECTION_STATE_CONNECTED),
            3 => ::std::option::Option::Some(DOTAConnectionState_t::DOTA_CONNECTION_STATE_DISCONNECTED),
            4 => ::std::option::Option::Some(DOTAConnectionState_t::DOTA_CONNECTION_STATE_ABANDONED),
            5 => ::std::option::Option::Some(DOTAConnectionState_t::DOTA_CONNECTION_STATE_LOADING),
            6 => ::std::option::Option::Some(DOTAConnectionState_t::DOTA_CONNECTION_STATE_FAILED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTAConnectionState_t] = &[
            DOTAConnectionState_t::DOTA_CONNECTION_STATE_UNKNOWN,
            DOTAConnectionState_t::DOTA_CONNECTION_STATE_NOT_YET_CONNECTED,
            DOTAConnectionState_t::DOTA_CONNECTION_STATE_CONNECTED,
            DOTAConnectionState_t::DOTA_CONNECTION_STATE_DISCONNECTED,
            DOTAConnectionState_t::DOTA_CONNECTION_STATE_ABANDONED,
            DOTAConnectionState_t::DOTA_CONNECTION_STATE_LOADING,
            DOTAConnectionState_t::DOTA_CONNECTION_STATE_FAILED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTAConnectionState_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTAConnectionState_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTAConnectionState_t {
}

impl ::protobuf::reflect::ProtobufValue for DOTAConnectionState_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Fantasy_Roles {
    FANTASY_ROLE_UNDEFINED = 0,
    FANTASY_ROLE_CORE = 1,
    FANTASY_ROLE_SUPPORT = 2,
    FANTASY_ROLE_OFFLANE = 3,
}

impl ::protobuf::ProtobufEnum for Fantasy_Roles {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Fantasy_Roles> {
        match value {
            0 => ::std::option::Option::Some(Fantasy_Roles::FANTASY_ROLE_UNDEFINED),
            1 => ::std::option::Option::Some(Fantasy_Roles::FANTASY_ROLE_CORE),
            2 => ::std::option::Option::Some(Fantasy_Roles::FANTASY_ROLE_SUPPORT),
            3 => ::std::option::Option::Some(Fantasy_Roles::FANTASY_ROLE_OFFLANE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Fantasy_Roles] = &[
            Fantasy_Roles::FANTASY_ROLE_UNDEFINED,
            Fantasy_Roles::FANTASY_ROLE_CORE,
            Fantasy_Roles::FANTASY_ROLE_SUPPORT,
            Fantasy_Roles::FANTASY_ROLE_OFFLANE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Fantasy_Roles>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Fantasy_Roles", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Fantasy_Roles {
}

impl ::protobuf::reflect::ProtobufValue for Fantasy_Roles {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Fantasy_Team_Slots {
    FANTASY_SLOT_NONE = 0,
    FANTASY_SLOT_CORE = 1,
    FANTASY_SLOT_SUPPORT = 2,
    FANTASY_SLOT_ANY = 3,
    FANTASY_SLOT_BENCH = 4,
}

impl ::protobuf::ProtobufEnum for Fantasy_Team_Slots {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Fantasy_Team_Slots> {
        match value {
            0 => ::std::option::Option::Some(Fantasy_Team_Slots::FANTASY_SLOT_NONE),
            1 => ::std::option::Option::Some(Fantasy_Team_Slots::FANTASY_SLOT_CORE),
            2 => ::std::option::Option::Some(Fantasy_Team_Slots::FANTASY_SLOT_SUPPORT),
            3 => ::std::option::Option::Some(Fantasy_Team_Slots::FANTASY_SLOT_ANY),
            4 => ::std::option::Option::Some(Fantasy_Team_Slots::FANTASY_SLOT_BENCH),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Fantasy_Team_Slots] = &[
            Fantasy_Team_Slots::FANTASY_SLOT_NONE,
            Fantasy_Team_Slots::FANTASY_SLOT_CORE,
            Fantasy_Team_Slots::FANTASY_SLOT_SUPPORT,
            Fantasy_Team_Slots::FANTASY_SLOT_ANY,
            Fantasy_Team_Slots::FANTASY_SLOT_BENCH,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Fantasy_Team_Slots>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Fantasy_Team_Slots", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Fantasy_Team_Slots {
}

impl ::protobuf::reflect::ProtobufValue for Fantasy_Team_Slots {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Fantasy_Selection_Mode {
    FANTASY_SELECTION_INVALID = 0,
    FANTASY_SELECTION_LOCKED = 1,
    FANTASY_SELECTION_SHUFFLE = 2,
    FANTASY_SELECTION_FREE_PICK = 3,
    FANTASY_SELECTION_ENDED = 4,
    FANTASY_SELECTION_PRE_SEASON = 5,
    FANTASY_SELECTION_PRE_DRAFT = 6,
    FANTASY_SELECTION_DRAFTING = 7,
    FANTASY_SELECTION_REGULAR_SEASON = 8,
    FANTASY_SELECTION_CARD_BASED = 9,
}

impl ::protobuf::ProtobufEnum for Fantasy_Selection_Mode {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Fantasy_Selection_Mode> {
        match value {
            0 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_INVALID),
            1 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_LOCKED),
            2 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_SHUFFLE),
            3 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_FREE_PICK),
            4 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_ENDED),
            5 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_PRE_SEASON),
            6 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_PRE_DRAFT),
            7 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_DRAFTING),
            8 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_REGULAR_SEASON),
            9 => ::std::option::Option::Some(Fantasy_Selection_Mode::FANTASY_SELECTION_CARD_BASED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Fantasy_Selection_Mode] = &[
            Fantasy_Selection_Mode::FANTASY_SELECTION_INVALID,
            Fantasy_Selection_Mode::FANTASY_SELECTION_LOCKED,
            Fantasy_Selection_Mode::FANTASY_SELECTION_SHUFFLE,
            Fantasy_Selection_Mode::FANTASY_SELECTION_FREE_PICK,
            Fantasy_Selection_Mode::FANTASY_SELECTION_ENDED,
            Fantasy_Selection_Mode::FANTASY_SELECTION_PRE_SEASON,
            Fantasy_Selection_Mode::FANTASY_SELECTION_PRE_DRAFT,
            Fantasy_Selection_Mode::FANTASY_SELECTION_DRAFTING,
            Fantasy_Selection_Mode::FANTASY_SELECTION_REGULAR_SEASON,
            Fantasy_Selection_Mode::FANTASY_SELECTION_CARD_BASED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Fantasy_Selection_Mode>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Fantasy_Selection_Mode", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Fantasy_Selection_Mode {
}

impl ::protobuf::reflect::ProtobufValue for Fantasy_Selection_Mode {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTAChatChannelType_t {
    DOTAChannelType_Regional = 0,
    DOTAChannelType_Custom = 1,
    DOTAChannelType_Party = 2,
    DOTAChannelType_Lobby = 3,
    DOTAChannelType_Team = 4,
    DOTAChannelType_Guild = 5,
    DOTAChannelType_Fantasy = 6,
    DOTAChannelType_Whisper = 7,
    DOTAChannelType_Console = 8,
    DOTAChannelType_Tab = 9,
    DOTAChannelType_Invalid = 10,
    DOTAChannelType_GameAll = 11,
    DOTAChannelType_GameAllies = 12,
    DOTAChannelType_GameSpectator = 13,
    DOTAChannelType_Cafe = 15,
    DOTAChannelType_CustomGame = 16,
    DOTAChannelType_Private = 17,
    DOTAChannelType_PostGame = 18,
    DOTAChannelType_BattleCup = 19,
    DOTAChannelType_HLTVSpectator = 20,
    DOTAChannelType_GameEvents = 21,
    DOTAChannelType_Trivia = 22,
}

impl ::protobuf::ProtobufEnum for DOTAChatChannelType_t {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTAChatChannelType_t> {
        match value {
            0 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Regional),
            1 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Custom),
            2 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Party),
            3 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Lobby),
            4 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Team),
            5 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Guild),
            6 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Fantasy),
            7 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Whisper),
            8 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Console),
            9 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Tab),
            10 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Invalid),
            11 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_GameAll),
            12 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_GameAllies),
            13 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_GameSpectator),
            15 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Cafe),
            16 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_CustomGame),
            17 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Private),
            18 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_PostGame),
            19 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_BattleCup),
            20 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_HLTVSpectator),
            21 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_GameEvents),
            22 => ::std::option::Option::Some(DOTAChatChannelType_t::DOTAChannelType_Trivia),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTAChatChannelType_t] = &[
            DOTAChatChannelType_t::DOTAChannelType_Regional,
            DOTAChatChannelType_t::DOTAChannelType_Custom,
            DOTAChatChannelType_t::DOTAChannelType_Party,
            DOTAChatChannelType_t::DOTAChannelType_Lobby,
            DOTAChatChannelType_t::DOTAChannelType_Team,
            DOTAChatChannelType_t::DOTAChannelType_Guild,
            DOTAChatChannelType_t::DOTAChannelType_Fantasy,
            DOTAChatChannelType_t::DOTAChannelType_Whisper,
            DOTAChatChannelType_t::DOTAChannelType_Console,
            DOTAChatChannelType_t::DOTAChannelType_Tab,
            DOTAChatChannelType_t::DOTAChannelType_Invalid,
            DOTAChatChannelType_t::DOTAChannelType_GameAll,
            DOTAChatChannelType_t::DOTAChannelType_GameAllies,
            DOTAChatChannelType_t::DOTAChannelType_GameSpectator,
            DOTAChatChannelType_t::DOTAChannelType_Cafe,
            DOTAChatChannelType_t::DOTAChannelType_CustomGame,
            DOTAChatChannelType_t::DOTAChannelType_Private,
            DOTAChatChannelType_t::DOTAChannelType_PostGame,
            DOTAChatChannelType_t::DOTAChannelType_BattleCup,
            DOTAChatChannelType_t::DOTAChannelType_HLTVSpectator,
            DOTAChatChannelType_t::DOTAChannelType_GameEvents,
            DOTAChatChannelType_t::DOTAChannelType_Trivia,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTAChatChannelType_t>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTAChatChannelType_t", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTAChatChannelType_t {
}

impl ::protobuf::reflect::ProtobufValue for DOTAChatChannelType_t {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EProfileCardSlotType {
    k_EProfileCardSlotType_Empty = 0,
    k_EProfileCardSlotType_Stat = 1,
    k_EProfileCardSlotType_Trophy = 2,
    k_EProfileCardSlotType_Item = 3,
    k_EProfileCardSlotType_Hero = 4,
    k_EProfileCardSlotType_Emoticon = 5,
    k_EProfileCardSlotType_Team = 6,
}

impl ::protobuf::ProtobufEnum for EProfileCardSlotType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EProfileCardSlotType> {
        match value {
            0 => ::std::option::Option::Some(EProfileCardSlotType::k_EProfileCardSlotType_Empty),
            1 => ::std::option::Option::Some(EProfileCardSlotType::k_EProfileCardSlotType_Stat),
            2 => ::std::option::Option::Some(EProfileCardSlotType::k_EProfileCardSlotType_Trophy),
            3 => ::std::option::Option::Some(EProfileCardSlotType::k_EProfileCardSlotType_Item),
            4 => ::std::option::Option::Some(EProfileCardSlotType::k_EProfileCardSlotType_Hero),
            5 => ::std::option::Option::Some(EProfileCardSlotType::k_EProfileCardSlotType_Emoticon),
            6 => ::std::option::Option::Some(EProfileCardSlotType::k_EProfileCardSlotType_Team),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EProfileCardSlotType] = &[
            EProfileCardSlotType::k_EProfileCardSlotType_Empty,
            EProfileCardSlotType::k_EProfileCardSlotType_Stat,
            EProfileCardSlotType::k_EProfileCardSlotType_Trophy,
            EProfileCardSlotType::k_EProfileCardSlotType_Item,
            EProfileCardSlotType::k_EProfileCardSlotType_Hero,
            EProfileCardSlotType::k_EProfileCardSlotType_Emoticon,
            EProfileCardSlotType::k_EProfileCardSlotType_Team,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EProfileCardSlotType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EProfileCardSlotType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EProfileCardSlotType {
}

impl ::protobuf::reflect::ProtobufValue for EProfileCardSlotType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EMatchGroupServerStatus {
    k_EMatchGroupServerStatus_OK = 0,
    k_EMatchGroupServerStatus_LimitedAvailability = 1,
    k_EMatchGroupServerStatus_Offline = 2,
}

impl ::protobuf::ProtobufEnum for EMatchGroupServerStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EMatchGroupServerStatus> {
        match value {
            0 => ::std::option::Option::Some(EMatchGroupServerStatus::k_EMatchGroupServerStatus_OK),
            1 => ::std::option::Option::Some(EMatchGroupServerStatus::k_EMatchGroupServerStatus_LimitedAvailability),
            2 => ::std::option::Option::Some(EMatchGroupServerStatus::k_EMatchGroupServerStatus_Offline),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EMatchGroupServerStatus] = &[
            EMatchGroupServerStatus::k_EMatchGroupServerStatus_OK,
            EMatchGroupServerStatus::k_EMatchGroupServerStatus_LimitedAvailability,
            EMatchGroupServerStatus::k_EMatchGroupServerStatus_Offline,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EMatchGroupServerStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EMatchGroupServerStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EMatchGroupServerStatus {
}

impl ::protobuf::reflect::ProtobufValue for EMatchGroupServerStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_CM_PICK {
    DOTA_CM_RANDOM = 0,
    DOTA_CM_GOOD_GUYS = 1,
    DOTA_CM_BAD_GUYS = 2,
}

impl ::protobuf::ProtobufEnum for DOTA_CM_PICK {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_CM_PICK> {
        match value {
            0 => ::std::option::Option::Some(DOTA_CM_PICK::DOTA_CM_RANDOM),
            1 => ::std::option::Option::Some(DOTA_CM_PICK::DOTA_CM_GOOD_GUYS),
            2 => ::std::option::Option::Some(DOTA_CM_PICK::DOTA_CM_BAD_GUYS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_CM_PICK] = &[
            DOTA_CM_PICK::DOTA_CM_RANDOM,
            DOTA_CM_PICK::DOTA_CM_GOOD_GUYS,
            DOTA_CM_PICK::DOTA_CM_BAD_GUYS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTA_CM_PICK>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_CM_PICK", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_CM_PICK {
}

impl ::protobuf::reflect::ProtobufValue for DOTA_CM_PICK {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTALowPriorityBanType {
    DOTA_LOW_PRIORITY_BAN_ABANDON = 0,
    DOTA_LOW_PRIORITY_BAN_REPORTS = 1,
    DOTA_LOW_PRIORITY_BAN_SECONDARY_ABANDON = 2,
}

impl ::protobuf::ProtobufEnum for DOTALowPriorityBanType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTALowPriorityBanType> {
        match value {
            0 => ::std::option::Option::Some(DOTALowPriorityBanType::DOTA_LOW_PRIORITY_BAN_ABANDON),
            1 => ::std::option::Option::Some(DOTALowPriorityBanType::DOTA_LOW_PRIORITY_BAN_REPORTS),
            2 => ::std::option::Option::Some(DOTALowPriorityBanType::DOTA_LOW_PRIORITY_BAN_SECONDARY_ABANDON),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTALowPriorityBanType] = &[
            DOTALowPriorityBanType::DOTA_LOW_PRIORITY_BAN_ABANDON,
            DOTALowPriorityBanType::DOTA_LOW_PRIORITY_BAN_REPORTS,
            DOTALowPriorityBanType::DOTA_LOW_PRIORITY_BAN_SECONDARY_ABANDON,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTALowPriorityBanType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTALowPriorityBanType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTALowPriorityBanType {
}

impl ::protobuf::reflect::ProtobufValue for DOTALowPriorityBanType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTALobbyReadyState {
    DOTALobbyReadyState_UNDECLARED = 0,
    DOTALobbyReadyState_ACCEPTED = 1,
    DOTALobbyReadyState_DECLINED = 2,
}

impl ::protobuf::ProtobufEnum for DOTALobbyReadyState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTALobbyReadyState> {
        match value {
            0 => ::std::option::Option::Some(DOTALobbyReadyState::DOTALobbyReadyState_UNDECLARED),
            1 => ::std::option::Option::Some(DOTALobbyReadyState::DOTALobbyReadyState_ACCEPTED),
            2 => ::std::option::Option::Some(DOTALobbyReadyState::DOTALobbyReadyState_DECLINED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTALobbyReadyState] = &[
            DOTALobbyReadyState::DOTALobbyReadyState_UNDECLARED,
            DOTALobbyReadyState::DOTALobbyReadyState_ACCEPTED,
            DOTALobbyReadyState::DOTALobbyReadyState_DECLINED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTALobbyReadyState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTALobbyReadyState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTALobbyReadyState {
}

impl ::protobuf::reflect::ProtobufValue for DOTALobbyReadyState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTAGameVersion {
    GAME_VERSION_CURRENT = 0,
    GAME_VERSION_STABLE = 1,
}

impl ::protobuf::ProtobufEnum for DOTAGameVersion {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTAGameVersion> {
        match value {
            0 => ::std::option::Option::Some(DOTAGameVersion::GAME_VERSION_CURRENT),
            1 => ::std::option::Option::Some(DOTAGameVersion::GAME_VERSION_STABLE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTAGameVersion] = &[
            DOTAGameVersion::GAME_VERSION_CURRENT,
            DOTAGameVersion::GAME_VERSION_STABLE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTAGameVersion>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTAGameVersion", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTAGameVersion {
}

impl ::protobuf::reflect::ProtobufValue for DOTAGameVersion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTAJoinLobbyResult {
    DOTA_JOIN_RESULT_SUCCESS = 0,
    DOTA_JOIN_RESULT_ALREADY_IN_GAME = 1,
    DOTA_JOIN_RESULT_INVALID_LOBBY = 2,
    DOTA_JOIN_RESULT_INCORRECT_PASSWORD = 3,
    DOTA_JOIN_RESULT_ACCESS_DENIED = 4,
    DOTA_JOIN_RESULT_GENERIC_ERROR = 5,
    DOTA_JOIN_RESULT_INCORRECT_VERSION = 6,
    DOTA_JOIN_RESULT_IN_TEAM_PARTY = 7,
    DOTA_JOIN_RESULT_NO_LOBBY_FOUND = 8,
    DOTA_JOIN_RESULT_LOBBY_FULL = 9,
    DOTA_JOIN_RESULT_CUSTOM_GAME_INCORRECT_VERSION = 10,
    DOTA_JOIN_RESULT_TIMEOUT = 11,
}

impl ::protobuf::ProtobufEnum for DOTAJoinLobbyResult {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTAJoinLobbyResult> {
        match value {
            0 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_SUCCESS),
            1 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_ALREADY_IN_GAME),
            2 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_INVALID_LOBBY),
            3 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_INCORRECT_PASSWORD),
            4 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_ACCESS_DENIED),
            5 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_GENERIC_ERROR),
            6 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_INCORRECT_VERSION),
            7 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_IN_TEAM_PARTY),
            8 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_NO_LOBBY_FOUND),
            9 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_LOBBY_FULL),
            10 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_CUSTOM_GAME_INCORRECT_VERSION),
            11 => ::std::option::Option::Some(DOTAJoinLobbyResult::DOTA_JOIN_RESULT_TIMEOUT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTAJoinLobbyResult] = &[
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_SUCCESS,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_ALREADY_IN_GAME,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_INVALID_LOBBY,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_INCORRECT_PASSWORD,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_ACCESS_DENIED,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_GENERIC_ERROR,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_INCORRECT_VERSION,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_IN_TEAM_PARTY,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_NO_LOBBY_FOUND,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_LOBBY_FULL,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_CUSTOM_GAME_INCORRECT_VERSION,
            DOTAJoinLobbyResult::DOTA_JOIN_RESULT_TIMEOUT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTAJoinLobbyResult>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTAJoinLobbyResult", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTAJoinLobbyResult {
}

impl ::protobuf::reflect::ProtobufValue for DOTAJoinLobbyResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTASelectionPriorityRules {
    k_DOTASelectionPriorityRules_Manual = 0,
    k_DOTASelectionPriorityRules_Automatic = 1,
}

impl ::protobuf::ProtobufEnum for DOTASelectionPriorityRules {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTASelectionPriorityRules> {
        match value {
            0 => ::std::option::Option::Some(DOTASelectionPriorityRules::k_DOTASelectionPriorityRules_Manual),
            1 => ::std::option::Option::Some(DOTASelectionPriorityRules::k_DOTASelectionPriorityRules_Automatic),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTASelectionPriorityRules] = &[
            DOTASelectionPriorityRules::k_DOTASelectionPriorityRules_Manual,
            DOTASelectionPriorityRules::k_DOTASelectionPriorityRules_Automatic,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTASelectionPriorityRules>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTASelectionPriorityRules", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTASelectionPriorityRules {
}

impl ::protobuf::reflect::ProtobufValue for DOTASelectionPriorityRules {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTASelectionPriorityChoice {
    k_DOTASelectionPriorityChoice_Invalid = 0,
    k_DOTASelectionPriorityChoice_FirstPick = 1,
    k_DOTASelectionPriorityChoice_SecondPick = 2,
    k_DOTASelectionPriorityChoice_Radiant = 3,
    k_DOTASelectionPriorityChoice_Dire = 4,
}

impl ::protobuf::ProtobufEnum for DOTASelectionPriorityChoice {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTASelectionPriorityChoice> {
        match value {
            0 => ::std::option::Option::Some(DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_Invalid),
            1 => ::std::option::Option::Some(DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_FirstPick),
            2 => ::std::option::Option::Some(DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_SecondPick),
            3 => ::std::option::Option::Some(DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_Radiant),
            4 => ::std::option::Option::Some(DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_Dire),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTASelectionPriorityChoice] = &[
            DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_Invalid,
            DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_FirstPick,
            DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_SecondPick,
            DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_Radiant,
            DOTASelectionPriorityChoice::k_DOTASelectionPriorityChoice_Dire,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTASelectionPriorityChoice>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTASelectionPriorityChoice", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTASelectionPriorityChoice {
}

impl ::protobuf::reflect::ProtobufValue for DOTASelectionPriorityChoice {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTAMatchVote {
    DOTAMatchVote_INVALID = 0,
    DOTAMatchVote_POSITIVE = 1,
    DOTAMatchVote_NEGATIVE = 2,
}

impl ::protobuf::ProtobufEnum for DOTAMatchVote {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTAMatchVote> {
        match value {
            0 => ::std::option::Option::Some(DOTAMatchVote::DOTAMatchVote_INVALID),
            1 => ::std::option::Option::Some(DOTAMatchVote::DOTAMatchVote_POSITIVE),
            2 => ::std::option::Option::Some(DOTAMatchVote::DOTAMatchVote_NEGATIVE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTAMatchVote] = &[
            DOTAMatchVote::DOTAMatchVote_INVALID,
            DOTAMatchVote::DOTAMatchVote_POSITIVE,
            DOTAMatchVote::DOTAMatchVote_NEGATIVE,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTAMatchVote>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTAMatchVote", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTAMatchVote {
}

impl ::protobuf::reflect::ProtobufValue for DOTAMatchVote {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_LobbyMemberXPBonus {
    DOTA_LobbyMemberXPBonus_DEFAULT = 0,
    DOTA_LobbyMemberXPBonus_BATTLE_BOOSTER = 1,
    DOTA_LobbyMemberXPBonus_SHARE_BONUS = 2,
    DOTA_LobbyMemberXPBonus_PARTY = 3,
    DOTA_LobbyMemberXPBonus_RECRUITMENT = 4,
    DOTA_LobbyMemberXPBonus_PCBANG = 5,
}

impl ::protobuf::ProtobufEnum for DOTA_LobbyMemberXPBonus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_LobbyMemberXPBonus> {
        match value {
            0 => ::std::option::Option::Some(DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_DEFAULT),
            1 => ::std::option::Option::Some(DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_BATTLE_BOOSTER),
            2 => ::std::option::Option::Some(DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_SHARE_BONUS),
            3 => ::std::option::Option::Some(DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_PARTY),
            4 => ::std::option::Option::Some(DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_RECRUITMENT),
            5 => ::std::option::Option::Some(DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_PCBANG),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_LobbyMemberXPBonus] = &[
            DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_DEFAULT,
            DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_BATTLE_BOOSTER,
            DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_SHARE_BONUS,
            DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_PARTY,
            DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_RECRUITMENT,
            DOTA_LobbyMemberXPBonus::DOTA_LobbyMemberXPBonus_PCBANG,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTA_LobbyMemberXPBonus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_LobbyMemberXPBonus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_LobbyMemberXPBonus {
}

impl ::protobuf::reflect::ProtobufValue for DOTA_LobbyMemberXPBonus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTALobbyVisibility {
    DOTALobbyVisibility_Public = 0,
    DOTALobbyVisibility_Friends = 1,
    DOTALobbyVisibility_Unlisted = 2,
}

impl ::protobuf::ProtobufEnum for DOTALobbyVisibility {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTALobbyVisibility> {
        match value {
            0 => ::std::option::Option::Some(DOTALobbyVisibility::DOTALobbyVisibility_Public),
            1 => ::std::option::Option::Some(DOTALobbyVisibility::DOTALobbyVisibility_Friends),
            2 => ::std::option::Option::Some(DOTALobbyVisibility::DOTALobbyVisibility_Unlisted),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTALobbyVisibility] = &[
            DOTALobbyVisibility::DOTALobbyVisibility_Public,
            DOTALobbyVisibility::DOTALobbyVisibility_Friends,
            DOTALobbyVisibility::DOTALobbyVisibility_Unlisted,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTALobbyVisibility>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTALobbyVisibility", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTALobbyVisibility {
}

impl ::protobuf::reflect::ProtobufValue for DOTALobbyVisibility {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EDOTAPlayerMMRType {
    k_EDOTAPlayerMMRType_Invalid = 0,
    k_EDOTAPlayerMMRType_GeneralHidden = 1,
    k_EDOTAPlayerMMRType_SoloHidden = 2,
    k_EDOTAPlayerMMRType_GeneralCompetitive = 3,
    k_EDOTAPlayerMMRType_SoloCompetitive = 4,
    k_EDOTAPlayerMMRType_1v1Competitive_UNUSED = 5,
    k_EDOTAPlayerMMRType_GeneralSeasonalRanked = 6,
    k_EDOTAPlayerMMRType_SoloSeasonalRanked = 7,
}

impl ::protobuf::ProtobufEnum for EDOTAPlayerMMRType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EDOTAPlayerMMRType> {
        match value {
            0 => ::std::option::Option::Some(EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_Invalid),
            1 => ::std::option::Option::Some(EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_GeneralHidden),
            2 => ::std::option::Option::Some(EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_SoloHidden),
            3 => ::std::option::Option::Some(EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_GeneralCompetitive),
            4 => ::std::option::Option::Some(EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_SoloCompetitive),
            5 => ::std::option::Option::Some(EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_1v1Competitive_UNUSED),
            6 => ::std::option::Option::Some(EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_GeneralSeasonalRanked),
            7 => ::std::option::Option::Some(EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_SoloSeasonalRanked),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EDOTAPlayerMMRType] = &[
            EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_Invalid,
            EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_GeneralHidden,
            EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_SoloHidden,
            EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_GeneralCompetitive,
            EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_SoloCompetitive,
            EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_1v1Competitive_UNUSED,
            EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_GeneralSeasonalRanked,
            EDOTAPlayerMMRType::k_EDOTAPlayerMMRType_SoloSeasonalRanked,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EDOTAPlayerMMRType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EDOTAPlayerMMRType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EDOTAPlayerMMRType {
}

impl ::protobuf::reflect::ProtobufValue for EDOTAPlayerMMRType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MatchType {
    MATCH_TYPE_CASUAL = 0,
    MATCH_TYPE_COOP_BOTS = 1,
    MATCH_TYPE_TEAM_RANKED = 2,
    MATCH_TYPE_LEGACY_SOLO_QUEUE = 3,
    MATCH_TYPE_COMPETITIVE = 4,
    MATCH_TYPE_WEEKEND_TOURNEY = 5,
    MATCH_TYPE_CASUAL_1V1 = 6,
    MATCH_TYPE_EVENT = 7,
    MATCH_TYPE_SEASONAL_RANKED = 8,
}

impl ::protobuf::ProtobufEnum for MatchType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MatchType> {
        match value {
            0 => ::std::option::Option::Some(MatchType::MATCH_TYPE_CASUAL),
            1 => ::std::option::Option::Some(MatchType::MATCH_TYPE_COOP_BOTS),
            2 => ::std::option::Option::Some(MatchType::MATCH_TYPE_TEAM_RANKED),
            3 => ::std::option::Option::Some(MatchType::MATCH_TYPE_LEGACY_SOLO_QUEUE),
            4 => ::std::option::Option::Some(MatchType::MATCH_TYPE_COMPETITIVE),
            5 => ::std::option::Option::Some(MatchType::MATCH_TYPE_WEEKEND_TOURNEY),
            6 => ::std::option::Option::Some(MatchType::MATCH_TYPE_CASUAL_1V1),
            7 => ::std::option::Option::Some(MatchType::MATCH_TYPE_EVENT),
            8 => ::std::option::Option::Some(MatchType::MATCH_TYPE_SEASONAL_RANKED),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MatchType] = &[
            MatchType::MATCH_TYPE_CASUAL,
            MatchType::MATCH_TYPE_COOP_BOTS,
            MatchType::MATCH_TYPE_TEAM_RANKED,
            MatchType::MATCH_TYPE_LEGACY_SOLO_QUEUE,
            MatchType::MATCH_TYPE_COMPETITIVE,
            MatchType::MATCH_TYPE_WEEKEND_TOURNEY,
            MatchType::MATCH_TYPE_CASUAL_1V1,
            MatchType::MATCH_TYPE_EVENT,
            MatchType::MATCH_TYPE_SEASONAL_RANKED,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MatchType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MatchType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MatchType {
}

impl ::protobuf::reflect::ProtobufValue for MatchType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTABotDifficulty {
    BOT_DIFFICULTY_PASSIVE = 0,
    BOT_DIFFICULTY_EASY = 1,
    BOT_DIFFICULTY_MEDIUM = 2,
    BOT_DIFFICULTY_HARD = 3,
    BOT_DIFFICULTY_UNFAIR = 4,
    BOT_DIFFICULTY_INVALID = 5,
    BOT_DIFFICULTY_EXTRA1 = 6,
    BOT_DIFFICULTY_EXTRA2 = 7,
    BOT_DIFFICULTY_EXTRA3 = 8,
}

impl ::protobuf::ProtobufEnum for DOTABotDifficulty {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTABotDifficulty> {
        match value {
            0 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_PASSIVE),
            1 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_EASY),
            2 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_MEDIUM),
            3 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_HARD),
            4 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_UNFAIR),
            5 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_INVALID),
            6 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_EXTRA1),
            7 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_EXTRA2),
            8 => ::std::option::Option::Some(DOTABotDifficulty::BOT_DIFFICULTY_EXTRA3),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTABotDifficulty] = &[
            DOTABotDifficulty::BOT_DIFFICULTY_PASSIVE,
            DOTABotDifficulty::BOT_DIFFICULTY_EASY,
            DOTABotDifficulty::BOT_DIFFICULTY_MEDIUM,
            DOTABotDifficulty::BOT_DIFFICULTY_HARD,
            DOTABotDifficulty::BOT_DIFFICULTY_UNFAIR,
            DOTABotDifficulty::BOT_DIFFICULTY_INVALID,
            DOTABotDifficulty::BOT_DIFFICULTY_EXTRA1,
            DOTABotDifficulty::BOT_DIFFICULTY_EXTRA2,
            DOTABotDifficulty::BOT_DIFFICULTY_EXTRA3,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTABotDifficulty>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTABotDifficulty", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTABotDifficulty {
}

impl ::protobuf::reflect::ProtobufValue for DOTABotDifficulty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DOTA_BOT_MODE {
    DOTA_BOT_MODE_NONE = 0,
    DOTA_BOT_MODE_LANING = 1,
    DOTA_BOT_MODE_ATTACK = 2,
    DOTA_BOT_MODE_ROAM = 3,
    DOTA_BOT_MODE_RETREAT = 4,
    DOTA_BOT_MODE_SECRET_SHOP = 5,
    DOTA_BOT_MODE_SIDE_SHOP = 6,
    DOTA_BOT_MODE_RUNE = 7,
    DOTA_BOT_MODE_PUSH_TOWER_TOP = 8,
    DOTA_BOT_MODE_PUSH_TOWER_MID = 9,
    DOTA_BOT_MODE_PUSH_TOWER_BOT = 10,
    DOTA_BOT_MODE_DEFEND_TOWER_TOP = 11,
    DOTA_BOT_MODE_DEFEND_TOWER_MID = 12,
    DOTA_BOT_MODE_DEFEND_TOWER_BOT = 13,
    DOTA_BOT_MODE_ASSEMBLE = 14,
    DOTA_BOT_MODE_ASSEMBLE_WITH_HUMANS = 15,
    DOTA_BOT_MODE_TEAM_ROAM = 16,
    DOTA_BOT_MODE_FARM = 17,
    DOTA_BOT_MODE_DEFEND_ALLY = 18,
    DOTA_BOT_MODE_EVASIVE_MANEUVERS = 19,
    DOTA_BOT_MODE_ROSHAN = 20,
    DOTA_BOT_MODE_ITEM = 21,
    DOTA_BOT_MODE_WARD = 22,
    DOTA_BOT_MODE_COMPANION = 23,
    DOTA_BOT_MODE_TUTORIAL_BOSS = 24,
    DOTA_BOT_MODE_MINION = 25,
}

impl ::protobuf::ProtobufEnum for DOTA_BOT_MODE {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DOTA_BOT_MODE> {
        match value {
            0 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_NONE),
            1 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_LANING),
            2 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_ATTACK),
            3 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_ROAM),
            4 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_RETREAT),
            5 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_SECRET_SHOP),
            6 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_SIDE_SHOP),
            7 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_RUNE),
            8 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_PUSH_TOWER_TOP),
            9 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_PUSH_TOWER_MID),
            10 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_PUSH_TOWER_BOT),
            11 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_DEFEND_TOWER_TOP),
            12 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_DEFEND_TOWER_MID),
            13 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_DEFEND_TOWER_BOT),
            14 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_ASSEMBLE),
            15 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_ASSEMBLE_WITH_HUMANS),
            16 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_TEAM_ROAM),
            17 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_FARM),
            18 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_DEFEND_ALLY),
            19 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_EVASIVE_MANEUVERS),
            20 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_ROSHAN),
            21 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_ITEM),
            22 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_WARD),
            23 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_COMPANION),
            24 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_TUTORIAL_BOSS),
            25 => ::std::option::Option::Some(DOTA_BOT_MODE::DOTA_BOT_MODE_MINION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DOTA_BOT_MODE] = &[
            DOTA_BOT_MODE::DOTA_BOT_MODE_NONE,
            DOTA_BOT_MODE::DOTA_BOT_MODE_LANING,
            DOTA_BOT_MODE::DOTA_BOT_MODE_ATTACK,
            DOTA_BOT_MODE::DOTA_BOT_MODE_ROAM,
            DOTA_BOT_MODE::DOTA_BOT_MODE_RETREAT,
            DOTA_BOT_MODE::DOTA_BOT_MODE_SECRET_SHOP,
            DOTA_BOT_MODE::DOTA_BOT_MODE_SIDE_SHOP,
            DOTA_BOT_MODE::DOTA_BOT_MODE_RUNE,
            DOTA_BOT_MODE::DOTA_BOT_MODE_PUSH_TOWER_TOP,
            DOTA_BOT_MODE::DOTA_BOT_MODE_PUSH_TOWER_MID,
            DOTA_BOT_MODE::DOTA_BOT_MODE_PUSH_TOWER_BOT,
            DOTA_BOT_MODE::DOTA_BOT_MODE_DEFEND_TOWER_TOP,
            DOTA_BOT_MODE::DOTA_BOT_MODE_DEFEND_TOWER_MID,
            DOTA_BOT_MODE::DOTA_BOT_MODE_DEFEND_TOWER_BOT,
            DOTA_BOT_MODE::DOTA_BOT_MODE_ASSEMBLE,
            DOTA_BOT_MODE::DOTA_BOT_MODE_ASSEMBLE_WITH_HUMANS,
            DOTA_BOT_MODE::DOTA_BOT_MODE_TEAM_ROAM,
            DOTA_BOT_MODE::DOTA_BOT_MODE_FARM,
            DOTA_BOT_MODE::DOTA_BOT_MODE_DEFEND_ALLY,
            DOTA_BOT_MODE::DOTA_BOT_MODE_EVASIVE_MANEUVERS,
            DOTA_BOT_MODE::DOTA_BOT_MODE_ROSHAN,
            DOTA_BOT_MODE::DOTA_BOT_MODE_ITEM,
            DOTA_BOT_MODE::DOTA_BOT_MODE_WARD,
            DOTA_BOT_MODE::DOTA_BOT_MODE_COMPANION,
            DOTA_BOT_MODE::DOTA_BOT_MODE_TUTORIAL_BOSS,
            DOTA_BOT_MODE::DOTA_BOT_MODE_MINION,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DOTA_BOT_MODE>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DOTA_BOT_MODE", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DOTA_BOT_MODE {
}

impl ::protobuf::reflect::ProtobufValue for DOTA_BOT_MODE {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MatchLanguages {
    MATCH_LANGUAGE_INVALID = 0,
    MATCH_LANGUAGE_ENGLISH = 1,
    MATCH_LANGUAGE_RUSSIAN = 2,
    MATCH_LANGUAGE_CHINESE = 3,
    MATCH_LANGUAGE_KOREAN = 4,
    MATCH_LANGUAGE_SPANISH = 5,
    MATCH_LANGUAGE_PORTUGUESE = 6,
    MATCH_LANGUAGE_ENGLISH2 = 7,
}

impl ::protobuf::ProtobufEnum for MatchLanguages {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MatchLanguages> {
        match value {
            0 => ::std::option::Option::Some(MatchLanguages::MATCH_LANGUAGE_INVALID),
            1 => ::std::option::Option::Some(MatchLanguages::MATCH_LANGUAGE_ENGLISH),
            2 => ::std::option::Option::Some(MatchLanguages::MATCH_LANGUAGE_RUSSIAN),
            3 => ::std::option::Option::Some(MatchLanguages::MATCH_LANGUAGE_CHINESE),
            4 => ::std::option::Option::Some(MatchLanguages::MATCH_LANGUAGE_KOREAN),
            5 => ::std::option::Option::Some(MatchLanguages::MATCH_LANGUAGE_SPANISH),
            6 => ::std::option::Option::Some(MatchLanguages::MATCH_LANGUAGE_PORTUGUESE),
            7 => ::std::option::Option::Some(MatchLanguages::MATCH_LANGUAGE_ENGLISH2),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MatchLanguages] = &[
            MatchLanguages::MATCH_LANGUAGE_INVALID,
            MatchLanguages::MATCH_LANGUAGE_ENGLISH,
            MatchLanguages::MATCH_LANGUAGE_RUSSIAN,
            MatchLanguages::MATCH_LANGUAGE_CHINESE,
            MatchLanguages::MATCH_LANGUAGE_KOREAN,
            MatchLanguages::MATCH_LANGUAGE_SPANISH,
            MatchLanguages::MATCH_LANGUAGE_PORTUGUESE,
            MatchLanguages::MATCH_LANGUAGE_ENGLISH2,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MatchLanguages>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MatchLanguages", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MatchLanguages {
}

impl ::protobuf::reflect::ProtobufValue for MatchLanguages {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ETourneyQueueDeadlineState {
    k_ETourneyQueueDeadlineState_Normal = 0,
    k_ETourneyQueueDeadlineState_Missed = 1,
    k_ETourneyQueueDeadlineState_ExpiredOK = 2,
    k_ETourneyQueueDeadlineState_SeekingBye = 3,
    k_ETourneyQueueDeadlineState_EligibleForRefund = 4,
    k_ETourneyQueueDeadlineState_NA = -1,
    k_ETourneyQueueDeadlineState_ExpiringSoon = 101,
}

impl ::protobuf::ProtobufEnum for ETourneyQueueDeadlineState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ETourneyQueueDeadlineState> {
        match value {
            0 => ::std::option::Option::Some(ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_Normal),
            1 => ::std::option::Option::Some(ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_Missed),
            2 => ::std::option::Option::Some(ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_ExpiredOK),
            3 => ::std::option::Option::Some(ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_SeekingBye),
            4 => ::std::option::Option::Some(ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_EligibleForRefund),
            -1 => ::std::option::Option::Some(ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_NA),
            101 => ::std::option::Option::Some(ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_ExpiringSoon),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ETourneyQueueDeadlineState] = &[
            ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_Normal,
            ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_Missed,
            ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_ExpiredOK,
            ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_SeekingBye,
            ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_EligibleForRefund,
            ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_NA,
            ETourneyQueueDeadlineState::k_ETourneyQueueDeadlineState_ExpiringSoon,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<ETourneyQueueDeadlineState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ETourneyQueueDeadlineState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ETourneyQueueDeadlineState {
}

impl ::protobuf::reflect::ProtobufValue for ETourneyQueueDeadlineState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EMatchOutcome {
    k_EMatchOutcome_Unknown = 0,
    k_EMatchOutcome_RadVictory = 2,
    k_EMatchOutcome_DireVictory = 3,
    k_EMatchOutcome_NotScored_PoorNetworkConditions = 64,
    k_EMatchOutcome_NotScored_Leaver = 65,
    k_EMatchOutcome_NotScored_ServerCrash = 66,
    k_EMatchOutcome_NotScored_NeverStarted = 67,
    k_EMatchOutcome_NotScored_Canceled = 68,
}

impl ::protobuf::ProtobufEnum for EMatchOutcome {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EMatchOutcome> {
        match value {
            0 => ::std::option::Option::Some(EMatchOutcome::k_EMatchOutcome_Unknown),
            2 => ::std::option::Option::Some(EMatchOutcome::k_EMatchOutcome_RadVictory),
            3 => ::std::option::Option::Some(EMatchOutcome::k_EMatchOutcome_DireVictory),
            64 => ::std::option::Option::Some(EMatchOutcome::k_EMatchOutcome_NotScored_PoorNetworkConditions),
            65 => ::std::option::Option::Some(EMatchOutcome::k_EMatchOutcome_NotScored_Leaver),
            66 => ::std::option::Option::Some(EMatchOutcome::k_EMatchOutcome_NotScored_ServerCrash),
            67 => ::std::option::Option::Some(EMatchOutcome::k_EMatchOutcome_NotScored_NeverStarted),
            68 => ::std::option::Option::Some(EMatchOutcome::k_EMatchOutcome_NotScored_Canceled),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EMatchOutcome] = &[
            EMatchOutcome::k_EMatchOutcome_Unknown,
            EMatchOutcome::k_EMatchOutcome_RadVictory,
            EMatchOutcome::k_EMatchOutcome_DireVictory,
            EMatchOutcome::k_EMatchOutcome_NotScored_PoorNetworkConditions,
            EMatchOutcome::k_EMatchOutcome_NotScored_Leaver,
            EMatchOutcome::k_EMatchOutcome_NotScored_ServerCrash,
            EMatchOutcome::k_EMatchOutcome_NotScored_NeverStarted,
            EMatchOutcome::k_EMatchOutcome_NotScored_Canceled,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EMatchOutcome>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EMatchOutcome", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EMatchOutcome {
}

impl ::protobuf::reflect::ProtobufValue for EMatchOutcome {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EBadgeType {
    k_EBadgeType_TI7_Midweek = 1,
    k_EBadgeType_TI7_Finals = 2,
    k_EBadgeType_TI7_AllEvent = 3,
}

impl ::protobuf::ProtobufEnum for EBadgeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EBadgeType> {
        match value {
            1 => ::std::option::Option::Some(EBadgeType::k_EBadgeType_TI7_Midweek),
            2 => ::std::option::Option::Some(EBadgeType::k_EBadgeType_TI7_Finals),
            3 => ::std::option::Option::Some(EBadgeType::k_EBadgeType_TI7_AllEvent),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [EBadgeType] = &[
            EBadgeType::k_EBadgeType_TI7_Midweek,
            EBadgeType::k_EBadgeType_TI7_Finals,
            EBadgeType::k_EBadgeType_TI7_AllEvent,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<EBadgeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EBadgeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EBadgeType {
}

impl ::protobuf::reflect::ProtobufValue for EBadgeType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17dota_shared_enums.proto\"\xa8\x02\n\x18CDOTAClientHardwareSpecs\
    \x12-\n\x12logical_processors\x18\x01\x20\x01(\rR\x11logicalProcessors\
    \x121\n\x15cpu_cycles_per_second\x18\x02\x20\x01(\x06R\x12cpuCyclesPerSe\
    cond\x122\n\x15total_physical_memory\x18\x03\x20\x01(\x06R\x13totalPhysi\
    calMemory\x12\x1f\n\x0cis_64_bit_os\x18\x04\x20\x01(\x08R\tis64BitOs\x12\
    -\n\x12upload_measurement\x18\x05\x20\x01(\x04R\x11uploadMeasurement\x12\
    &\n\x0fprefer_not_host\x18\x06\x20\x01(\x08R\rpreferNotHost\"\xd6\x04\n\
    \rCDOTASaveGame\x12\x19\n\x08match_id\x18\x05\x20\x01(\x04R\x07matchId\
    \x12\x1b\n\tsave_time\x18\x02\x20\x01(\rR\x08saveTime\x12/\n\x07players\
    \x18\x03\x20\x03(\x0b2\x15.CDOTASaveGame.PlayerR\x07players\x12B\n\x0esa\
    ve_instances\x18\x04\x20\x03(\x0b2\x1b.CDOTASaveGame.SaveInstanceR\rsave\
    Instances\x1ak\n\x06Player\x129\n\x04team\x18\x01\x20\x01(\x0e2\r.DOTA_G\
    C_TEAM:\x16DOTA_GC_TEAM_GOOD_GUYSR\x04team\x12\x12\n\x04name\x18\x02\x20\
    \x01(\tR\x04name\x12\x12\n\x04hero\x18\x03\x20\x01(\tR\x04hero\x1a\xaa\
    \x02\n\x0cSaveInstance\x12\x1b\n\tgame_time\x18\x02\x20\x01(\rR\x08gameT\
    ime\x12\x1f\n\x0bteam1_score\x18\x03\x20\x01(\rR\nteam1Score\x12\x1f\n\
    \x0bteam2_score\x18\x04\x20\x01(\rR\nteam2Score\x12V\n\x10player_positio\
    ns\x18\x05\x20\x03(\x0b2+.CDOTASaveGame.SaveInstance.PlayerPositionsR\
    \x0fplayerPositions\x12\x17\n\x07save_id\x18\x06\x20\x01(\rR\x06saveId\
    \x12\x1b\n\tsave_time\x18\x07\x20\x01(\rR\x08saveTime\x1a-\n\x0fPlayerPo\
    sitions\x12\x0c\n\x01x\x18\x01\x20\x01(\x02R\x01x\x12\x0c\n\x01y\x18\x02\
    \x20\x01(\x02R\x01y*\xc0\x04\n\rDOTA_GameMode\x12\x16\n\x12DOTA_GAMEMODE\
    _NONE\x10\0\x12\x14\n\x10DOTA_GAMEMODE_AP\x10\x01\x12\x14\n\x10DOTA_GAME\
    MODE_CM\x10\x02\x12\x14\n\x10DOTA_GAMEMODE_RD\x10\x03\x12\x14\n\x10DOTA_\
    GAMEMODE_SD\x10\x04\x12\x14\n\x10DOTA_GAMEMODE_AR\x10\x05\x12\x17\n\x13D\
    OTA_GAMEMODE_INTRO\x10\x06\x12\x14\n\x10DOTA_GAMEMODE_HW\x10\x07\x12\x1c\
    \n\x18DOTA_GAMEMODE_REVERSE_CM\x10\x08\x12\x16\n\x12DOTA_GAMEMODE_XMAS\
    \x10\t\x12\x1a\n\x16DOTA_GAMEMODE_TUTORIAL\x10\n\x12\x14\n\x10DOTA_GAMEM\
    ODE_MO\x10\x0b\x12\x14\n\x10DOTA_GAMEMODE_LP\x10\x0c\x12\x17\n\x13DOTA_G\
    AMEMODE_POOL1\x10\r\x12\x14\n\x10DOTA_GAMEMODE_FH\x10\x0e\x12\x18\n\x14D\
    OTA_GAMEMODE_CUSTOM\x10\x0f\x12\x14\n\x10DOTA_GAMEMODE_CD\x10\x10\x12\
    \x14\n\x10DOTA_GAMEMODE_BD\x10\x11\x12\x1f\n\x1bDOTA_GAMEMODE_ABILITY_DR\
    AFT\x10\x12\x12\x17\n\x13DOTA_GAMEMODE_EVENT\x10\x13\x12\x16\n\x12DOTA_G\
    AMEMODE_ARDM\x10\x14\x12\x18\n\x14DOTA_GAMEMODE_1V1MID\x10\x15\x12\x1b\n\
    \x17DOTA_GAMEMODE_ALL_DRAFT\x10\x16*\xec\x03\n\x0eDOTA_GameState\x12\x1d\
    \n\x19DOTA_GAMERULES_STATE_INIT\x10\0\x121\n-DOTA_GAMERULES_STATE_WAIT_F\
    OR_PLAYERS_TO_LOAD\x10\x01\x12'\n#DOTA_GAMERULES_STATE_HERO_SELECTION\
    \x10\x02\x12&\n\"DOTA_GAMERULES_STATE_STRATEGY_TIME\x10\x03\x12!\n\x1dDO\
    TA_GAMERULES_STATE_PRE_GAME\x10\x04\x12)\n%DOTA_GAMERULES_STATE_GAME_IN_\
    PROGRESS\x10\x05\x12\"\n\x1eDOTA_GAMERULES_STATE_POST_GAME\x10\x06\x12#\
    \n\x1fDOTA_GAMERULES_STATE_DISCONNECT\x10\x07\x12&\n\"DOTA_GAMERULES_STA\
    TE_TEAM_SHOWCASE\x10\x08\x12*\n&DOTA_GAMERULES_STATE_CUSTOM_GAME_SETUP\
    \x10\t\x12-\n)DOTA_GAMERULES_STATE_WAIT_FOR_MAP_TO_LOAD\x10\n\x12\x1d\n\
    \x19DOTA_GAMERULES_STATE_LAST\x10\x0b*\xb6\x01\n\x0cDOTA_GC_TEAM\x12\x1a\
    \n\x16DOTA_GC_TEAM_GOOD_GUYS\x10\0\x12\x19\n\x15DOTA_GC_TEAM_BAD_GUYS\
    \x10\x01\x12\x1c\n\x18DOTA_GC_TEAM_BROADCASTER\x10\x02\x12\x1a\n\x16DOTA\
    _GC_TEAM_SPECTATOR\x10\x03\x12\x1c\n\x18DOTA_GC_TEAM_PLAYER_POOL\x10\x04\
    \x12\x17\n\x13DOTA_GC_TEAM_NOTEAM\x10\x05*\xc6\x04\n\x06EEvent\x12\x11\n\
    \rEVENT_ID_NONE\x10\0\x12\x15\n\x11EVENT_ID_DIRETIDE\x10\x01\x12\x1c\n\
    \x18EVENT_ID_SPRING_FESTIVAL\x10\x02\x12\x1b\n\x17EVENT_ID_FROSTIVUS_201\
    3\x10\x03\x12\x1c\n\x18EVENT_ID_COMPENDIUM_2014\x10\x04\x12\x1a\n\x16EVE\
    NT_ID_NEXON_PC_BANG\x10\x05\x12\x1a\n\x16EVENT_ID_PWRD_DAC_2015\x10\x06\
    \x12\x1b\n\x17EVENT_ID_NEW_BLOOM_2015\x10\x07\x12\x1f\n\x1bEVENT_ID_INTE\
    RNATIONAL_2015\x10\x08\x12\x1c\n\x18EVENT_ID_FALL_MAJOR_2015\x10\t\x12\
    \x16\n\x12EVENT_ID_ORACLE_PA\x10\n\x12$\n\x20EVENT_ID_NEW_BLOOM_2015_PRE\
    BEAST\x10\x0b\x12\x16\n\x12EVENT_ID_FROSTIVUS\x10\x0c\x12\x1e\n\x1aEVENT\
    _ID_WINTER_MAJOR_2016\x10\r\x12\x1f\n\x1bEVENT_ID_INTERNATIONAL_2016\x10\
    \x0e\x12\x1c\n\x18EVENT_ID_FALL_MAJOR_2016\x10\x0f\x12\x1e\n\x1aEVENT_ID\
    _WINTER_MAJOR_2017\x10\x10\x12\x1b\n\x17EVENT_ID_NEW_BLOOM_2017\x10\x11\
    \x12\x1f\n\x1bEVENT_ID_INTERNATIONAL_2017\x10\x12\x12\x12\n\x0eEVENT_ID_\
    COUNT\x10\x13*\xa8\x02\n\x12DOTALeaverStatus_t\x12\x14\n\x10DOTA_LEAVER_\
    NONE\x10\0\x12\x1c\n\x18DOTA_LEAVER_DISCONNECTED\x10\x01\x12%\n!DOTA_LEA\
    VER_DISCONNECTED_TOO_LONG\x10\x02\x12\x19\n\x15DOTA_LEAVER_ABANDONED\x10\
    \x03\x12\x13\n\x0fDOTA_LEAVER_AFK\x10\x04\x12\x1f\n\x1bDOTA_LEAVER_NEVER\
    _CONNECTED\x10\x05\x12(\n$DOTA_LEAVER_NEVER_CONNECTED_TOO_LONG\x10\x06\
    \x12\"\n\x1eDOTA_LEAVER_FAILED_TO_READY_UP\x10\x07\x12\x18\n\x14DOTA_LEA\
    VER_DECLINED\x10\x08*\x9e\x02\n\x15DOTAConnectionState_t\x12!\n\x1dDOTA_\
    CONNECTION_STATE_UNKNOWN\x10\0\x12+\n'DOTA_CONNECTION_STATE_NOT_YET_CONN\
    ECTED\x10\x01\x12#\n\x1fDOTA_CONNECTION_STATE_CONNECTED\x10\x02\x12&\n\"\
    DOTA_CONNECTION_STATE_DISCONNECTED\x10\x03\x12#\n\x1fDOTA_CONNECTION_STA\
    TE_ABANDONED\x10\x04\x12!\n\x1dDOTA_CONNECTION_STATE_LOADING\x10\x05\x12\
    \x20\n\x1cDOTA_CONNECTION_STATE_FAILED\x10\x06*v\n\rFantasy_Roles\x12\
    \x1a\n\x16FANTASY_ROLE_UNDEFINED\x10\0\x12\x15\n\x11FANTASY_ROLE_CORE\
    \x10\x01\x12\x18\n\x14FANTASY_ROLE_SUPPORT\x10\x02\x12\x18\n\x14FANTASY_\
    ROLE_OFFLANE\x10\x03*\x8a\x01\n\x12Fantasy_Team_Slots\x12\x15\n\x11FANTA\
    SY_SLOT_NONE\x10\0\x12\x15\n\x11FANTASY_SLOT_CORE\x10\x01\x12\x18\n\x14F\
    ANTASY_SLOT_SUPPORT\x10\x02\x12\x14\n\x10FANTASY_SLOT_ANY\x10\x03\x12\
    \x16\n\x12FANTASY_SLOT_BENCH\x10\x04*\xdd\x02\n\x16Fantasy_Selection_Mod\
    e\x12\x1d\n\x19FANTASY_SELECTION_INVALID\x10\0\x12\x1c\n\x18FANTASY_SELE\
    CTION_LOCKED\x10\x01\x12\x1d\n\x19FANTASY_SELECTION_SHUFFLE\x10\x02\x12\
    \x1f\n\x1bFANTASY_SELECTION_FREE_PICK\x10\x03\x12\x1b\n\x17FANTASY_SELEC\
    TION_ENDED\x10\x04\x12\x20\n\x1cFANTASY_SELECTION_PRE_SEASON\x10\x05\x12\
    \x1f\n\x1bFANTASY_SELECTION_PRE_DRAFT\x10\x06\x12\x1e\n\x1aFANTASY_SELEC\
    TION_DRAFTING\x10\x07\x12$\n\x20FANTASY_SELECTION_REGULAR_SEASON\x10\x08\
    \x12\x20\n\x1cFANTASY_SELECTION_CARD_BASED\x10\t*\x9c\x05\n\x15DOTAChatC\
    hannelType_t\x12\x1c\n\x18DOTAChannelType_Regional\x10\0\x12\x1a\n\x16DO\
    TAChannelType_Custom\x10\x01\x12\x19\n\x15DOTAChannelType_Party\x10\x02\
    \x12\x19\n\x15DOTAChannelType_Lobby\x10\x03\x12\x18\n\x14DOTAChannelType\
    _Team\x10\x04\x12\x19\n\x15DOTAChannelType_Guild\x10\x05\x12\x1b\n\x17DO\
    TAChannelType_Fantasy\x10\x06\x12\x1b\n\x17DOTAChannelType_Whisper\x10\
    \x07\x12\x1b\n\x17DOTAChannelType_Console\x10\x08\x12\x17\n\x13DOTAChann\
    elType_Tab\x10\t\x12\x1b\n\x17DOTAChannelType_Invalid\x10\n\x12\x1b\n\
    \x17DOTAChannelType_GameAll\x10\x0b\x12\x1e\n\x1aDOTAChannelType_GameAll\
    ies\x10\x0c\x12!\n\x1dDOTAChannelType_GameSpectator\x10\r\x12\x18\n\x14D\
    OTAChannelType_Cafe\x10\x0f\x12\x1e\n\x1aDOTAChannelType_CustomGame\x10\
    \x10\x12\x1b\n\x17DOTAChannelType_Private\x10\x11\x12\x1c\n\x18DOTAChann\
    elType_PostGame\x10\x12\x12\x1d\n\x19DOTAChannelType_BattleCup\x10\x13\
    \x12!\n\x1dDOTAChannelType_HLTVSpectator\x10\x14\x12\x1e\n\x1aDOTAChanne\
    lType_GameEvents\x10\x15\x12\x1a\n\x16DOTAChannelType_Trivia\x10\x16*\
    \x84\x02\n\x14EProfileCardSlotType\x12\x20\n\x1ck_EProfileCardSlotType_E\
    mpty\x10\0\x12\x1f\n\x1bk_EProfileCardSlotType_Stat\x10\x01\x12!\n\x1dk_\
    EProfileCardSlotType_Trophy\x10\x02\x12\x1f\n\x1bk_EProfileCardSlotType_\
    Item\x10\x03\x12\x1f\n\x1bk_EProfileCardSlotType_Hero\x10\x04\x12#\n\x1f\
    k_EProfileCardSlotType_Emoticon\x10\x05\x12\x1f\n\x1bk_EProfileCardSlotT\
    ype_Team\x10\x06*\x95\x01\n\x17EMatchGroupServerStatus\x12\x20\n\x1ck_EM\
    atchGroupServerStatus_OK\x10\0\x121\n-k_EMatchGroupServerStatus_LimitedA\
    vailability\x10\x01\x12%\n!k_EMatchGroupServerStatus_Offline\x10\x02*O\n\
    \x0cDOTA_CM_PICK\x12\x12\n\x0eDOTA_CM_RANDOM\x10\0\x12\x15\n\x11DOTA_CM_\
    GOOD_GUYS\x10\x01\x12\x14\n\x10DOTA_CM_BAD_GUYS\x10\x02*\x8b\x01\n\x16DO\
    TALowPriorityBanType\x12!\n\x1dDOTA_LOW_PRIORITY_BAN_ABANDON\x10\0\x12!\
    \n\x1dDOTA_LOW_PRIORITY_BAN_REPORTS\x10\x01\x12+\n'DOTA_LOW_PRIORITY_BAN\
    _SECONDARY_ABANDON\x10\x02*}\n\x13DOTALobbyReadyState\x12\"\n\x1eDOTALob\
    byReadyState_UNDECLARED\x10\0\x12\x20\n\x1cDOTALobbyReadyState_ACCEPTED\
    \x10\x01\x12\x20\n\x1cDOTALobbyReadyState_DECLINED\x10\x02*D\n\x0fDOTAGa\
    meVersion\x12\x18\n\x14GAME_VERSION_CURRENT\x10\0\x12\x17\n\x13GAME_VERS\
    ION_STABLE\x10\x01*\xd2\x03\n\x13DOTAJoinLobbyResult\x12\x1c\n\x18DOTA_J\
    OIN_RESULT_SUCCESS\x10\0\x12$\n\x20DOTA_JOIN_RESULT_ALREADY_IN_GAME\x10\
    \x01\x12\"\n\x1eDOTA_JOIN_RESULT_INVALID_LOBBY\x10\x02\x12'\n#DOTA_JOIN_\
    RESULT_INCORRECT_PASSWORD\x10\x03\x12\"\n\x1eDOTA_JOIN_RESULT_ACCESS_DEN\
    IED\x10\x04\x12\"\n\x1eDOTA_JOIN_RESULT_GENERIC_ERROR\x10\x05\x12&\n\"DO\
    TA_JOIN_RESULT_INCORRECT_VERSION\x10\x06\x12\"\n\x1eDOTA_JOIN_RESULT_IN_\
    TEAM_PARTY\x10\x07\x12#\n\x1fDOTA_JOIN_RESULT_NO_LOBBY_FOUND\x10\x08\x12\
    \x1f\n\x1bDOTA_JOIN_RESULT_LOBBY_FULL\x10\t\x122\n.DOTA_JOIN_RESULT_CUST\
    OM_GAME_INCORRECT_VERSION\x10\n\x12\x1c\n\x18DOTA_JOIN_RESULT_TIMEOUT\
    \x10\x0b*q\n\x1aDOTASelectionPriorityRules\x12'\n#k_DOTASelectionPriorit\
    yRules_Manual\x10\0\x12*\n&k_DOTASelectionPriorityRules_Automatic\x10\
    \x01*\xf6\x01\n\x1bDOTASelectionPriorityChoice\x12)\n%k_DOTASelectionPri\
    orityChoice_Invalid\x10\0\x12+\n'k_DOTASelectionPriorityChoice_FirstPick\
    \x10\x01\x12,\n(k_DOTASelectionPriorityChoice_SecondPick\x10\x02\x12)\n%\
    k_DOTASelectionPriorityChoice_Radiant\x10\x03\x12&\n\"k_DOTASelectionPri\
    orityChoice_Dire\x10\x04*b\n\rDOTAMatchVote\x12\x19\n\x15DOTAMatchVote_I\
    NVALID\x10\0\x12\x1a\n\x16DOTAMatchVote_POSITIVE\x10\x01\x12\x1a\n\x16DO\
    TAMatchVote_NEGATIVE\x10\x02*\x83\x02\n\x17DOTA_LobbyMemberXPBonus\x12#\
    \n\x1fDOTA_LobbyMemberXPBonus_DEFAULT\x10\0\x12*\n&DOTA_LobbyMemberXPBon\
    us_BATTLE_BOOSTER\x10\x01\x12'\n#DOTA_LobbyMemberXPBonus_SHARE_BONUS\x10\
    \x02\x12!\n\x1dDOTA_LobbyMemberXPBonus_PARTY\x10\x03\x12'\n#DOTA_LobbyMe\
    mberXPBonus_RECRUITMENT\x10\x04\x12\"\n\x1eDOTA_LobbyMemberXPBonus_PCBAN\
    G\x10\x05*x\n\x13DOTALobbyVisibility\x12\x1e\n\x1aDOTALobbyVisibility_Pu\
    blic\x10\0\x12\x1f\n\x1bDOTALobbyVisibility_Friends\x10\x01\x12\x20\n\
    \x1cDOTALobbyVisibility_Unlisted\x10\x02*\xe7\x02\n\x12EDOTAPlayerMMRTyp\
    e\x12\x20\n\x1ck_EDOTAPlayerMMRType_Invalid\x10\0\x12&\n\"k_EDOTAPlayerM\
    MRType_GeneralHidden\x10\x01\x12#\n\x1fk_EDOTAPlayerMMRType_SoloHidden\
    \x10\x02\x12+\n'k_EDOTAPlayerMMRType_GeneralCompetitive\x10\x03\x12(\n$k\
    _EDOTAPlayerMMRType_SoloCompetitive\x10\x04\x12.\n*k_EDOTAPlayerMMRType_\
    1v1Competitive_UNUSED\x10\x05\x12.\n*k_EDOTAPlayerMMRType_GeneralSeasona\
    lRanked\x10\x06\x12+\n'k_EDOTAPlayerMMRType_SoloSeasonalRanked\x10\x07*\
    \x87\x02\n\tMatchType\x12\x15\n\x11MATCH_TYPE_CASUAL\x10\0\x12\x18\n\x14\
    MATCH_TYPE_COOP_BOTS\x10\x01\x12\x1a\n\x16MATCH_TYPE_TEAM_RANKED\x10\x02\
    \x12\x20\n\x1cMATCH_TYPE_LEGACY_SOLO_QUEUE\x10\x03\x12\x1a\n\x16MATCH_TY\
    PE_COMPETITIVE\x10\x04\x12\x1e\n\x1aMATCH_TYPE_WEEKEND_TOURNEY\x10\x05\
    \x12\x19\n\x15MATCH_TYPE_CASUAL_1V1\x10\x06\x12\x14\n\x10MATCH_TYPE_EVEN\
    T\x10\x07\x12\x1e\n\x1aMATCH_TYPE_SEASONAL_RANKED\x10\x08*\x84\x02\n\x11\
    DOTABotDifficulty\x12\x1a\n\x16BOT_DIFFICULTY_PASSIVE\x10\0\x12\x17\n\
    \x13BOT_DIFFICULTY_EASY\x10\x01\x12\x19\n\x15BOT_DIFFICULTY_MEDIUM\x10\
    \x02\x12\x17\n\x13BOT_DIFFICULTY_HARD\x10\x03\x12\x19\n\x15BOT_DIFFICULT\
    Y_UNFAIR\x10\x04\x12\x1a\n\x16BOT_DIFFICULTY_INVALID\x10\x05\x12\x19\n\
    \x15BOT_DIFFICULTY_EXTRA1\x10\x06\x12\x19\n\x15BOT_DIFFICULTY_EXTRA2\x10\
    \x07\x12\x19\n\x15BOT_DIFFICULTY_EXTRA3\x10\x08*\x93\x06\n\rDOTA_BOT_MOD\
    E\x12\x16\n\x12DOTA_BOT_MODE_NONE\x10\0\x12\x18\n\x14DOTA_BOT_MODE_LANIN\
    G\x10\x01\x12\x18\n\x14DOTA_BOT_MODE_ATTACK\x10\x02\x12\x16\n\x12DOTA_BO\
    T_MODE_ROAM\x10\x03\x12\x19\n\x15DOTA_BOT_MODE_RETREAT\x10\x04\x12\x1d\n\
    \x19DOTA_BOT_MODE_SECRET_SHOP\x10\x05\x12\x1b\n\x17DOTA_BOT_MODE_SIDE_SH\
    OP\x10\x06\x12\x16\n\x12DOTA_BOT_MODE_RUNE\x10\x07\x12\x20\n\x1cDOTA_BOT\
    _MODE_PUSH_TOWER_TOP\x10\x08\x12\x20\n\x1cDOTA_BOT_MODE_PUSH_TOWER_MID\
    \x10\t\x12\x20\n\x1cDOTA_BOT_MODE_PUSH_TOWER_BOT\x10\n\x12\"\n\x1eDOTA_B\
    OT_MODE_DEFEND_TOWER_TOP\x10\x0b\x12\"\n\x1eDOTA_BOT_MODE_DEFEND_TOWER_M\
    ID\x10\x0c\x12\"\n\x1eDOTA_BOT_MODE_DEFEND_TOWER_BOT\x10\r\x12\x1a\n\x16\
    DOTA_BOT_MODE_ASSEMBLE\x10\x0e\x12&\n\"DOTA_BOT_MODE_ASSEMBLE_WITH_HUMAN\
    S\x10\x0f\x12\x1b\n\x17DOTA_BOT_MODE_TEAM_ROAM\x10\x10\x12\x16\n\x12DOTA\
    _BOT_MODE_FARM\x10\x11\x12\x1d\n\x19DOTA_BOT_MODE_DEFEND_ALLY\x10\x12\
    \x12#\n\x1fDOTA_BOT_MODE_EVASIVE_MANEUVERS\x10\x13\x12\x18\n\x14DOTA_BOT\
    _MODE_ROSHAN\x10\x14\x12\x16\n\x12DOTA_BOT_MODE_ITEM\x10\x15\x12\x16\n\
    \x12DOTA_BOT_MODE_WARD\x10\x16\x12\x1b\n\x17DOTA_BOT_MODE_COMPANION\x10\
    \x17\x12\x1f\n\x1bDOTA_BOT_MODE_TUTORIAL_BOSS\x10\x18\x12\x18\n\x14DOTA_\
    BOT_MODE_MINION\x10\x19*\xf3\x01\n\x0eMatchLanguages\x12\x1a\n\x16MATCH_\
    LANGUAGE_INVALID\x10\0\x12\x1a\n\x16MATCH_LANGUAGE_ENGLISH\x10\x01\x12\
    \x1a\n\x16MATCH_LANGUAGE_RUSSIAN\x10\x02\x12\x1a\n\x16MATCH_LANGUAGE_CHI\
    NESE\x10\x03\x12\x19\n\x15MATCH_LANGUAGE_KOREAN\x10\x04\x12\x1a\n\x16MAT\
    CH_LANGUAGE_SPANISH\x10\x05\x12\x1d\n\x19MATCH_LANGUAGE_PORTUGUESE\x10\
    \x06\x12\x1b\n\x17MATCH_LANGUAGE_ENGLISH2\x10\x07*\xd8\x02\n\x1aETourney\
    QueueDeadlineState\x12'\n#k_ETourneyQueueDeadlineState_Normal\x10\0\x12'\
    \n#k_ETourneyQueueDeadlineState_Missed\x10\x01\x12*\n&k_ETourneyQueueDea\
    dlineState_ExpiredOK\x10\x02\x12+\n'k_ETourneyQueueDeadlineState_Seeking\
    Bye\x10\x03\x122\n.k_ETourneyQueueDeadlineState_EligibleForRefund\x10\
    \x04\x12,\n\x1fk_ETourneyQueueDeadlineState_NA\x10\xff\xff\xff\xff\xff\
    \xff\xff\xff\xff\x01\x12-\n)k_ETourneyQueueDeadlineState_ExpiringSoon\
    \x10e*\xc7\x02\n\rEMatchOutcome\x12\x1b\n\x17k_EMatchOutcome_Unknown\x10\
    \0\x12\x1e\n\x1ak_EMatchOutcome_RadVictory\x10\x02\x12\x1f\n\x1bk_EMatch\
    Outcome_DireVictory\x10\x03\x123\n/k_EMatchOutcome_NotScored_PoorNetwork\
    Conditions\x10@\x12$\n\x20k_EMatchOutcome_NotScored_Leaver\x10A\x12)\n%k\
    _EMatchOutcome_NotScored_ServerCrash\x10B\x12*\n&k_EMatchOutcome_NotScor\
    ed_NeverStarted\x10C\x12&\n\"k_EMatchOutcome_NotScored_Canceled\x10D*f\n\
    \nEBadgeType\x12\x1c\n\x18k_EBadgeType_TI7_Midweek\x10\x01\x12\x1b\n\x17\
    k_EBadgeType_TI7_Finals\x10\x02\x12\x1d\n\x19k_EBadgeType_TI7_AllEvent\
    \x10\x03B\x05H\x01\x80\x01\0\
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
