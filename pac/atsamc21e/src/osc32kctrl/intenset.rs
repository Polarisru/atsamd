#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `XOSC32KRDY` reader - XOSC32K Ready Interrupt Enable"]
pub type Xosc32krdyR = crate::BitReader;
#[doc = "Field `XOSC32KRDY` writer - XOSC32K Ready Interrupt Enable"]
pub type Xosc32krdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC32KRDY` reader - OSC32K Ready Interrupt Enable"]
pub type Osc32krdyR = crate::BitReader;
#[doc = "Field `OSC32KRDY` writer - OSC32K Ready Interrupt Enable"]
pub type Osc32krdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKFAIL` reader - XOSC32K Clock Failure Detector Interrupt Enable"]
pub type ClkfailR = crate::BitReader;
#[doc = "Field `CLKFAIL` writer - XOSC32K Clock Failure Detector Interrupt Enable"]
pub type ClkfailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xosc32krdy(&self) -> Xosc32krdyR {
        Xosc32krdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc32krdy(&self) -> Osc32krdyR {
        Osc32krdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn clkfail(&self) -> ClkfailR {
        ClkfailR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xosc32krdy(&mut self) -> Xosc32krdyW<IntensetSpec> {
        Xosc32krdyW::new(self, 0)
    }
    #[doc = "Bit 1 - OSC32K Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc32krdy(&mut self) -> Osc32krdyW<IntensetSpec> {
        Osc32krdyW::new(self, 1)
    }
    #[doc = "Bit 2 - XOSC32K Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clkfail(&mut self) -> ClkfailW<IntensetSpec> {
        ClkfailW::new(self, 2)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
