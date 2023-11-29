extern crate alloc;

mod bist;
mod command;
mod header_type;
mod status;
mod type_specific;

use {
    alloc::{
        collections::{
            btree_map::BTreeMap,
            btree_set::BTreeSet,
        },
        vec::Vec,
    },
    core::mem,
    super::super::asm,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Address {
    bus: u8,
    device: u8,
    function: u8,
}

impl Address {
    const OFFSET_SHIFT_BEGIN: usize = 0;
    const OFFSET_SHIFT_LENGTH: usize = 8;
    const OFFSET_SHIFT_END: usize = Self::OFFSET_SHIFT_BEGIN + Self::OFFSET_SHIFT_LENGTH;
    const FUNCTION_SHIFT_BEGIN: usize = Self::OFFSET_SHIFT_END;
    const FUNCTION_SHIFT_LENGTH: usize = 3;
    const FUNCTION_SHIFT_END: usize = Self::FUNCTION_SHIFT_BEGIN + Self::FUNCTION_SHIFT_LENGTH;
    const FUNCTION_MAX: u8 = (1 << Self::FUNCTION_SHIFT_LENGTH) - 1;
    const DEVICE_SHIFT_BEGIN: usize = Self::FUNCTION_SHIFT_END;
    const DEVICE_SHIFT_LENGTH: usize = 5;
    const DEVICE_SHIFT_END: usize = Self::DEVICE_SHIFT_BEGIN + Self::DEVICE_SHIFT_LENGTH;
    const DEVICE_MAX: u8 = (1 << Self::DEVICE_SHIFT_LENGTH) - 1;
    const BUS_SHIFT_BEGIN: usize = Self::DEVICE_SHIFT_END;
    #[allow(dead_code)]
    const BUS_SHIFT_LENGTH: usize = 8;
    #[allow(dead_code)]
    const BUS_SHIFT_END: usize = Self::BUS_SHIFT_BEGIN + Self::BUS_SHIFT_LENGTH;
    const ENABLE_BIT_SHIFT: usize = 31;
    const ENABLE_BIT: u32 = 1 << Self::ENABLE_BIT_SHIFT;
    const ADDRESS_PORT: u16 = 0x0cf8;
    const VALUE_PORT: u16 = 0x0cfc;

    pub fn new(bus: u8, device: u8, function: u8) -> Self {
        assert!(device <= Self::DEVICE_MAX && function <= Self::FUNCTION_MAX);
        Self {
            bus,
            device,
            function,
        }
    }

    fn address(&self, offset: u8) -> u32 {
        let Self {
            bus,
            device,
            function
        } = self;
        Self::ENABLE_BIT + ((*bus as u32) << Self::BUS_SHIFT_BEGIN) + ((*device as u32) << Self::DEVICE_SHIFT_BEGIN) + ((*function as u32) << Self::FUNCTION_SHIFT_BEGIN) + (offset as u32)
    }

    fn read(&self, offset: u8) -> u32 {
        asm::outl(Self::ADDRESS_PORT, self.address(offset));
        asm::inl(Self::VALUE_PORT)
    }

    fn scan_device(self, address2device: &mut BTreeMap<Self, Device>) {
        let device: Option<Device> = (&self).into();
        if let Some(device) = device {
            let mut next_addresses: BTreeSet<Self> = BTreeSet::<Self>::new();
            match device.class_code {
                ClassCode::HostBridge => {
                    let bus: u8 = self.function;
                    let function: u8 = 0;
                    next_addresses
                        .extend((u8::MIN..=Self::DEVICE_MAX)
                            .map(|device| Self::new(bus, device, function))
                            .filter(|address| address != &self && !address2device.contains_key(address)));
                },
                ClassCode::PCI2PCIBridge |
                ClassCode::SubtractiveDecodePCI2PCIBridge => if let Some(secondary_bus_number) = device.type_specific.secondary_bus_number() {
                    let bus: u8 = secondary_bus_number;
                    let function: u8 = 0;
                    next_addresses
                        .extend((u8::MIN..=Self::DEVICE_MAX)
                            .map(|device| Self::new(bus, device, function))
                            .filter(|address| address != &self && !address2device.contains_key(address)));
                },
                _ => {},
            }
            if self.function == 0 && device.is_multi_function() {
                let bus: u8 = self.bus;
                let device: u8 = self.device;
                next_addresses
                    .extend((u8::MIN..=Self::FUNCTION_MAX)
                        .map(|function| Self::new(bus, device, function))
                        .filter(|address| address != &self && !address2device.contains_key(address)));
            }
            address2device.insert(self, device);
            next_addresses
                .into_iter()
                .for_each(|next_address| next_address
                    .scan_device(address2device))
        }
    }
}

const CONFIGURATION_SIZE: usize = 0x100;

impl Into<[u8; CONFIGURATION_SIZE]> for &Address {
    fn into(self) -> [u8; CONFIGURATION_SIZE] {
        (0..CONFIGURATION_SIZE)
            .step_by(mem::size_of::<u32>())
            .flat_map(|offset| self
                .read(offset as u8)
                .to_le_bytes()
                .into_iter())
            .collect::<Vec<u8>>()
            .try_into()
            .expect("Can't get a PCI configuration!")
    }
}

impl Into<Option<Device>> for &Address {
    fn into(self) -> Option<Device> {
        let configuration: [u8; CONFIGURATION_SIZE] = self.into();
        let configuration: Result<Device, ()> = configuration.try_into();
        configuration.ok()
    }
}

// PCI Express Base Specification Revision 5.0 Version 1.0 7.5.1 PCI-Compatible Configuration Registers
#[allow(dead_code)]
#[derive(Debug)]
pub struct Device {
    vendor_id: u16,
    device_id: u16,
    command: command::Register,
    status: status::Register,
    revision_id: u8,
    class_code: ClassCode,
    cache_line_size: u8,
    latency_timer: u8,
    header_type: header_type::Register,
    bist: bist::Register,
    type_specific: type_specific::Registers,
    capabilities_pointer: u8,
    interrupt_line: u8,
    interrupt_pin: u8,
}

impl Device {
    const VENDOR_ID_BEGIN: usize = 0;
    const VENDOR_ID_SIZE: usize = mem::size_of::<u16>();
    const VENDOR_ID_END: usize = Self::VENDOR_ID_BEGIN + Self::VENDOR_ID_SIZE;
    const DEVICE_ID_BEGIN: usize = Self::VENDOR_ID_END;
    const DEVICE_ID_SIZE: usize = mem::size_of::<u16>();
    const DEVICE_ID_END: usize = Self::DEVICE_ID_BEGIN + Self::DEVICE_ID_SIZE;
    const COMMAND_BEGIN: usize = Self::DEVICE_ID_END;
    const COMMAND_SIZE: usize = mem::size_of::<u16>();
    const COMMAND_END: usize = Self::COMMAND_BEGIN + Self::COMMAND_SIZE;
    const STATUS_BEGIN: usize = Self::COMMAND_END;
    const STATUS_SIZE: usize = mem::size_of::<u16>();
    const STATUS_END: usize = Self::STATUS_BEGIN + Self::STATUS_SIZE;
    const REVISION_ID_BEGIN: usize = Self::STATUS_END;
    const REVISION_ID_SIZE: usize = mem::size_of::<u8>();
    const REVISION_ID_END: usize = Self::REVISION_ID_BEGIN + Self::REVISION_ID_SIZE;
    const PROGRAMMING_INTERFACE_BEGIN: usize = Self::REVISION_ID_END;
    const PROGRAMMING_INTERFACE_SIZE: usize = mem::size_of::<u8>();
    const PROGRAMMING_INTERFACE_END: usize = Self::PROGRAMMING_INTERFACE_BEGIN + Self::PROGRAMMING_INTERFACE_SIZE;
    const SUB_CLASS_BEGIN: usize = Self::PROGRAMMING_INTERFACE_END;
    const SUB_CLASS_SIZE: usize = mem::size_of::<u8>();
    const SUB_CLASS_END: usize = Self::SUB_CLASS_BEGIN + Self::SUB_CLASS_SIZE;
    const BASE_CLASS_BEGIN: usize = Self::SUB_CLASS_END;
    const BASE_CLASS_SIZE: usize = mem::size_of::<u8>();
    const BASE_CLASS_END: usize = Self::BASE_CLASS_BEGIN + Self::BASE_CLASS_SIZE;
    const CACHE_LINE_SIZE_BEGIN: usize = Self::BASE_CLASS_END;
    const CACHE_LINE_SIZE_SIZE: usize = mem::size_of::<u8>();
    const CACHE_LINE_SIZE_END: usize = Self::CACHE_LINE_SIZE_BEGIN + Self::CACHE_LINE_SIZE_SIZE;
    const LATENCY_TIMER_BEGIN: usize = Self::CACHE_LINE_SIZE_END;
    const LATENCY_TIMER_SIZE: usize = mem::size_of::<u8>();
    const LATENCY_TIMER_END: usize = Self::LATENCY_TIMER_BEGIN + Self::LATENCY_TIMER_SIZE;
    const HEADER_TYPE_BEGIN: usize = Self::LATENCY_TIMER_END;
    const HEADER_TYPE_SIZE: usize = mem::size_of::<u8>();
    const HEADER_TYPE_END: usize = Self::HEADER_TYPE_BEGIN + Self::HEADER_TYPE_SIZE;
    const BIST_BEGIN: usize = Self::HEADER_TYPE_END;
    const BIST_SIZE: usize = mem::size_of::<u8>();
    const BIST_END: usize = Self::BIST_BEGIN + Self::BIST_SIZE;
    const CAPABILITIES_POINTER_BEGIN: usize = 0x34;
    #[allow(dead_code)]
    const CAPABILITIES_POINTER_SIZE: usize = mem::size_of::<u8>();
    #[allow(dead_code)]
    const CAPABILITIES_POINTER_END: usize = Self::CAPABILITIES_POINTER_BEGIN + Self::CAPABILITIES_POINTER_SIZE;
    const INTERRUPT_LINE_BEGIN: usize = 0x3c;
    #[allow(dead_code)]
    const INTERRUPT_LINE_SIZE: usize = mem::size_of::<u8>();
    #[allow(dead_code)]
    const INTERRUPT_LINE_END: usize = Self::INTERRUPT_LINE_BEGIN + Self::INTERRUPT_LINE_SIZE;
    const INTERRUPT_PIN_BEGIN: usize = 0x3d;
    const INTERRUPT_PIN_SIZE: usize = mem::size_of::<u8>();
    const INTERRUPT_PIN_END: usize = Self::INTERRUPT_PIN_BEGIN + Self::INTERRUPT_PIN_SIZE;

    pub fn get_all_devices() -> BTreeMap<Address, Self> {
        let mut address2device = BTreeMap::<Address, Self>::new();
        Address::new(0, 0, 0).scan_device(&mut address2device);
        address2device
    }

    fn is_multi_function(&self) -> bool {
        self.header_type.is_multi_function()
    }
}

impl TryFrom<[u8; CONFIGURATION_SIZE]> for Device {
    type Error = ();

    fn try_from(configuration: [u8; CONFIGURATION_SIZE]) -> Result<Self, Self::Error> {
        let vendor_id: [u8; Self::VENDOR_ID_SIZE] = configuration[Self::VENDOR_ID_BEGIN..Self::VENDOR_ID_END]
            .try_into()
            .expect("Can't get a PCI configuration!");
        let vendor_id: u16 = u16::from_le_bytes(vendor_id);
        match vendor_id {
            0xffffu16 => Err(()),
            vendor_id => {
                let device_id: [u8; Self::DEVICE_ID_SIZE] = configuration[Self::DEVICE_ID_BEGIN..Self::DEVICE_ID_END]
                    .try_into()
                    .expect("Can't get a PCI configuration!");
                let device_id: u16 = u16::from_le_bytes(device_id);
                let command: [u8; Self::COMMAND_SIZE] = configuration[Self::COMMAND_BEGIN..Self::COMMAND_END]
                    .try_into()
                    .expect("Can't get a PCI configuration!");
                let command: command::Register = u16::from_le_bytes(command).into();
                let status: [u8; Self::STATUS_SIZE] = configuration[Self::STATUS_BEGIN..Self::STATUS_END]
                    .try_into()
                    .expect("Can't get a PCI configuration!");
                let status: status::Register = u16::from_le_bytes(status).into();
                let revision_id: u8 = configuration[Self::REVISION_ID_BEGIN];
                let programming_interface: u8 = configuration[Self::PROGRAMMING_INTERFACE_BEGIN];
                let sub_class: u8 = configuration[Self::SUB_CLASS_BEGIN];
                let base_class: u8 = configuration[Self::BASE_CLASS_BEGIN];
                let class_code: ClassCode = ClassCode::new(base_class, sub_class, programming_interface);
                let cache_line_size: u8 = configuration[Self::CACHE_LINE_SIZE_BEGIN];
                let latency_timer: u8 = configuration[Self::LATENCY_TIMER_BEGIN];
                let header_type: header_type::Register = configuration[Self::HEADER_TYPE_BEGIN].into();
                let bist: bist::Register = configuration[Self::BIST_BEGIN].into();
                let type_specific = type_specific::Registers::new(header_type.header_layout(), &configuration);
                let capabilities_pointer: u8 = configuration[Self::CAPABILITIES_POINTER_BEGIN];
                let interrupt_line: u8 = configuration[Self::INTERRUPT_LINE_BEGIN];
                let interrupt_pin: u8 = configuration[Self::INTERRUPT_PIN_BEGIN];
                Ok(Self {
                    vendor_id,
                    device_id,
                    command,
                    status,
                    revision_id,
                    class_code,
                    cache_line_size,
                    latency_timer,
                    header_type,
                    bist,
                    type_specific,
                    capabilities_pointer,
                    interrupt_line,
                    interrupt_pin,
                })
            },
        }
    }
}

// https://pcisig.com/sites/default/files/files/PCI_Code-ID_r_1_11__v24_Jan_2019.pdf
#[allow(dead_code)]
#[derive(Debug)]
enum ClassCode {
    AllCurrentlyImplemented,                    // 00 00 00
    VGACompatibleDevice,                        // 00 01 00
    SCSI {
        programming_interface: u8,
    },                                          // 01 00 xx
    IDE {
        programming_interface: u8,
    },                                          // 01 01 xx
    FloppyDisk,                                 // 01 02 00
    IPIBus,                                     // 01 03 00
    RAID,                                       // 01 04 00
    ATASingleStepping,                          // 01 05 20
    ATAContinuousOperation,                     // 01 05 30
    SerialATAVendorSpecific,                    // 01 06 00
    SerialATAAHCI,                              // 01 06 01
    SerialStorage,                              // 01 06 02
    SerialAttachedSCSI,                         // 01 07 00
    Obsolete,                                   // 01 07 01
    NoneVolatileMemorySubsystemVendorSpecific,  // 01 08 00
    NoneVolatileMemorySubsystemNVMHCI,          // 01 08 01
    NVMExpressIO,                               // 01 08 02
    NVMExpressAdministrative,                   // 01 08 03
    UniversalFlashStorageVendorSpecific,        // 01 09 00
    UniversalFlashStorageHostController,        // 01 09 01
    OtherMassStorage,                           // 01 80 00
    Ethernet,                                   // 02 00 00
    TokenRing,                                  // 02 01 00
    FDDI,                                       // 02 02 00
    ATM,                                        // 02 03 00
    ISDN,                                       // 02 04 00
    WorldFip,                                   // 02 05 00
    PICMG {
        programming_interface: u8,
    },                                          // 02 06 xx
    InfiniBand,                                 // 02 07 00
    HostFabric,                                 // 02 08 00
    OtherNetwork,                               // 02 80 00
    VGACompatibleController,                    // 03 00 00
    Display8514Compatible,                      // 03 00 01
    XGA,                                        // 03 01 00
    Display3D,                                  // 03 02 00
    OtherDisplay,                               // 03 80 00
    Video,                                      // 04 00 00
    AudioDevice,                                // 04 01 00
    ComputerTelephony,                          // 04 02 00
    HighDefinitionAudio,                        // 04 03 00
    HighDefinitionAudioVendorSpecific,          // 04 03 80
    OtherMultimedia,                            // 04 80 00
    RAM,                                        // 05 00 00
    Flash,                                      // 05 01 00
    OtherMemory,                                // 05 80 00
    HostBridge,                                 // 06 00 00
    ISABridge,                                  // 06 01 00
    EISABridge,                                 // 06 02 00
    MCABridge,                                  // 06 03 00
    PCI2PCIBridge,                              // 06 04 00
    SubtractiveDecodePCI2PCIBridge,             // 06 04 01
    PCMCIABridge,                               // 06 05 00
    NuBusBridge,                                // 06 06 00
    CardBusBridge,                              // 06 07 00
    RACEwayBridge {
        programming_interface: u8,
    },                                          // 06 08 xx
    SemiTransparentPCI2PCIBridgePrimary,        // 06 09 40
    SemiTransparentPCI2PCIBridgeSecondary,      // 06 09 80
    InfiniBand2PCIHostBridge,                   // 06 0a 00
    AdvancedSwitching2PCIHostBridgeCustom,      // 06 0b 00
    AdvancedSwitching2PCIHostBridgeASISIG,      // 06 0b 01
    OtherBridge,                                // 06 80 00
    SerialGenericXTCompatible,                  // 07 00 00
    Serial16450Compatible,                      // 07 00 01
    Serial16550Compatible,                      // 07 00 02
    Serial16650Compatible,                      // 07 00 03
    Serial16750Compatible,                      // 07 00 04
    Serial16850Compatible,                      // 07 00 05
    Serial16950Compatible,                      // 07 00 06
    ParallelPort,                               // 07 01 00
    BidirectionalParallelPort,                  // 07 01 01
    ECP1XCompliantParallelPort,                 // 07 01 02
    IEEE1284Controller,                         // 07 01 03
    IEEE1284TargetDevice,                       // 07 01 fe
    MultiportSerial,                            // 07 02 00
    GenericModem,                               // 07 03 00
    HayesCompatibleModem16450Compatible,        // 07 03 01
    HayesCompatibleModem16550Compatible,        // 07 03 02
    HayesCompatibleModem16650Compatible,        // 07 03 03
    HayesCompatibleModem16750Compatible,        // 07 03 04
    GPIB,                                       // 07 04 00
    SmartCard,                                  // 07 05 00
    OtherCommunication,                         // 07 80 00
    Generic8259PIC,                             // 08 00 00
    ISAPIC,                                     // 08 00 01
    EISAPIC,                                    // 08 00 02
    IOAPICInterrupt,                            // 08 00 10
    IOxAPICInterrupt,                           // 08 00 20
    Generic8237DMA,                             // 08 01 00
    ISADMA,                                     // 08 01 01
    EISADMA,                                    // 08 01 02
    Generic8254SystemTimer,                     // 08 02 00
    ISASystemTimer,                             // 08 02 01
    EISASystemTimer,                            // 08 02 02
    HighPerformanceEventTimer,                  // 08 02 03
    GenericRTC,                                 // 08 03 00
    ISARTC,                                     // 08 03 01
    GenericPCIHotPlug,                          // 08 04 00
    SDHost,                                     // 08 05 00
    IOMMU,                                      // 08 06 00
    RootComplexEvent,                           // 08 07 00
    OtherSystemPeripheral,                      // 08 80 00
    Kerboard,                                   // 09 00 00
    Digitizer,                                  // 09 01 00
    Mouse,                                      // 09 02 00
    Scanner,                                    // 09 03 00
    GenericGameport,                            // 09 04 00
    Gameport,                                   // 09 04 10
    OtherInput,                                 // 09 80 00
    GenericDockingStation,                      // 0a 00 00
    OtherDockingStation,                        // 0a 80 00
    Processor386,                               // 0b 00 00
    Processor486,                               // 0b 01 00
    Pentium,                                    // 0b 02 00
    Alpha,                                      // 0b 10 00
    PowerPC,                                    // 0b 20 00
    MIPS,                                       // 0b 30 00
    Coprocessor,                                // 0b 40 00
    OtherProcessor,                             // 0b 80 00
    IEEE1394,                                   // 0c 00 00
    IEEE1394OpenHCI,                            // 0c 00 10
    ACCESSBus,                                  // 0c 01 00
    SSA,                                        // 0c 02 00
    USBUHCI,                                    // 0c 03 00
    USBOHCI,                                    // 0c 03 10
    USBEHCI,                                    // 0c 03 20
    USBxHCI,                                    // 0c 03 30
    USBNoSpecificProgrammingInterface,          // 0c 03 80
    USBNoHostController,                        // 0c 03 fe
    FibreChannel,                               // 0c 04 00
    SMBus,                                      // 0c 05 00
    InfiniBandDeprecated,                       // 0c 06 00
    IPMISMIC,                                   // 0c 07 00
    IPMIKeyboardControllerStyle,                // 0c 07 01
    IPMIBlockTransfer,                          // 0c 07 02
    SERCOS,                                     // 0c 08 00
    CANbus,                                     // 0c 09 00
    MIPII3C,                                    // 0c 0a 00
    OtherSerialBus,                             // 0c 80 00
    IRDA,                                       // 0d 00 00
    ConsumerIR,                                 // 0d 01 00
    UWBRadio,                                   // 0d 01 10
    RF,                                         // 0d 10 00
    Bluetooth,                                  // 0d 11 00
    Broadband,                                  // 0d 12 00
    Ethernet80211a,                             // 0d 20 00
    Ethernet20811b,                             // 0d 21 00
    Cellular,                                   // 0d 40 00
    CellularPlusEthernet,                       // 0d 41 00
    OtherWireless,                              // 0d 80 00
    IntelligentIO {
        programming_interface: u8,
    },                                          // 0e 00 xx
    MessageFIFO,                                // 0e 00 00
    TV,                                         // 0f 01 00
    Audio,                                      // 0f 02 00
    Voice,                                      // 0f 03 00
    Data,                                       // 0f 04 00
    OtherSatelliteCommunication,                // 0f 80 00
    NetworkAndComputingEncryptionAndDecryption, // 10 00 00
    EntertainmentEncryptionAndDecryption,       // 10 10 00
    OtherEncryptionAndDecryption,               // 10 80 00
    DPIO,                                       // 11 00 00
    PerformanceCounter,                         // 11 01 00
    CommunicationSynchronizationPlusTime,       // 11 10 00
    ManagementCard,                             // 11 20 00
    OtherDataAcquisitionAndSignalProcessing,    // 11 80 00
    ProcessingAccelerator,                      // 12 00 00
    NonEssentialInstrumentationFunction,        // 13 00 00
    Other {
        base_class: u8,
        sub_class: u8,
        programming_interface: u8,
    },
}

impl ClassCode {
    fn new(base_class: u8, sub_class: u8, programming_interface: u8) -> Self {
        match (base_class, sub_class, programming_interface) {
            (0x00, 0x00, 0x00) => Self::AllCurrentlyImplemented,                    // 00 00 00
            (0x00, 0x01, 0x00) => Self::VGACompatibleDevice,                        // 00 01 00
            (0x01, 0x00, programming_interface) => Self::SCSI {
                programming_interface,
            },                                                                      // 01 00 xx
            (0x01, 0x01, programming_interface) => Self::IDE {
                programming_interface,
            },                                                                      // 01 01 xx
            (0x01, 0x02, 0x00) => Self::FloppyDisk,                                 // 01 02 00
            (0x01, 0x03, 0x00) => Self::IPIBus,                                     // 01 03 00
            (0x01, 0x04, 0x00) => Self::RAID,                                       // 01 04 00
            (0x01, 0x05, 0x20) => Self::ATASingleStepping,                          // 01 05 20
            (0x01, 0x05, 0x30) => Self::ATAContinuousOperation,                     // 01 05 30
            (0x01, 0x06, 0x00) => Self::SerialATAVendorSpecific,                    // 01 06 00
            (0x01, 0x06, 0x01) => Self::SerialATAAHCI,                              // 01 06 01
            (0x01, 0x06, 0x02) => Self::SerialStorage,                              // 01 06 02
            (0x01, 0x07, 0x00) => Self::SerialAttachedSCSI,                         // 01 07 00
            (0x01, 0x07, 0x01) => Self::Obsolete,                                   // 01 07 01
            (0x01, 0x08, 0x00) => Self::NoneVolatileMemorySubsystemVendorSpecific,  // 01 08 00
            (0x01, 0x08, 0x01) => Self::NoneVolatileMemorySubsystemNVMHCI,          // 01 08 01
            (0x01, 0x08, 0x02) => Self::NVMExpressIO,                               // 01 08 02
            (0x01, 0x08, 0x03) => Self::NVMExpressAdministrative,                   // 01 08 03
            (0x01, 0x09, 0x00) => Self::UniversalFlashStorageVendorSpecific,        // 01 09 00
            (0x01, 0x09, 0x01) => Self::UniversalFlashStorageHostController,        // 01 09 01
            (0x01, 0x80, 0x00) => Self::OtherMassStorage,                           // 01 80 00
            (0x02, 0x00, 0x00) => Self::Ethernet,                                   // 02 00 00
            (0x02, 0x01, 0x00) => Self::TokenRing,                                  // 02 01 00
            (0x02, 0x02, 0x00) => Self::FDDI,                                       // 02 02 00
            (0x02, 0x03, 0x00) => Self::ATM,                                        // 02 03 00
            (0x02, 0x04, 0x00) => Self::ISDN,                                       // 02 04 00
            (0x02, 0x05, 0x00) => Self::WorldFip,                                   // 02 05 00
            (0x02, 0x06, programming_interface) => Self::PICMG {
                programming_interface
            },                                                                      // 02 06 xx
            (0x02, 0x07, 0x00) => Self::InfiniBand,                                 // 02 07 00
            (0x02, 0x08, 0x00) => Self::HostFabric,                                 // 02 08 00
            (0x02, 0x80, 0x00) => Self::OtherNetwork,                               // 02 80 00
            (0x03, 0x00, 0x00) => Self::VGACompatibleController,                    // 03 00 00
            (0x03, 0x00, 0x01) => Self::Display8514Compatible,                      // 03 00 01
            (0x03, 0x01, 0x00) => Self::XGA,                                        // 03 01 00
            (0x03, 0x02, 0x00) => Self::Display3D,                                  // 03 02 00
            (0x03, 0x80, 0x00) => Self::OtherDisplay,                               // 03 80 00
            (0x04, 0x00, 0x00) => Self::Video,                                      // 04 00 00
            (0x04, 0x01, 0x00) => Self::AudioDevice,                                // 04 01 00
            (0x04, 0x02, 0x00) => Self::ComputerTelephony,                          // 04 02 00
            (0x04, 0x03, 0x00) => Self::HighDefinitionAudio,                        // 04 03 00
            (0x04, 0x03, 0x80) => Self::HighDefinitionAudioVendorSpecific,          // 04 03 80
            (0x04, 0x80, 0x00) => Self::OtherMultimedia,                            // 04 80 00
            (0x05, 0x00, 0x00) => Self::RAM,                                        // 05 00 00
            (0x05, 0x01, 0x00) => Self::Flash,                                      // 05 01 00
            (0x05, 0x80, 0x00) => Self::OtherMemory,                                // 05 80 00
            (0x06, 0x00, 0x00) => Self::HostBridge,                                 // 06 00 00
            (0x06, 0x01, 0x00) => Self::ISABridge,                                  // 06 01 00
            (0x06, 0x02, 0x00) => Self::EISABridge,                                 // 06 02 00
            (0x06, 0x03, 0x00) => Self::MCABridge,                                  // 06 03 00
            (0x06, 0x04, 0x00) => Self::PCI2PCIBridge,                              // 06 04 00
            (0x06, 0x04, 0x01) => Self::SubtractiveDecodePCI2PCIBridge,             // 06 04 01
            (0x06, 0x05, 0x00) => Self::PCMCIABridge,                               // 06 05 00
            (0x06, 0x06, 0x00) => Self::NuBusBridge,                                // 06 06 00
            (0x06, 0x07, 0x00) => Self::CardBusBridge,                              // 06 07 00
            (0x06, 0x08, programming_interface) => Self::RACEwayBridge {
                programming_interface,
            },                                                                      // 06 08 xx
            (0x06, 0x09, 0x40) => Self::SemiTransparentPCI2PCIBridgePrimary,        // 06 09 40
            (0x06, 0x09, 0x80) => Self::SemiTransparentPCI2PCIBridgeSecondary,      // 06 09 80
            (0x06, 0x0a, 0x00) => Self::InfiniBand2PCIHostBridge,                   // 06 0a 00
            (0x06, 0x0b, 0x00) => Self::AdvancedSwitching2PCIHostBridgeCustom,      // 06 0b 00
            (0x06, 0x0b, 0x01) => Self::AdvancedSwitching2PCIHostBridgeASISIG,      // 06 0b 01
            (0x06, 0x80, 0x00) => Self::OtherBridge,                                // 06 80 00
            (0x07, 0x00, 0x00) => Self::SerialGenericXTCompatible,                  // 07 00 00
            (0x07, 0x00, 0x01) => Self::Serial16450Compatible,                      // 07 00 01
            (0x07, 0x00, 0x02) => Self::Serial16550Compatible,                      // 07 00 02
            (0x07, 0x00, 0x03) => Self::Serial16650Compatible,                      // 07 00 03
            (0x07, 0x00, 0x04) => Self::Serial16750Compatible,                      // 07 00 04
            (0x07, 0x00, 0x05) => Self::Serial16850Compatible,                      // 07 00 05
            (0x07, 0x00, 0x06) => Self::Serial16950Compatible,                      // 07 00 06
            (0x07, 0x01, 0x00) => Self::ParallelPort,                               // 07 01 00
            (0x07, 0x01, 0x01) => Self::BidirectionalParallelPort,                  // 07 01 01
            (0x07, 0x01, 0x02) => Self::ECP1XCompliantParallelPort,                 // 07 01 02
            (0x07, 0x01, 0x03) => Self::IEEE1284Controller,                         // 07 01 03
            (0x07, 0x01, 0xfe) => Self::IEEE1284TargetDevice,                       // 07 01 fe
            (0x07, 0x02, 0x00) => Self::MultiportSerial,                            // 07 02 00
            (0x07, 0x03, 0x00) => Self::GenericModem,                               // 07 03 00
            (0x07, 0x03, 0x01) => Self::HayesCompatibleModem16450Compatible,        // 07 03 01
            (0x07, 0x03, 0x02) => Self::HayesCompatibleModem16550Compatible,        // 07 03 01
            (0x07, 0x03, 0x03) => Self::HayesCompatibleModem16650Compatible,        // 07 03 01
            (0x07, 0x03, 0x04) => Self::HayesCompatibleModem16750Compatible,        // 07 03 01
            (0x07, 0x04, 0x00) => Self::GPIB,                                       // 07 04 00
            (0x07, 0x05, 0x00) => Self::SmartCard,                                  // 07 05 00
            (0x07, 0x80, 0x00) => Self::OtherCommunication,                         // 07 80 00
            (0x08, 0x00, 0x00) => Self::Generic8259PIC,                             // 08 00 00
            (0x08, 0x00, 0x01) => Self::ISAPIC,                                     // 08 00 01
            (0x08, 0x00, 0x02) => Self::EISAPIC,                                    // 08 00 02
            (0x08, 0x00, 0x10) => Self::IOAPICInterrupt,                            // 08 00 10
            (0x08, 0x00, 0x20) => Self::IOxAPICInterrupt,                           // 08 00 20
            (0x08, 0x01, 0x00) => Self::Generic8237DMA,                             // 08 01 00
            (0x08, 0x01, 0x01) => Self::ISADMA,                                     // 08 01 01
            (0x08, 0x01, 0x02) => Self::EISADMA,                                    // 08 01 02
            (0x08, 0x02, 0x00) => Self::Generic8254SystemTimer,                     // 08 02 00
            (0x08, 0x02, 0x01) => Self::ISASystemTimer,                             // 08 02 01
            (0x08, 0x02, 0x02) => Self::EISASystemTimer,                            // 08 02 02
            (0x08, 0x02, 0x03) => Self::HighPerformanceEventTimer,                  // 08 02 03
            (0x08, 0x03, 0x00) => Self::GenericRTC,                                 // 08 03 00
            (0x08, 0x03, 0x01) => Self::ISARTC,                                     // 08 03 01
            (0x08, 0x04, 0x00) => Self::GenericPCIHotPlug,                          // 08 04 00
            (0x08, 0x05, 0x00) => Self::SDHost,                                     // 08 05 00
            (0x08, 0x06, 0x00) => Self::IOMMU,                                      // 08 06 00
            (0x08, 0x07, 0x00) => Self::RootComplexEvent,                           // 08 07 00
            (0x08, 0x80, 0x00) => Self::OtherSystemPeripheral,                      // 08 80 00
            (0x09, 0x00, 0x00) => Self::Kerboard,                                   // 09 00 00
            (0x09, 0x01, 0x00) => Self::Digitizer,                                  // 09 01 00
            (0x09, 0x02, 0x00) => Self::Mouse,                                      // 09 02 00
            (0x09, 0x03, 0x00) => Self::Scanner,                                    // 09 03 00
            (0x09, 0x04, 0x00) => Self::GenericGameport,                            // 09 04 00
            (0x09, 0x04, 0x10) => Self::Gameport,                                   // 09 04 10
            (0x09, 0x80, 0x00) => Self::OtherInput,                                 // 09 80 00
            (0x0a, 0x00, 0x00) => Self::GenericDockingStation,                      // 0a 00 00
            (0x0a, 0x80, 0x00) => Self::OtherDockingStation,                        // 0a 80 00
            (0x0b, 0x00, 0x00) => Self::Processor386,                               // 0b 00 00
            (0x0b, 0x01, 0x00) => Self::Processor486,                               // 0b 01 00
            (0x0b, 0x02, 0x00) => Self::Pentium,                                    // 0b 02 00
            (0x0b, 0x10, 0x00) => Self::Alpha,                                      // 0b 10 00
            (0x0b, 0x20, 0x00) => Self::PowerPC,                                    // 0b 20 00
            (0x0b, 0x30, 0x00) => Self::MIPS,                                       // 0b 30 00
            (0x0b, 0x40, 0x00) => Self::Coprocessor,                                // 0b 40 00
            (0x0b, 0x80, 0x00) => Self::OtherProcessor,                             // 0b 80 00
            (0x0c, 0x00, 0x00) => Self::IEEE1394,                                   // 0c 00 00
            (0x0c, 0x00, 0x10) => Self::IEEE1394OpenHCI,                            // 0c 00 10
            (0x0c, 0x01, 0x00) => Self::ACCESSBus,                                  // 0c 01 00
            (0x0c, 0x02, 0x00) => Self::SSA,                                        // 0c 02 00
            (0x0c, 0x03, 0x00) => Self::USBUHCI,                                    // 0c 03 00
            (0x0c, 0x03, 0x10) => Self::USBOHCI,                                    // 0c 03 10
            (0x0c, 0x03, 0x20) => Self::USBEHCI,                                    // 0c 03 20
            (0x0c, 0x03, 0x30) => Self::USBxHCI,                                    // 0c 03 30
            (0x0c, 0x03, 0x80) => Self::USBNoSpecificProgrammingInterface,          // 0c 03 80
            (0x0c, 0x03, 0xfe) => Self::USBNoHostController,                        // 0c 03 fe
            (0x0c, 0x04, 0x00) => Self::FibreChannel,                               // 0c 04 00
            (0x0c, 0x05, 0x00) => Self::SMBus,                                      // 0c 05 00
            (0x0c, 0x06, 0x00) => Self::InfiniBandDeprecated,                       // 0c 06 00
            (0x0c, 0x07, 0x00) => Self::IPMISMIC,                                   // 0c 07 00
            (0x0c, 0x07, 0x01) => Self::IPMIKeyboardControllerStyle,                // 0c 07 01
            (0x0c, 0x07, 0x02) => Self::IPMIBlockTransfer,                          // 0c 07 02
            (0x0c, 0x08, 0x00) => Self::SERCOS,                                     // 0c 08 00
            (0x0c, 0x09, 0x00) => Self::CANbus,                                     // 0c 09 00
            (0x0c, 0x0a, 0x00) => Self::MIPII3C,                                    // 0c 0a 00
            (0x0c, 0x80, 0x00) => Self::OtherSerialBus,                             // 0c 80 00
            (0x0d, 0x00, 0x00) => Self::IRDA,                                       // 0d 00 00
            (0x0d, 0x01, 0x00) => Self::ConsumerIR,                                 // 0d 01 00
            (0x0d, 0x01, 0x10) => Self::UWBRadio,                                   // 0d 01 10
            (0x0d, 0x10, 0x00) => Self::RF,                                         // 0d 10 00
            (0x0d, 0x11, 0x00) => Self::Bluetooth,                                  // 0d 11 00
            (0x0d, 0x12, 0x00) => Self::Broadband,                                  // 0d 12 00
            (0x0d, 0x20, 0x00) => Self::Ethernet80211a,                             // 0d 20 00
            (0x0d, 0x21, 0x00) => Self::Ethernet20811b,                             // 0d 21 00
            (0x0d, 0x40, 0x00) => Self::Cellular,                                   // 0d 40 00
            (0x0d, 0x41, 0x00) => Self::CellularPlusEthernet,                       // 0d 41 00
            (0x0d, 0x80, 0x00) => Self::OtherWireless,                              // 0d 80 00
            (0x0e, 0x00, 0x00) => Self::MessageFIFO,                                // 0e 00 00
            (0x0e, 0x00, programming_interface) => Self::IntelligentIO {
                programming_interface,
            },                                                                      // 0e 00 xx
            (0x0f, 0x01, 0x00) => Self::TV,                                         // 0f 01 00
            (0x0f, 0x02, 0x00) => Self::Audio,                                      // 0f 02 00
            (0x0f, 0x03, 0x00) => Self::Voice,                                      // 0f 03 00
            (0x0f, 0x04, 0x00) => Self::Data,                                       // 0f 04 00
            (0x0f, 0x80, 0x00) => Self::OtherSatelliteCommunication,                // 0f 80 00
            (0x10, 0x00, 0x00) => Self::NetworkAndComputingEncryptionAndDecryption, // 10 00 00
            (0x10, 0x10, 0x00) => Self::EntertainmentEncryptionAndDecryption,       // 10 10 00
            (0x10, 0x80, 0x00) => Self::OtherEncryptionAndDecryption,               // 10 80 00
            (0x11, 0x00, 0x00) => Self::DPIO,                                       // 11 00 00
            (0x11, 0x01, 0x00) => Self::PerformanceCounter,                         // 11 01 00
            (0x11, 0x10, 0x00) => Self::CommunicationSynchronizationPlusTime,       // 11 10 00
            (0x11, 0x20, 0x00) => Self::ManagementCard,                             // 11 20 00
            (0x11, 0x80, 0x00) => Self::OtherDataAcquisitionAndSignalProcessing,    // 11 80 00
            (0x12, 0x00, 0x00) => Self::ProcessingAccelerator,                      // 12 00 00
            (0x13, 0x00, 0x00) => Self::NonEssentialInstrumentationFunction,        // 13 00 00
            (base_class, sub_class, programming_interface) => Self::Other {
                base_class,
                sub_class,
                programming_interface,
            },
        }
    }
}

