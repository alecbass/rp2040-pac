#[doc = "Register `IC_COMP_TYPE` reader"]
pub struct R(crate::R<IC_COMP_TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_COMP_TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_COMP_TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_COMP_TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IC_COMP_TYPE` reader - Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number."]
pub type IC_COMP_TYPE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Designware Component Type number = 0x44_57_01_40. This assigned unique hex value is constant and is derived from the two ASCII letters 'DW' followed by a 16-bit unsigned number."]
    #[inline(always)]
    pub fn ic_comp_type(&self) -> IC_COMP_TYPE_R {
        IC_COMP_TYPE_R::new(self.bits)
    }
}
#[doc = "I2C Component Type Register  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_comp_type](index.html) module"]
pub struct IC_COMP_TYPE_SPEC;
impl crate::RegisterSpec for IC_COMP_TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_comp_type::R](R) reader structure"]
impl crate::Readable for IC_COMP_TYPE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IC_COMP_TYPE to value 0x4457_0140"]
impl crate::Resettable for IC_COMP_TYPE_SPEC {
    const RESET_VALUE: Self::Ux = 0x4457_0140;
}
