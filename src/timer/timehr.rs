#[doc = "Register `TIMEHR` reader"]
pub struct R(crate::R<TIMEHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEHR_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIMEHR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Read from bits 63:32 of time  
 always read timelr before timehr  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [timehr](index.html) module"]
pub struct TIMEHR_SPEC;
impl crate::RegisterSpec for TIMEHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timehr::R](R) reader structure"]
impl crate::Readable for TIMEHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMEHR to value 0"]
impl crate::Resettable for TIMEHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
