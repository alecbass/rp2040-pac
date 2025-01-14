#[doc = "Register `TIMERAWL` reader"]
pub struct R(crate::R<TIMERAWL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMERAWL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMERAWL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMERAWL_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TIMERAWL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Raw read from bits 31:0 of time (no side effects)  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [timerawl](index.html) module"]
pub struct TIMERAWL_SPEC;
impl crate::RegisterSpec for TIMERAWL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timerawl::R](R) reader structure"]
impl crate::Readable for TIMERAWL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMERAWL to value 0"]
impl crate::Resettable for TIMERAWL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
