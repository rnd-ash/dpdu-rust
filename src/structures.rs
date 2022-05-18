use std::ffi::c_void;

use crate::{PduIt, PduFilter, PduQueueMode, VidPreselectMode, CombinationMode, PduPt, PduPc, PduStatus, PduInfo, PduErrorEvt, PduCpst, TimingSet};


#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Generic structure containing item type
pub struct PduItem {
    /// Item type
    pub item_type: PduIt
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Generic IOCTL type structure
pub struct PduDataItem {
    /// IOCTL Item type
    pub item_type: PduIt,
    /// IOCTL data structure pointer
    pub p_data: *mut c_void
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// IOCTL programming voltage structure
pub struct IoProgVoltageData {
    /// Programming voltage (mV)
    pub prog_voltage_mv: u32,
    /// Pin number on connector
    pub pin_on_dlc: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// IOCTL Byte array structure
pub struct IoByteArrayData {
    /// Data size in bytes
    pub data_size: u32,
    /// Pointer to data
    pub p_data: *mut u8 
}

impl From<&mut [u8]> for IoByteArrayData {
    fn from(x: &mut [u8]) -> Self {
        IoByteArrayData { data_size: x.len() as u32, p_data: x.as_mut_ptr() }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// IOCTL Filter data structure
/// 
/// MASK & RAW == PATTERN
pub struct IoFilterData {
    /// Filter type
    pub filter_type: PduFilter,
    /// Filter number
    pub filter_number: u32,
    /// Compare size of the mask and pattern message
    pub filter_compare_size: u32,
    /// Mask message
    pub filter_mask_msg: [u8; 12],
    /// Pattern message
    pub filter_pattern_msg: [u8; 12]
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// IOCTL Event queue property data
pub struct IoEventQueuePropertyData {
    /// Max size of the event queue
    pub queue_size: u32,
    /// Queue mode
    pub queue_mode: PduQueueMode
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// IOCTL Vehicle ID request 
pub struct VehicleIdRequest {
    /// Preselection mode
    pub preselection_mode: VidPreselectMode,
    /// Preselection ASCII string
    pub preselection_value: *mut u8,
    /// Combination mode
    pub combination_mode: CombinationMode,
    /// discovery time in milliseconds
    pub vehicle_discovery_time: u32,
    /// Number of broadcast / multicast addresses found in [VehicleIdRequest::destination_addresses] array
    pub num_destination_addresses: u32,
    /// Pointer to array of IP addresses
    pub destination_addresses: *mut IpAddrInfo
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Address info
pub struct IpAddrInfo {
    /// IP version 4 = Ipv4, 6 = Ipv6
    pub ip_version: u32,
    /// Pointer to broadcast address (4 bytes for Ipv4, 16 bytes for Ipv6)
    pub p_address: *mut u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// IOCTL ethernet switch state
pub struct EthSwitchState {
    /// Sense state - 0 = pin off, 1 = pin on
    pub eth_sense_state: u32,
    /// Pin number for ethernet activation
    pub eth_act_pin_num: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Resource status data
pub struct RscStatusData {
    /// Item type
    pub item_type: PduIt,
    /// Number of entries
    pub num_entries: u32,
    /// Pointer to array of entries
    pub p_resource_status_data: *mut RscStatusItem
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Resource status item
pub struct RscStatusItem {
    /// MVCI handle ID
    pub h_mod: u32,
    /// Resource ID
    pub resource_id: u32,
    /// Resource information status
    pub resource_status: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// ComParam data information
pub struct ParamItem {
    /// Item type
    pub item_type: PduIt,
    /// Com param ID
    pub com_param_id: u32,
    /// Com param data type
    pub com_param_data_type: PduPt,
    /// Com param class type
    pub com_param_class: PduPc,
    /// Pointer to data of ComParam (of type specified in [ParamItem::com_param_class])
    pub p_com_param_data: *mut c_void
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Module identification information
pub struct ModuleItem {
    /// Item type
    pub item_type: PduIt,
    /// Number of entries in [ModuleItem::p_module_data]
    pub num_entries: u32,
    /// Pointer to array of [ModuleData]
    pub p_module_data: *mut ModuleData
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Module identification data
pub struct ModuleData {
    /// Module protocol type
    pub module_type_id: u32,
    /// Module handle ID
    pub h_mod: u32,
    /// Null terminated string of module name
    pub vendor_module_name: *mut u8,
    /// Null terminated string pointer of any additional info
    pub vendor_additional_info: *mut u8,
    /// Module status
    pub status: PduStatus
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Item resource identification item
pub struct RscIdItem {
    /// Item type
    pub item_type: PduIt,
    /// Number of entries in [RscIdItem::p_id_item_data]
    pub num_modules: u32,
    /// Pointer to array of [RscIdItemData]
    pub p_id_item_data: *mut RscIdItemData
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Item resource identification data
pub struct RscIdItemData {
    /// MVCI Handle ID
    pub h_mod: u32,
    /// Number of IDs
    pub num_ids: u32,
    /// Pointer to list of resource IDs
    pub p_resource_id_array: *mut u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Resource data
pub struct RscData {
    /// Bus type ID
    pub bus_type_id: u32,
    /// Protocol ID
    pub protocol_id: u32,
    /// Number of items in [RscData::p_dlc_pin_data]
    pub num_pin_data: u32,
    /// Pointer to array of [PinData]
    pub p_dlc_pin_data: *mut PinData
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Pin Data
pub struct PinData {
    /// Pin number on data connector
    pub dlc_pin_number: u32,
    /// Pin ID
    pub dlc_pin_type_id: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Resource conflict item
pub struct RscConflictItem {
    /// Item type
    pub item_type: PduIt,
    /// Number of entries in [RscConflictItem::p_rsc_conflict_data]
    pub num_entries: u32,
    /// Pointer to array of [RscConflictData]
    pub p_rsc_conflict_data: *mut RscConflictData
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Resource conflict data
pub struct RscConflictData {
    /// MVCI handle ID with conflict
    pub h_mod: u32,
    /// The conflicting resource ID
    pub resource_id: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Unique response identification data
pub struct UniqueRespIdTableItem {
    /// Item type
    pub item_type: PduIt,
    /// Number of entries in [UniqueRespIdTableItem::p_unique_data]
    pub num_entries: u32,
    /// Pointer to array of [EcuUniqueRespData]
    pub p_unique_data: *mut EcuUniqueRespData
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// ECU Unique response data
pub struct EcuUniqueRespData {
    /// Unique response identifier
    pub unique_resp_identifier: u32,
    /// Number of ComParams for the unique identifier
    pub num_param_items: u32,
    /// Pointer to array of ComParams used to uniquely define the ECUs unique response
    pub p_params: *mut ParamItem
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Event notification item
pub struct EventItem {
    /// Item type
    pub item_type: PduIt,
    /// If from ComPrimitive, then this is the ComPrimitive handle, otherwise [PDU_HANDLE_UNDEF]
    pub h_cop: u32,
    /// ComPrimitive tag
    pub p_cop_tag: *mut c_void,
    /// Timestamp in microseconds
    pub timestamp: u32,
    /// Pointer to the data of the event
    pub p_data: *mut c_void
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Async event information data
pub struct InfoData {
    /// Information code
    pub info_code: PduInfo,
    /// optional extra information data
    pub extra_info_data: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Async error notification data
pub struct ErrorData {
    /// Error code
    pub error_code_id: PduErrorEvt,
    /// optional extra error information
    pub extra_error_info_id: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Async result notification
pub struct ResultData {
    /// Receive message status
    pub rx_flag: FlagData,
    /// ECU Response unique ID
    pub unique_resp_identifier: u32,
    /// Acceptance ID
    pub acceptance_id: u32,
    /// Timestamp indicator flag
    pub timestamp_flags: FlagData,
    /// Transmit message done in microseconds
    pub tx_msg_done_timestamp: u32,
    /// Start message timestamp in microseconds
    pub start_msg_timestamp: u32,
    /// Pointer to extra information
    pub p_extra_info: *mut ExtraInfo,
    /// Number of bytes in [ResultData::p_data_bytes]
    pub num_data_bytes: u32,
    /// Pointer to payload data structure
    pub p_data_bytes: *mut u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Extra information data
pub struct ExtraInfo {
    /// Number of bytes in [ExtraInfo::p_header_bytes]
    pub num_header_bytes: u32,
    /// Number of bytes in [ExtraInfo::p_footer_bytes]
    pub num_footer_bytes: u32,
    /// Reference to response header bytes
    pub p_header_bytes: *mut u8,
    /// Reference to response footer bytes
    pub p_footer_bytes: *mut u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Flag data
pub struct FlagData {
    /// Number of bytes in [FlagData::p_flag_data]
    pub num_flag_bytes: u32,
    /// Pointer to flag bytes
    pub p_flag_data: *mut u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Version information
pub struct VersionData {
    /// Release version of MVCI standard part 1 (Bit encoded)
    pub mvci_part1_standard_version: u32,
    /// Release version of MVCI standard part 2 (Bit encoded)
    pub mvci_part2_standard_version: u32,
    /// Hardware serial number from vendor
    pub hw_serial_number: u32,
    /// Hardware name
    pub hw_name: [u8; 64],
    /// Hardware version (Bit encoded)
    pub hw_version: u32,
    /// hardware date (Bit encoded)
    pub hw_data: u32,
    /// Type of MVCI module
    pub hw_inferface: u32,
    /// MVCI Firmware name
    pub fw_name: [u8; 64],
    /// MVCI firmware version (Bit encoded)
    pub fw_version: u32,
    /// MVCI firmware date (Bit encoded)
    pub fw_date: u32,
    /// MVCI vendor name
    pub vendor_name: [u8; 64],
    /// PDU API software name
    pub pdu_api_sw_name: [u8; 64],
    /// PDU API software version (Bit encoded)
    pub pdu_api_sw_version: u32,
    /// PDU API software date (Bit encoded)
    pub pdi_api_sw_date: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Com primitive control data
pub struct CopCtrlData {
    /// Cycle time in milliseconds or delay time of the ComPrimitive
    pub time: u32,
    /// Number of send cycles. -1 for infinite
    pub num_send_cycles: i32,
    /// Number of receives. -1 for infinite
    pub num_receive_cycles: i32,
    /// Optional temporary setting for the ComPrimitive
    pub temp_param_update: u32,
    /// Transmit flag
    pub tx_flag: FlagData,
    /// Number of elements in [CopCtrlData::expected_response_array]
    pub num_possible_expected_responses: u32,
    /// Pointer to an array of expected responses
    pub expected_response_array: *mut ExpRespData
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// DoIP IO Entity address data
pub struct IoEntityAddressData {
    /// Logical addresses 
    pub logical_address: u32,
    /// Timeout in milliseconds to wait for responses
    pub doip_ctrl_timeout: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// DoIP IO Entity status data
pub struct IoEntityStatusData {
    /// Type of DoIP entitiy
    pub entity_type: u32,
    /// Max number of Sockets allowed
    pub tcp_clients_max: u32,
    /// Number of current sockets
    pub tcp_clients: u32,
    /// Optional limit in bytes for max size of a single DoIP request
    pub max_data_size: u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Expected response structure
pub struct ExpRespData {
    /// Expected response type (0 = positive, 1 = negative)
    pub response_type: u32,
    /// ID assigned by application to be returned
    pub acceptance_id: u32,
    /// Number of bytes in [ExpRespData::p_mask_data] and [ExpRespData::p_pattern_data]
    pub num_mask_pattern_bytes: u32,
    /// Pointer to mask data
    pub p_mask_data: *mut u8,
    /// Pointer to pattern data
    pub p_pattern_data: *mut u8,
    /// Number of items in [ExpRespData::p_unique_resp_ids]
    pub num_unique_resp_ids: u32,
    /// Array containing unique response IDs
    pub p_unique_resp_ids: *mut u32
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// ComParam byte field data
pub struct ParamByteFieldData {
    /// Maximum number of bytes the [ParamByteFieldData::p_data_array] can contain
    pub param_max_len: u32,
    /// Current (actual) number of bytes in [ParamByteFieldData::p_data_array]
    pub param_act_len: u32,
    /// Pointer to data array
    pub p_data_array: *mut u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// ComParam struct field data
pub struct ParamStructFieldData {
    /// Struct type
    pub com_param_struct_type: PduCpst,
    /// Maximum number of structures the [ParamStructFieldData::p_struct_array] can contain
    pub param_max_entries: u32,
    /// Current (actual) number of structures in [ParamStructFieldData::p_struct_array]
    pub param_act_entries: u32,
    /// Pointer to structure array (See [ParamStructSessionTiming] and [ParamStructAccessTiming])
    pub p_struct_array: *mut c_void
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Session timing for [ParamStructFieldData] when [ParamStructFieldData::com_param_struct_type] is [PduCpst::SessionTiming]
pub struct ParamStructSessionTiming {
    /// Session ID
    pub session: u16,
    /// 1ms resolution
    pub p2_max_high: u8,
    /// 1ms resolution
    pub p2_max_low: u8,
    /// 10ms resolution
    pub p2_star_high: u8,
    /// 10ms resolution
    pub p2_star_low: u8
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Access timing for [ParamStructFieldData] when [ParamStructFieldData::com_param_struct_type] is [PduCpst::AccessTiming]
pub struct ParamStructAccessTiming {
    /// 0.5ms resolution - Minimum time between tester request and ECU response
    pub p2_min: u8,
    /// 0.5ms resolution - Maximum time between tester request and ECU response
    pub p2_max: u8,
    /// 250ms resolution - Minimum time between ECU response and start of new tester request
    pub p3_min: u8,
    /// 250ms resolution - Maximum time between ECU response and start of new tester request
    pub p3_max: u8,
    /// 0.5ms resolution - Minimum inter-byte time for tester request
    pub p4_min: u8,
    /// Timing set type
    pub timing_set: TimingSet
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Long field data
pub struct ParamLongFieldData {
    /// Maximum number of entries the [ParamLongFieldData::p_data_array] can contain
    pub param_max_len: u32,
    /// Current (actual) number of entries in [ParamLongFieldData::p_data_array]
    pub param_act_len: u32,
    /// Pointer to data array
    pub p_data_array: *mut u32
}