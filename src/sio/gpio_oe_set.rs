#[doc = "Register `GPIO_OE_SET` writer"]
pub struct W(crate::W<GPIO_OE_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_OE_SET_SPEC>;
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
impl From<crate::W<GPIO_OE_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_OE_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIO_OE_SET` writer - Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`"]
pub type GPIO_OE_SET_W<'a, const O: u8> = crate::FieldWriter<'a, GPIO_OE_SET_SPEC, 30, O, u32>;
impl W {
    #[doc = "Bits 0:29 - Perform an atomic bit-set on GPIO_OE, i.e. `GPIO_OE |= wdata`"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_oe_set(&mut self) -> GPIO_OE_SET_W<0> {
        GPIO_OE_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO output enable set  

This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [gpio_oe_set](index.html) module"]
pub struct GPIO_OE_SET_SPEC;
impl crate::RegisterSpec for GPIO_OE_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpio_oe_set::W](W) writer structure"]
impl crate::Writable for GPIO_OE_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO_OE_SET to value 0"]
impl crate::Resettable for GPIO_OE_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
