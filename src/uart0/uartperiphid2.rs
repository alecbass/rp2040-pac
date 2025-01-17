#[doc = "Register `UARTPERIPHID2` reader"]
pub struct R(crate::R<UARTPERIPHID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTPERIPHID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTPERIPHID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTPERIPHID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DESIGNER1` reader - These bits read back as 0x4"]
pub type DESIGNER1_R = crate::FieldReader;
#[doc = "Field `REVISION` reader - This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3"]
pub type REVISION_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x4"]
    #[inline(always)]
    pub fn designer1(&self) -> DESIGNER1_R {
        DESIGNER1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "UARTPeriphID2 Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartperiphid2](index.html) module"]
pub struct UARTPERIPHID2_SPEC;
impl crate::RegisterSpec for UARTPERIPHID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartperiphid2::R](R) reader structure"]
impl crate::Readable for UARTPERIPHID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTPERIPHID2 to value 0x34"]
impl crate::Resettable for UARTPERIPHID2_SPEC {
    const RESET_VALUE: Self::Ux = 0x34;
}
