#[doc = "Register `UARTPCELLID2` reader"]
pub struct R(crate::R<UARTPCELLID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTPCELLID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTPCELLID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTPCELLID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UARTPCELLID2` reader - These bits read back as 0x05"]
pub type UARTPCELLID2_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x05"]
    #[inline(always)]
    pub fn uartpcellid2(&self) -> UARTPCELLID2_R {
        UARTPCELLID2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UARTPCellID2 Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartpcellid2](index.html) module"]
pub struct UARTPCELLID2_SPEC;
impl crate::RegisterSpec for UARTPCELLID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartpcellid2::R](R) reader structure"]
impl crate::Readable for UARTPCELLID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UARTPCELLID2 to value 0x05"]
impl crate::Resettable for UARTPCELLID2_SPEC {
    const RESET_VALUE: Self::Ux = 0x05;
}
