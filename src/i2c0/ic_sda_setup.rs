#[doc = "Register `IC_SDA_SETUP` reader"]
pub struct R(crate::R<IC_SDA_SETUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_SDA_SETUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_SDA_SETUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_SDA_SETUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_SDA_SETUP` writer"]
pub struct W(crate::W<IC_SDA_SETUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_SDA_SETUP_SPEC>;
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
impl From<crate::W<IC_SDA_SETUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_SDA_SETUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SDA_SETUP` reader - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
pub type SDA_SETUP_R = crate::FieldReader;
#[doc = "Field `SDA_SETUP` writer - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
pub type SDA_SETUP_W<'a, const O: u8> = crate::FieldWriter<'a, IC_SDA_SETUP_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
    #[inline(always)]
    pub fn sda_setup(&self) -> SDA_SETUP_R {
        SDA_SETUP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
    #[inline(always)]
    #[must_use]
    pub fn sda_setup(&mut self) -> SDA_SETUP_W<0> {
        SDA_SETUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SDA Setup Register  

 This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2.  

 Writes to this register succeed only when IC_ENABLE\\[0\\]
= 0.  

 Note: The length of setup time is calculated using \\[(IC_SDA_SETUP - 1) * (ic_clk_period)\\], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_sda_setup](index.html) module"]
pub struct IC_SDA_SETUP_SPEC;
impl crate::RegisterSpec for IC_SDA_SETUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_sda_setup::R](R) reader structure"]
impl crate::Readable for IC_SDA_SETUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_sda_setup::W](W) writer structure"]
impl crate::Writable for IC_SDA_SETUP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC_SDA_SETUP to value 0x64"]
impl crate::Resettable for IC_SDA_SETUP_SPEC {
    const RESET_VALUE: Self::Ux = 0x64;
}
