
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Item type values
pub enum PduIt {
    /// IOCTL UNUM32
    IoUnum32 = 0x1000,
    /// IOCTL program voltage
    IoProgVoltage = 0x1001,
    /// IOCTL Byte Array
    IoByteArray = 0x1002,
    /// IOCTL Filter
    IoFilter = 0x1003,
    /// IOCTL event queue priority
    IoEventQueueProperty = 0x1004,
    /// Resource status 
    RscStatus = 0x1100,
    /// Communication parameter (ComParam)
    Param = 0x1200,
    /// Result
    Result = 0x1300,
    /// Status notification
    Status = 0x1301,
    /// Error notification
    Error = 0x1302,
    /// Information notification
    Info = 0x1303,
    /// Resource ID
    RscId = 0x1400,
    /// Resource conflict
    RscConflict = 0x1500,
    /// Module ID
    ModuleId = 0x1600,
    /// Unique response ID table
    UniqueRespIdTable = 0x1700,
    /// DoIP Vehicle ID request
    IoVehicleIdRequest = 1800,
    /// DoIP ethernet activation
    EthSwitchState = 0x1801,
    /// DoIP entity addressing
    EntityAddress = 0x1802,
    /// DoIP entity status
    EntityStatus = 0x1803
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Communication primitive (ComParam) type
pub enum PduCopt {
    /// Start communication with an ECU
    StartComm = 0x8001,
    /// Stop communication with an ECU
    StopComm = 0x8002,
    /// Updates an existing [ComParameter] on an active logical communication link
    UpdateParam = 0x8003,
    /// Send request or response data
    SendRecv = 0x8004,
    /// Wait a specified time before executing the next [ComPrimitive]
    Delay = 0x8005,
    /// Opposite of [PduCopt::UpdateParam], copies active com param from logical communication
    /// link to a working buffer
    RestoreParam = 0x8006
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Object type
pub enum PduObjt {
    /// Protocol object
    Protocol = 0x8021,
    /// Bus type object
    BusType = 0x8022,
    /// IO control object
    IoCtrl = 0x8023,
    /// Communication Parameter object
    ComParam = 0x8024,
    /// Pin type object
    PinType = 0x8025,
    /// resource object
    Resource = 0x8026
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Status codes
pub enum PduStatus {
    /// Communication parameter has not been acted upon yet
    CopstIdle = 0x8010,
    /// Communication parameter is being run
    CopstExecuting = 0x8011,
    /// Communication parameter is finished being run
    CopstFinished = 0x8012,
    /// Communication parameter was cancelled
    CopstCancelled = 0x8013,
    /// Communication parameter is waiting to be executed again (Cyclic communication parameter)
    CopstWaiting = 0x8014,
    /// Communication logical link is offline
    CllstOffline = 0x8050,
    /// Communication logical link is online
    CllstOnline = 0x8051,
    /// Communication logical link is online and has been started (In a Tx/Rx state)
    CllstCommStarted = 0x8052,
    /// Vehicle communication interface is ready for communication
    ModstReady = 0x8060,
    /// Vehicle communication interface is not ready for communication
    ModstNotReady = 0x8061,
    /// Vehicle communication interface is unavailable for connection
    ModstNotAvail = 0x8062,
    /// Vehicle communication interface is available for connection
    ModstAvail = 0x8063
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Information events
pub enum PduInfo {
    /// New vehicle communication list is available
    ModuleListChange = 0x8070,
    /// A change has occurred with the lock status on a shared resource
    ResourceLockChange = 0x8071,
    /// A communication parameter on a logical link has been changed
    ComParamChange = 0x8072
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Event callback
pub enum PduEvtData {
    /// There is event data available to read by the application
    Available = 0x801,
    /// The ComLogicalLink has lost event data due to a buffer overrun
    Lost = 0x0802
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Filter type
pub enum PduFilter {
    /// Matched messages go into the receive queue
    Pass = 0x00000001,
    /// Matched messages stay out of the receive queue
    Block = 0x00000002,
    /// Matches messages go into the receive queue that are UUDT only (For ISO1765)
    PassUUDT = 0x00000011,
    /// Matches messages stay out of the receive queue that are UUDT only (For ISO1765)
    BlockUUDT = 0x00000012
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// IOCTL queue mode
pub enum PduQueueMode {
    /// Attempt to allocate memory for every event coming in to the receive queue. This queue size can keep
    /// growing until the API runs out of allocation memory
    Unlimited = 0x00000000,
    /// Attempt to allocate a fixed buffer size for events coming into the receive queue. Events are discarded
    /// from the receive queue if the buffer is full
    Limited = 0x00000001,
    /// Attempt to allocate a fixed buffer size for events coming into the receive queue. Events overwrite
    /// stored events if the buffer is full (Like a circular buffer)
    Circular = 0x00000002
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Function return values
pub enum PduError {
    /// No Error (Function call OK)
    StatusNoError = 0x00000000,
    /// Function call failed (Generic failure)
    FctFailed = 0x00000001,
    /// Reserved for ISO 22900-2
    Reserved1 = 0x00000010,
    /// Communication failed between host and MVCI
    CommPcToVciFailed = 0x00000011,
    /// PDU API has not yet been constructed
    PduApiNotConstructed = 0x00000020,
    /// PDU Destruct was not called before another PDU Construct
    SharingViolation = 0x00000021,
    /// Resource is already in use
    ResourceBusy = 0x00000030,
    /// Resource table changed
    ResourceTableChanged = 0x00000031,
    /// Generic resource error
    ResourceError = 0x00000032,
    /// ComLogicalLink cannot be offline and perform the requested action
    CllNotConnected = 0x00000040,
    /// ComLogicalLink must be started to perform the requested action
    CllNotStarted = 0x00000041,
    /// A parameter parsed into the function was invalid
    InvalidParameters = 0x00000050,
    /// A handle provided was invalid
    InvalidHandle = 0x00000060,
    /// Option value was unsupported
    ValueNotSupported = 0x00000061,
    /// IOCTL Command ID was unsupported
    IdNotSupported = 0x00000062,
    /// Communication parameter was unsupported
    ComParamNotSupported = 0x00000063,
    /// Physical communication parameter cannot be changed as it is locked by another LogicalLink
    ComParamLocked = 0x00000064,
    /// Transmit queue is full
    TxQueueFull = 0x00000070,
    /// No more events are available to read
    EventQueueEmpty = 0x00000071,
    /// IOCTL - Voltage value supplied is unsupported by the MVCI module
    VoltageNotSupported = 0x00000080,
    /// IOCTL - Pin or resource is not supported by the MVCI module
    MuxRscNotSupported = 0x00000081,
    /// Cable attached to MVCI module is unknown
    CableUnknown = 0x00000082,
    /// No cable attached to the MVCI module
    NoCableDetected = 0x00000083,
    /// ComLogicalLink is already connected
    CllConnected = 0x00000084,
    /// Physical Com parameters cannot be changes as a temporary one
    TempParamNotAllowed = 0x00000090,
    /// Resource is already locked
    RscLocked = 0x000000A0,
    /// Resource is already locked by another ComLogicalLink
    RscLockedByAnotherCll = 0x000000A1,
    /// Resource is already unlocked
    RscNotLocked = 0x000000A2,
    /// Module is not connected or ready
    ModuleNotConnected = 0x000000A3,
    /// API software is out of date
    ApiSwOutOfDate = 0x000000A4,
    /// VCI firmware is out of date
    ModuleFwOutOfDate = 0x000000A5,
    /// Requested pin is not routed by the MVCI's cable
    PinNotConnected = 0x000000A6,
    /// IP protocol not supported
    IpProtocolNotSupported = 0x000000B0,
    /// DoIP Routing activation failed (Generic failure)
    DoIPRoutingActivationFailed = 0x000000B1,
    /// DoIP Routing activation failed - missing / wrong authentication
    DoIPRoutingActivationAuthFailed = 0x000000B2,
    /// DoIP Logical address is defined multiple times so it is ambiguous
    DoIPAmbiguousLogicalAddress = 0x000000B3,
    /// DoIP Routing activation failed - Unknown or invalid source address
    DoIPRoutineActivationInvalidSrcAddress = 0x000000B4,
    /// DoIP Routing activation failed - No more free sockets available
    DoIPRoutingActivationNoDataSocketAvailable = 0x000000B5,
    /// DoIP Routing activation failed - The source address changed
    DoIPRoutineActivationSourceAddressChanged = 0x000000B6,
    /// DoIP Routing activation failed - Source address already in use
    DoIPRoutingActivationSourceAddressInUse = 0x000000B7,
    /// DoIP Routing activation failed - Rejected confirmation
    DoIPRoutineActivationConfirmationRejected = 0x000000B8,
    /// DoIP Routing activation failed - Requested activation type was unsupported
    DoIPRoutineActivationTypeUnsupported = 0x000000B9,
    /// DoIP Routing activation failed - Response code was unknown
    DoIPRoutineActivationResponseCodeUnknown = 0x000000BA,
    /// DoIP Routing activation failed - Timeout waiting for activation response
    DoIPRoutingActivationResponseTimeout = 0x000000BB,
    /// DoIP general timeout
    DoIPResponseTimeout = 0x000000BC
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Function error events (Used in asynchronous situations)
pub enum PduErrorEvt {
    /// No error
    NoError = 0x00000000,
    /// Structure of the received data frame was incorrect
    FrameStruct = 0x00000100,
    /// Transmit error
    TxError = 0x00000101,
    /// Tester present transmit error or ECU responded negatively to the request
    TesterPresentError = 0x00000102,
    /// ComParam could not be set as resource was locked
    RscLocked = 0x00000109,
    /// Receive message timeout
    RxTimeout = 0x00000103,
    /// Receive message error at a protocol level
    RxError = 0x00000104,
    /// ComPrimitive error by protocol
    ProtErr = 0x00000105,
    /// Communication to MVCI module was lost
    LostCommToVCI = 0x00000106,
    /// MVCI hardware fault
    VCIHardwareFault = 0x00000107,
    /// Protocol initialization error
    InitError = 0x00000108
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// ComParam data type
pub enum PduPt {
    /// Unsigned 8 bit
    Unum8 = 0x000000101,
    /// Signed 8 bit
    Snum8 = 0x000000102,
    /// Unsigned 16 bit
    Unum16 = 0x000000103,
    /// Signed 16 bit
    Snum16 = 0x000000104,
    /// Unsigned 32 bit
    Unum32 = 0x000000105,
    /// Signed 32 bit
    Snum32 = 0x000000106,
    /// Byte array
    ByteField = 0x000000107,
    /// Structure
    StructField = 0x000000108,
    /// Array of 32bit values
    LongField = 0x00000109
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// ComParam data class
pub enum PduPc {
    /// Message timing
    Timing = 1,
    /// Initialization of communication
    Init = 2,
    /// General com param
    Com = 3,
    /// Error handling ComParam
    ErrHdl = 4,
    /// BusType specific ComParam
    BusType = 5,
    /// 
    UniqueId = 6,
    /// Tester present ComParam
    TesterPresent = 7
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// ComParam struct type
pub enum PduCpst {
    /// Session timing
    SessionTiming = 0x00000001,
    /// Access timing
    AccessTiming = 0x00000002,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Vehicle preselection mode
pub enum VidPreselectMode {
    /// No preselection
    None = 0,
    /// DoIP with given VIN
    VIN = 1,
    /// DoIP with given EID
    EID = 2
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// DoIP Combination mode
pub enum CombinationMode {
    /// No combination
    None = 0,
    /// Combine common VIN
    VIN = 1,
    /// Combine common GroupID
    Group = 2,
    /// Combine all
    All = 3
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Timing set types used by [ParamStructAccessTiming]
pub enum TimingSet {
    /// Default timing set
    Default = 1,
    /// Override received timing from ECU
    OverrideReceived = 2,
    /// Override received timing from tester
    OverrideTester = 3,
    /// Normal timing set
    Normal = 4,
    /// Extended timing set
    Extended = 0xFF
}