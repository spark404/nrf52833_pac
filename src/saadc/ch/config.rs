#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Positive channel resistor control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resp {
    #[doc = "0: Bypass resistor ladder"]
    Bypass = 0,
    #[doc = "1: Pull-down to GND"]
    Pulldown = 1,
    #[doc = "2: Pull-up to VDD"]
    Pullup = 2,
    #[doc = "3: Set input at VDD/2"]
    Vdd1_2 = 3,
}
impl From<Resp> for u8 {
    #[inline(always)]
    fn from(variant: Resp) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resp {
    type Ux = u8;
}
impl crate::IsEnum for Resp {}
#[doc = "Field `RESP` reader - Positive channel resistor control"]
pub type RespR = crate::FieldReader<Resp>;
impl RespR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resp {
        match self.bits {
            0 => Resp::Bypass,
            1 => Resp::Pulldown,
            2 => Resp::Pullup,
            3 => Resp::Vdd1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Bypass resistor ladder"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Resp::Bypass
    }
    #[doc = "Pull-down to GND"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Resp::Pulldown
    }
    #[doc = "Pull-up to VDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Resp::Pullup
    }
    #[doc = "Set input at VDD/2"]
    #[inline(always)]
    pub fn is_vdd1_2(&self) -> bool {
        *self == Resp::Vdd1_2
    }
}
#[doc = "Field `RESP` writer - Positive channel resistor control"]
pub type RespW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resp, crate::Safe>;
impl<'a, REG> RespW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass resistor ladder"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Resp::Bypass)
    }
    #[doc = "Pull-down to GND"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Resp::Pulldown)
    }
    #[doc = "Pull-up to VDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Resp::Pullup)
    }
    #[doc = "Set input at VDD/2"]
    #[inline(always)]
    pub fn vdd1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Resp::Vdd1_2)
    }
}
#[doc = "Negative channel resistor control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resn {
    #[doc = "0: Bypass resistor ladder"]
    Bypass = 0,
    #[doc = "1: Pull-down to GND"]
    Pulldown = 1,
    #[doc = "2: Pull-up to VDD"]
    Pullup = 2,
    #[doc = "3: Set input at VDD/2"]
    Vdd1_2 = 3,
}
impl From<Resn> for u8 {
    #[inline(always)]
    fn from(variant: Resn) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resn {
    type Ux = u8;
}
impl crate::IsEnum for Resn {}
#[doc = "Field `RESN` reader - Negative channel resistor control"]
pub type ResnR = crate::FieldReader<Resn>;
impl ResnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resn {
        match self.bits {
            0 => Resn::Bypass,
            1 => Resn::Pulldown,
            2 => Resn::Pullup,
            3 => Resn::Vdd1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Bypass resistor ladder"]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Resn::Bypass
    }
    #[doc = "Pull-down to GND"]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Resn::Pulldown
    }
    #[doc = "Pull-up to VDD"]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Resn::Pullup
    }
    #[doc = "Set input at VDD/2"]
    #[inline(always)]
    pub fn is_vdd1_2(&self) -> bool {
        *self == Resn::Vdd1_2
    }
}
#[doc = "Field `RESN` writer - Negative channel resistor control"]
pub type ResnW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resn, crate::Safe>;
impl<'a, REG> ResnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bypass resistor ladder"]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Resn::Bypass)
    }
    #[doc = "Pull-down to GND"]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Resn::Pulldown)
    }
    #[doc = "Pull-up to VDD"]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Resn::Pullup)
    }
    #[doc = "Set input at VDD/2"]
    #[inline(always)]
    pub fn vdd1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Resn::Vdd1_2)
    }
}
#[doc = "Gain control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gain {
    #[doc = "0: 1/6"]
    Gain1_6 = 0,
    #[doc = "1: 1/5"]
    Gain1_5 = 1,
    #[doc = "2: 1/4"]
    Gain1_4 = 2,
    #[doc = "3: 1/3"]
    Gain1_3 = 3,
    #[doc = "4: 1/2"]
    Gain1_2 = 4,
    #[doc = "5: 1"]
    Gain1 = 5,
    #[doc = "6: 2"]
    Gain2 = 6,
    #[doc = "7: 4"]
    Gain4 = 7,
}
impl From<Gain> for u8 {
    #[inline(always)]
    fn from(variant: Gain) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gain {
    type Ux = u8;
}
impl crate::IsEnum for Gain {}
#[doc = "Field `GAIN` reader - Gain control"]
pub type GainR = crate::FieldReader<Gain>;
impl GainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gain {
        match self.bits {
            0 => Gain::Gain1_6,
            1 => Gain::Gain1_5,
            2 => Gain::Gain1_4,
            3 => Gain::Gain1_3,
            4 => Gain::Gain1_2,
            5 => Gain::Gain1,
            6 => Gain::Gain2,
            7 => Gain::Gain4,
            _ => unreachable!(),
        }
    }
    #[doc = "1/6"]
    #[inline(always)]
    pub fn is_gain1_6(&self) -> bool {
        *self == Gain::Gain1_6
    }
    #[doc = "1/5"]
    #[inline(always)]
    pub fn is_gain1_5(&self) -> bool {
        *self == Gain::Gain1_5
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn is_gain1_4(&self) -> bool {
        *self == Gain::Gain1_4
    }
    #[doc = "1/3"]
    #[inline(always)]
    pub fn is_gain1_3(&self) -> bool {
        *self == Gain::Gain1_3
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn is_gain1_2(&self) -> bool {
        *self == Gain::Gain1_2
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn is_gain1(&self) -> bool {
        *self == Gain::Gain1
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == Gain::Gain2
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == Gain::Gain4
    }
}
#[doc = "Field `GAIN` writer - Gain control"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 3, Gain, crate::Safe>;
impl<'a, REG> GainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/6"]
    #[inline(always)]
    pub fn gain1_6(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain1_6)
    }
    #[doc = "1/5"]
    #[inline(always)]
    pub fn gain1_5(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain1_5)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn gain1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain1_4)
    }
    #[doc = "1/3"]
    #[inline(always)]
    pub fn gain1_3(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain1_3)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn gain1_2(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain1_2)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn gain1(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain2)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn gain4(self) -> &'a mut crate::W<REG> {
        self.variant(Gain::Gain4)
    }
}
#[doc = "Reference control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refsel {
    #[doc = "0: Internal reference (0.6 V)"]
    Internal = 0,
    #[doc = "1: VDD/4 as reference"]
    Vdd1_4 = 1,
}
impl From<Refsel> for bool {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFSEL` reader - Reference control"]
pub type RefselR = crate::BitReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refsel {
        match self.bits {
            false => Refsel::Internal,
            true => Refsel::Vdd1_4,
        }
    }
    #[doc = "Internal reference (0.6 V)"]
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == Refsel::Internal
    }
    #[doc = "VDD/4 as reference"]
    #[inline(always)]
    pub fn is_vdd1_4(&self) -> bool {
        *self == Refsel::Vdd1_4
    }
}
#[doc = "Field `REFSEL` writer - Reference control"]
pub type RefselW<'a, REG> = crate::BitWriter<'a, REG, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal reference (0.6 V)"]
    #[inline(always)]
    pub fn internal(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Internal)
    }
    #[doc = "VDD/4 as reference"]
    #[inline(always)]
    pub fn vdd1_4(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Vdd1_4)
    }
}
#[doc = "Acquisition time, the time the SAADC uses to sample the input voltage\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tacq {
    #[doc = "0: 3 us"]
    _3us = 0,
    #[doc = "1: 5 us"]
    _5us = 1,
    #[doc = "2: 10 us"]
    _10us = 2,
    #[doc = "3: 15 us"]
    _15us = 3,
    #[doc = "4: 20 us"]
    _20us = 4,
    #[doc = "5: 40 us"]
    _40us = 5,
}
impl From<Tacq> for u8 {
    #[inline(always)]
    fn from(variant: Tacq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tacq {
    type Ux = u8;
}
impl crate::IsEnum for Tacq {}
#[doc = "Field `TACQ` reader - Acquisition time, the time the SAADC uses to sample the input voltage"]
pub type TacqR = crate::FieldReader<Tacq>;
impl TacqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tacq> {
        match self.bits {
            0 => Some(Tacq::_3us),
            1 => Some(Tacq::_5us),
            2 => Some(Tacq::_10us),
            3 => Some(Tacq::_15us),
            4 => Some(Tacq::_20us),
            5 => Some(Tacq::_40us),
            _ => None,
        }
    }
    #[doc = "3 us"]
    #[inline(always)]
    pub fn is_3us(&self) -> bool {
        *self == Tacq::_3us
    }
    #[doc = "5 us"]
    #[inline(always)]
    pub fn is_5us(&self) -> bool {
        *self == Tacq::_5us
    }
    #[doc = "10 us"]
    #[inline(always)]
    pub fn is_10us(&self) -> bool {
        *self == Tacq::_10us
    }
    #[doc = "15 us"]
    #[inline(always)]
    pub fn is_15us(&self) -> bool {
        *self == Tacq::_15us
    }
    #[doc = "20 us"]
    #[inline(always)]
    pub fn is_20us(&self) -> bool {
        *self == Tacq::_20us
    }
    #[doc = "40 us"]
    #[inline(always)]
    pub fn is_40us(&self) -> bool {
        *self == Tacq::_40us
    }
}
#[doc = "Field `TACQ` writer - Acquisition time, the time the SAADC uses to sample the input voltage"]
pub type TacqW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tacq>;
impl<'a, REG> TacqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "3 us"]
    #[inline(always)]
    pub fn _3us(self) -> &'a mut crate::W<REG> {
        self.variant(Tacq::_3us)
    }
    #[doc = "5 us"]
    #[inline(always)]
    pub fn _5us(self) -> &'a mut crate::W<REG> {
        self.variant(Tacq::_5us)
    }
    #[doc = "10 us"]
    #[inline(always)]
    pub fn _10us(self) -> &'a mut crate::W<REG> {
        self.variant(Tacq::_10us)
    }
    #[doc = "15 us"]
    #[inline(always)]
    pub fn _15us(self) -> &'a mut crate::W<REG> {
        self.variant(Tacq::_15us)
    }
    #[doc = "20 us"]
    #[inline(always)]
    pub fn _20us(self) -> &'a mut crate::W<REG> {
        self.variant(Tacq::_20us)
    }
    #[doc = "40 us"]
    #[inline(always)]
    pub fn _40us(self) -> &'a mut crate::W<REG> {
        self.variant(Tacq::_40us)
    }
}
#[doc = "Enable differential mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Single-ended, PSELN will be ignored, negative input to SAADC shorted to GND"]
    Se = 0,
    #[doc = "1: Differential"]
    Diff = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Enable differential mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Se,
            true => Mode::Diff,
        }
    }
    #[doc = "Single-ended, PSELN will be ignored, negative input to SAADC shorted to GND"]
    #[inline(always)]
    pub fn is_se(&self) -> bool {
        *self == Mode::Se
    }
    #[doc = "Differential"]
    #[inline(always)]
    pub fn is_diff(&self) -> bool {
        *self == Mode::Diff
    }
}
#[doc = "Field `MODE` writer - Enable differential mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-ended, PSELN will be ignored, negative input to SAADC shorted to GND"]
    #[inline(always)]
    pub fn se(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Se)
    }
    #[doc = "Differential"]
    #[inline(always)]
    pub fn diff(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Diff)
    }
}
#[doc = "Enable burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Burst {
    #[doc = "0: Burst mode is disabled (normal operation)"]
    Disabled = 0,
    #[doc = "1: Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    Enabled = 1,
}
impl From<Burst> for bool {
    #[inline(always)]
    fn from(variant: Burst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST` reader - Enable burst mode"]
pub type BurstR = crate::BitReader<Burst>;
impl BurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burst {
        match self.bits {
            false => Burst::Disabled,
            true => Burst::Enabled,
        }
    }
    #[doc = "Burst mode is disabled (normal operation)"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Burst::Disabled
    }
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Burst::Enabled
    }
}
#[doc = "Field `BURST` writer - Enable burst mode"]
pub type BurstW<'a, REG> = crate::BitWriter<'a, REG, Burst>;
impl<'a, REG> BurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst mode is disabled (normal operation)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Disabled)
    }
    #[doc = "Burst mode is enabled. SAADC takes 2^OVERSAMPLE number of samples as fast as it can, and sends the average to Data RAM."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - Positive channel resistor control"]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Negative channel resistor control"]
    #[inline(always)]
    pub fn resn(&self) -> ResnR {
        ResnR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Gain control"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Reference control"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Acquisition time, the time the SAADC uses to sample the input voltage"]
    #[inline(always)]
    pub fn tacq(&self) -> TacqR {
        TacqR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Enable differential mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable burst mode"]
    #[inline(always)]
    pub fn burst(&self) -> BurstR {
        BurstR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Positive channel resistor control"]
    #[inline(always)]
    pub fn resp(&mut self) -> RespW<ConfigSpec> {
        RespW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Negative channel resistor control"]
    #[inline(always)]
    pub fn resn(&mut self) -> ResnW<ConfigSpec> {
        ResnW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Gain control"]
    #[inline(always)]
    pub fn gain(&mut self) -> GainW<ConfigSpec> {
        GainW::new(self, 8)
    }
    #[doc = "Bit 12 - Reference control"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<ConfigSpec> {
        RefselW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Acquisition time, the time the SAADC uses to sample the input voltage"]
    #[inline(always)]
    pub fn tacq(&mut self) -> TacqW<ConfigSpec> {
        TacqW::new(self, 16)
    }
    #[doc = "Bit 20 - Enable differential mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<ConfigSpec> {
        ModeW::new(self, 20)
    }
    #[doc = "Bit 24 - Enable burst mode"]
    #[inline(always)]
    pub fn burst(&mut self) -> BurstW<ConfigSpec> {
        BurstW::new(self, 24)
    }
}
#[doc = "Description cluster: Input configuration for CH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x0002_0000"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
