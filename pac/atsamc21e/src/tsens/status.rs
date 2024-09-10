#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `OVF` reader - Result Overflow"]
pub type OvfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Result Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0;
}
