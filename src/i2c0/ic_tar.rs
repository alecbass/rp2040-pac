#[doc = "Register `IC_TAR` reader"]
pub struct R(crate::R<IC_TAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IC_TAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IC_TAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IC_TAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IC_TAR` writer"]
pub struct W(crate::W<IC_TAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IC_TAR_SPEC>;
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
impl From<crate::W<IC_TAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IC_TAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IC_TAR` reader - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.  

 If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave."]
pub type IC_TAR_R = crate::FieldReader<u16>;
#[doc = "Field `IC_TAR` writer - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.  

 If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave."]
pub type IC_TAR_W<'a, const O: u8> = crate::FieldWriter<'a, IC_TAR_SPEC, 10, O, u16>;
#[doc = "Field `GC_OR_START` reader - If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0"]
pub type GC_OR_START_R = crate::BitReader<GC_OR_START_A>;
#[doc = "If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GC_OR_START_A {
    #[doc = "0: GENERAL_CALL byte transmission"]
    GENERAL_CALL = 0,
    #[doc = "1: START byte transmission"]
    START_BYTE = 1,
}
impl From<GC_OR_START_A> for bool {
    #[inline(always)]
    fn from(variant: GC_OR_START_A) -> Self {
        variant as u8 != 0
    }
}
impl GC_OR_START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GC_OR_START_A {
        match self.bits {
            false => GC_OR_START_A::GENERAL_CALL,
            true => GC_OR_START_A::START_BYTE,
        }
    }
    #[doc = "Checks if the value of the field is `GENERAL_CALL`"]
    #[inline(always)]
    pub fn is_general_call(&self) -> bool {
        *self == GC_OR_START_A::GENERAL_CALL
    }
    #[doc = "Checks if the value of the field is `START_BYTE`"]
    #[inline(always)]
    pub fn is_start_byte(&self) -> bool {
        *self == GC_OR_START_A::START_BYTE
    }
}
#[doc = "Field `GC_OR_START` writer - If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0"]
pub type GC_OR_START_W<'a, const O: u8> = crate::BitWriter<'a, IC_TAR_SPEC, O, GC_OR_START_A>;
impl<'a, const O: u8> GC_OR_START_W<'a, O> {
    #[doc = "GENERAL_CALL byte transmission"]
    #[inline(always)]
    pub fn general_call(self) -> &'a mut W {
        self.variant(GC_OR_START_A::GENERAL_CALL)
    }
    #[doc = "START byte transmission"]
    #[inline(always)]
    pub fn start_byte(self) -> &'a mut W {
        self.variant(GC_OR_START_A::START_BYTE)
    }
}
#[doc = "Field `SPECIAL` reader - This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0"]
pub type SPECIAL_R = crate::BitReader<SPECIAL_A>;
#[doc = "This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0  

Value on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPECIAL_A {
    #[doc = "0: Disables programming of GENERAL_CALL or START_BYTE transmission"]
    DISABLED = 0,
    #[doc = "1: Enables programming of GENERAL_CALL or START_BYTE transmission"]
    ENABLED = 1,
}
impl From<SPECIAL_A> for bool {
    #[inline(always)]
    fn from(variant: SPECIAL_A) -> Self {
        variant as u8 != 0
    }
}
impl SPECIAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPECIAL_A {
        match self.bits {
            false => SPECIAL_A::DISABLED,
            true => SPECIAL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPECIAL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPECIAL_A::ENABLED
    }
}
#[doc = "Field `SPECIAL` writer - This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0"]
pub type SPECIAL_W<'a, const O: u8> = crate::BitWriter<'a, IC_TAR_SPEC, O, SPECIAL_A>;
impl<'a, const O: u8> SPECIAL_W<'a, O> {
    #[doc = "Disables programming of GENERAL_CALL or START_BYTE transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPECIAL_A::DISABLED)
    }
    #[doc = "Enables programming of GENERAL_CALL or START_BYTE transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPECIAL_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:9 - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.  

 If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave."]
    #[inline(always)]
    pub fn ic_tar(&self) -> IC_TAR_R {
        IC_TAR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0"]
    #[inline(always)]
    pub fn gc_or_start(&self) -> GC_OR_START_R {
        GC_OR_START_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0"]
    #[inline(always)]
    pub fn special(&self) -> SPECIAL_R {
        SPECIAL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits.  

 If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave."]
    #[inline(always)]
    #[must_use]
    pub fn ic_tar(&mut self) -> IC_TAR_W<0> {
        IC_TAR_W::new(self)
    }
    #[doc = "Bit 10 - If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn gc_or_start(&mut self) -> GC_OR_START_W<10> {
        GC_OR_START_W::new(self)
    }
    #[doc = "Bit 11 - This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0"]
    #[inline(always)]
    #[must_use]
    pub fn special(&mut self) -> SPECIAL_W<11> {
        SPECIAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Target Address Register  

 This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE\\[0\\]
is set to 0.  

 Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS\\[2\\]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [ic_tar](index.html) module"]
pub struct IC_TAR_SPEC;
impl crate::RegisterSpec for IC_TAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ic_tar::R](R) reader structure"]
impl crate::Readable for IC_TAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ic_tar::W](W) writer structure"]
impl crate::Writable for IC_TAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IC_TAR to value 0x55"]
impl crate::Resettable for IC_TAR_SPEC {
    const RESET_VALUE: Self::Ux = 0x55;
}
