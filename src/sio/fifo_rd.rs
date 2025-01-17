#[doc = "Register `FIFO_RD` reader"]
pub struct R(crate::R<FIFO_RD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_RD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_RD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_RD_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FIFO_RD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Read access to this core's RX FIFO  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [fifo_rd](index.html) module"]
pub struct FIFO_RD_SPEC;
impl crate::RegisterSpec for FIFO_RD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_rd::R](R) reader structure"]
impl crate::Readable for FIFO_RD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FIFO_RD to value 0"]
impl crate::Resettable for FIFO_RD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
