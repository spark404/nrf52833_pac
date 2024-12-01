#[doc = "Register `TRACECONFIG` reader"]
pub type R = crate::R<TraceconfigSpec>;
#[doc = "Register `TRACECONFIG` writer"]
pub type W = crate::W<TraceconfigSpec>;
#[doc = "Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Traceportspeed {
    #[doc = "0: 32 MHz trace port clock (TRACECLK = 16 MHz)"]
    _32mhz = 0,
    #[doc = "1: 16 MHz trace port clock (TRACECLK = 8 MHz)"]
    _16mhz = 1,
    #[doc = "2: 8 MHz trace port clock (TRACECLK = 4 MHz)"]
    _8mhz = 2,
    #[doc = "3: 4 MHz trace port clock (TRACECLK = 2 MHz)"]
    _4mhz = 3,
}
impl From<Traceportspeed> for u8 {
    #[inline(always)]
    fn from(variant: Traceportspeed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Traceportspeed {
    type Ux = u8;
}
impl crate::IsEnum for Traceportspeed {}
#[doc = "Field `TRACEPORTSPEED` reader - Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
pub type TraceportspeedR = crate::FieldReader<Traceportspeed>;
impl TraceportspeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Traceportspeed {
        match self.bits {
            0 => Traceportspeed::_32mhz,
            1 => Traceportspeed::_16mhz,
            2 => Traceportspeed::_8mhz,
            3 => Traceportspeed::_4mhz,
            _ => unreachable!(),
        }
    }
    #[doc = "32 MHz trace port clock (TRACECLK = 16 MHz)"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == Traceportspeed::_32mhz
    }
    #[doc = "16 MHz trace port clock (TRACECLK = 8 MHz)"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == Traceportspeed::_16mhz
    }
    #[doc = "8 MHz trace port clock (TRACECLK = 4 MHz)"]
    #[inline(always)]
    pub fn is_8mhz(&self) -> bool {
        *self == Traceportspeed::_8mhz
    }
    #[doc = "4 MHz trace port clock (TRACECLK = 2 MHz)"]
    #[inline(always)]
    pub fn is_4mhz(&self) -> bool {
        *self == Traceportspeed::_4mhz
    }
}
#[doc = "Field `TRACEPORTSPEED` writer - Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
pub type TraceportspeedW<'a, REG> = crate::FieldWriter<'a, REG, 2, Traceportspeed, crate::Safe>;
impl<'a, REG> TraceportspeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32 MHz trace port clock (TRACECLK = 16 MHz)"]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::_32mhz)
    }
    #[doc = "16 MHz trace port clock (TRACECLK = 8 MHz)"]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::_16mhz)
    }
    #[doc = "8 MHz trace port clock (TRACECLK = 4 MHz)"]
    #[inline(always)]
    pub fn _8mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::_8mhz)
    }
    #[doc = "4 MHz trace port clock (TRACECLK = 2 MHz)"]
    #[inline(always)]
    pub fn _4mhz(self) -> &'a mut crate::W<REG> {
        self.variant(Traceportspeed::_4mhz)
    }
}
#[doc = "Pin multiplexing of trace signals. See pin assignment chapter for more details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tracemux {
    #[doc = "0: No trace signals routed to pins. All pins can be used as regular GPIOs."]
    Gpio = 0,
    #[doc = "1: SWO trace signal routed to pin. Remaining pins can be used as regular GPIOs."]
    Serial = 1,
    #[doc = "2: All trace signals (TRACECLK and TRACEDATA\\[n\\]) routed to pins."]
    Parallel = 2,
}
impl From<Tracemux> for u8 {
    #[inline(always)]
    fn from(variant: Tracemux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tracemux {
    type Ux = u8;
}
impl crate::IsEnum for Tracemux {}
#[doc = "Field `TRACEMUX` reader - Pin multiplexing of trace signals. See pin assignment chapter for more details."]
pub type TracemuxR = crate::FieldReader<Tracemux>;
impl TracemuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tracemux> {
        match self.bits {
            0 => Some(Tracemux::Gpio),
            1 => Some(Tracemux::Serial),
            2 => Some(Tracemux::Parallel),
            _ => None,
        }
    }
    #[doc = "No trace signals routed to pins. All pins can be used as regular GPIOs."]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == Tracemux::Gpio
    }
    #[doc = "SWO trace signal routed to pin. Remaining pins can be used as regular GPIOs."]
    #[inline(always)]
    pub fn is_serial(&self) -> bool {
        *self == Tracemux::Serial
    }
    #[doc = "All trace signals (TRACECLK and TRACEDATA\\[n\\]) routed to pins."]
    #[inline(always)]
    pub fn is_parallel(&self) -> bool {
        *self == Tracemux::Parallel
    }
}
#[doc = "Field `TRACEMUX` writer - Pin multiplexing of trace signals. See pin assignment chapter for more details."]
pub type TracemuxW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tracemux>;
impl<'a, REG> TracemuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No trace signals routed to pins. All pins can be used as regular GPIOs."]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(Tracemux::Gpio)
    }
    #[doc = "SWO trace signal routed to pin. Remaining pins can be used as regular GPIOs."]
    #[inline(always)]
    pub fn serial(self) -> &'a mut crate::W<REG> {
        self.variant(Tracemux::Serial)
    }
    #[doc = "All trace signals (TRACECLK and TRACEDATA\\[n\\]) routed to pins."]
    #[inline(always)]
    pub fn parallel(self) -> &'a mut crate::W<REG> {
        self.variant(Tracemux::Parallel)
    }
}
impl R {
    #[doc = "Bits 0:1 - Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&self) -> TraceportspeedR {
        TraceportspeedR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals. See pin assignment chapter for more details."]
    #[inline(always)]
    pub fn tracemux(&self) -> TracemuxR {
        TracemuxR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Speed of trace port clock. Note that the TRACECLK pin will output this clock divided by two."]
    #[inline(always)]
    pub fn traceportspeed(&mut self) -> TraceportspeedW<TraceconfigSpec> {
        TraceportspeedW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Pin multiplexing of trace signals. See pin assignment chapter for more details."]
    #[inline(always)]
    pub fn tracemux(&mut self) -> TracemuxW<TraceconfigSpec> {
        TracemuxW::new(self, 16)
    }
}
#[doc = "Clocking options for the trace port debug interface\n\nYou can [`read`](crate::Reg::read) this register and get [`traceconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TraceconfigSpec;
impl crate::RegisterSpec for TraceconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`traceconfig::R`](R) reader structure"]
impl crate::Readable for TraceconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`traceconfig::W`](W) writer structure"]
impl crate::Writable for TraceconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRACECONFIG to value 0"]
impl crate::Resettable for TraceconfigSpec {
    const RESET_VALUE: u32 = 0;
}
