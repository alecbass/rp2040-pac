#[doc = "Register `UARTIBRD` reader"]
pub struct R(crate::R<UARTIBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTIBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTIBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTIBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTIBRD` writer"]
pub struct W(crate::W<UARTIBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTIBRD_SPEC>;
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
impl From<crate::W<UARTIBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTIBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD_DIVINT` reader - The integer baud rate divisor. These bits are cleared to 0 on reset."]
pub type BAUD_DIVINT_R = crate::FieldReader<u16>;
#[doc = "Field `BAUD_DIVINT` writer - The integer baud rate divisor. These bits are cleared to 0 on reset."]
pub type BAUD_DIVINT_W<'a, const O: u8> = crate::FieldWriter<'a, UARTIBRD_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn baud_divint(&self) -> BAUD_DIVINT_R {
        BAUD_DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    #[must_use]
    pub fn baud_divint(&mut self) -> BAUD_DIVINT_W<0> {
        BAUD_DIVINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Integer Baud Rate Register, UARTIBRD  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [uartibrd](index.html) module"]
pub struct UARTIBRD_SPEC;
impl crate::RegisterSpec for UARTIBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartibrd::R](R) reader structure"]
impl crate::Readable for UARTIBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartibrd::W](W) writer structure"]
impl crate::Writable for UARTIBRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTIBRD to value 0"]
impl crate::Resettable for UARTIBRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
