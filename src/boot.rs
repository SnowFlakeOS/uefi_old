use ::{Void, Event, Handle, TableHeader};
use guid::Guid;
use memory::{MemoryType, MemoryDescriptor};
use status::Status;

pub type PoolPointer<T> = *mut T;
pub type Tpl = usize;
pub type EventNotifyFcn = extern "win64" fn(Event, *mut Void) -> ();

#[repr(C)]
pub enum TimerDelay {
	Cancel,
	Periodic,
	Relative
}

#[repr(C)]
pub enum InterfaceType {
    Native
}

#[repr(C)]
pub enum LocateSearchType {
    /// Retrieve all the handles in the handle database.
    AllHandles,
    /// Retrieve the next handle fron a RegisterProtocolNotify() event.
    ByRegisterNotify,
    /// Retrieve the set of handles from the handle database that support a specified protocol.
    ByProtocol
}

#[repr(C)]
pub enum AllocType {
	AnyPages,
	MaxAddress,
	Address
}

#[repr(C)]
pub struct BootServices {
    pub Hdr: TableHeader,
    RaiseTpl: extern "win64" fn(NewTpl: usize) -> usize,
    RestoreTpl: extern "win64" fn(OldTpl: usize),
    pub AllocatePages: extern "win64" fn(AllocType: AllocType, MemoryType: MemoryType, Pages: usize, Memory: &mut usize) -> Status,
    pub FreePages: extern "win64" fn(Memory: usize, Pages: usize) -> Status,
    pub GetMemoryMap: extern "win64" fn(size: &mut usize, map: *mut MemoryDescriptor, key: &mut usize, &mut usize, &mut u32) -> Status,
    pub AllocatePool: extern "win64" fn(MemoryType, usize, &mut PoolPointer<Void>) -> Status,
    pub FreePool: extern "win64" fn(*mut Void) -> Status,
    pub CreateEvent: extern "win64" fn (u32, /*notify_tpl:*/ Tpl, /*notify_function:*/ Option<EventNotifyFcn>, *mut Void, &mut Event) -> Status,
    pub SetTimer: extern "win64" fn (Event, TimerDelay, u64) -> Status,
<<<<<<< HEAD
    pub WaitForEvent: extern "win64" fn (NumberOfEvents: usize, Event: *const Event, Index: &mut usize) -> Status,
=======
    pub WaitForEvent: extern "win64" fn (NumberOfEvents: usize, Event: [Event], Index: &mut usize) -> Status,
>>>>>>> parent of 60aea76... boot: Fix WaitForEvent
    pub SignalEvent: extern "win64" fn (Event) -> Status,
    pub CloseEvent: extern "win64" fn (Event) -> Status,
    pub CheckEvent: extern "win64" fn (Event) -> Status,
    pub InstallProtocolInterface: extern "win64" fn (Handle: &mut Handle, Protocol: &Guid, InterfaceType: InterfaceType, Interface: usize) -> Status,
    ReinstallProtocolInterface: extern "win64" fn (),
    pub UninstallProtocolInterface: extern "win64" fn (Handle: Handle, Protocol: &Guid, Interface: usize) -> Status,
    pub HandleProtocol: extern "win64" fn (Handle: Handle, Protocol: &Guid, Interface: &mut usize) -> Status,
    _rsvd: usize,
    RegisterProtocolNotify: extern "win64" fn (),
    pub LocateHandle: extern "win64" fn (SearchType: LocateSearchType, Protocol: &Guid, SearchKey: usize, BufferSize: &mut usize, Buffer: *mut Handle) -> Status,
    LocateDevicePath: extern "win64" fn (),
    InstallConfigurationTable: extern "win64" fn (),
    pub LoadImage: extern "win64" fn (BootPolicy: bool, ParentImageHandle: Handle, DevicePath: usize /*TODO*/, SourceBuffer: *const u8, SourceSize: usize, ImageHandle: &mut Handle) -> Status,
    pub StartImage: extern "win64" fn (ImageHandle: Handle, ExitDataSize: &mut usize, ExitData: &mut *mut u16) -> Status,
    pub Exit: extern "win64" fn (ImageHandle: Handle, ExitStatus: isize, ExitDataSize: usize, ExitData: *const u16) -> Status,
    UnloadImage: extern "win64" fn (),
    pub ExitBootServices: extern "win64" fn (ImageHandle: Handle, MapKey: usize) -> Status,
    GetNextMonotonicCount: extern "win64" fn (),
    pub Stall: extern "win64" fn (Microseconds: usize) -> Status,
    pub SetWatchdogTimer: extern "win64" fn (Timeout: usize, WatchdogCode: u64, DataSize: usize, WatchdogData: *const u16) -> Status,
    ConnectController: extern "win64" fn (),
    DisconnectController: extern "win64" fn (),
    OpenProtocol: extern "win64" fn (),
    CloseProtocol: extern "win64" fn (),
    OpenProtocolInformation: extern "win64" fn (),
    pub ProtocolsPerHandle: extern "win64" fn (Handle: Handle, ProtocolBuffer: *mut Guid, ProtocolBufferCount: usize) -> Status,
    LocateHandleBuffer: extern "win64" fn (SearchType: LocateSearchType, Protocol: &Guid, SearchKey: usize, NoHandles: &mut usize, Buffer: &mut *mut Handle),
    pub LocateProtocol: extern "win64" fn (Protocol: &Guid, Registration: usize, Interface: &mut usize) -> Status,
    InstallMultipleProtocolInterfaces: extern "win64" fn (),
    UninstallMultipleProtocolInterfaces: extern "win64" fn (),
    CalculateCrc32: extern "win64" fn (),
    CopyMem: extern "win64" fn (),
    SetMem: extern "win64" fn (),
    pub CreateEventEx: extern "win64" fn (u32, /*notify_tpl:*/ Tpl, /*notify_function:*/ Option<EventNotifyFcn>, *mut Void, &Guid, &mut Event) -> Status,
}
