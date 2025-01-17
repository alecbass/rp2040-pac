#[doc = "Register `TIMELW` writer"]
pub struct W(crate::W<TIMELW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMELW_SPEC>;
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
impl From<crate::W<TIMELW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMELW_SPEC>) -> Self {
        W(writer)
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIMELW_SPEC> {
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
#[doc = "Write to bits 31:0 of time  
 writes do not get copied to time until timehw is written  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [timelw](index.html) module"]
pub struct TIMELW_SPEC;
impl crate::RegisterSpec for TIMELW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [timelw::W](W) writer structure"]
impl crate::Writable for TIMELW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMELW to value 0"]
impl crate::Resettable for TIMELW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
