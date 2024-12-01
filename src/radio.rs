#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_txen: TasksTxen,
    tasks_rxen: TasksRxen,
    tasks_start: TasksStart,
    tasks_stop: TasksStop,
    tasks_disable: TasksDisable,
    tasks_rssistart: TasksRssistart,
    tasks_rssistop: TasksRssistop,
    tasks_bcstart: TasksBcstart,
    tasks_bcstop: TasksBcstop,
    tasks_edstart: TasksEdstart,
    tasks_edstop: TasksEdstop,
    tasks_ccastart: TasksCcastart,
    tasks_ccastop: TasksCcastop,
    _reserved13: [u8; 0xcc],
    events_ready: EventsReady,
    events_address: EventsAddress,
    events_payload: EventsPayload,
    events_end: EventsEnd,
    events_disabled: EventsDisabled,
    events_devmatch: EventsDevmatch,
    events_devmiss: EventsDevmiss,
    events_rssiend: EventsRssiend,
    _reserved21: [u8; 0x08],
    events_bcmatch: EventsBcmatch,
    _reserved22: [u8; 0x04],
    events_crcok: EventsCrcok,
    events_crcerror: EventsCrcerror,
    events_framestart: EventsFramestart,
    events_edend: EventsEdend,
    events_edstopped: EventsEdstopped,
    events_ccaidle: EventsCcaidle,
    events_ccabusy: EventsCcabusy,
    events_ccastopped: EventsCcastopped,
    events_rateboost: EventsRateboost,
    events_txready: EventsTxready,
    events_rxready: EventsRxready,
    events_mhrmatch: EventsMhrmatch,
    _reserved34: [u8; 0x08],
    events_sync: EventsSync,
    events_phyend: EventsPhyend,
    events_ctepresent: EventsCtepresent,
    _reserved37: [u8; 0x8c],
    shorts: Shorts,
    _reserved38: [u8; 0x0100],
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved40: [u8; 0xf4],
    crcstatus: Crcstatus,
    _reserved41: [u8; 0x04],
    rxmatch: Rxmatch,
    rxcrc: Rxcrc,
    dai: Dai,
    pdustat: Pdustat,
    _reserved45: [u8; 0x34],
    ctestatus: Ctestatus,
    _reserved46: [u8; 0x08],
    dfestatus: Dfestatus,
    _reserved47: [u8; 0xa8],
    packetptr: Packetptr,
    frequency: Frequency,
    txpower: Txpower,
    mode: Mode,
    pcnf0: Pcnf0,
    pcnf1: Pcnf1,
    base0: Base0,
    base1: Base1,
    prefix0: Prefix0,
    prefix1: Prefix1,
    txaddress: Txaddress,
    rxaddresses: Rxaddresses,
    crccnf: Crccnf,
    crcpoly: Crcpoly,
    crcinit: Crcinit,
    _reserved62: [u8; 0x04],
    tifs: Tifs,
    rssisample: Rssisample,
    _reserved64: [u8; 0x04],
    state: State,
    datawhiteiv: Datawhiteiv,
    _reserved66: [u8; 0x08],
    bcc: Bcc,
    _reserved67: [u8; 0x9c],
    dab: [Dab; 8],
    dap: [Dap; 8],
    dacnf: Dacnf,
    mhrmatchconf: Mhrmatchconf,
    mhrmatchmas: Mhrmatchmas,
    _reserved72: [u8; 0x04],
    modecnf0: Modecnf0,
    _reserved73: [u8; 0x0c],
    sfd: Sfd,
    edcnt: Edcnt,
    edsample: Edsample,
    ccactrl: Ccactrl,
    _reserved77: [u8; 0x0290],
    dfemode: Dfemode,
    cteinlineconf: Cteinlineconf,
    _reserved79: [u8; 0x08],
    dfectrl1: Dfectrl1,
    dfectrl2: Dfectrl2,
    _reserved81: [u8; 0x10],
    switchpattern: Switchpattern,
    clearpattern: Clearpattern,
    psel: Psel,
    dfepacket: Dfepacket,
    _reserved85: [u8; 0x06a0],
    power: Power,
}
impl RegisterBlock {
    #[doc = "0x00 - Enable RADIO in TX mode"]
    #[inline(always)]
    pub const fn tasks_txen(&self) -> &TasksTxen {
        &self.tasks_txen
    }
    #[doc = "0x04 - Enable RADIO in RX mode"]
    #[inline(always)]
    pub const fn tasks_rxen(&self) -> &TasksRxen {
        &self.tasks_rxen
    }
    #[doc = "0x08 - Start RADIO"]
    #[inline(always)]
    pub const fn tasks_start(&self) -> &TasksStart {
        &self.tasks_start
    }
    #[doc = "0x0c - Stop RADIO"]
    #[inline(always)]
    pub const fn tasks_stop(&self) -> &TasksStop {
        &self.tasks_stop
    }
    #[doc = "0x10 - Disable RADIO"]
    #[inline(always)]
    pub const fn tasks_disable(&self) -> &TasksDisable {
        &self.tasks_disable
    }
    #[doc = "0x14 - Start the RSSI and take one single sample of the receive signal strength"]
    #[inline(always)]
    pub const fn tasks_rssistart(&self) -> &TasksRssistart {
        &self.tasks_rssistart
    }
    #[doc = "0x18 - Stop the RSSI measurement"]
    #[inline(always)]
    pub const fn tasks_rssistop(&self) -> &TasksRssistop {
        &self.tasks_rssistop
    }
    #[doc = "0x1c - Start the bit counter"]
    #[inline(always)]
    pub const fn tasks_bcstart(&self) -> &TasksBcstart {
        &self.tasks_bcstart
    }
    #[doc = "0x20 - Stop the bit counter"]
    #[inline(always)]
    pub const fn tasks_bcstop(&self) -> &TasksBcstop {
        &self.tasks_bcstop
    }
    #[doc = "0x24 - Start the energy detect measurement used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub const fn tasks_edstart(&self) -> &TasksEdstart {
        &self.tasks_edstart
    }
    #[doc = "0x28 - Stop the energy detect measurement"]
    #[inline(always)]
    pub const fn tasks_edstop(&self) -> &TasksEdstop {
        &self.tasks_edstop
    }
    #[doc = "0x2c - Start the clear channel assessment used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub const fn tasks_ccastart(&self) -> &TasksCcastart {
        &self.tasks_ccastart
    }
    #[doc = "0x30 - Stop the clear channel assessment"]
    #[inline(always)]
    pub const fn tasks_ccastop(&self) -> &TasksCcastop {
        &self.tasks_ccastop
    }
    #[doc = "0x100 - RADIO has ramped up and is ready to be started"]
    #[inline(always)]
    pub const fn events_ready(&self) -> &EventsReady {
        &self.events_ready
    }
    #[doc = "0x104 - Address sent or received"]
    #[inline(always)]
    pub const fn events_address(&self) -> &EventsAddress {
        &self.events_address
    }
    #[doc = "0x108 - Packet payload sent or received"]
    #[inline(always)]
    pub const fn events_payload(&self) -> &EventsPayload {
        &self.events_payload
    }
    #[doc = "0x10c - Packet sent or received"]
    #[inline(always)]
    pub const fn events_end(&self) -> &EventsEnd {
        &self.events_end
    }
    #[doc = "0x110 - RADIO has been disabled"]
    #[inline(always)]
    pub const fn events_disabled(&self) -> &EventsDisabled {
        &self.events_disabled
    }
    #[doc = "0x114 - A device address match occurred on the last received packet"]
    #[inline(always)]
    pub const fn events_devmatch(&self) -> &EventsDevmatch {
        &self.events_devmatch
    }
    #[doc = "0x118 - No device address match occurred on the last received packet"]
    #[inline(always)]
    pub const fn events_devmiss(&self) -> &EventsDevmiss {
        &self.events_devmiss
    }
    #[doc = "0x11c - Sampling of receive signal strength complete"]
    #[inline(always)]
    pub const fn events_rssiend(&self) -> &EventsRssiend {
        &self.events_rssiend
    }
    #[doc = "0x128 - Bit counter reached bit count value"]
    #[inline(always)]
    pub const fn events_bcmatch(&self) -> &EventsBcmatch {
        &self.events_bcmatch
    }
    #[doc = "0x130 - Packet received with CRC ok"]
    #[inline(always)]
    pub const fn events_crcok(&self) -> &EventsCrcok {
        &self.events_crcok
    }
    #[doc = "0x134 - Packet received with CRC error"]
    #[inline(always)]
    pub const fn events_crcerror(&self) -> &EventsCrcerror {
        &self.events_crcerror
    }
    #[doc = "0x138 - IEEE 802.15.4 length field received"]
    #[inline(always)]
    pub const fn events_framestart(&self) -> &EventsFramestart {
        &self.events_framestart
    }
    #[doc = "0x13c - Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
    #[inline(always)]
    pub const fn events_edend(&self) -> &EventsEdend {
        &self.events_edend
    }
    #[doc = "0x140 - The sampling of energy detection has stopped"]
    #[inline(always)]
    pub const fn events_edstopped(&self) -> &EventsEdstopped {
        &self.events_edstopped
    }
    #[doc = "0x144 - Wireless medium in idle - clear to send"]
    #[inline(always)]
    pub const fn events_ccaidle(&self) -> &EventsCcaidle {
        &self.events_ccaidle
    }
    #[doc = "0x148 - Wireless medium busy - do not send"]
    #[inline(always)]
    pub const fn events_ccabusy(&self) -> &EventsCcabusy {
        &self.events_ccabusy
    }
    #[doc = "0x14c - The CCA has stopped"]
    #[inline(always)]
    pub const fn events_ccastopped(&self) -> &EventsCcastopped {
        &self.events_ccastopped
    }
    #[doc = "0x150 - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    #[inline(always)]
    pub const fn events_rateboost(&self) -> &EventsRateboost {
        &self.events_rateboost
    }
    #[doc = "0x154 - RADIO has ramped up and is ready to be started TX path"]
    #[inline(always)]
    pub const fn events_txready(&self) -> &EventsTxready {
        &self.events_txready
    }
    #[doc = "0x158 - RADIO has ramped up and is ready to be started RX path"]
    #[inline(always)]
    pub const fn events_rxready(&self) -> &EventsRxready {
        &self.events_rxready
    }
    #[doc = "0x15c - MAC header match found"]
    #[inline(always)]
    pub const fn events_mhrmatch(&self) -> &EventsMhrmatch {
        &self.events_mhrmatch
    }
    #[doc = "0x168 - Preamble indicator"]
    #[inline(always)]
    pub const fn events_sync(&self) -> &EventsSync {
        &self.events_sync
    }
    #[doc = "0x16c - Generated when last bit is sent on air, or received from air"]
    #[inline(always)]
    pub const fn events_phyend(&self) -> &EventsPhyend {
        &self.events_phyend
    }
    #[doc = "0x170 - CTE is present (early warning right after receiving CTEInfo byte)"]
    #[inline(always)]
    pub const fn events_ctepresent(&self) -> &EventsCtepresent {
        &self.events_ctepresent
    }
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    #[inline(always)]
    pub const fn shorts(&self) -> &Shorts {
        &self.shorts
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x400 - CRC status"]
    #[inline(always)]
    pub const fn crcstatus(&self) -> &Crcstatus {
        &self.crcstatus
    }
    #[doc = "0x408 - Received address"]
    #[inline(always)]
    pub const fn rxmatch(&self) -> &Rxmatch {
        &self.rxmatch
    }
    #[doc = "0x40c - CRC field of previously received packet"]
    #[inline(always)]
    pub const fn rxcrc(&self) -> &Rxcrc {
        &self.rxcrc
    }
    #[doc = "0x410 - Device address match index"]
    #[inline(always)]
    pub const fn dai(&self) -> &Dai {
        &self.dai
    }
    #[doc = "0x414 - Payload status"]
    #[inline(always)]
    pub const fn pdustat(&self) -> &Pdustat {
        &self.pdustat
    }
    #[doc = "0x44c - CTEInfo parsed from received packet"]
    #[inline(always)]
    pub const fn ctestatus(&self) -> &Ctestatus {
        &self.ctestatus
    }
    #[doc = "0x458 - DFE status information"]
    #[inline(always)]
    pub const fn dfestatus(&self) -> &Dfestatus {
        &self.dfestatus
    }
    #[doc = "0x504 - Packet pointer"]
    #[inline(always)]
    pub const fn packetptr(&self) -> &Packetptr {
        &self.packetptr
    }
    #[doc = "0x508 - Frequency"]
    #[inline(always)]
    pub const fn frequency(&self) -> &Frequency {
        &self.frequency
    }
    #[doc = "0x50c - Output power"]
    #[inline(always)]
    pub const fn txpower(&self) -> &Txpower {
        &self.txpower
    }
    #[doc = "0x510 - Data rate and modulation"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x514 - Packet configuration register 0"]
    #[inline(always)]
    pub const fn pcnf0(&self) -> &Pcnf0 {
        &self.pcnf0
    }
    #[doc = "0x518 - Packet configuration register 1"]
    #[inline(always)]
    pub const fn pcnf1(&self) -> &Pcnf1 {
        &self.pcnf1
    }
    #[doc = "0x51c - Base address 0"]
    #[inline(always)]
    pub const fn base0(&self) -> &Base0 {
        &self.base0
    }
    #[doc = "0x520 - Base address 1"]
    #[inline(always)]
    pub const fn base1(&self) -> &Base1 {
        &self.base1
    }
    #[doc = "0x524 - Prefixes bytes for logical addresses 0-3"]
    #[inline(always)]
    pub const fn prefix0(&self) -> &Prefix0 {
        &self.prefix0
    }
    #[doc = "0x528 - Prefixes bytes for logical addresses 4-7"]
    #[inline(always)]
    pub const fn prefix1(&self) -> &Prefix1 {
        &self.prefix1
    }
    #[doc = "0x52c - Transmit address select"]
    #[inline(always)]
    pub const fn txaddress(&self) -> &Txaddress {
        &self.txaddress
    }
    #[doc = "0x530 - Receive address select"]
    #[inline(always)]
    pub const fn rxaddresses(&self) -> &Rxaddresses {
        &self.rxaddresses
    }
    #[doc = "0x534 - CRC configuration"]
    #[inline(always)]
    pub const fn crccnf(&self) -> &Crccnf {
        &self.crccnf
    }
    #[doc = "0x538 - CRC polynomial"]
    #[inline(always)]
    pub const fn crcpoly(&self) -> &Crcpoly {
        &self.crcpoly
    }
    #[doc = "0x53c - CRC initial value"]
    #[inline(always)]
    pub const fn crcinit(&self) -> &Crcinit {
        &self.crcinit
    }
    #[doc = "0x544 - Interframe spacing in us"]
    #[inline(always)]
    pub const fn tifs(&self) -> &Tifs {
        &self.tifs
    }
    #[doc = "0x548 - RSSI sample"]
    #[inline(always)]
    pub const fn rssisample(&self) -> &Rssisample {
        &self.rssisample
    }
    #[doc = "0x550 - Current radio state"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x554 - Data whitening initial value"]
    #[inline(always)]
    pub const fn datawhiteiv(&self) -> &Datawhiteiv {
        &self.datawhiteiv
    }
    #[doc = "0x560 - Bit counter compare"]
    #[inline(always)]
    pub const fn bcc(&self) -> &Bcc {
        &self.bcc
    }
    #[doc = "0x600..0x620 - Description collection: Device address base segment n"]
    #[inline(always)]
    pub const fn dab(&self, n: usize) -> &Dab {
        &self.dab[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x600..0x620 - Description collection: Device address base segment n"]
    #[inline(always)]
    pub fn dab_iter(&self) -> impl Iterator<Item = &Dab> {
        self.dab.iter()
    }
    #[doc = "0x620..0x640 - Description collection: Device address prefix n"]
    #[inline(always)]
    pub const fn dap(&self, n: usize) -> &Dap {
        &self.dap[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x620..0x640 - Description collection: Device address prefix n"]
    #[inline(always)]
    pub fn dap_iter(&self) -> impl Iterator<Item = &Dap> {
        self.dap.iter()
    }
    #[doc = "0x640 - Device address match configuration"]
    #[inline(always)]
    pub const fn dacnf(&self) -> &Dacnf {
        &self.dacnf
    }
    #[doc = "0x644 - Search pattern configuration"]
    #[inline(always)]
    pub const fn mhrmatchconf(&self) -> &Mhrmatchconf {
        &self.mhrmatchconf
    }
    #[doc = "0x648 - Pattern mask"]
    #[inline(always)]
    pub const fn mhrmatchmas(&self) -> &Mhrmatchmas {
        &self.mhrmatchmas
    }
    #[doc = "0x650 - Radio mode configuration register 0"]
    #[inline(always)]
    pub const fn modecnf0(&self) -> &Modecnf0 {
        &self.modecnf0
    }
    #[doc = "0x660 - IEEE 802.15.4 start of frame delimiter"]
    #[inline(always)]
    pub const fn sfd(&self) -> &Sfd {
        &self.sfd
    }
    #[doc = "0x664 - IEEE 802.15.4 energy detect loop count"]
    #[inline(always)]
    pub const fn edcnt(&self) -> &Edcnt {
        &self.edcnt
    }
    #[doc = "0x668 - IEEE 802.15.4 energy detect level"]
    #[inline(always)]
    pub const fn edsample(&self) -> &Edsample {
        &self.edsample
    }
    #[doc = "0x66c - IEEE 802.15.4 clear channel assessment control"]
    #[inline(always)]
    pub const fn ccactrl(&self) -> &Ccactrl {
        &self.ccactrl
    }
    #[doc = "0x900 - Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
    #[inline(always)]
    pub const fn dfemode(&self) -> &Dfemode {
        &self.dfemode
    }
    #[doc = "0x904 - Configuration for CTE inline mode"]
    #[inline(always)]
    pub const fn cteinlineconf(&self) -> &Cteinlineconf {
        &self.cteinlineconf
    }
    #[doc = "0x910 - Various configuration for Direction finding"]
    #[inline(always)]
    pub const fn dfectrl1(&self) -> &Dfectrl1 {
        &self.dfectrl1
    }
    #[doc = "0x914 - Start offset for Direction finding"]
    #[inline(always)]
    pub const fn dfectrl2(&self) -> &Dfectrl2 {
        &self.dfectrl2
    }
    #[doc = "0x928 - GPIO patterns to be used for each antenna"]
    #[inline(always)]
    pub const fn switchpattern(&self) -> &Switchpattern {
        &self.switchpattern
    }
    #[doc = "0x92c - Clear the GPIO pattern array for antenna control"]
    #[inline(always)]
    pub const fn clearpattern(&self) -> &Clearpattern {
        &self.clearpattern
    }
    #[doc = "0x930..0x950 - Unspecified"]
    #[inline(always)]
    pub const fn psel(&self) -> &Psel {
        &self.psel
    }
    #[doc = "0x950..0x95c - DFE packet EasyDMA channel"]
    #[inline(always)]
    pub const fn dfepacket(&self) -> &Dfepacket {
        &self.dfepacket
    }
    #[doc = "0xffc - Peripheral power control"]
    #[inline(always)]
    pub const fn power(&self) -> &Power {
        &self.power
    }
}
#[doc = "TASKS_TXEN (w) register accessor: Enable RADIO in TX mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_txen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_txen`]
module"]
#[doc(alias = "TASKS_TXEN")]
pub type TasksTxen = crate::Reg<tasks_txen::TasksTxenSpec>;
#[doc = "Enable RADIO in TX mode"]
pub mod tasks_txen;
#[doc = "TASKS_RXEN (w) register accessor: Enable RADIO in RX mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rxen::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rxen`]
module"]
#[doc(alias = "TASKS_RXEN")]
pub type TasksRxen = crate::Reg<tasks_rxen::TasksRxenSpec>;
#[doc = "Enable RADIO in RX mode"]
pub mod tasks_rxen;
#[doc = "TASKS_START (w) register accessor: Start RADIO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_start`]
module"]
#[doc(alias = "TASKS_START")]
pub type TasksStart = crate::Reg<tasks_start::TasksStartSpec>;
#[doc = "Start RADIO"]
pub mod tasks_start;
#[doc = "TASKS_STOP (w) register accessor: Stop RADIO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_stop`]
module"]
#[doc(alias = "TASKS_STOP")]
pub type TasksStop = crate::Reg<tasks_stop::TasksStopSpec>;
#[doc = "Stop RADIO"]
pub mod tasks_stop;
#[doc = "TASKS_DISABLE (w) register accessor: Disable RADIO\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_disable::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_disable`]
module"]
#[doc(alias = "TASKS_DISABLE")]
pub type TasksDisable = crate::Reg<tasks_disable::TasksDisableSpec>;
#[doc = "Disable RADIO"]
pub mod tasks_disable;
#[doc = "TASKS_RSSISTART (w) register accessor: Start the RSSI and take one single sample of the receive signal strength\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rssistart`]
module"]
#[doc(alias = "TASKS_RSSISTART")]
pub type TasksRssistart = crate::Reg<tasks_rssistart::TasksRssistartSpec>;
#[doc = "Start the RSSI and take one single sample of the receive signal strength"]
pub mod tasks_rssistart;
#[doc = "TASKS_RSSISTOP (w) register accessor: Stop the RSSI measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_rssistop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_rssistop`]
module"]
#[doc(alias = "TASKS_RSSISTOP")]
pub type TasksRssistop = crate::Reg<tasks_rssistop::TasksRssistopSpec>;
#[doc = "Stop the RSSI measurement"]
pub mod tasks_rssistop;
#[doc = "TASKS_BCSTART (w) register accessor: Start the bit counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_bcstart`]
module"]
#[doc(alias = "TASKS_BCSTART")]
pub type TasksBcstart = crate::Reg<tasks_bcstart::TasksBcstartSpec>;
#[doc = "Start the bit counter"]
pub mod tasks_bcstart;
#[doc = "TASKS_BCSTOP (w) register accessor: Stop the bit counter\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_bcstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_bcstop`]
module"]
#[doc(alias = "TASKS_BCSTOP")]
pub type TasksBcstop = crate::Reg<tasks_bcstop::TasksBcstopSpec>;
#[doc = "Stop the bit counter"]
pub mod tasks_bcstop;
#[doc = "TASKS_EDSTART (w) register accessor: Start the energy detect measurement used in IEEE 802.15.4 mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_edstart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_edstart`]
module"]
#[doc(alias = "TASKS_EDSTART")]
pub type TasksEdstart = crate::Reg<tasks_edstart::TasksEdstartSpec>;
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode"]
pub mod tasks_edstart;
#[doc = "TASKS_EDSTOP (w) register accessor: Stop the energy detect measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_edstop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_edstop`]
module"]
#[doc(alias = "TASKS_EDSTOP")]
pub type TasksEdstop = crate::Reg<tasks_edstop::TasksEdstopSpec>;
#[doc = "Stop the energy detect measurement"]
pub mod tasks_edstop;
#[doc = "TASKS_CCASTART (w) register accessor: Start the clear channel assessment used in IEEE 802.15.4 mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ccastart::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ccastart`]
module"]
#[doc(alias = "TASKS_CCASTART")]
pub type TasksCcastart = crate::Reg<tasks_ccastart::TasksCcastartSpec>;
#[doc = "Start the clear channel assessment used in IEEE 802.15.4 mode"]
pub mod tasks_ccastart;
#[doc = "TASKS_CCASTOP (w) register accessor: Stop the clear channel assessment\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_ccastop::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_ccastop`]
module"]
#[doc(alias = "TASKS_CCASTOP")]
pub type TasksCcastop = crate::Reg<tasks_ccastop::TasksCcastopSpec>;
#[doc = "Stop the clear channel assessment"]
pub mod tasks_ccastop;
#[doc = "EVENTS_READY (rw) register accessor: RADIO has ramped up and is ready to be started\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ready`]
module"]
#[doc(alias = "EVENTS_READY")]
pub type EventsReady = crate::Reg<events_ready::EventsReadySpec>;
#[doc = "RADIO has ramped up and is ready to be started"]
pub mod events_ready;
#[doc = "EVENTS_ADDRESS (rw) register accessor: Address sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_address::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_address::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_address`]
module"]
#[doc(alias = "EVENTS_ADDRESS")]
pub type EventsAddress = crate::Reg<events_address::EventsAddressSpec>;
#[doc = "Address sent or received"]
pub mod events_address;
#[doc = "EVENTS_PAYLOAD (rw) register accessor: Packet payload sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_payload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_payload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_payload`]
module"]
#[doc(alias = "EVENTS_PAYLOAD")]
pub type EventsPayload = crate::Reg<events_payload::EventsPayloadSpec>;
#[doc = "Packet payload sent or received"]
pub mod events_payload;
#[doc = "EVENTS_END (rw) register accessor: Packet sent or received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_end::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_end::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_end`]
module"]
#[doc(alias = "EVENTS_END")]
pub type EventsEnd = crate::Reg<events_end::EventsEndSpec>;
#[doc = "Packet sent or received"]
pub mod events_end;
#[doc = "EVENTS_DISABLED (rw) register accessor: RADIO has been disabled\n\nYou can [`read`](crate::Reg::read) this register and get [`events_disabled::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_disabled::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_disabled`]
module"]
#[doc(alias = "EVENTS_DISABLED")]
pub type EventsDisabled = crate::Reg<events_disabled::EventsDisabledSpec>;
#[doc = "RADIO has been disabled"]
pub mod events_disabled;
#[doc = "EVENTS_DEVMATCH (rw) register accessor: A device address match occurred on the last received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_devmatch`]
module"]
#[doc(alias = "EVENTS_DEVMATCH")]
pub type EventsDevmatch = crate::Reg<events_devmatch::EventsDevmatchSpec>;
#[doc = "A device address match occurred on the last received packet"]
pub mod events_devmatch;
#[doc = "EVENTS_DEVMISS (rw) register accessor: No device address match occurred on the last received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`events_devmiss::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_devmiss::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_devmiss`]
module"]
#[doc(alias = "EVENTS_DEVMISS")]
pub type EventsDevmiss = crate::Reg<events_devmiss::EventsDevmissSpec>;
#[doc = "No device address match occurred on the last received packet"]
pub mod events_devmiss;
#[doc = "EVENTS_RSSIEND (rw) register accessor: Sampling of receive signal strength complete\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rssiend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rssiend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rssiend`]
module"]
#[doc(alias = "EVENTS_RSSIEND")]
pub type EventsRssiend = crate::Reg<events_rssiend::EventsRssiendSpec>;
#[doc = "Sampling of receive signal strength complete"]
pub mod events_rssiend;
#[doc = "EVENTS_BCMATCH (rw) register accessor: Bit counter reached bit count value\n\nYou can [`read`](crate::Reg::read) this register and get [`events_bcmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_bcmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_bcmatch`]
module"]
#[doc(alias = "EVENTS_BCMATCH")]
pub type EventsBcmatch = crate::Reg<events_bcmatch::EventsBcmatchSpec>;
#[doc = "Bit counter reached bit count value"]
pub mod events_bcmatch;
#[doc = "EVENTS_CRCOK (rw) register accessor: Packet received with CRC ok\n\nYou can [`read`](crate::Reg::read) this register and get [`events_crcok::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_crcok::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_crcok`]
module"]
#[doc(alias = "EVENTS_CRCOK")]
pub type EventsCrcok = crate::Reg<events_crcok::EventsCrcokSpec>;
#[doc = "Packet received with CRC ok"]
pub mod events_crcok;
#[doc = "EVENTS_CRCERROR (rw) register accessor: Packet received with CRC error\n\nYou can [`read`](crate::Reg::read) this register and get [`events_crcerror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_crcerror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_crcerror`]
module"]
#[doc(alias = "EVENTS_CRCERROR")]
pub type EventsCrcerror = crate::Reg<events_crcerror::EventsCrcerrorSpec>;
#[doc = "Packet received with CRC error"]
pub mod events_crcerror;
#[doc = "EVENTS_FRAMESTART (rw) register accessor: IEEE 802.15.4 length field received\n\nYou can [`read`](crate::Reg::read) this register and get [`events_framestart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_framestart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_framestart`]
module"]
#[doc(alias = "EVENTS_FRAMESTART")]
pub type EventsFramestart = crate::Reg<events_framestart::EventsFramestartSpec>;
#[doc = "IEEE 802.15.4 length field received"]
pub mod events_framestart;
#[doc = "EVENTS_EDEND (rw) register accessor: Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_edend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_edend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_edend`]
module"]
#[doc(alias = "EVENTS_EDEND")]
pub type EventsEdend = crate::Reg<events_edend::EventsEdendSpec>;
#[doc = "Sampling of energy detection complete. A new ED sample is ready for readout from the RADIO.EDSAMPLE register."]
pub mod events_edend;
#[doc = "EVENTS_EDSTOPPED (rw) register accessor: The sampling of energy detection has stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_edstopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_edstopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_edstopped`]
module"]
#[doc(alias = "EVENTS_EDSTOPPED")]
pub type EventsEdstopped = crate::Reg<events_edstopped::EventsEdstoppedSpec>;
#[doc = "The sampling of energy detection has stopped"]
pub mod events_edstopped;
#[doc = "EVENTS_CCAIDLE (rw) register accessor: Wireless medium in idle - clear to send\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccaidle::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccaidle::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ccaidle`]
module"]
#[doc(alias = "EVENTS_CCAIDLE")]
pub type EventsCcaidle = crate::Reg<events_ccaidle::EventsCcaidleSpec>;
#[doc = "Wireless medium in idle - clear to send"]
pub mod events_ccaidle;
#[doc = "EVENTS_CCABUSY (rw) register accessor: Wireless medium busy - do not send\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccabusy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccabusy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ccabusy`]
module"]
#[doc(alias = "EVENTS_CCABUSY")]
pub type EventsCcabusy = crate::Reg<events_ccabusy::EventsCcabusySpec>;
#[doc = "Wireless medium busy - do not send"]
pub mod events_ccabusy;
#[doc = "EVENTS_CCASTOPPED (rw) register accessor: The CCA has stopped\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ccastopped::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ccastopped::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ccastopped`]
module"]
#[doc(alias = "EVENTS_CCASTOPPED")]
pub type EventsCcastopped = crate::Reg<events_ccastopped::EventsCcastoppedSpec>;
#[doc = "The CCA has stopped"]
pub mod events_ccastopped;
#[doc = "EVENTS_RATEBOOST (rw) register accessor: Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rateboost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rateboost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rateboost`]
module"]
#[doc(alias = "EVENTS_RATEBOOST")]
pub type EventsRateboost = crate::Reg<events_rateboost::EventsRateboostSpec>;
#[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
pub mod events_rateboost;
#[doc = "EVENTS_TXREADY (rw) register accessor: RADIO has ramped up and is ready to be started TX path\n\nYou can [`read`](crate::Reg::read) this register and get [`events_txready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_txready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_txready`]
module"]
#[doc(alias = "EVENTS_TXREADY")]
pub type EventsTxready = crate::Reg<events_txready::EventsTxreadySpec>;
#[doc = "RADIO has ramped up and is ready to be started TX path"]
pub mod events_txready;
#[doc = "EVENTS_RXREADY (rw) register accessor: RADIO has ramped up and is ready to be started RX path\n\nYou can [`read`](crate::Reg::read) this register and get [`events_rxready::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_rxready::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_rxready`]
module"]
#[doc(alias = "EVENTS_RXREADY")]
pub type EventsRxready = crate::Reg<events_rxready::EventsRxreadySpec>;
#[doc = "RADIO has ramped up and is ready to be started RX path"]
pub mod events_rxready;
#[doc = "EVENTS_MHRMATCH (rw) register accessor: MAC header match found\n\nYou can [`read`](crate::Reg::read) this register and get [`events_mhrmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_mhrmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_mhrmatch`]
module"]
#[doc(alias = "EVENTS_MHRMATCH")]
pub type EventsMhrmatch = crate::Reg<events_mhrmatch::EventsMhrmatchSpec>;
#[doc = "MAC header match found"]
pub mod events_mhrmatch;
#[doc = "EVENTS_SYNC (rw) register accessor: Preamble indicator\n\nYou can [`read`](crate::Reg::read) this register and get [`events_sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_sync`]
module"]
#[doc(alias = "EVENTS_SYNC")]
pub type EventsSync = crate::Reg<events_sync::EventsSyncSpec>;
#[doc = "Preamble indicator"]
pub mod events_sync;
#[doc = "EVENTS_PHYEND (rw) register accessor: Generated when last bit is sent on air, or received from air\n\nYou can [`read`](crate::Reg::read) this register and get [`events_phyend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_phyend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_phyend`]
module"]
#[doc(alias = "EVENTS_PHYEND")]
pub type EventsPhyend = crate::Reg<events_phyend::EventsPhyendSpec>;
#[doc = "Generated when last bit is sent on air, or received from air"]
pub mod events_phyend;
#[doc = "EVENTS_CTEPRESENT (rw) register accessor: CTE is present (early warning right after receiving CTEInfo byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`events_ctepresent::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_ctepresent::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_ctepresent`]
module"]
#[doc(alias = "EVENTS_CTEPRESENT")]
pub type EventsCtepresent = crate::Reg<events_ctepresent::EventsCtepresentSpec>;
#[doc = "CTE is present (early warning right after receiving CTEInfo byte)"]
pub mod events_ctepresent;
#[doc = "SHORTS (rw) register accessor: Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shorts`]
module"]
#[doc(alias = "SHORTS")]
pub type Shorts = crate::Reg<shorts::ShortsSpec>;
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "CRCSTATUS (r) register accessor: CRC status\n\nYou can [`read`](crate::Reg::read) this register and get [`crcstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcstatus`]
module"]
#[doc(alias = "CRCSTATUS")]
pub type Crcstatus = crate::Reg<crcstatus::CrcstatusSpec>;
#[doc = "CRC status"]
pub mod crcstatus;
#[doc = "RXMATCH (r) register accessor: Received address\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmatch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmatch`]
module"]
#[doc(alias = "RXMATCH")]
pub type Rxmatch = crate::Reg<rxmatch::RxmatchSpec>;
#[doc = "Received address"]
pub mod rxmatch;
#[doc = "RXCRC (r) register accessor: CRC field of previously received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrc`]
module"]
#[doc(alias = "RXCRC")]
pub type Rxcrc = crate::Reg<rxcrc::RxcrcSpec>;
#[doc = "CRC field of previously received packet"]
pub mod rxcrc;
#[doc = "DAI (r) register accessor: Device address match index\n\nYou can [`read`](crate::Reg::read) this register and get [`dai::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dai`]
module"]
#[doc(alias = "DAI")]
pub type Dai = crate::Reg<dai::DaiSpec>;
#[doc = "Device address match index"]
pub mod dai;
#[doc = "PDUSTAT (r) register accessor: Payload status\n\nYou can [`read`](crate::Reg::read) this register and get [`pdustat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdustat`]
module"]
#[doc(alias = "PDUSTAT")]
pub type Pdustat = crate::Reg<pdustat::PdustatSpec>;
#[doc = "Payload status"]
pub mod pdustat;
#[doc = "CTESTATUS (r) register accessor: CTEInfo parsed from received packet\n\nYou can [`read`](crate::Reg::read) this register and get [`ctestatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctestatus`]
module"]
#[doc(alias = "CTESTATUS")]
pub type Ctestatus = crate::Reg<ctestatus::CtestatusSpec>;
#[doc = "CTEInfo parsed from received packet"]
pub mod ctestatus;
#[doc = "DFESTATUS (r) register accessor: DFE status information\n\nYou can [`read`](crate::Reg::read) this register and get [`dfestatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfestatus`]
module"]
#[doc(alias = "DFESTATUS")]
pub type Dfestatus = crate::Reg<dfestatus::DfestatusSpec>;
#[doc = "DFE status information"]
pub mod dfestatus;
#[doc = "PACKETPTR (rw) register accessor: Packet pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`packetptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`packetptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packetptr`]
module"]
#[doc(alias = "PACKETPTR")]
pub type Packetptr = crate::Reg<packetptr::PacketptrSpec>;
#[doc = "Packet pointer"]
pub mod packetptr;
#[doc = "FREQUENCY (rw) register accessor: Frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`frequency::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frequency::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frequency`]
module"]
#[doc(alias = "FREQUENCY")]
pub type Frequency = crate::Reg<frequency::FrequencySpec>;
#[doc = "Frequency"]
pub mod frequency;
#[doc = "TXPOWER (rw) register accessor: Output power\n\nYou can [`read`](crate::Reg::read) this register and get [`txpower::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txpower::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txpower`]
module"]
#[doc(alias = "TXPOWER")]
pub type Txpower = crate::Reg<txpower::TxpowerSpec>;
#[doc = "Output power"]
pub mod txpower;
#[doc = "MODE (rw) register accessor: Data rate and modulation\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`]
module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "Data rate and modulation"]
pub mod mode;
#[doc = "PCNF0 (rw) register accessor: Packet configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnf0`]
module"]
#[doc(alias = "PCNF0")]
pub type Pcnf0 = crate::Reg<pcnf0::Pcnf0Spec>;
#[doc = "Packet configuration register 0"]
pub mod pcnf0;
#[doc = "PCNF1 (rw) register accessor: Packet configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnf1`]
module"]
#[doc(alias = "PCNF1")]
pub type Pcnf1 = crate::Reg<pcnf1::Pcnf1Spec>;
#[doc = "Packet configuration register 1"]
pub mod pcnf1;
#[doc = "BASE0 (rw) register accessor: Base address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`base0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base0`]
module"]
#[doc(alias = "BASE0")]
pub type Base0 = crate::Reg<base0::Base0Spec>;
#[doc = "Base address 0"]
pub mod base0;
#[doc = "BASE1 (rw) register accessor: Base address 1\n\nYou can [`read`](crate::Reg::read) this register and get [`base1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base1`]
module"]
#[doc(alias = "BASE1")]
pub type Base1 = crate::Reg<base1::Base1Spec>;
#[doc = "Base address 1"]
pub mod base1;
#[doc = "PREFIX0 (rw) register accessor: Prefixes bytes for logical addresses 0-3\n\nYou can [`read`](crate::Reg::read) this register and get [`prefix0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefix0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefix0`]
module"]
#[doc(alias = "PREFIX0")]
pub type Prefix0 = crate::Reg<prefix0::Prefix0Spec>;
#[doc = "Prefixes bytes for logical addresses 0-3"]
pub mod prefix0;
#[doc = "PREFIX1 (rw) register accessor: Prefixes bytes for logical addresses 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`prefix1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefix1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prefix1`]
module"]
#[doc(alias = "PREFIX1")]
pub type Prefix1 = crate::Reg<prefix1::Prefix1Spec>;
#[doc = "Prefixes bytes for logical addresses 4-7"]
pub mod prefix1;
#[doc = "TXADDRESS (rw) register accessor: Transmit address select\n\nYou can [`read`](crate::Reg::read) this register and get [`txaddress::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txaddress::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txaddress`]
module"]
#[doc(alias = "TXADDRESS")]
pub type Txaddress = crate::Reg<txaddress::TxaddressSpec>;
#[doc = "Transmit address select"]
pub mod txaddress;
#[doc = "RXADDRESSES (rw) register accessor: Receive address select\n\nYou can [`read`](crate::Reg::read) this register and get [`rxaddresses::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxaddresses::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxaddresses`]
module"]
#[doc(alias = "RXADDRESSES")]
pub type Rxaddresses = crate::Reg<rxaddresses::RxaddressesSpec>;
#[doc = "Receive address select"]
pub mod rxaddresses;
#[doc = "CRCCNF (rw) register accessor: CRC configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`crccnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccnf`]
module"]
#[doc(alias = "CRCCNF")]
pub type Crccnf = crate::Reg<crccnf::CrccnfSpec>;
#[doc = "CRC configuration"]
pub mod crccnf;
#[doc = "CRCPOLY (rw) register accessor: CRC polynomial\n\nYou can [`read`](crate::Reg::read) this register and get [`crcpoly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcpoly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcpoly`]
module"]
#[doc(alias = "CRCPOLY")]
pub type Crcpoly = crate::Reg<crcpoly::CrcpolySpec>;
#[doc = "CRC polynomial"]
pub mod crcpoly;
#[doc = "CRCINIT (rw) register accessor: CRC initial value\n\nYou can [`read`](crate::Reg::read) this register and get [`crcinit::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcinit::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcinit`]
module"]
#[doc(alias = "CRCINIT")]
pub type Crcinit = crate::Reg<crcinit::CrcinitSpec>;
#[doc = "CRC initial value"]
pub mod crcinit;
#[doc = "TIFS (rw) register accessor: Interframe spacing in us\n\nYou can [`read`](crate::Reg::read) this register and get [`tifs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tifs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tifs`]
module"]
#[doc(alias = "TIFS")]
pub type Tifs = crate::Reg<tifs::TifsSpec>;
#[doc = "Interframe spacing in us"]
pub mod tifs;
#[doc = "RSSISAMPLE (r) register accessor: RSSI sample\n\nYou can [`read`](crate::Reg::read) this register and get [`rssisample::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rssisample`]
module"]
#[doc(alias = "RSSISAMPLE")]
pub type Rssisample = crate::Reg<rssisample::RssisampleSpec>;
#[doc = "RSSI sample"]
pub mod rssisample;
#[doc = "STATE (r) register accessor: Current radio state\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`]
module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "Current radio state"]
pub mod state;
#[doc = "DATAWHITEIV (rw) register accessor: Data whitening initial value\n\nYou can [`read`](crate::Reg::read) this register and get [`datawhiteiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datawhiteiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datawhiteiv`]
module"]
#[doc(alias = "DATAWHITEIV")]
pub type Datawhiteiv = crate::Reg<datawhiteiv::DatawhiteivSpec>;
#[doc = "Data whitening initial value"]
pub mod datawhiteiv;
#[doc = "BCC (rw) register accessor: Bit counter compare\n\nYou can [`read`](crate::Reg::read) this register and get [`bcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bcc`]
module"]
#[doc(alias = "BCC")]
pub type Bcc = crate::Reg<bcc::BccSpec>;
#[doc = "Bit counter compare"]
pub mod bcc;
#[doc = "DAB (rw) register accessor: Description collection: Device address base segment n\n\nYou can [`read`](crate::Reg::read) this register and get [`dab::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dab::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dab`]
module"]
#[doc(alias = "DAB")]
pub type Dab = crate::Reg<dab::DabSpec>;
#[doc = "Description collection: Device address base segment n"]
pub mod dab;
#[doc = "DAP (rw) register accessor: Description collection: Device address prefix n\n\nYou can [`read`](crate::Reg::read) this register and get [`dap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dap`]
module"]
#[doc(alias = "DAP")]
pub type Dap = crate::Reg<dap::DapSpec>;
#[doc = "Description collection: Device address prefix n"]
pub mod dap;
#[doc = "DACNF (rw) register accessor: Device address match configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dacnf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacnf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacnf`]
module"]
#[doc(alias = "DACNF")]
pub type Dacnf = crate::Reg<dacnf::DacnfSpec>;
#[doc = "Device address match configuration"]
pub mod dacnf;
#[doc = "MHRMATCHCONF (rw) register accessor: Search pattern configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mhrmatchconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mhrmatchconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mhrmatchconf`]
module"]
#[doc(alias = "MHRMATCHCONF")]
pub type Mhrmatchconf = crate::Reg<mhrmatchconf::MhrmatchconfSpec>;
#[doc = "Search pattern configuration"]
pub mod mhrmatchconf;
#[doc = "MHRMATCHMAS (rw) register accessor: Pattern mask\n\nYou can [`read`](crate::Reg::read) this register and get [`mhrmatchmas::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mhrmatchmas::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mhrmatchmas`]
module"]
#[doc(alias = "MHRMATCHMAS")]
pub type Mhrmatchmas = crate::Reg<mhrmatchmas::MhrmatchmasSpec>;
#[doc = "Pattern mask"]
pub mod mhrmatchmas;
#[doc = "MODECNF0 (rw) register accessor: Radio mode configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`modecnf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modecnf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modecnf0`]
module"]
#[doc(alias = "MODECNF0")]
pub type Modecnf0 = crate::Reg<modecnf0::Modecnf0Spec>;
#[doc = "Radio mode configuration register 0"]
pub mod modecnf0;
#[doc = "SFD (rw) register accessor: IEEE 802.15.4 start of frame delimiter\n\nYou can [`read`](crate::Reg::read) this register and get [`sfd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfd`]
module"]
#[doc(alias = "SFD")]
pub type Sfd = crate::Reg<sfd::SfdSpec>;
#[doc = "IEEE 802.15.4 start of frame delimiter"]
pub mod sfd;
#[doc = "EDCNT (rw) register accessor: IEEE 802.15.4 energy detect loop count\n\nYou can [`read`](crate::Reg::read) this register and get [`edcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edcnt`]
module"]
#[doc(alias = "EDCNT")]
pub type Edcnt = crate::Reg<edcnt::EdcntSpec>;
#[doc = "IEEE 802.15.4 energy detect loop count"]
pub mod edcnt;
#[doc = "EDSAMPLE (r) register accessor: IEEE 802.15.4 energy detect level\n\nYou can [`read`](crate::Reg::read) this register and get [`edsample::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edsample`]
module"]
#[doc(alias = "EDSAMPLE")]
pub type Edsample = crate::Reg<edsample::EdsampleSpec>;
#[doc = "IEEE 802.15.4 energy detect level"]
pub mod edsample;
#[doc = "CCACTRL (rw) register accessor: IEEE 802.15.4 clear channel assessment control\n\nYou can [`read`](crate::Reg::read) this register and get [`ccactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccactrl`]
module"]
#[doc(alias = "CCACTRL")]
pub type Ccactrl = crate::Reg<ccactrl::CcactrlSpec>;
#[doc = "IEEE 802.15.4 clear channel assessment control"]
pub mod ccactrl;
#[doc = "DFEMODE (rw) register accessor: Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)\n\nYou can [`read`](crate::Reg::read) this register and get [`dfemode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfemode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfemode`]
module"]
#[doc(alias = "DFEMODE")]
pub type Dfemode = crate::Reg<dfemode::DfemodeSpec>;
#[doc = "Whether to use Angle-of-Arrival (AOA) or Angle-of-Departure (AOD)"]
pub mod dfemode;
#[doc = "CTEINLINECONF (rw) register accessor: Configuration for CTE inline mode\n\nYou can [`read`](crate::Reg::read) this register and get [`cteinlineconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cteinlineconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cteinlineconf`]
module"]
#[doc(alias = "CTEINLINECONF")]
pub type Cteinlineconf = crate::Reg<cteinlineconf::CteinlineconfSpec>;
#[doc = "Configuration for CTE inline mode"]
pub mod cteinlineconf;
#[doc = "DFECTRL1 (rw) register accessor: Various configuration for Direction finding\n\nYou can [`read`](crate::Reg::read) this register and get [`dfectrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfectrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfectrl1`]
module"]
#[doc(alias = "DFECTRL1")]
pub type Dfectrl1 = crate::Reg<dfectrl1::Dfectrl1Spec>;
#[doc = "Various configuration for Direction finding"]
pub mod dfectrl1;
#[doc = "DFECTRL2 (rw) register accessor: Start offset for Direction finding\n\nYou can [`read`](crate::Reg::read) this register and get [`dfectrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfectrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dfectrl2`]
module"]
#[doc(alias = "DFECTRL2")]
pub type Dfectrl2 = crate::Reg<dfectrl2::Dfectrl2Spec>;
#[doc = "Start offset for Direction finding"]
pub mod dfectrl2;
#[doc = "SWITCHPATTERN (rw) register accessor: GPIO patterns to be used for each antenna\n\nYou can [`read`](crate::Reg::read) this register and get [`switchpattern::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`switchpattern::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@switchpattern`]
module"]
#[doc(alias = "SWITCHPATTERN")]
pub type Switchpattern = crate::Reg<switchpattern::SwitchpatternSpec>;
#[doc = "GPIO patterns to be used for each antenna"]
pub mod switchpattern;
#[doc = "CLEARPATTERN (rw) register accessor: Clear the GPIO pattern array for antenna control\n\nYou can [`read`](crate::Reg::read) this register and get [`clearpattern::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clearpattern::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearpattern`]
module"]
#[doc(alias = "CLEARPATTERN")]
pub type Clearpattern = crate::Reg<clearpattern::ClearpatternSpec>;
#[doc = "Clear the GPIO pattern array for antenna control"]
pub mod clearpattern;
#[doc = "Unspecified"]
pub use self::psel::Psel;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod psel;
#[doc = "DFE packet EasyDMA channel"]
pub use self::dfepacket::Dfepacket;
#[doc = r"Cluster"]
#[doc = "DFE packet EasyDMA channel"]
pub mod dfepacket;
#[doc = "POWER (rw) register accessor: Peripheral power control\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@power`]
module"]
#[doc(alias = "POWER")]
pub type Power = crate::Reg<power::PowerSpec>;
#[doc = "Peripheral power control"]
pub mod power;
