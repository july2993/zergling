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
pub struct Heartbeat {
    // message fields
    pub ip: ::std::string::String,
    pub port: u32,
    pub public_url: ::std::string::String,
    pub max_volume_count: u32,
    pub max_file_key: u64,
    pub data_center: ::std::string::String,
    pub rack: ::std::string::String,
    pub admin_port: u32,
    pub volumes: ::protobuf::RepeatedField<VolumeInformationMessage>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Heartbeat {}

impl Heartbeat {
    pub fn new() -> Heartbeat {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Heartbeat {
        static mut instance: ::protobuf::lazy::Lazy<Heartbeat> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Heartbeat,
        };
        unsafe {
            instance.get(Heartbeat::new)
        }
    }

    // string ip = 1;

    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // Take field
    pub fn take_ip(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.ip, ::std::string::String::new())
    }

    pub fn get_ip(&self) -> &str {
        &self.ip
    }

    fn get_ip_for_reflect(&self) -> &::std::string::String {
        &self.ip
    }

    fn mut_ip_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.ip
    }

    // uint32 port = 2;

    pub fn clear_port(&mut self) {
        self.port = 0;
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: u32) {
        self.port = v;
    }

    pub fn get_port(&self) -> u32 {
        self.port
    }

    fn get_port_for_reflect(&self) -> &u32 {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut u32 {
        &mut self.port
    }

    // string public_url = 3;

    pub fn clear_public_url(&mut self) {
        self.public_url.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_url(&mut self, v: ::std::string::String) {
        self.public_url = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_url(&mut self) -> &mut ::std::string::String {
        &mut self.public_url
    }

    // Take field
    pub fn take_public_url(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.public_url, ::std::string::String::new())
    }

    pub fn get_public_url(&self) -> &str {
        &self.public_url
    }

    fn get_public_url_for_reflect(&self) -> &::std::string::String {
        &self.public_url
    }

    fn mut_public_url_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.public_url
    }

    // uint32 max_volume_count = 4;

    pub fn clear_max_volume_count(&mut self) {
        self.max_volume_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_volume_count(&mut self, v: u32) {
        self.max_volume_count = v;
    }

    pub fn get_max_volume_count(&self) -> u32 {
        self.max_volume_count
    }

    fn get_max_volume_count_for_reflect(&self) -> &u32 {
        &self.max_volume_count
    }

    fn mut_max_volume_count_for_reflect(&mut self) -> &mut u32 {
        &mut self.max_volume_count
    }

    // uint64 max_file_key = 5;

    pub fn clear_max_file_key(&mut self) {
        self.max_file_key = 0;
    }

    // Param is passed by value, moved
    pub fn set_max_file_key(&mut self, v: u64) {
        self.max_file_key = v;
    }

    pub fn get_max_file_key(&self) -> u64 {
        self.max_file_key
    }

    fn get_max_file_key_for_reflect(&self) -> &u64 {
        &self.max_file_key
    }

    fn mut_max_file_key_for_reflect(&mut self) -> &mut u64 {
        &mut self.max_file_key
    }

    // string data_center = 6;

    pub fn clear_data_center(&mut self) {
        self.data_center.clear();
    }

    // Param is passed by value, moved
    pub fn set_data_center(&mut self, v: ::std::string::String) {
        self.data_center = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data_center(&mut self) -> &mut ::std::string::String {
        &mut self.data_center
    }

    // Take field
    pub fn take_data_center(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.data_center, ::std::string::String::new())
    }

    pub fn get_data_center(&self) -> &str {
        &self.data_center
    }

    fn get_data_center_for_reflect(&self) -> &::std::string::String {
        &self.data_center
    }

    fn mut_data_center_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.data_center
    }

    // string rack = 7;

    pub fn clear_rack(&mut self) {
        self.rack.clear();
    }

    // Param is passed by value, moved
    pub fn set_rack(&mut self, v: ::std::string::String) {
        self.rack = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rack(&mut self) -> &mut ::std::string::String {
        &mut self.rack
    }

    // Take field
    pub fn take_rack(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.rack, ::std::string::String::new())
    }

    pub fn get_rack(&self) -> &str {
        &self.rack
    }

    fn get_rack_for_reflect(&self) -> &::std::string::String {
        &self.rack
    }

    fn mut_rack_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.rack
    }

    // uint32 admin_port = 8;

    pub fn clear_admin_port(&mut self) {
        self.admin_port = 0;
    }

    // Param is passed by value, moved
    pub fn set_admin_port(&mut self, v: u32) {
        self.admin_port = v;
    }

    pub fn get_admin_port(&self) -> u32 {
        self.admin_port
    }

    fn get_admin_port_for_reflect(&self) -> &u32 {
        &self.admin_port
    }

    fn mut_admin_port_for_reflect(&mut self) -> &mut u32 {
        &mut self.admin_port
    }

    // repeated .pb.VolumeInformationMessage volumes = 9;

    pub fn clear_volumes(&mut self) {
        self.volumes.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumes(&mut self, v: ::protobuf::RepeatedField<VolumeInformationMessage>) {
        self.volumes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumes(&mut self) -> &mut ::protobuf::RepeatedField<VolumeInformationMessage> {
        &mut self.volumes
    }

    // Take field
    pub fn take_volumes(&mut self) -> ::protobuf::RepeatedField<VolumeInformationMessage> {
        ::std::mem::replace(&mut self.volumes, ::protobuf::RepeatedField::new())
    }

    pub fn get_volumes(&self) -> &[VolumeInformationMessage] {
        &self.volumes
    }

    fn get_volumes_for_reflect(&self) -> &::protobuf::RepeatedField<VolumeInformationMessage> {
        &self.volumes
    }

    fn mut_volumes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<VolumeInformationMessage> {
        &mut self.volumes
    }
}

impl ::protobuf::Message for Heartbeat {
    fn is_initialized(&self) -> bool {
        for v in &self.volumes {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.ip)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.port = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.public_url)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.max_volume_count = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.max_file_key = tmp;
                },
                6 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.data_center)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.rack)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.admin_port = tmp;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.volumes)?;
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
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.ip);
        }
        if self.port != 0 {
            my_size += ::protobuf::rt::value_size(2, self.port, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.public_url.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.public_url);
        }
        if self.max_volume_count != 0 {
            my_size += ::protobuf::rt::value_size(4, self.max_volume_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.max_file_key != 0 {
            my_size += ::protobuf::rt::value_size(5, self.max_file_key, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.data_center.is_empty() {
            my_size += ::protobuf::rt::string_size(6, &self.data_center);
        }
        if !self.rack.is_empty() {
            my_size += ::protobuf::rt::string_size(7, &self.rack);
        }
        if self.admin_port != 0 {
            my_size += ::protobuf::rt::value_size(8, self.admin_port, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.volumes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.ip.is_empty() {
            os.write_string(1, &self.ip)?;
        }
        if self.port != 0 {
            os.write_uint32(2, self.port)?;
        }
        if !self.public_url.is_empty() {
            os.write_string(3, &self.public_url)?;
        }
        if self.max_volume_count != 0 {
            os.write_uint32(4, self.max_volume_count)?;
        }
        if self.max_file_key != 0 {
            os.write_uint64(5, self.max_file_key)?;
        }
        if !self.data_center.is_empty() {
            os.write_string(6, &self.data_center)?;
        }
        if !self.rack.is_empty() {
            os.write_string(7, &self.rack)?;
        }
        if self.admin_port != 0 {
            os.write_uint32(8, self.admin_port)?;
        }
        for v in &self.volumes {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Heartbeat {
    fn new() -> Heartbeat {
        Heartbeat::new()
    }

    fn descriptor_static(_: ::std::option::Option<Heartbeat>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "ip",
                    Heartbeat::get_ip_for_reflect,
                    Heartbeat::mut_ip_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "port",
                    Heartbeat::get_port_for_reflect,
                    Heartbeat::mut_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "public_url",
                    Heartbeat::get_public_url_for_reflect,
                    Heartbeat::mut_public_url_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "max_volume_count",
                    Heartbeat::get_max_volume_count_for_reflect,
                    Heartbeat::mut_max_volume_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "max_file_key",
                    Heartbeat::get_max_file_key_for_reflect,
                    Heartbeat::mut_max_file_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "data_center",
                    Heartbeat::get_data_center_for_reflect,
                    Heartbeat::mut_data_center_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rack",
                    Heartbeat::get_rack_for_reflect,
                    Heartbeat::mut_rack_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "admin_port",
                    Heartbeat::get_admin_port_for_reflect,
                    Heartbeat::mut_admin_port_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VolumeInformationMessage>>(
                    "volumes",
                    Heartbeat::get_volumes_for_reflect,
                    Heartbeat::mut_volumes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Heartbeat>(
                    "Heartbeat",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Heartbeat {
    fn clear(&mut self) {
        self.clear_ip();
        self.clear_port();
        self.clear_public_url();
        self.clear_max_volume_count();
        self.clear_max_file_key();
        self.clear_data_center();
        self.clear_rack();
        self.clear_admin_port();
        self.clear_volumes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Heartbeat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Heartbeat {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HeartbeatResponse {
    // message fields
    pub volumeSizeLimit: u64,
    pub secretKey: ::std::string::String,
    pub leader: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HeartbeatResponse {}

impl HeartbeatResponse {
    pub fn new() -> HeartbeatResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HeartbeatResponse {
        static mut instance: ::protobuf::lazy::Lazy<HeartbeatResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HeartbeatResponse,
        };
        unsafe {
            instance.get(HeartbeatResponse::new)
        }
    }

    // uint64 volumeSizeLimit = 1;

    pub fn clear_volumeSizeLimit(&mut self) {
        self.volumeSizeLimit = 0;
    }

    // Param is passed by value, moved
    pub fn set_volumeSizeLimit(&mut self, v: u64) {
        self.volumeSizeLimit = v;
    }

    pub fn get_volumeSizeLimit(&self) -> u64 {
        self.volumeSizeLimit
    }

    fn get_volumeSizeLimit_for_reflect(&self) -> &u64 {
        &self.volumeSizeLimit
    }

    fn mut_volumeSizeLimit_for_reflect(&mut self) -> &mut u64 {
        &mut self.volumeSizeLimit
    }

    // string secretKey = 2;

    pub fn clear_secretKey(&mut self) {
        self.secretKey.clear();
    }

    // Param is passed by value, moved
    pub fn set_secretKey(&mut self, v: ::std::string::String) {
        self.secretKey = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secretKey(&mut self) -> &mut ::std::string::String {
        &mut self.secretKey
    }

    // Take field
    pub fn take_secretKey(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.secretKey, ::std::string::String::new())
    }

    pub fn get_secretKey(&self) -> &str {
        &self.secretKey
    }

    fn get_secretKey_for_reflect(&self) -> &::std::string::String {
        &self.secretKey
    }

    fn mut_secretKey_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.secretKey
    }

    // string leader = 3;

    pub fn clear_leader(&mut self) {
        self.leader.clear();
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: ::std::string::String) {
        self.leader = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader(&mut self) -> &mut ::std::string::String {
        &mut self.leader
    }

    // Take field
    pub fn take_leader(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.leader, ::std::string::String::new())
    }

    pub fn get_leader(&self) -> &str {
        &self.leader
    }

    fn get_leader_for_reflect(&self) -> &::std::string::String {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.leader
    }
}

impl ::protobuf::Message for HeartbeatResponse {
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
                    let tmp = is.read_uint64()?;
                    self.volumeSizeLimit = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.secretKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.leader)?;
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
        if self.volumeSizeLimit != 0 {
            my_size += ::protobuf::rt::value_size(1, self.volumeSizeLimit, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.secretKey.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.secretKey);
        }
        if !self.leader.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.leader);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.volumeSizeLimit != 0 {
            os.write_uint64(1, self.volumeSizeLimit)?;
        }
        if !self.secretKey.is_empty() {
            os.write_string(2, &self.secretKey)?;
        }
        if !self.leader.is_empty() {
            os.write_string(3, &self.leader)?;
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

impl ::protobuf::MessageStatic for HeartbeatResponse {
    fn new() -> HeartbeatResponse {
        HeartbeatResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<HeartbeatResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "volumeSizeLimit",
                    HeartbeatResponse::get_volumeSizeLimit_for_reflect,
                    HeartbeatResponse::mut_volumeSizeLimit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "secretKey",
                    HeartbeatResponse::get_secretKey_for_reflect,
                    HeartbeatResponse::mut_secretKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "leader",
                    HeartbeatResponse::get_leader_for_reflect,
                    HeartbeatResponse::mut_leader_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HeartbeatResponse>(
                    "HeartbeatResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HeartbeatResponse {
    fn clear(&mut self) {
        self.clear_volumeSizeLimit();
        self.clear_secretKey();
        self.clear_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HeartbeatResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HeartbeatResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VolumeInformationMessage {
    // message fields
    pub id: u32,
    pub size: u64,
    pub collection: ::std::string::String,
    pub file_count: u64,
    pub delete_count: u64,
    pub deleted_byte_count: u64,
    pub read_only: bool,
    pub replica_placement: u32,
    pub version: u32,
    pub ttl: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VolumeInformationMessage {}

impl VolumeInformationMessage {
    pub fn new() -> VolumeInformationMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VolumeInformationMessage {
        static mut instance: ::protobuf::lazy::Lazy<VolumeInformationMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VolumeInformationMessage,
        };
        unsafe {
            instance.get(VolumeInformationMessage::new)
        }
    }

    // uint32 id = 1;

    pub fn clear_id(&mut self) {
        self.id = 0;
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u32) {
        self.id = v;
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    fn get_id_for_reflect(&self) -> &u32 {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut u32 {
        &mut self.id
    }

    // uint64 size = 2;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = v;
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &u64 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut u64 {
        &mut self.size
    }

    // string collection = 3;

    pub fn clear_collection(&mut self) {
        self.collection.clear();
    }

    // Param is passed by value, moved
    pub fn set_collection(&mut self, v: ::std::string::String) {
        self.collection = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_collection(&mut self) -> &mut ::std::string::String {
        &mut self.collection
    }

    // Take field
    pub fn take_collection(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.collection, ::std::string::String::new())
    }

    pub fn get_collection(&self) -> &str {
        &self.collection
    }

    fn get_collection_for_reflect(&self) -> &::std::string::String {
        &self.collection
    }

    fn mut_collection_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.collection
    }

    // uint64 file_count = 4;

    pub fn clear_file_count(&mut self) {
        self.file_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_file_count(&mut self, v: u64) {
        self.file_count = v;
    }

    pub fn get_file_count(&self) -> u64 {
        self.file_count
    }

    fn get_file_count_for_reflect(&self) -> &u64 {
        &self.file_count
    }

    fn mut_file_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.file_count
    }

    // uint64 delete_count = 5;

    pub fn clear_delete_count(&mut self) {
        self.delete_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_delete_count(&mut self, v: u64) {
        self.delete_count = v;
    }

    pub fn get_delete_count(&self) -> u64 {
        self.delete_count
    }

    fn get_delete_count_for_reflect(&self) -> &u64 {
        &self.delete_count
    }

    fn mut_delete_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.delete_count
    }

    // uint64 deleted_byte_count = 6;

    pub fn clear_deleted_byte_count(&mut self) {
        self.deleted_byte_count = 0;
    }

    // Param is passed by value, moved
    pub fn set_deleted_byte_count(&mut self, v: u64) {
        self.deleted_byte_count = v;
    }

    pub fn get_deleted_byte_count(&self) -> u64 {
        self.deleted_byte_count
    }

    fn get_deleted_byte_count_for_reflect(&self) -> &u64 {
        &self.deleted_byte_count
    }

    fn mut_deleted_byte_count_for_reflect(&mut self) -> &mut u64 {
        &mut self.deleted_byte_count
    }

    // bool read_only = 7;

    pub fn clear_read_only(&mut self) {
        self.read_only = false;
    }

    // Param is passed by value, moved
    pub fn set_read_only(&mut self, v: bool) {
        self.read_only = v;
    }

    pub fn get_read_only(&self) -> bool {
        self.read_only
    }

    fn get_read_only_for_reflect(&self) -> &bool {
        &self.read_only
    }

    fn mut_read_only_for_reflect(&mut self) -> &mut bool {
        &mut self.read_only
    }

    // uint32 replica_placement = 8;

    pub fn clear_replica_placement(&mut self) {
        self.replica_placement = 0;
    }

    // Param is passed by value, moved
    pub fn set_replica_placement(&mut self, v: u32) {
        self.replica_placement = v;
    }

    pub fn get_replica_placement(&self) -> u32 {
        self.replica_placement
    }

    fn get_replica_placement_for_reflect(&self) -> &u32 {
        &self.replica_placement
    }

    fn mut_replica_placement_for_reflect(&mut self) -> &mut u32 {
        &mut self.replica_placement
    }

    // uint32 version = 9;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u32) {
        self.version = v;
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u32 {
        &mut self.version
    }

    // uint32 ttl = 10;

    pub fn clear_ttl(&mut self) {
        self.ttl = 0;
    }

    // Param is passed by value, moved
    pub fn set_ttl(&mut self, v: u32) {
        self.ttl = v;
    }

    pub fn get_ttl(&self) -> u32 {
        self.ttl
    }

    fn get_ttl_for_reflect(&self) -> &u32 {
        &self.ttl
    }

    fn mut_ttl_for_reflect(&mut self) -> &mut u32 {
        &mut self.ttl
    }
}

impl ::protobuf::Message for VolumeInformationMessage {
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
                    self.id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.size = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.collection)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.file_count = tmp;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.delete_count = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.deleted_byte_count = tmp;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.read_only = tmp;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.replica_placement = tmp;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.version = tmp;
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.ttl = tmp;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.collection.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.collection);
        }
        if self.file_count != 0 {
            my_size += ::protobuf::rt::value_size(4, self.file_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.delete_count != 0 {
            my_size += ::protobuf::rt::value_size(5, self.delete_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.deleted_byte_count != 0 {
            my_size += ::protobuf::rt::value_size(6, self.deleted_byte_count, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.read_only != false {
            my_size += 2;
        }
        if self.replica_placement != 0 {
            my_size += ::protobuf::rt::value_size(8, self.replica_placement, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(9, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.ttl != 0 {
            my_size += ::protobuf::rt::value_size(10, self.ttl, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.id != 0 {
            os.write_uint32(1, self.id)?;
        }
        if self.size != 0 {
            os.write_uint64(2, self.size)?;
        }
        if !self.collection.is_empty() {
            os.write_string(3, &self.collection)?;
        }
        if self.file_count != 0 {
            os.write_uint64(4, self.file_count)?;
        }
        if self.delete_count != 0 {
            os.write_uint64(5, self.delete_count)?;
        }
        if self.deleted_byte_count != 0 {
            os.write_uint64(6, self.deleted_byte_count)?;
        }
        if self.read_only != false {
            os.write_bool(7, self.read_only)?;
        }
        if self.replica_placement != 0 {
            os.write_uint32(8, self.replica_placement)?;
        }
        if self.version != 0 {
            os.write_uint32(9, self.version)?;
        }
        if self.ttl != 0 {
            os.write_uint32(10, self.ttl)?;
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

impl ::protobuf::MessageStatic for VolumeInformationMessage {
    fn new() -> VolumeInformationMessage {
        VolumeInformationMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<VolumeInformationMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "id",
                    VolumeInformationMessage::get_id_for_reflect,
                    VolumeInformationMessage::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    VolumeInformationMessage::get_size_for_reflect,
                    VolumeInformationMessage::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "collection",
                    VolumeInformationMessage::get_collection_for_reflect,
                    VolumeInformationMessage::mut_collection_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "file_count",
                    VolumeInformationMessage::get_file_count_for_reflect,
                    VolumeInformationMessage::mut_file_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "delete_count",
                    VolumeInformationMessage::get_delete_count_for_reflect,
                    VolumeInformationMessage::mut_delete_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "deleted_byte_count",
                    VolumeInformationMessage::get_deleted_byte_count_for_reflect,
                    VolumeInformationMessage::mut_deleted_byte_count_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "read_only",
                    VolumeInformationMessage::get_read_only_for_reflect,
                    VolumeInformationMessage::mut_read_only_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "replica_placement",
                    VolumeInformationMessage::get_replica_placement_for_reflect,
                    VolumeInformationMessage::mut_replica_placement_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "version",
                    VolumeInformationMessage::get_version_for_reflect,
                    VolumeInformationMessage::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "ttl",
                    VolumeInformationMessage::get_ttl_for_reflect,
                    VolumeInformationMessage::mut_ttl_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VolumeInformationMessage>(
                    "VolumeInformationMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VolumeInformationMessage {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_size();
        self.clear_collection();
        self.clear_file_count();
        self.clear_delete_count();
        self.clear_deleted_byte_count();
        self.clear_read_only();
        self.clear_replica_placement();
        self.clear_version();
        self.clear_ttl();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VolumeInformationMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VolumeInformationMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ezergling.proto\x12\x02pb\"\xa6\x02\n\tHeartbeat\x12\x0e\n\x02ip\
    \x18\x01\x20\x01(\tR\x02ip\x12\x12\n\x04port\x18\x02\x20\x01(\rR\x04port\
    \x12\x1d\n\npublic_url\x18\x03\x20\x01(\tR\tpublicUrl\x12(\n\x10max_volu\
    me_count\x18\x04\x20\x01(\rR\x0emaxVolumeCount\x12\x20\n\x0cmax_file_key\
    \x18\x05\x20\x01(\x04R\nmaxFileKey\x12\x1f\n\x0bdata_center\x18\x06\x20\
    \x01(\tR\ndataCenter\x12\x12\n\x04rack\x18\x07\x20\x01(\tR\x04rack\x12\
    \x1d\n\nadmin_port\x18\x08\x20\x01(\rR\tadminPort\x126\n\x07volumes\x18\
    \t\x20\x03(\x0b2\x1c.pb.VolumeInformationMessageR\x07volumes\"s\n\x11Hea\
    rtbeatResponse\x12(\n\x0fvolumeSizeLimit\x18\x01\x20\x01(\x04R\x0fvolume\
    SizeLimit\x12\x1c\n\tsecretKey\x18\x02\x20\x01(\tR\tsecretKey\x12\x16\n\
    \x06leader\x18\x03\x20\x01(\tR\x06leader\"\xc4\x02\n\x18VolumeInformatio\
    nMessage\x12\x0e\n\x02id\x18\x01\x20\x01(\rR\x02id\x12\x12\n\x04size\x18\
    \x02\x20\x01(\x04R\x04size\x12\x1e\n\ncollection\x18\x03\x20\x01(\tR\nco\
    llection\x12\x1d\n\nfile_count\x18\x04\x20\x01(\x04R\tfileCount\x12!\n\
    \x0cdelete_count\x18\x05\x20\x01(\x04R\x0bdeleteCount\x12,\n\x12deleted_\
    byte_count\x18\x06\x20\x01(\x04R\x10deletedByteCount\x12\x1b\n\tread_onl\
    y\x18\x07\x20\x01(\x08R\x08readOnly\x12+\n\x11replica_placement\x18\x08\
    \x20\x01(\rR\x10replicaPlacement\x12\x18\n\x07version\x18\t\x20\x01(\rR\
    \x07version\x12\x10\n\x03ttl\x18\n\x20\x01(\rR\x03ttl2G\n\x08Zergling\
    \x12;\n\rSendHeartbeat\x12\r.pb.Heartbeat\x1a\x15.pb.HeartbeatResponse\"\
    \0(\x010\x01J\xb8\x0e\n\x06\x12\x04\0\0(\x01\n\x08\n\x01\x0c\x12\x03\0\0\
    \x12\n\x08\n\x01\x02\x12\x03\x02\x08\n\n=\n\x02\x06\0\x12\x04\x06\0\x08\
    \x0121////////////////////////////////////////////////\n\n\n\n\x03\x06\0\
    \x01\x12\x03\x06\x08\x10\n\x0b\n\x04\x06\0\x02\0\x12\x03\x07\x02K\n\x0c\
    \n\x05\x06\0\x02\0\x01\x12\x03\x07\x06\x13\n\x0c\n\x05\x06\0\x02\0\x05\
    \x12\x03\x07\x14\x1a\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x07\x1b$\n\x0c\
    \n\x05\x06\0\x02\0\x06\x12\x03\x07/5\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\
    \x076G\n=\n\x02\x04\0\x12\x04\x0c\0\x16\x0121///////////////////////////\
    /////////////////////\n\n\n\n\x03\x04\0\x01\x12\x03\x0c\x08\x11\n\x0b\n\
    \x04\x04\0\x02\0\x12\x03\r\x02\x10\n\r\n\x05\x04\0\x02\0\x04\x12\x04\r\
    \x02\x0c\x13\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\r\x02\x08\n\x0c\n\x05\
    \x04\0\x02\0\x01\x12\x03\r\t\x0b\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\r\
    \x0e\x0f\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0e\x02\x12\n\r\n\x05\x04\0\
    \x02\x01\x04\x12\x04\x0e\x02\r\x10\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\
    \x0e\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x0e\t\r\n\x0c\n\x05\
    \x04\0\x02\x01\x03\x12\x03\x0e\x10\x11\n\x0b\n\x04\x04\0\x02\x02\x12\x03\
    \x0f\x02\x18\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x0f\x02\x0e\x12\n\x0c\n\
    \x05\x04\0\x02\x02\x05\x12\x03\x0f\x02\x08\n\x0c\n\x05\x04\0\x02\x02\x01\
    \x12\x03\x0f\t\x13\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x0f\x16\x17\n\
    \x0b\n\x04\x04\0\x02\x03\x12\x03\x10\x02\x1e\n\r\n\x05\x04\0\x02\x03\x04\
    \x12\x04\x10\x02\x0f\x18\n\x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x10\x02\
    \x08\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x10\t\x19\n\x0c\n\x05\x04\0\
    \x02\x03\x03\x12\x03\x10\x1c\x1d\n\x0b\n\x04\x04\0\x02\x04\x12\x03\x11\
    \x02\x1a\n\r\n\x05\x04\0\x02\x04\x04\x12\x04\x11\x02\x10\x1e\n\x0c\n\x05\
    \x04\0\x02\x04\x05\x12\x03\x11\x02\x08\n\x0c\n\x05\x04\0\x02\x04\x01\x12\
    \x03\x11\t\x15\n\x0c\n\x05\x04\0\x02\x04\x03\x12\x03\x11\x18\x19\n\x0b\n\
    \x04\x04\0\x02\x05\x12\x03\x12\x02\x19\n\r\n\x05\x04\0\x02\x05\x04\x12\
    \x04\x12\x02\x11\x1a\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x12\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x05\x01\x12\x03\x12\t\x14\n\x0c\n\x05\x04\0\x02\x05\
    \x03\x12\x03\x12\x17\x18\n\x0b\n\x04\x04\0\x02\x06\x12\x03\x13\x02\x12\n\
    \r\n\x05\x04\0\x02\x06\x04\x12\x04\x13\x02\x12\x19\n\x0c\n\x05\x04\0\x02\
    \x06\x05\x12\x03\x13\x02\x08\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x13\t\
    \r\n\x0c\n\x05\x04\0\x02\x06\x03\x12\x03\x13\x10\x11\n\x0b\n\x04\x04\0\
    \x02\x07\x12\x03\x14\x02\x18\n\r\n\x05\x04\0\x02\x07\x04\x12\x04\x14\x02\
    \x13\x12\n\x0c\n\x05\x04\0\x02\x07\x05\x12\x03\x14\x02\x08\n\x0c\n\x05\
    \x04\0\x02\x07\x01\x12\x03\x14\t\x13\n\x0c\n\x05\x04\0\x02\x07\x03\x12\
    \x03\x14\x16\x17\n\x0b\n\x04\x04\0\x02\x08\x12\x03\x15\x020\n\x0c\n\x05\
    \x04\0\x02\x08\x04\x12\x03\x15\x02\n\n\x0c\n\x05\x04\0\x02\x08\x06\x12\
    \x03\x15\x0b#\n\x0c\n\x05\x04\0\x02\x08\x01\x12\x03\x15$+\n\x0c\n\x05\
    \x04\0\x02\x08\x03\x12\x03\x15./\n\n\n\x02\x04\x01\x12\x04\x17\0\x1b\x01\
    \n\n\n\x03\x04\x01\x01\x12\x03\x17\x08\x19\n\x0b\n\x04\x04\x01\x02\0\x12\
    \x03\x18\x02\x1d\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\x18\x02\x17\x1b\n\
    \x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x18\x02\x08\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03\x18\t\x18\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x18\x1b\
    \x1c\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x19\x02\x17\n\r\n\x05\x04\x01\
    \x02\x01\x04\x12\x04\x19\x02\x18\x1d\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\
    \x03\x19\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x19\t\x12\n\x0c\
    \n\x05\x04\x01\x02\x01\x03\x12\x03\x19\x15\x16\n\x0b\n\x04\x04\x01\x02\
    \x02\x12\x03\x1a\x02\x14\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04\x1a\x02\
    \x19\x17\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x1a\x02\x08\n\x0c\n\x05\
    \x04\x01\x02\x02\x01\x12\x03\x1a\t\x0f\n\x0c\n\x05\x04\x01\x02\x02\x03\
    \x12\x03\x1a\x12\x13\n\n\n\x02\x04\x02\x12\x04\x1d\0(\x01\n\n\n\x03\x04\
    \x02\x01\x12\x03\x1d\x08\x20\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1e\x02\
    \x10\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x1e\x02\x1d\"\n\x0c\n\x05\x04\
    \x02\x02\0\x05\x12\x03\x1e\x02\x08\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\
    \x1e\t\x0b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x1e\x0e\x0f\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03\x1f\x02\x12\n\r\n\x05\x04\x02\x02\x01\x04\x12\
    \x04\x1f\x02\x1e\x10\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x1f\x02\x08\
    \n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x1f\t\r\n\x0c\n\x05\x04\x02\x02\
    \x01\x03\x12\x03\x1f\x10\x11\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x20\x02\
    \x18\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04\x20\x02\x1f\x12\n\x0c\n\x05\
    \x04\x02\x02\x02\x05\x12\x03\x20\x02\x08\n\x0c\n\x05\x04\x02\x02\x02\x01\
    \x12\x03\x20\t\x13\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x20\x16\x17\n\
    \x0b\n\x04\x04\x02\x02\x03\x12\x03!\x02\x18\n\r\n\x05\x04\x02\x02\x03\
    \x04\x12\x04!\x02\x20\x18\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03!\x02\
    \x08\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03!\t\x13\n\x0c\n\x05\x04\x02\
    \x02\x03\x03\x12\x03!\x16\x17\n\x0b\n\x04\x04\x02\x02\x04\x12\x03\"\x02\
    \x1a\n\r\n\x05\x04\x02\x02\x04\x04\x12\x04\"\x02!\x18\n\x0c\n\x05\x04\
    \x02\x02\x04\x05\x12\x03\"\x02\x08\n\x0c\n\x05\x04\x02\x02\x04\x01\x12\
    \x03\"\t\x15\n\x0c\n\x05\x04\x02\x02\x04\x03\x12\x03\"\x18\x19\n\x0b\n\
    \x04\x04\x02\x02\x05\x12\x03#\x02\x20\n\r\n\x05\x04\x02\x02\x05\x04\x12\
    \x04#\x02\"\x1a\n\x0c\n\x05\x04\x02\x02\x05\x05\x12\x03#\x02\x08\n\x0c\n\
    \x05\x04\x02\x02\x05\x01\x12\x03#\t\x1b\n\x0c\n\x05\x04\x02\x02\x05\x03\
    \x12\x03#\x1e\x1f\n\x0b\n\x04\x04\x02\x02\x06\x12\x03$\x02\x15\n\r\n\x05\
    \x04\x02\x02\x06\x04\x12\x04$\x02#\x20\n\x0c\n\x05\x04\x02\x02\x06\x05\
    \x12\x03$\x02\x06\n\x0c\n\x05\x04\x02\x02\x06\x01\x12\x03$\x07\x10\n\x0c\
    \n\x05\x04\x02\x02\x06\x03\x12\x03$\x13\x14\n\x0b\n\x04\x04\x02\x02\x07\
    \x12\x03%\x02\x1f\n\r\n\x05\x04\x02\x02\x07\x04\x12\x04%\x02$\x15\n\x0c\
    \n\x05\x04\x02\x02\x07\x05\x12\x03%\x02\x08\n\x0c\n\x05\x04\x02\x02\x07\
    \x01\x12\x03%\t\x1a\n\x0c\n\x05\x04\x02\x02\x07\x03\x12\x03%\x1d\x1e\n\
    \x0b\n\x04\x04\x02\x02\x08\x12\x03&\x02\x15\n\r\n\x05\x04\x02\x02\x08\
    \x04\x12\x04&\x02%\x1f\n\x0c\n\x05\x04\x02\x02\x08\x05\x12\x03&\x02\x08\
    \n\x0c\n\x05\x04\x02\x02\x08\x01\x12\x03&\t\x10\n\x0c\n\x05\x04\x02\x02\
    \x08\x03\x12\x03&\x13\x14\n\x0b\n\x04\x04\x02\x02\t\x12\x03'\x02\x12\n\r\
    \n\x05\x04\x02\x02\t\x04\x12\x04'\x02&\x15\n\x0c\n\x05\x04\x02\x02\t\x05\
    \x12\x03'\x02\x08\n\x0c\n\x05\x04\x02\x02\t\x01\x12\x03'\t\x0c\n\x0c\n\
    \x05\x04\x02\x02\t\x03\x12\x03'\x0f\x11b\x06proto3\
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
