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
pub struct ChangeMapToolEvent {
    // message fields
    mapname: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChangeMapToolEvent {}

impl ChangeMapToolEvent {
    pub fn new() -> ChangeMapToolEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChangeMapToolEvent {
        static mut instance: ::protobuf::lazy::Lazy<ChangeMapToolEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChangeMapToolEvent,
        };
        unsafe {
            instance.get(ChangeMapToolEvent::new)
        }
    }

    // optional string mapname = 1;

    pub fn clear_mapname(&mut self) {
        self.mapname.clear();
    }

    pub fn has_mapname(&self) -> bool {
        self.mapname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mapname(&mut self, v: ::std::string::String) {
        self.mapname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mapname(&mut self) -> &mut ::std::string::String {
        if self.mapname.is_none() {
            self.mapname.set_default();
        }
        self.mapname.as_mut().unwrap()
    }

    // Take field
    pub fn take_mapname(&mut self) -> ::std::string::String {
        self.mapname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_mapname(&self) -> &str {
        match self.mapname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_mapname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.mapname
    }

    fn mut_mapname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.mapname
    }
}

impl ::protobuf::Message for ChangeMapToolEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.mapname)?;
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
        if let Some(ref v) = self.mapname.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.mapname.as_ref() {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for ChangeMapToolEvent {
    fn new() -> ChangeMapToolEvent {
        ChangeMapToolEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChangeMapToolEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "mapname",
                    ChangeMapToolEvent::get_mapname_for_reflect,
                    ChangeMapToolEvent::mut_mapname_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChangeMapToolEvent>(
                    "ChangeMapToolEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChangeMapToolEvent {
    fn clear(&mut self) {
        self.clear_mapname();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChangeMapToolEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangeMapToolEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TraceRayServerToolEvent {
    // message fields
    start: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    end: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TraceRayServerToolEvent {}

impl TraceRayServerToolEvent {
    pub fn new() -> TraceRayServerToolEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TraceRayServerToolEvent {
        static mut instance: ::protobuf::lazy::Lazy<TraceRayServerToolEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TraceRayServerToolEvent,
        };
        unsafe {
            instance.get(TraceRayServerToolEvent::new)
        }
    }

    // optional .CMsgVector start = 1;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.start = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.start.is_none() {
            self.start.set_default();
        }
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> super::networkbasetypes::CMsgVector {
        self.start.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_start(&self) -> &super::networkbasetypes::CMsgVector {
        self.start.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_start_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.start
    }

    // optional .CMsgVector end = 2;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.end = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.end.is_none() {
            self.end.set_default();
        }
        self.end.as_mut().unwrap()
    }

    // Take field
    pub fn take_end(&mut self) -> super::networkbasetypes::CMsgVector {
        self.end.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_end(&self) -> &super::networkbasetypes::CMsgVector {
        self.end.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_end_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.end
    }
}

impl ::protobuf::Message for TraceRayServerToolEvent {
    fn is_initialized(&self) -> bool {
        for v in &self.start {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.end {
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
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end)?;
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
        if let Some(ref v) = self.start.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.end.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.start.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.end.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for TraceRayServerToolEvent {
    fn new() -> TraceRayServerToolEvent {
        TraceRayServerToolEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<TraceRayServerToolEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "start",
                    TraceRayServerToolEvent::get_start_for_reflect,
                    TraceRayServerToolEvent::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "end",
                    TraceRayServerToolEvent::get_end_for_reflect,
                    TraceRayServerToolEvent::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TraceRayServerToolEvent>(
                    "TraceRayServerToolEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TraceRayServerToolEvent {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TraceRayServerToolEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TraceRayServerToolEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolTraceRayResult {
    // message fields
    hit: ::std::option::Option<bool>,
    impact: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    normal: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    distance: ::std::option::Option<f32>,
    fraction: ::std::option::Option<f32>,
    ehandle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolTraceRayResult {}

impl ToolTraceRayResult {
    pub fn new() -> ToolTraceRayResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolTraceRayResult {
        static mut instance: ::protobuf::lazy::Lazy<ToolTraceRayResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolTraceRayResult,
        };
        unsafe {
            instance.get(ToolTraceRayResult::new)
        }
    }

    // optional bool hit = 1;

    pub fn clear_hit(&mut self) {
        self.hit = ::std::option::Option::None;
    }

    pub fn has_hit(&self) -> bool {
        self.hit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hit(&mut self, v: bool) {
        self.hit = ::std::option::Option::Some(v);
    }

    pub fn get_hit(&self) -> bool {
        self.hit.unwrap_or(false)
    }

    fn get_hit_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hit
    }

    fn mut_hit_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hit
    }

    // optional .CMsgVector impact = 2;

    pub fn clear_impact(&mut self) {
        self.impact.clear();
    }

    pub fn has_impact(&self) -> bool {
        self.impact.is_some()
    }

    // Param is passed by value, moved
    pub fn set_impact(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.impact = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_impact(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.impact.is_none() {
            self.impact.set_default();
        }
        self.impact.as_mut().unwrap()
    }

    // Take field
    pub fn take_impact(&mut self) -> super::networkbasetypes::CMsgVector {
        self.impact.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_impact(&self) -> &super::networkbasetypes::CMsgVector {
        self.impact.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_impact_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.impact
    }

    fn mut_impact_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.impact
    }

    // optional .CMsgVector normal = 3;

    pub fn clear_normal(&mut self) {
        self.normal.clear();
    }

    pub fn has_normal(&self) -> bool {
        self.normal.is_some()
    }

    // Param is passed by value, moved
    pub fn set_normal(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.normal = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_normal(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.normal.is_none() {
            self.normal.set_default();
        }
        self.normal.as_mut().unwrap()
    }

    // Take field
    pub fn take_normal(&mut self) -> super::networkbasetypes::CMsgVector {
        self.normal.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_normal(&self) -> &super::networkbasetypes::CMsgVector {
        self.normal.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_normal_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.normal
    }

    fn mut_normal_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.normal
    }

    // optional float distance = 4;

    pub fn clear_distance(&mut self) {
        self.distance = ::std::option::Option::None;
    }

    pub fn has_distance(&self) -> bool {
        self.distance.is_some()
    }

    // Param is passed by value, moved
    pub fn set_distance(&mut self, v: f32) {
        self.distance = ::std::option::Option::Some(v);
    }

    pub fn get_distance(&self) -> f32 {
        self.distance.unwrap_or(0.)
    }

    fn get_distance_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.distance
    }

    fn mut_distance_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.distance
    }

    // optional float fraction = 5;

    pub fn clear_fraction(&mut self) {
        self.fraction = ::std::option::Option::None;
    }

    pub fn has_fraction(&self) -> bool {
        self.fraction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fraction(&mut self, v: f32) {
        self.fraction = ::std::option::Option::Some(v);
    }

    pub fn get_fraction(&self) -> f32 {
        self.fraction.unwrap_or(0.)
    }

    fn get_fraction_for_reflect(&self) -> &::std::option::Option<f32> {
        &self.fraction
    }

    fn mut_fraction_for_reflect(&mut self) -> &mut ::std::option::Option<f32> {
        &mut self.fraction
    }

    // optional int32 ehandle = 6;

    pub fn clear_ehandle(&mut self) {
        self.ehandle = ::std::option::Option::None;
    }

    pub fn has_ehandle(&self) -> bool {
        self.ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ehandle(&mut self, v: i32) {
        self.ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_ehandle(&self) -> i32 {
        self.ehandle.unwrap_or(0)
    }

    fn get_ehandle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ehandle
    }

    fn mut_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ehandle
    }
}

impl ::protobuf::Message for ToolTraceRayResult {
    fn is_initialized(&self) -> bool {
        for v in &self.impact {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.normal {
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
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hit = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.impact)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.normal)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.distance = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_float()?;
                    self.fraction = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.ehandle = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.hit {
            my_size += 2;
        }
        if let Some(ref v) = self.impact.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.normal.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.distance {
            my_size += 5;
        }
        if let Some(v) = self.fraction {
            my_size += 5;
        }
        if let Some(v) = self.ehandle {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hit {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.impact.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.normal.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.distance {
            os.write_float(4, v)?;
        }
        if let Some(v) = self.fraction {
            os.write_float(5, v)?;
        }
        if let Some(v) = self.ehandle {
            os.write_int32(6, v)?;
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

impl ::protobuf::MessageStatic for ToolTraceRayResult {
    fn new() -> ToolTraceRayResult {
        ToolTraceRayResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolTraceRayResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hit",
                    ToolTraceRayResult::get_hit_for_reflect,
                    ToolTraceRayResult::mut_hit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "impact",
                    ToolTraceRayResult::get_impact_for_reflect,
                    ToolTraceRayResult::mut_impact_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "normal",
                    ToolTraceRayResult::get_normal_for_reflect,
                    ToolTraceRayResult::mut_normal_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "distance",
                    ToolTraceRayResult::get_distance_for_reflect,
                    ToolTraceRayResult::mut_distance_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeFloat>(
                    "fraction",
                    ToolTraceRayResult::get_fraction_for_reflect,
                    ToolTraceRayResult::mut_fraction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ehandle",
                    ToolTraceRayResult::get_ehandle_for_reflect,
                    ToolTraceRayResult::mut_ehandle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolTraceRayResult>(
                    "ToolTraceRayResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolTraceRayResult {
    fn clear(&mut self) {
        self.clear_hit();
        self.clear_impact();
        self.clear_normal();
        self.clear_distance();
        self.clear_fraction();
        self.clear_ehandle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolTraceRayResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolTraceRayResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SpawnEntityToolEvent {
    // message fields
    entity_keyvalues: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    clientsideentity: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SpawnEntityToolEvent {}

impl SpawnEntityToolEvent {
    pub fn new() -> SpawnEntityToolEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpawnEntityToolEvent {
        static mut instance: ::protobuf::lazy::Lazy<SpawnEntityToolEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpawnEntityToolEvent,
        };
        unsafe {
            instance.get(SpawnEntityToolEvent::new)
        }
    }

    // optional bytes entity_keyvalues = 1;

    pub fn clear_entity_keyvalues(&mut self) {
        self.entity_keyvalues.clear();
    }

    pub fn has_entity_keyvalues(&self) -> bool {
        self.entity_keyvalues.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entity_keyvalues(&mut self, v: ::std::vec::Vec<u8>) {
        self.entity_keyvalues = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entity_keyvalues(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.entity_keyvalues.is_none() {
            self.entity_keyvalues.set_default();
        }
        self.entity_keyvalues.as_mut().unwrap()
    }

    // Take field
    pub fn take_entity_keyvalues(&mut self) -> ::std::vec::Vec<u8> {
        self.entity_keyvalues.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_entity_keyvalues(&self) -> &[u8] {
        match self.entity_keyvalues.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_entity_keyvalues_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.entity_keyvalues
    }

    fn mut_entity_keyvalues_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.entity_keyvalues
    }

    // optional bool clientsideentity = 2;

    pub fn clear_clientsideentity(&mut self) {
        self.clientsideentity = ::std::option::Option::None;
    }

    pub fn has_clientsideentity(&self) -> bool {
        self.clientsideentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientsideentity(&mut self, v: bool) {
        self.clientsideentity = ::std::option::Option::Some(v);
    }

    pub fn get_clientsideentity(&self) -> bool {
        self.clientsideentity.unwrap_or(false)
    }

    fn get_clientsideentity_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.clientsideentity
    }

    fn mut_clientsideentity_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.clientsideentity
    }
}

impl ::protobuf::Message for SpawnEntityToolEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.entity_keyvalues)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.clientsideentity = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.entity_keyvalues.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.clientsideentity {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.entity_keyvalues.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.clientsideentity {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for SpawnEntityToolEvent {
    fn new() -> SpawnEntityToolEvent {
        SpawnEntityToolEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpawnEntityToolEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "entity_keyvalues",
                    SpawnEntityToolEvent::get_entity_keyvalues_for_reflect,
                    SpawnEntityToolEvent::mut_entity_keyvalues_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "clientsideentity",
                    SpawnEntityToolEvent::get_clientsideentity_for_reflect,
                    SpawnEntityToolEvent::mut_clientsideentity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpawnEntityToolEvent>(
                    "SpawnEntityToolEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpawnEntityToolEvent {
    fn clear(&mut self) {
        self.clear_entity_keyvalues();
        self.clear_clientsideentity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SpawnEntityToolEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SpawnEntityToolEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SpawnEntityToolEventResult {
    // message fields
    ehandle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SpawnEntityToolEventResult {}

impl SpawnEntityToolEventResult {
    pub fn new() -> SpawnEntityToolEventResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SpawnEntityToolEventResult {
        static mut instance: ::protobuf::lazy::Lazy<SpawnEntityToolEventResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SpawnEntityToolEventResult,
        };
        unsafe {
            instance.get(SpawnEntityToolEventResult::new)
        }
    }

    // optional int32 ehandle = 1;

    pub fn clear_ehandle(&mut self) {
        self.ehandle = ::std::option::Option::None;
    }

    pub fn has_ehandle(&self) -> bool {
        self.ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ehandle(&mut self, v: i32) {
        self.ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_ehandle(&self) -> i32 {
        self.ehandle.unwrap_or(0)
    }

    fn get_ehandle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ehandle
    }

    fn mut_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ehandle
    }
}

impl ::protobuf::Message for SpawnEntityToolEventResult {
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
                    let tmp = is.read_int32()?;
                    self.ehandle = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ehandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ehandle {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for SpawnEntityToolEventResult {
    fn new() -> SpawnEntityToolEventResult {
        SpawnEntityToolEventResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<SpawnEntityToolEventResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ehandle",
                    SpawnEntityToolEventResult::get_ehandle_for_reflect,
                    SpawnEntityToolEventResult::mut_ehandle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SpawnEntityToolEventResult>(
                    "SpawnEntityToolEventResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SpawnEntityToolEventResult {
    fn clear(&mut self) {
        self.clear_ehandle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SpawnEntityToolEventResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SpawnEntityToolEventResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DestroyEntityToolEvent {
    // message fields
    ehandle: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DestroyEntityToolEvent {}

impl DestroyEntityToolEvent {
    pub fn new() -> DestroyEntityToolEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DestroyEntityToolEvent {
        static mut instance: ::protobuf::lazy::Lazy<DestroyEntityToolEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DestroyEntityToolEvent,
        };
        unsafe {
            instance.get(DestroyEntityToolEvent::new)
        }
    }

    // optional int32 ehandle = 1;

    pub fn clear_ehandle(&mut self) {
        self.ehandle = ::std::option::Option::None;
    }

    pub fn has_ehandle(&self) -> bool {
        self.ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ehandle(&mut self, v: i32) {
        self.ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_ehandle(&self) -> i32 {
        self.ehandle.unwrap_or(0)
    }

    fn get_ehandle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ehandle
    }

    fn mut_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ehandle
    }
}

impl ::protobuf::Message for DestroyEntityToolEvent {
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
                    let tmp = is.read_int32()?;
                    self.ehandle = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ehandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ehandle {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for DestroyEntityToolEvent {
    fn new() -> DestroyEntityToolEvent {
        DestroyEntityToolEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<DestroyEntityToolEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ehandle",
                    DestroyEntityToolEvent::get_ehandle_for_reflect,
                    DestroyEntityToolEvent::mut_ehandle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DestroyEntityToolEvent>(
                    "DestroyEntityToolEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DestroyEntityToolEvent {
    fn clear(&mut self) {
        self.clear_ehandle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DestroyEntityToolEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DestroyEntityToolEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DestroyAllEntitiesToolEvent {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DestroyAllEntitiesToolEvent {}

impl DestroyAllEntitiesToolEvent {
    pub fn new() -> DestroyAllEntitiesToolEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DestroyAllEntitiesToolEvent {
        static mut instance: ::protobuf::lazy::Lazy<DestroyAllEntitiesToolEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DestroyAllEntitiesToolEvent,
        };
        unsafe {
            instance.get(DestroyAllEntitiesToolEvent::new)
        }
    }
}

impl ::protobuf::Message for DestroyAllEntitiesToolEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for DestroyAllEntitiesToolEvent {
    fn new() -> DestroyAllEntitiesToolEvent {
        DestroyAllEntitiesToolEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<DestroyAllEntitiesToolEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DestroyAllEntitiesToolEvent>(
                    "DestroyAllEntitiesToolEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DestroyAllEntitiesToolEvent {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DestroyAllEntitiesToolEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DestroyAllEntitiesToolEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RestartMapToolEvent {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RestartMapToolEvent {}

impl RestartMapToolEvent {
    pub fn new() -> RestartMapToolEvent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RestartMapToolEvent {
        static mut instance: ::protobuf::lazy::Lazy<RestartMapToolEvent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RestartMapToolEvent,
        };
        unsafe {
            instance.get(RestartMapToolEvent::new)
        }
    }
}

impl ::protobuf::Message for RestartMapToolEvent {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for RestartMapToolEvent {
    fn new() -> RestartMapToolEvent {
        RestartMapToolEvent::new()
    }

    fn descriptor_static(_: ::std::option::Option<RestartMapToolEvent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RestartMapToolEvent>(
                    "RestartMapToolEvent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RestartMapToolEvent {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RestartMapToolEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RestartMapToolEvent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolEvent_GetEntityInfo {
    // message fields
    ehandle: ::std::option::Option<i32>,
    clientsideentity: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolEvent_GetEntityInfo {}

impl ToolEvent_GetEntityInfo {
    pub fn new() -> ToolEvent_GetEntityInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolEvent_GetEntityInfo {
        static mut instance: ::protobuf::lazy::Lazy<ToolEvent_GetEntityInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolEvent_GetEntityInfo,
        };
        unsafe {
            instance.get(ToolEvent_GetEntityInfo::new)
        }
    }

    // optional int32 ehandle = 1;

    pub fn clear_ehandle(&mut self) {
        self.ehandle = ::std::option::Option::None;
    }

    pub fn has_ehandle(&self) -> bool {
        self.ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ehandle(&mut self, v: i32) {
        self.ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_ehandle(&self) -> i32 {
        self.ehandle.unwrap_or(0)
    }

    fn get_ehandle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ehandle
    }

    fn mut_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ehandle
    }

    // optional bool clientsideentity = 2;

    pub fn clear_clientsideentity(&mut self) {
        self.clientsideentity = ::std::option::Option::None;
    }

    pub fn has_clientsideentity(&self) -> bool {
        self.clientsideentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientsideentity(&mut self, v: bool) {
        self.clientsideentity = ::std::option::Option::Some(v);
    }

    pub fn get_clientsideentity(&self) -> bool {
        self.clientsideentity.unwrap_or(false)
    }

    fn get_clientsideentity_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.clientsideentity
    }

    fn mut_clientsideentity_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.clientsideentity
    }
}

impl ::protobuf::Message for ToolEvent_GetEntityInfo {
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
                    let tmp = is.read_int32()?;
                    self.ehandle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.clientsideentity = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ehandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.clientsideentity {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ehandle {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.clientsideentity {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for ToolEvent_GetEntityInfo {
    fn new() -> ToolEvent_GetEntityInfo {
        ToolEvent_GetEntityInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolEvent_GetEntityInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ehandle",
                    ToolEvent_GetEntityInfo::get_ehandle_for_reflect,
                    ToolEvent_GetEntityInfo::mut_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "clientsideentity",
                    ToolEvent_GetEntityInfo::get_clientsideentity_for_reflect,
                    ToolEvent_GetEntityInfo::mut_clientsideentity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolEvent_GetEntityInfo>(
                    "ToolEvent_GetEntityInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolEvent_GetEntityInfo {
    fn clear(&mut self) {
        self.clear_ehandle();
        self.clear_clientsideentity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolEvent_GetEntityInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolEvent_GetEntityInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolEvent_GetEntityInfoResult {
    // message fields
    cppclass: ::protobuf::SingularField<::std::string::String>,
    classname: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    origin: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    mins: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    maxs: ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolEvent_GetEntityInfoResult {}

impl ToolEvent_GetEntityInfoResult {
    pub fn new() -> ToolEvent_GetEntityInfoResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolEvent_GetEntityInfoResult {
        static mut instance: ::protobuf::lazy::Lazy<ToolEvent_GetEntityInfoResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolEvent_GetEntityInfoResult,
        };
        unsafe {
            instance.get(ToolEvent_GetEntityInfoResult::new)
        }
    }

    // optional string cppclass = 1;

    pub fn clear_cppclass(&mut self) {
        self.cppclass.clear();
    }

    pub fn has_cppclass(&self) -> bool {
        self.cppclass.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cppclass(&mut self, v: ::std::string::String) {
        self.cppclass = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cppclass(&mut self) -> &mut ::std::string::String {
        if self.cppclass.is_none() {
            self.cppclass.set_default();
        }
        self.cppclass.as_mut().unwrap()
    }

    // Take field
    pub fn take_cppclass(&mut self) -> ::std::string::String {
        self.cppclass.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cppclass(&self) -> &str {
        match self.cppclass.as_ref() {
            Some(v) => &v,
            None => "shithead",
        }
    }

    fn get_cppclass_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cppclass
    }

    fn mut_cppclass_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cppclass
    }

    // optional string classname = 2;

    pub fn clear_classname(&mut self) {
        self.classname.clear();
    }

    pub fn has_classname(&self) -> bool {
        self.classname.is_some()
    }

    // Param is passed by value, moved
    pub fn set_classname(&mut self, v: ::std::string::String) {
        self.classname = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_classname(&mut self) -> &mut ::std::string::String {
        if self.classname.is_none() {
            self.classname.set_default();
        }
        self.classname.as_mut().unwrap()
    }

    // Take field
    pub fn take_classname(&mut self) -> ::std::string::String {
        self.classname.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_classname(&self) -> &str {
        match self.classname.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_classname_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.classname
    }

    fn mut_classname_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.classname
    }

    // optional string name = 3;

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

    // optional .CMsgVector origin = 4;

    pub fn clear_origin(&mut self) {
        self.origin.clear();
    }

    pub fn has_origin(&self) -> bool {
        self.origin.is_some()
    }

    // Param is passed by value, moved
    pub fn set_origin(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.origin = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_origin(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.origin.is_none() {
            self.origin.set_default();
        }
        self.origin.as_mut().unwrap()
    }

    // Take field
    pub fn take_origin(&mut self) -> super::networkbasetypes::CMsgVector {
        self.origin.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_origin(&self) -> &super::networkbasetypes::CMsgVector {
        self.origin.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_origin_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.origin
    }

    fn mut_origin_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.origin
    }

    // optional .CMsgVector mins = 5;

    pub fn clear_mins(&mut self) {
        self.mins.clear();
    }

    pub fn has_mins(&self) -> bool {
        self.mins.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mins(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.mins = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_mins(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.mins.is_none() {
            self.mins.set_default();
        }
        self.mins.as_mut().unwrap()
    }

    // Take field
    pub fn take_mins(&mut self) -> super::networkbasetypes::CMsgVector {
        self.mins.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_mins(&self) -> &super::networkbasetypes::CMsgVector {
        self.mins.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_mins_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.mins
    }

    fn mut_mins_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.mins
    }

    // optional .CMsgVector maxs = 6;

    pub fn clear_maxs(&mut self) {
        self.maxs.clear();
    }

    pub fn has_maxs(&self) -> bool {
        self.maxs.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxs(&mut self, v: super::networkbasetypes::CMsgVector) {
        self.maxs = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maxs(&mut self) -> &mut super::networkbasetypes::CMsgVector {
        if self.maxs.is_none() {
            self.maxs.set_default();
        }
        self.maxs.as_mut().unwrap()
    }

    // Take field
    pub fn take_maxs(&mut self) -> super::networkbasetypes::CMsgVector {
        self.maxs.take().unwrap_or_else(|| super::networkbasetypes::CMsgVector::new())
    }

    pub fn get_maxs(&self) -> &super::networkbasetypes::CMsgVector {
        self.maxs.as_ref().unwrap_or_else(|| super::networkbasetypes::CMsgVector::default_instance())
    }

    fn get_maxs_for_reflect(&self) -> &::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &self.maxs
    }

    fn mut_maxs_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::networkbasetypes::CMsgVector> {
        &mut self.maxs
    }
}

impl ::protobuf::Message for ToolEvent_GetEntityInfoResult {
    fn is_initialized(&self) -> bool {
        for v in &self.origin {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.mins {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.maxs {
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
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cppclass)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.classname)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.origin)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.mins)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.maxs)?;
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
        if let Some(ref v) = self.cppclass.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.classname.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.origin.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.mins.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.maxs.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.cppclass.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.classname.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.origin.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.mins.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.maxs.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for ToolEvent_GetEntityInfoResult {
    fn new() -> ToolEvent_GetEntityInfoResult {
        ToolEvent_GetEntityInfoResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolEvent_GetEntityInfoResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cppclass",
                    ToolEvent_GetEntityInfoResult::get_cppclass_for_reflect,
                    ToolEvent_GetEntityInfoResult::mut_cppclass_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "classname",
                    ToolEvent_GetEntityInfoResult::get_classname_for_reflect,
                    ToolEvent_GetEntityInfoResult::mut_classname_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ToolEvent_GetEntityInfoResult::get_name_for_reflect,
                    ToolEvent_GetEntityInfoResult::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "origin",
                    ToolEvent_GetEntityInfoResult::get_origin_for_reflect,
                    ToolEvent_GetEntityInfoResult::mut_origin_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "mins",
                    ToolEvent_GetEntityInfoResult::get_mins_for_reflect,
                    ToolEvent_GetEntityInfoResult::mut_mins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::networkbasetypes::CMsgVector>>(
                    "maxs",
                    ToolEvent_GetEntityInfoResult::get_maxs_for_reflect,
                    ToolEvent_GetEntityInfoResult::mut_maxs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolEvent_GetEntityInfoResult>(
                    "ToolEvent_GetEntityInfoResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolEvent_GetEntityInfoResult {
    fn clear(&mut self) {
        self.clear_cppclass();
        self.clear_classname();
        self.clear_name();
        self.clear_origin();
        self.clear_mins();
        self.clear_maxs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolEvent_GetEntityInfoResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolEvent_GetEntityInfoResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolEvent_GetEntityInputs {
    // message fields
    ehandle: ::std::option::Option<i32>,
    clientsideentity: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolEvent_GetEntityInputs {}

impl ToolEvent_GetEntityInputs {
    pub fn new() -> ToolEvent_GetEntityInputs {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolEvent_GetEntityInputs {
        static mut instance: ::protobuf::lazy::Lazy<ToolEvent_GetEntityInputs> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolEvent_GetEntityInputs,
        };
        unsafe {
            instance.get(ToolEvent_GetEntityInputs::new)
        }
    }

    // optional int32 ehandle = 1;

    pub fn clear_ehandle(&mut self) {
        self.ehandle = ::std::option::Option::None;
    }

    pub fn has_ehandle(&self) -> bool {
        self.ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ehandle(&mut self, v: i32) {
        self.ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_ehandle(&self) -> i32 {
        self.ehandle.unwrap_or(0)
    }

    fn get_ehandle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ehandle
    }

    fn mut_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ehandle
    }

    // optional bool clientsideentity = 2;

    pub fn clear_clientsideentity(&mut self) {
        self.clientsideentity = ::std::option::Option::None;
    }

    pub fn has_clientsideentity(&self) -> bool {
        self.clientsideentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientsideentity(&mut self, v: bool) {
        self.clientsideentity = ::std::option::Option::Some(v);
    }

    pub fn get_clientsideentity(&self) -> bool {
        self.clientsideentity.unwrap_or(false)
    }

    fn get_clientsideentity_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.clientsideentity
    }

    fn mut_clientsideentity_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.clientsideentity
    }
}

impl ::protobuf::Message for ToolEvent_GetEntityInputs {
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
                    let tmp = is.read_int32()?;
                    self.ehandle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.clientsideentity = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.ehandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.clientsideentity {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ehandle {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.clientsideentity {
            os.write_bool(2, v)?;
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

impl ::protobuf::MessageStatic for ToolEvent_GetEntityInputs {
    fn new() -> ToolEvent_GetEntityInputs {
        ToolEvent_GetEntityInputs::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolEvent_GetEntityInputs>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ehandle",
                    ToolEvent_GetEntityInputs::get_ehandle_for_reflect,
                    ToolEvent_GetEntityInputs::mut_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "clientsideentity",
                    ToolEvent_GetEntityInputs::get_clientsideentity_for_reflect,
                    ToolEvent_GetEntityInputs::mut_clientsideentity_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolEvent_GetEntityInputs>(
                    "ToolEvent_GetEntityInputs",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolEvent_GetEntityInputs {
    fn clear(&mut self) {
        self.clear_ehandle();
        self.clear_clientsideentity();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolEvent_GetEntityInputs {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolEvent_GetEntityInputs {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolEvent_GetEntityInputsResult {
    // message fields
    input_list: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolEvent_GetEntityInputsResult {}

impl ToolEvent_GetEntityInputsResult {
    pub fn new() -> ToolEvent_GetEntityInputsResult {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolEvent_GetEntityInputsResult {
        static mut instance: ::protobuf::lazy::Lazy<ToolEvent_GetEntityInputsResult> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolEvent_GetEntityInputsResult,
        };
        unsafe {
            instance.get(ToolEvent_GetEntityInputsResult::new)
        }
    }

    // repeated string input_list = 1;

    pub fn clear_input_list(&mut self) {
        self.input_list.clear();
    }

    // Param is passed by value, moved
    pub fn set_input_list(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.input_list = v;
    }

    // Mutable pointer to the field.
    pub fn mut_input_list(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.input_list
    }

    // Take field
    pub fn take_input_list(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.input_list, ::protobuf::RepeatedField::new())
    }

    pub fn get_input_list(&self) -> &[::std::string::String] {
        &self.input_list
    }

    fn get_input_list_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.input_list
    }

    fn mut_input_list_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.input_list
    }
}

impl ::protobuf::Message for ToolEvent_GetEntityInputsResult {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.input_list)?;
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
        for value in &self.input_list {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.input_list {
            os.write_string(1, &v)?;
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

impl ::protobuf::MessageStatic for ToolEvent_GetEntityInputsResult {
    fn new() -> ToolEvent_GetEntityInputsResult {
        ToolEvent_GetEntityInputsResult::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolEvent_GetEntityInputsResult>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "input_list",
                    ToolEvent_GetEntityInputsResult::get_input_list_for_reflect,
                    ToolEvent_GetEntityInputsResult::mut_input_list_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolEvent_GetEntityInputsResult>(
                    "ToolEvent_GetEntityInputsResult",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolEvent_GetEntityInputsResult {
    fn clear(&mut self) {
        self.clear_input_list();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolEvent_GetEntityInputsResult {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolEvent_GetEntityInputsResult {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolEvent_FireEntityInput {
    // message fields
    ehandle: ::std::option::Option<i32>,
    clientsideentity: ::std::option::Option<bool>,
    input_name: ::protobuf::SingularField<::std::string::String>,
    input_param: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolEvent_FireEntityInput {}

impl ToolEvent_FireEntityInput {
    pub fn new() -> ToolEvent_FireEntityInput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolEvent_FireEntityInput {
        static mut instance: ::protobuf::lazy::Lazy<ToolEvent_FireEntityInput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolEvent_FireEntityInput,
        };
        unsafe {
            instance.get(ToolEvent_FireEntityInput::new)
        }
    }

    // optional int32 ehandle = 1;

    pub fn clear_ehandle(&mut self) {
        self.ehandle = ::std::option::Option::None;
    }

    pub fn has_ehandle(&self) -> bool {
        self.ehandle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ehandle(&mut self, v: i32) {
        self.ehandle = ::std::option::Option::Some(v);
    }

    pub fn get_ehandle(&self) -> i32 {
        self.ehandle.unwrap_or(0)
    }

    fn get_ehandle_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.ehandle
    }

    fn mut_ehandle_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.ehandle
    }

    // optional bool clientsideentity = 2;

    pub fn clear_clientsideentity(&mut self) {
        self.clientsideentity = ::std::option::Option::None;
    }

    pub fn has_clientsideentity(&self) -> bool {
        self.clientsideentity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientsideentity(&mut self, v: bool) {
        self.clientsideentity = ::std::option::Option::Some(v);
    }

    pub fn get_clientsideentity(&self) -> bool {
        self.clientsideentity.unwrap_or(false)
    }

    fn get_clientsideentity_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.clientsideentity
    }

    fn mut_clientsideentity_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.clientsideentity
    }

    // optional string input_name = 3;

    pub fn clear_input_name(&mut self) {
        self.input_name.clear();
    }

    pub fn has_input_name(&self) -> bool {
        self.input_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_name(&mut self, v: ::std::string::String) {
        self.input_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input_name(&mut self) -> &mut ::std::string::String {
        if self.input_name.is_none() {
            self.input_name.set_default();
        }
        self.input_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_input_name(&mut self) -> ::std::string::String {
        self.input_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_input_name(&self) -> &str {
        match self.input_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_input_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.input_name
    }

    fn mut_input_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.input_name
    }

    // optional string input_param = 4;

    pub fn clear_input_param(&mut self) {
        self.input_param.clear();
    }

    pub fn has_input_param(&self) -> bool {
        self.input_param.is_some()
    }

    // Param is passed by value, moved
    pub fn set_input_param(&mut self, v: ::std::string::String) {
        self.input_param = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_input_param(&mut self) -> &mut ::std::string::String {
        if self.input_param.is_none() {
            self.input_param.set_default();
        }
        self.input_param.as_mut().unwrap()
    }

    // Take field
    pub fn take_input_param(&mut self) -> ::std::string::String {
        self.input_param.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_input_param(&self) -> &str {
        match self.input_param.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_input_param_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.input_param
    }

    fn mut_input_param_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.input_param
    }
}

impl ::protobuf::Message for ToolEvent_FireEntityInput {
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
                    let tmp = is.read_int32()?;
                    self.ehandle = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.clientsideentity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.input_name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.input_param)?;
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
        if let Some(v) = self.ehandle {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.clientsideentity {
            my_size += 2;
        }
        if let Some(ref v) = self.input_name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.input_param.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ehandle {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.clientsideentity {
            os.write_bool(2, v)?;
        }
        if let Some(ref v) = self.input_name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.input_param.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for ToolEvent_FireEntityInput {
    fn new() -> ToolEvent_FireEntityInput {
        ToolEvent_FireEntityInput::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolEvent_FireEntityInput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "ehandle",
                    ToolEvent_FireEntityInput::get_ehandle_for_reflect,
                    ToolEvent_FireEntityInput::mut_ehandle_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "clientsideentity",
                    ToolEvent_FireEntityInput::get_clientsideentity_for_reflect,
                    ToolEvent_FireEntityInput::mut_clientsideentity_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "input_name",
                    ToolEvent_FireEntityInput::get_input_name_for_reflect,
                    ToolEvent_FireEntityInput::mut_input_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "input_param",
                    ToolEvent_FireEntityInput::get_input_param_for_reflect,
                    ToolEvent_FireEntityInput::mut_input_param_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolEvent_FireEntityInput>(
                    "ToolEvent_FireEntityInput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolEvent_FireEntityInput {
    fn clear(&mut self) {
        self.clear_ehandle();
        self.clear_clientsideentity();
        self.clear_input_name();
        self.clear_input_param();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolEvent_FireEntityInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolEvent_FireEntityInput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolEvent_SFMRecordingStateChanged {
    // message fields
    isrecording: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolEvent_SFMRecordingStateChanged {}

impl ToolEvent_SFMRecordingStateChanged {
    pub fn new() -> ToolEvent_SFMRecordingStateChanged {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolEvent_SFMRecordingStateChanged {
        static mut instance: ::protobuf::lazy::Lazy<ToolEvent_SFMRecordingStateChanged> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolEvent_SFMRecordingStateChanged,
        };
        unsafe {
            instance.get(ToolEvent_SFMRecordingStateChanged::new)
        }
    }

    // optional bool isrecording = 1;

    pub fn clear_isrecording(&mut self) {
        self.isrecording = ::std::option::Option::None;
    }

    pub fn has_isrecording(&self) -> bool {
        self.isrecording.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isrecording(&mut self, v: bool) {
        self.isrecording = ::std::option::Option::Some(v);
    }

    pub fn get_isrecording(&self) -> bool {
        self.isrecording.unwrap_or(false)
    }

    fn get_isrecording_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.isrecording
    }

    fn mut_isrecording_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.isrecording
    }
}

impl ::protobuf::Message for ToolEvent_SFMRecordingStateChanged {
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
                    let tmp = is.read_bool()?;
                    self.isrecording = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.isrecording {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.isrecording {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for ToolEvent_SFMRecordingStateChanged {
    fn new() -> ToolEvent_SFMRecordingStateChanged {
        ToolEvent_SFMRecordingStateChanged::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolEvent_SFMRecordingStateChanged>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isrecording",
                    ToolEvent_SFMRecordingStateChanged::get_isrecording_for_reflect,
                    ToolEvent_SFMRecordingStateChanged::mut_isrecording_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolEvent_SFMRecordingStateChanged>(
                    "ToolEvent_SFMRecordingStateChanged",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolEvent_SFMRecordingStateChanged {
    fn clear(&mut self) {
        self.clear_isrecording();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolEvent_SFMRecordingStateChanged {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolEvent_SFMRecordingStateChanged {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ToolEvent_SFMToolActiveStateChanged {
    // message fields
    isactive: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ToolEvent_SFMToolActiveStateChanged {}

impl ToolEvent_SFMToolActiveStateChanged {
    pub fn new() -> ToolEvent_SFMToolActiveStateChanged {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ToolEvent_SFMToolActiveStateChanged {
        static mut instance: ::protobuf::lazy::Lazy<ToolEvent_SFMToolActiveStateChanged> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ToolEvent_SFMToolActiveStateChanged,
        };
        unsafe {
            instance.get(ToolEvent_SFMToolActiveStateChanged::new)
        }
    }

    // optional bool isactive = 1;

    pub fn clear_isactive(&mut self) {
        self.isactive = ::std::option::Option::None;
    }

    pub fn has_isactive(&self) -> bool {
        self.isactive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isactive(&mut self, v: bool) {
        self.isactive = ::std::option::Option::Some(v);
    }

    pub fn get_isactive(&self) -> bool {
        self.isactive.unwrap_or(false)
    }

    fn get_isactive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.isactive
    }

    fn mut_isactive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.isactive
    }
}

impl ::protobuf::Message for ToolEvent_SFMToolActiveStateChanged {
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
                    let tmp = is.read_bool()?;
                    self.isactive = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.isactive {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.isactive {
            os.write_bool(1, v)?;
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

impl ::protobuf::MessageStatic for ToolEvent_SFMToolActiveStateChanged {
    fn new() -> ToolEvent_SFMToolActiveStateChanged {
        ToolEvent_SFMToolActiveStateChanged::new()
    }

    fn descriptor_static(_: ::std::option::Option<ToolEvent_SFMToolActiveStateChanged>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "isactive",
                    ToolEvent_SFMToolActiveStateChanged::get_isactive_for_reflect,
                    ToolEvent_SFMToolActiveStateChanged::mut_isactive_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ToolEvent_SFMToolActiveStateChanged>(
                    "ToolEvent_SFMToolActiveStateChanged",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ToolEvent_SFMToolActiveStateChanged {
    fn clear(&mut self) {
        self.clear_isactive();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ToolEvent_SFMToolActiveStateChanged {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ToolEvent_SFMToolActiveStateChanged {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14gametoolevents.proto\x1a\x16networkbasetypes.proto\".\n\x12ChangeM\
    apToolEvent\x12\x18\n\x07mapname\x18\x01\x20\x01(\tR\x07mapname\"[\n\x17\
    TraceRayServerToolEvent\x12!\n\x05start\x18\x01\x20\x01(\x0b2\x0b.CMsgVe\
    ctorR\x05start\x12\x1d\n\x03end\x18\x02\x20\x01(\x0b2\x0b.CMsgVectorR\
    \x03end\"\xc2\x01\n\x12ToolTraceRayResult\x12\x10\n\x03hit\x18\x01\x20\
    \x01(\x08R\x03hit\x12#\n\x06impact\x18\x02\x20\x01(\x0b2\x0b.CMsgVectorR\
    \x06impact\x12#\n\x06normal\x18\x03\x20\x01(\x0b2\x0b.CMsgVectorR\x06nor\
    mal\x12\x1a\n\x08distance\x18\x04\x20\x01(\x02R\x08distance\x12\x1a\n\
    \x08fraction\x18\x05\x20\x01(\x02R\x08fraction\x12\x18\n\x07ehandle\x18\
    \x06\x20\x01(\x05R\x07ehandle\"m\n\x14SpawnEntityToolEvent\x12)\n\x10ent\
    ity_keyvalues\x18\x01\x20\x01(\x0cR\x0fentityKeyvalues\x12*\n\x10clients\
    ideentity\x18\x02\x20\x01(\x08R\x10clientsideentity\"6\n\x1aSpawnEntityT\
    oolEventResult\x12\x18\n\x07ehandle\x18\x01\x20\x01(\x05R\x07ehandle\"2\
    \n\x16DestroyEntityToolEvent\x12\x18\n\x07ehandle\x18\x01\x20\x01(\x05R\
    \x07ehandle\"\x1d\n\x1bDestroyAllEntitiesToolEvent\"\x15\n\x13RestartMap\
    ToolEvent\"_\n\x17ToolEvent_GetEntityInfo\x12\x18\n\x07ehandle\x18\x01\
    \x20\x01(\x05R\x07ehandle\x12*\n\x10clientsideentity\x18\x02\x20\x01(\
    \x08R\x10clientsideentity\"\xde\x01\n\x1dToolEvent_GetEntityInfoResult\
    \x12$\n\x08cppclass\x18\x01\x20\x01(\t:\x08shitheadR\x08cppclass\x12\x1c\
    \n\tclassname\x18\x02\x20\x01(\tR\tclassname\x12\x12\n\x04name\x18\x03\
    \x20\x01(\tR\x04name\x12#\n\x06origin\x18\x04\x20\x01(\x0b2\x0b.CMsgVect\
    orR\x06origin\x12\x1f\n\x04mins\x18\x05\x20\x01(\x0b2\x0b.CMsgVectorR\
    \x04mins\x12\x1f\n\x04maxs\x18\x06\x20\x01(\x0b2\x0b.CMsgVectorR\x04maxs\
    \"a\n\x19ToolEvent_GetEntityInputs\x12\x18\n\x07ehandle\x18\x01\x20\x01(\
    \x05R\x07ehandle\x12*\n\x10clientsideentity\x18\x02\x20\x01(\x08R\x10cli\
    entsideentity\"@\n\x1fToolEvent_GetEntityInputsResult\x12\x1d\n\ninput_l\
    ist\x18\x01\x20\x03(\tR\tinputList\"\xa1\x01\n\x19ToolEvent_FireEntityIn\
    put\x12\x18\n\x07ehandle\x18\x01\x20\x01(\x05R\x07ehandle\x12*\n\x10clie\
    ntsideentity\x18\x02\x20\x01(\x08R\x10clientsideentity\x12\x1d\n\ninput_\
    name\x18\x03\x20\x01(\tR\tinputName\x12\x1f\n\x0binput_param\x18\x04\x20\
    \x01(\tR\ninputParam\"F\n\"ToolEvent_SFMRecordingStateChanged\x12\x20\n\
    \x0bisrecording\x18\x01\x20\x01(\x08R\x0bisrecording\"A\n#ToolEvent_SFMT\
    oolActiveStateChanged\x12\x1a\n\x08isactive\x18\x01\x20\x01(\x08R\x08isa\
    ctiveB\x03\x80\x01\0\
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
