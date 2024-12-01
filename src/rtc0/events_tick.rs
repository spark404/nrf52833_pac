#[doc = "Register `EVENTS_TICK` reader"]
pub type R = crate::R<EventsTickSpec>;
#[doc = "Register `EVENTS_TICK` writer"]
pub type W = crate::W<EventsTickSpec>;
#[doc = "Event on COUNTER increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventsTick {
    #[doc = "0: Event not generated"]
    NotGenerated = 0,
    #[doc = "1: Event generated"]
    Generated = 1,
}
impl From<EventsTick> for bool {
    #[inline(always)]
    fn from(variant: EventsTick) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EVENTS_TICK` reader - Event on COUNTER increment"]
pub type EventsTickR = crate::BitReader<EventsTick>;
impl EventsTickR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EventsTick {
        match self.bits {
            false => EventsTick::NotGenerated,
            true => EventsTick::Generated,
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EventsTick::NotGenerated
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EventsTick::Generated
    }
}
#[doc = "Field `EVENTS_TICK` writer - Event on COUNTER increment"]
pub type EventsTickW<'a, REG> = crate::BitWriter<'a, REG, EventsTick>;
impl<'a, REG> EventsTickW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTick::NotGenerated)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut crate::W<REG> {
        self.variant(EventsTick::Generated)
    }
}
impl R {
    #[doc = "Bit 0 - Event on COUNTER increment"]
    #[inline(always)]
    pub fn events_tick(&self) -> EventsTickR {
        EventsTickR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event on COUNTER increment"]
    #[inline(always)]
    pub fn events_tick(&mut self) -> EventsTickW<EventsTickSpec> {
        EventsTickW::new(self, 0)
    }
}
#[doc = "Event on COUNTER increment\n\nYou can [`read`](crate::Reg::read) this register and get [`events_tick::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_tick::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EventsTickSpec;
impl crate::RegisterSpec for EventsTickSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`events_tick::R`](R) reader structure"]
impl crate::Readable for EventsTickSpec {}
#[doc = "`write(|w| ..)` method takes [`events_tick::W`](W) writer structure"]
impl crate::Writable for EventsTickSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVENTS_TICK to value 0"]
impl crate::Resettable for EventsTickSpec {
    const RESET_VALUE: u32 = 0;
}
