#[doc = "Register `PID0` reader"]
pub type R = crate::R<Pid0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Peripheral Identification 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pid0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pid0Spec;
impl crate::RegisterSpec for Pid0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pid0::R`](R) reader structure"]
impl crate::Readable for Pid0Spec {}
#[doc = "`reset()` method sets PID0 to value 0"]
impl crate::Resettable for Pid0Spec {
    const RESET_VALUE: u32 = 0;
}
