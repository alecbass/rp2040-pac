#[doc = "Register `INTS` reader"]
pub struct R(crate::R<INTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFO` reader - Triggered when the sample FIFO reaches a certain level.  
 This level can be programmed via the FCS_THRESH field."]
pub type FIFO_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Triggered when the sample FIFO reaches a certain level.  
 This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    pub fn fifo(&self) -> FIFO_R {
        FIFO_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Interrupt status after masking &amp; forcing  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ints](index.html) module"]
pub struct INTS_SPEC;
impl crate::RegisterSpec for INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ints::R](R) reader structure"]
impl crate::Readable for INTS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INTS to value 0"]
impl crate::Resettable for INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
