use std::ffi::c_void;

use crate::*;



/// Constructs and initializes the PDU API library
/// 
/// ## Parameters
/// * option_str - A list of attributes and values specific to D-PDU API
/// * p_api_tag - Application defined tag value for callbacks
pub type PduConstructFn = extern "C" fn(
    option_str: *mut u8, 
    p_api_tag: *mut c_void
) -> PduError;

/// Closes all open communication channels and destructs the PDU API library
pub type PduDestructFn = extern "C" fn() -> PduError;

/// Performs generic IOCTL calls on a MVCI or ComLogicalLink
/// 
/// ## Parameters
/// * h_mod - Handle of MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * ioctl_commanded_id - IO Command to send to ComLogicalLink
/// * p_input_data - Pointer to input data item (Null if not required)
/// * p_output_data - Pointer to output data item (Null if not required)
/// 
pub type PduIoctlFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    ioctl_commanded_id: u32,
    p_input_data: *mut PduDataItem,
    p_output_data: *mut PduDataItem
) -> PduError;

/// Gets version information from MVCI module
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * p_version_data - Output pointer for the destination of the version data
pub type PduGetVersionFn = extern "C" fn(
    h_mod: u32,
    p_version_data: *mut VersionData
) -> PduError;

/// Gets runtime information from either a MVCI module, ComLogicalLink or ComPrimitive
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * h_cop - Handle of the ComPrimitive
/// * p_status_code - Pointer to store the status code
/// * p_timestamp - Pointer to store timestamp in microseconds
/// * p_extra_info - Pointer for storing any extra information
pub type PduGetStatusFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    h_cop: u32,
    p_status_code: *mut PduStatus,
    p_timestamp: *mut u32,
    p_extra_info: *mut u32
) -> PduError;

/// Gets the last runtime error from the MVCI module or ComLogicalLink
/// Used for SAE J2534-2 support
///
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * p_error_code - Pointer to store the error code
/// * ph_cop - If the last error persists to a ComPrimitive, then this will contain the handle of the ComPrimitive
/// * p_timestamp - Pointer to store timestamp
/// * Pointer for storing any extra information
pub type PduGetListErrorFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    p_error_code: *mut PduErrorEvt,
    ph_cop: *mut u32,
    p_timestamp: *mut u32,
    p_extra_error_info: *mut u32
) -> PduError;

/// Obtains resource status information from the PDU API
/// 
/// ## Parameters
/// * p_resource_status - Pointer to store the status of the requested resource IDs
pub type PduGetResourceStatusFn = extern "C" fn(
    p_resource_status: *mut RscStatusItem
) -> PduError;

/// Creates a ComLogicalLink for a given resource ID
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * p_rsc_data - Pointer to resource data objects
/// * resource_id - Resource ID
/// * p_cll_tag - Application defined tag value
/// * ph_cll - Pointer for storing the ComLogicalLink handle to
/// * p_cll_Create_flag - Pointer for storage of flag bits
pub type PduCreateComLogicalLinkFn = extern "C" fn(
    h_mod: u32,
    p_rsc_data: *mut RscData,
    resource_id: u32,
    p_cll_tag: *mut c_void,
    ph_cll: *mut u32,
    p_cll_create_flag: *mut FlagData
) -> PduError;

/// Destroys a given ComLogicalLink
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink to destroy
pub type PduDestroyComLogicalLinkFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32
) -> PduError;

/// Connects a ComLogicalLink to a vehicle interface
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink to connect
pub type PduConnectFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32
) -> PduError;

/// Disconnects a ComLogicalLink from a vehicle interface
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink to disconnect
pub type PduDisconnectFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32
) -> PduError;

/// Locks a physical resource so that a ComLogicalLink has exclusive access to it
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink to be granted exclusive access
/// * lock_mask - Bit encoded mask to request for locking
pub type PduLockResourceFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    lock_mask: u32
) -> PduError;

/// Unlocks a physical resource from a ComLogicalLink that has exclusive access to it
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink to unlock the resource from
/// * lock_mask - Bit encoded mask to request for release
pub type PduUnlockResourceFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    lock_mask: u32
) -> PduError;

/// Obtains a communication or bus ComParam out of the MVCIs working buffer of a ComLogicalLink
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * param_id - ID value of the ComParam that is being requested
/// * p_param_items - Pointer to store the requested ComParam into
pub type PduGetComParamFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    param_id: u32,
    p_param_items: *mut ParamItem
) -> PduError;

/// Sets a com param on a ComLogicalLink
///
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink to set the param on
/// * p_param_items - Pointer to a ComParams to set
pub type PduSetComParamFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    p_param_items: *mut ParamItem
) -> PduError;

/// Creates and starts a ComPrimitive
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink to start the ComPrimitive on
/// * cop_type - Type of ComPrimitive to start
/// * cop_data_size - Size of the data for the ComPrimitive
/// * p_cop_data - Pointer to data for the ComPrimitive
/// * p_cop_tag - Application specific tag
/// * ph_cop - Reference for storing the returned ComPrimitive handle
pub type PduStartComPrimitiveFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    cop_type: PduCopt,
    cop_data_size: u32,
    p_cop_data: *mut u8,
    p_cop_ctrl_data: *mut CopCtrlData,
    p_cop_tag: *mut c_void,
    ph_cop: *mut u32
) -> PduError;

/// Cancels and stops a ComPrimitive
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * h_cop - Handle of the ComPrimitive
pub type PduCancelComPrimitiveFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    h_cop: u32
) -> PduError;

/// Retrieves event data for a given event source
/// 
/// ## Parameter
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * p_event_item - Pointer to store the event item
pub type PduGetEventItemFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    p_event_item: *mut EventItem
) -> PduError;

/// Destroys a given item
/// 
/// ## Parameters
/// * p_item - Pointer to item to be destroyed
pub type PduDestroyItemFn = extern "C" fn(
    p_item: *mut PduItem
);

/// Registers a callback function
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * callback_fn - Callback function (null to deregister callback)
pub type PduRegisterCallbackFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    callback_fn: EventCallbackFn
) -> PduError;

/// Gets the Item ID of a given item
/// 
/// ## Parameters
/// * pdu_object_type - Type of object
/// * p_short_name - Short name of the object
/// * p_pdu_object_id - Reference to store the object ID
pub type PduGetObjectIdFn = extern "C" fn(
    pdu_object_type: PduObjt,
    p_short_name: *mut u8,
    p_pdu_object_id: *mut u32
) -> PduError;


/// Object module information
/// 
/// ## Parameters
/// * p_module_id_list - Pointer for storing the pointer of the module information list
pub type PduGetModuleIdsFn = extern "C" fn(
    p_module_id_list: *mut *mut ModuleItem
) -> PduError;

/// Get a list of resource IDs
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * p_resource_id_data - Pointer to store resource ID data
/// * p_resource_id_list - Pointer to store resource ID list
pub type PduGetResourceIdsFn = extern "C" fn(
    h_mod: u32,
    p_resource_id_data: *mut RscData,
    p_resource_id_list: *mut RscIdItem
) -> PduError;

/// Gets a list of conflicting resources
/// 
/// ## Parameters
/// * resource_id - Resource ID to check for conflicts
/// * p_input_module_list - Pointer to module to check for conflicts
/// * p_output_conflict_list - Pointer of destination to store a list of conflicting resources
pub type PduGetConflictingResourcesFn = extern "C" fn(
    resource_id: u32,
    p_input_module_list: *mut ModuleItem,
    p_output_conflict_list: *mut RscConflictItem
) -> PduError;

/// Gets a list of unique response IDs from a ComLogicalLink
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * p_unique_resp_id_table - Pointer to store the list of unique response IDs
pub type PduGetUniqueRespIdTableFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    p_unique_resp_id_table: *mut UniqueRespIdTableItem
) -> PduError;

/// Sets a unique response ID table for the ComLogicalLink
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module
/// * h_cll - Handle of the ComLogicalLink
/// * p_unique_resp_id_table - Pointer to the unique response ID table to set
pub type PduSetUniqueRespIdTableFn = extern "C" fn(
    h_mod: u32,
    h_cll: u32,
    p_unique_resp_id_table: *mut UniqueRespIdTableItem
) -> PduError;

/// Determines if a MVCI module is available to connect
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module to try and connect
pub type PduModuleConnectFn = extern "C" fn(
    h_mod: u32
) -> PduError;

/// Tries to close all communication channels of a MVCI module
/// 
/// ## Parameters
/// * h_mod - Handle of the MVCI module to disconnect
pub type PduModuleDisconnectFn = extern "C" fn(
    h_mod: u32
) -> PduError;

/// Obtains the current hardware clock of the MVCI module
/// 
/// ## Parameters
/// * p_timestamp - Pointer to store timestamp in microseconds
pub type PduGetTimestampFn = extern "C" fn(
    h_mod: u32,
    p_timestamp: *mut u32
) -> PduError;