#[doc = "Register `GPIO_IN` reader"]
pub struct R(crate::R<GPIO_IN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_IN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_IN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_IN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIO_IN` reader - Input value for GPIO0...29"]
pub type GPIO_IN_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:29 - Input value for GPIO0...29"]
    #[inline(always)]
    pub fn gpio_in(&self) -> GPIO_IN_R {
        GPIO_IN_R::new(self.bits & 0x3fff_ffff)
    }
}
#[doc = "Input value for GPIO pins  

This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_in](index.html) module"]
pub struct GPIO_IN_SPEC;
impl crate::RegisterSpec for GPIO_IN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_in::R](R) reader structure"]
impl crate::Readable for GPIO_IN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_IN to value 0"]
impl crate::Resettable for GPIO_IN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
