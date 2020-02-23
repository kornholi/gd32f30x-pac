#[doc = "Reader of register PCF0"]
pub type R = crate::R<u32, super::PCF0>;
#[doc = "Writer for register PCF0"]
pub type W = crate::W<u32, super::PCF0>;
#[doc = "Register PCF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWJ_CFG`"]
pub type SWJ_CFG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWJ_CFG`"]
pub struct SWJ_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWJ_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Reader of field `ADC1_ETRGREG_REMAP`"]
pub type ADC1_ETRGREG_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1_ETRGREG_REMAP`"]
pub struct ADC1_ETRGREG_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_ETRGREG_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `ADC1_ETRGINJ_REMAP`"]
pub type ADC1_ETRGINJ_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1_ETRGINJ_REMAP`"]
pub struct ADC1_ETRGINJ_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_ETRGINJ_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ADC0_ETRGREG_REMAP`"]
pub type ADC0_ETRGREG_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0_ETRGREG_REMAP`"]
pub struct ADC0_ETRGREG_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_ETRGREG_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `ADC0_ETRGINJ_REMAP`"]
pub type ADC0_ETRGINJ_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC0_ETRGINJ_REMAP`"]
pub struct ADC0_ETRGINJ_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC0_ETRGINJ_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `TIMER4CH3_IREMAP`"]
pub type TIMER4CH3_IREMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER4CH3_IREMAP`"]
pub struct TIMER4CH3_IREMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4CH3_IREMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `PD01_REMAP`"]
pub type PD01_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PD01_REMAP`"]
pub struct PD01_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> PD01_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `CAN_REMAP`"]
pub type CAN_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAN_REMAP`"]
pub struct CAN_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | (((value as u32) & 0x03) << 13);
        self.w
    }
}
#[doc = "Reader of field `TIMER3_REMAP`"]
pub type TIMER3_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIMER3_REMAP`"]
pub struct TIMER3_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TIMER2_REMAP`"]
pub type TIMER2_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER2_REMAP`"]
pub struct TIMER2_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `TIMER1_REMAP`"]
pub type TIMER1_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER1_REMAP`"]
pub struct TIMER1_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMER0_REMAP`"]
pub type TIMER0_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMER0_REMAP`"]
pub struct TIMER0_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `USART2_REMAP`"]
pub type USART2_REMAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `USART2_REMAP`"]
pub struct USART2_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2_REMAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `USART1_REMAP`"]
pub type USART1_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1_REMAP`"]
pub struct USART1_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `USART0_REMAP`"]
pub type USART0_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART0_REMAP`"]
pub struct USART0_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> USART0_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `I2C0_REMAP`"]
pub type I2C0_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0_REMAP`"]
pub struct I2C0_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SPI0_REMAP`"]
pub type SPI0_REMAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI0_REMAP`"]
pub struct SPI0_REMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI0_REMAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    pub fn swj_cfg(&self) -> SWJ_CFG_R {
        SWJ_CFG_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 20 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&self) -> ADC1_ETRGREG_REMAP_R {
        ADC1_ETRGREG_REMAP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ADC 1 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrginj_remap(&self) -> ADC1_ETRGINJ_REMAP_R {
        ADC1_ETRGINJ_REMAP_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ADC 0 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrgreg_remap(&self) -> ADC0_ETRGREG_REMAP_R {
        ADC0_ETRGREG_REMAP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ADC 0 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrginj_remap(&self) -> ADC0_ETRGINJ_REMAP_R {
        ADC0_ETRGINJ_REMAP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TIMER4 channel3 internal remapping"]
    #[inline(always)]
    pub fn timer4ch3_iremap(&self) -> TIMER4CH3_IREMAP_R {
        TIMER4CH3_IREMAP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_remap(&self) -> PD01_REMAP_R {
        PD01_REMAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - CAN interface remapping"]
    #[inline(always)]
    pub fn can_remap(&self) -> CAN_REMAP_R {
        CAN_REMAP_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 12 - TIMER3 remapping"]
    #[inline(always)]
    pub fn timer3_remap(&self) -> TIMER3_REMAP_R {
        TIMER3_REMAP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11 - TIMER2 remapping"]
    #[inline(always)]
    pub fn timer2_remap(&self) -> TIMER2_REMAP_R {
        TIMER2_REMAP_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - TIMER1 remapping"]
    #[inline(always)]
    pub fn timer1_remap(&self) -> TIMER1_REMAP_R {
        TIMER1_REMAP_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - TIMER0 remapping"]
    #[inline(always)]
    pub fn timer0_remap(&self) -> TIMER0_REMAP_R {
        TIMER0_REMAP_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&self) -> USART2_REMAP_R {
        USART2_REMAP_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&self) -> USART1_REMAP_R {
        USART1_REMAP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USART0 remapping"]
    #[inline(always)]
    pub fn usart0_remap(&self) -> USART0_REMAP_R {
        USART0_REMAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C0 remapping"]
    #[inline(always)]
    pub fn i2c0_remap(&self) -> I2C0_REMAP_R {
        I2C0_REMAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SPI0 remapping"]
    #[inline(always)]
    pub fn spi0_remap(&self) -> SPI0_REMAP_R {
        SPI0_REMAP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Serial wire JTAG configuration"]
    #[inline(always)]
    pub fn swj_cfg(&mut self) -> SWJ_CFG_W {
        SWJ_CFG_W { w: self }
    }
    #[doc = "Bit 20 - ADC 1 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrgreg_remap(&mut self) -> ADC1_ETRGREG_REMAP_W {
        ADC1_ETRGREG_REMAP_W { w: self }
    }
    #[doc = "Bit 19 - ADC 1 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc1_etrginj_remap(&mut self) -> ADC1_ETRGINJ_REMAP_W {
        ADC1_ETRGINJ_REMAP_W { w: self }
    }
    #[doc = "Bit 18 - ADC 0 external trigger regular conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrgreg_remap(&mut self) -> ADC0_ETRGREG_REMAP_W {
        ADC0_ETRGREG_REMAP_W { w: self }
    }
    #[doc = "Bit 17 - ADC 0 external trigger injected conversion remapping"]
    #[inline(always)]
    pub fn adc0_etrginj_remap(&mut self) -> ADC0_ETRGINJ_REMAP_W {
        ADC0_ETRGINJ_REMAP_W { w: self }
    }
    #[doc = "Bit 16 - TIMER4 channel3 internal remapping"]
    #[inline(always)]
    pub fn timer4ch3_iremap(&mut self) -> TIMER4CH3_IREMAP_W {
        TIMER4CH3_IREMAP_W { w: self }
    }
    #[doc = "Bit 15 - Port D0/Port D1 mapping on OSC_IN/OSC_OUT"]
    #[inline(always)]
    pub fn pd01_remap(&mut self) -> PD01_REMAP_W {
        PD01_REMAP_W { w: self }
    }
    #[doc = "Bits 13:14 - CAN interface remapping"]
    #[inline(always)]
    pub fn can_remap(&mut self) -> CAN_REMAP_W {
        CAN_REMAP_W { w: self }
    }
    #[doc = "Bit 12 - TIMER3 remapping"]
    #[inline(always)]
    pub fn timer3_remap(&mut self) -> TIMER3_REMAP_W {
        TIMER3_REMAP_W { w: self }
    }
    #[doc = "Bits 10:11 - TIMER2 remapping"]
    #[inline(always)]
    pub fn timer2_remap(&mut self) -> TIMER2_REMAP_W {
        TIMER2_REMAP_W { w: self }
    }
    #[doc = "Bits 8:9 - TIMER1 remapping"]
    #[inline(always)]
    pub fn timer1_remap(&mut self) -> TIMER1_REMAP_W {
        TIMER1_REMAP_W { w: self }
    }
    #[doc = "Bits 6:7 - TIMER0 remapping"]
    #[inline(always)]
    pub fn timer0_remap(&mut self) -> TIMER0_REMAP_W {
        TIMER0_REMAP_W { w: self }
    }
    #[doc = "Bits 4:5 - USART2 remapping"]
    #[inline(always)]
    pub fn usart2_remap(&mut self) -> USART2_REMAP_W {
        USART2_REMAP_W { w: self }
    }
    #[doc = "Bit 3 - USART1 remapping"]
    #[inline(always)]
    pub fn usart1_remap(&mut self) -> USART1_REMAP_W {
        USART1_REMAP_W { w: self }
    }
    #[doc = "Bit 2 - USART0 remapping"]
    #[inline(always)]
    pub fn usart0_remap(&mut self) -> USART0_REMAP_W {
        USART0_REMAP_W { w: self }
    }
    #[doc = "Bit 1 - I2C0 remapping"]
    #[inline(always)]
    pub fn i2c0_remap(&mut self) -> I2C0_REMAP_W {
        I2C0_REMAP_W { w: self }
    }
    #[doc = "Bit 0 - SPI0 remapping"]
    #[inline(always)]
    pub fn spi0_remap(&mut self) -> SPI0_REMAP_W {
        SPI0_REMAP_W { w: self }
    }
}
