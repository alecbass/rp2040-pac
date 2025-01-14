#[doc = "Register `TIMEHW` writer"]
pub struct W(crate::W<TIMEHW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEHW_SPEC>;
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
impl From<crate::W<TIMEHW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEHW_SPEC>) -> Self {
        W(writer)
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIMEHW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write to bits 63:32 of time  
 always write timelw before timehw  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [timehw](index.html) module"]
pub struct TIMEHW_SPEC;
impl crate::RegisterSpec for TIMEHW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [timehw::W](W) writer structure"]
impl crate::Writable for TIMEHW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMEHW to value 0"]
impl crate::Resettable for TIMEHW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
