#[doc = "Register `DPLLSYNCBUSY` reader"]
pub type R = crate::R<DpllsyncbusySpec>;
#[doc = "Field `ENABLE` reader - DPLL Enable Synchronization Status"]
pub type EnableR = crate::BitReader;
#[doc = "Field `DPLLRATIO` reader - DPLL Ratio Synchronization Status"]
pub type DpllratioR = crate::BitReader;
#[doc = "Field `DPLLPRESC` reader - DPLL Prescaler Synchronization Status"]
pub type DpllprescR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - DPLL Enable Synchronization Status"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DPLL Ratio Synchronization Status"]
    #[inline(always)]
    pub fn dpllratio(&self) -> DpllratioR {
        DpllratioR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DPLL Prescaler Synchronization Status"]
    #[inline(always)]
    pub fn dpllpresc(&self) -> DpllprescR {
        DpllprescR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "DPLL Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllsyncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllsyncbusySpec;
impl crate::RegisterSpec for DpllsyncbusySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dpllsyncbusy::R`](R) reader structure"]
impl crate::Readable for DpllsyncbusySpec {}
#[doc = "`reset()` method sets DPLLSYNCBUSY to value 0"]
impl crate::Resettable for DpllsyncbusySpec {
    const RESET_VALUE: u8 = 0;
}
