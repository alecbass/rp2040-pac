#[doc = "Register `SYST_RVR` reader"]
pub struct R(crate::R<SYST_RVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_RVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_RVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_RVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_RVR` writer"]
pub struct W(crate::W<SYST_RVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_RVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYST_RVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_RVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELOAD` reader - Value to load into the SysTick Current Value Register when the counter reaches 0."]
pub type RELOAD_R = crate::FieldReader<u32>;
#[doc = "Field `RELOAD` writer - Value to load into the SysTick Current Value Register when the counter reaches 0."]
pub type RELOAD_W<'a, const O: u8> = crate::FieldWriter<'a, SYST_RVR_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Value to load into the SysTick Current Value Register when the counter reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<0> {
        RELOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN.  
 To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [syst_rvr](index.html) module"]
pub struct SYST_RVR_SPEC;
impl crate::RegisterSpec for SYST_RVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_rvr::R](R) reader structure"]
impl crate::Readable for SYST_RVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_rvr::W](W) writer structure"]
impl crate::Writable for SYST_RVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYST_RVR to value 0"]
impl crate::Resettable for SYST_RVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
