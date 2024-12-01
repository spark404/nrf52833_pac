#[doc = "Register `PSELN` reader"]
pub type R = crate::R<PselnSpec>;
#[doc = "Register `PSELN` writer"]
pub type W = crate::W<PselnSpec>;
#[doc = "Analog negative input, enables differential channel\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pseln {
    #[doc = "0: Not connected"]
    Nc = 0,
    #[doc = "1: AIN0"]
    AnalogInput0 = 1,
    #[doc = "2: AIN1"]
    AnalogInput1 = 2,
    #[doc = "3: AIN2"]
    AnalogInput2 = 3,
    #[doc = "4: AIN3"]
    AnalogInput3 = 4,
    #[doc = "5: AIN4"]
    AnalogInput4 = 5,
    #[doc = "6: AIN5"]
    AnalogInput5 = 6,
    #[doc = "7: AIN6"]
    AnalogInput6 = 7,
    #[doc = "8: AIN7"]
    AnalogInput7 = 8,
    #[doc = "9: VDD"]
    Vdd = 9,
    #[doc = "13: VDDH/5"]
    Vddhdiv5 = 13,
}
impl From<Pseln> for u8 {
    #[inline(always)]
    fn from(variant: Pseln) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pseln {
    type Ux = u8;
}
impl crate::IsEnum for Pseln {}
#[doc = "Field `PSELN` reader - Analog negative input, enables differential channel"]
pub type PselnR = crate::FieldReader<Pseln>;
impl PselnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pseln> {
        match self.bits {
            0 => Some(Pseln::Nc),
            1 => Some(Pseln::AnalogInput0),
            2 => Some(Pseln::AnalogInput1),
            3 => Some(Pseln::AnalogInput2),
            4 => Some(Pseln::AnalogInput3),
            5 => Some(Pseln::AnalogInput4),
            6 => Some(Pseln::AnalogInput5),
            7 => Some(Pseln::AnalogInput6),
            8 => Some(Pseln::AnalogInput7),
            9 => Some(Pseln::Vdd),
            13 => Some(Pseln::Vddhdiv5),
            _ => None,
        }
    }
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn is_nc(&self) -> bool {
        *self == Pseln::Nc
    }
    #[doc = "AIN0"]
    #[inline(always)]
    pub fn is_analog_input0(&self) -> bool {
        *self == Pseln::AnalogInput0
    }
    #[doc = "AIN1"]
    #[inline(always)]
    pub fn is_analog_input1(&self) -> bool {
        *self == Pseln::AnalogInput1
    }
    #[doc = "AIN2"]
    #[inline(always)]
    pub fn is_analog_input2(&self) -> bool {
        *self == Pseln::AnalogInput2
    }
    #[doc = "AIN3"]
    #[inline(always)]
    pub fn is_analog_input3(&self) -> bool {
        *self == Pseln::AnalogInput3
    }
    #[doc = "AIN4"]
    #[inline(always)]
    pub fn is_analog_input4(&self) -> bool {
        *self == Pseln::AnalogInput4
    }
    #[doc = "AIN5"]
    #[inline(always)]
    pub fn is_analog_input5(&self) -> bool {
        *self == Pseln::AnalogInput5
    }
    #[doc = "AIN6"]
    #[inline(always)]
    pub fn is_analog_input6(&self) -> bool {
        *self == Pseln::AnalogInput6
    }
    #[doc = "AIN7"]
    #[inline(always)]
    pub fn is_analog_input7(&self) -> bool {
        *self == Pseln::AnalogInput7
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        *self == Pseln::Vdd
    }
    #[doc = "VDDH/5"]
    #[inline(always)]
    pub fn is_vddhdiv5(&self) -> bool {
        *self == Pseln::Vddhdiv5
    }
}
#[doc = "Field `PSELN` writer - Analog negative input, enables differential channel"]
pub type PselnW<'a, REG> = crate::FieldWriter<'a, REG, 5, Pseln>;
impl<'a, REG> PselnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not connected"]
    #[inline(always)]
    pub fn nc(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::Nc)
    }
    #[doc = "AIN0"]
    #[inline(always)]
    pub fn analog_input0(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::AnalogInput0)
    }
    #[doc = "AIN1"]
    #[inline(always)]
    pub fn analog_input1(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::AnalogInput1)
    }
    #[doc = "AIN2"]
    #[inline(always)]
    pub fn analog_input2(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::AnalogInput2)
    }
    #[doc = "AIN3"]
    #[inline(always)]
    pub fn analog_input3(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::AnalogInput3)
    }
    #[doc = "AIN4"]
    #[inline(always)]
    pub fn analog_input4(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::AnalogInput4)
    }
    #[doc = "AIN5"]
    #[inline(always)]
    pub fn analog_input5(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::AnalogInput5)
    }
    #[doc = "AIN6"]
    #[inline(always)]
    pub fn analog_input6(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::AnalogInput6)
    }
    #[doc = "AIN7"]
    #[inline(always)]
    pub fn analog_input7(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::AnalogInput7)
    }
    #[doc = "VDD"]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::Vdd)
    }
    #[doc = "VDDH/5"]
    #[inline(always)]
    pub fn vddhdiv5(self) -> &'a mut crate::W<REG> {
        self.variant(Pseln::Vddhdiv5)
    }
}
impl R {
    #[doc = "Bits 0:4 - Analog negative input, enables differential channel"]
    #[inline(always)]
    pub fn pseln(&self) -> PselnR {
        PselnR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Analog negative input, enables differential channel"]
    #[inline(always)]
    pub fn pseln(&mut self) -> PselnW<PselnSpec> {
        PselnW::new(self, 0)
    }
}
#[doc = "Description cluster: Input negative pin selection for CH\\[n\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`pseln::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pseln::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PselnSpec;
impl crate::RegisterSpec for PselnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pseln::R`](R) reader structure"]
impl crate::Readable for PselnSpec {}
#[doc = "`write(|w| ..)` method takes [`pseln::W`](W) writer structure"]
impl crate::Writable for PselnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSELN to value 0"]
impl crate::Resettable for PselnSpec {
    const RESET_VALUE: u32 = 0;
}
