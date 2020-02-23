#[doc = "Reader of register SPD"]
pub type R = crate::R<u32, super::SPD>;
#[doc = "Writer for register SPD"]
pub type W = crate::W<u32, super::SPD>;
#[doc = "Register SPD `reset()`'s with value 0"]
impl crate::ResetValue for super::SPD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPD15`"]
pub type SPD15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD15`"]
pub struct SPD15_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD15_W<'a> {
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
#[doc = "Reader of field `SPD14`"]
pub type SPD14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD14`"]
pub struct SPD14_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD14_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `SPD13`"]
pub type SPD13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD13`"]
pub struct SPD13_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD13_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SPD12`"]
pub type SPD12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD12`"]
pub struct SPD12_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD12_W<'a> {
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
#[doc = "Reader of field `SPD11`"]
pub type SPD11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD11`"]
pub struct SPD11_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `SPD10`"]
pub type SPD10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD10`"]
pub struct SPD10_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD10_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `SPD9`"]
pub type SPD9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD9`"]
pub struct SPD9_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD9_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `SPD8`"]
pub type SPD8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD8`"]
pub struct SPD8_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `SPD7`"]
pub type SPD7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD7`"]
pub struct SPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SPD6`"]
pub type SPD6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD6`"]
pub struct SPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SPD5`"]
pub type SPD5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD5`"]
pub struct SPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SPD4`"]
pub type SPD4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD4`"]
pub struct SPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SPD3`"]
pub type SPD3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD3`"]
pub struct SPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD3_W<'a> {
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
#[doc = "Reader of field `SPD2`"]
pub type SPD2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD2`"]
pub struct SPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD2_W<'a> {
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
#[doc = "Reader of field `SPD1`"]
pub type SPD1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD1`"]
pub struct SPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD1_W<'a> {
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
#[doc = "Reader of field `SPD0`"]
pub type SPD0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPD0`"]
pub struct SPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPD0_W<'a> {
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
    #[doc = "Bit 15 - Port 15 output max speed bits"]
    #[inline(always)]
    pub fn spd15(&self) -> SPD15_R {
        SPD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port 14 output max speed bits"]
    #[inline(always)]
    pub fn spd14(&self) -> SPD14_R {
        SPD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port 13 output max speed bits"]
    #[inline(always)]
    pub fn spd13(&self) -> SPD13_R {
        SPD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port 12 output max speed bits"]
    #[inline(always)]
    pub fn spd12(&self) -> SPD12_R {
        SPD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port 11 output max speed bits"]
    #[inline(always)]
    pub fn spd11(&self) -> SPD11_R {
        SPD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port 10 output max speed bits"]
    #[inline(always)]
    pub fn spd10(&self) -> SPD10_R {
        SPD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port 9 output max speed bits"]
    #[inline(always)]
    pub fn spd9(&self) -> SPD9_R {
        SPD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port 8 output max speed bits"]
    #[inline(always)]
    pub fn spd8(&self) -> SPD8_R {
        SPD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port 7 output max speed bits"]
    #[inline(always)]
    pub fn spd7(&self) -> SPD7_R {
        SPD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port 6 output max speed bits"]
    #[inline(always)]
    pub fn spd6(&self) -> SPD6_R {
        SPD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port 5 output max speed bits"]
    #[inline(always)]
    pub fn spd5(&self) -> SPD5_R {
        SPD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port 4 output max speed bits"]
    #[inline(always)]
    pub fn spd4(&self) -> SPD4_R {
        SPD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port 3 output max speed bits"]
    #[inline(always)]
    pub fn spd3(&self) -> SPD3_R {
        SPD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port 2 output max speed bits"]
    #[inline(always)]
    pub fn spd2(&self) -> SPD2_R {
        SPD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port 1 output max speed bits"]
    #[inline(always)]
    pub fn spd1(&self) -> SPD1_R {
        SPD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Port 0 output max speed bits"]
    #[inline(always)]
    pub fn spd0(&self) -> SPD0_R {
        SPD0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Port 15 output max speed bits"]
    #[inline(always)]
    pub fn spd15(&mut self) -> SPD15_W {
        SPD15_W { w: self }
    }
    #[doc = "Bit 14 - Port 14 output max speed bits"]
    #[inline(always)]
    pub fn spd14(&mut self) -> SPD14_W {
        SPD14_W { w: self }
    }
    #[doc = "Bit 13 - Port 13 output max speed bits"]
    #[inline(always)]
    pub fn spd13(&mut self) -> SPD13_W {
        SPD13_W { w: self }
    }
    #[doc = "Bit 12 - Port 12 output max speed bits"]
    #[inline(always)]
    pub fn spd12(&mut self) -> SPD12_W {
        SPD12_W { w: self }
    }
    #[doc = "Bit 11 - Port 11 output max speed bits"]
    #[inline(always)]
    pub fn spd11(&mut self) -> SPD11_W {
        SPD11_W { w: self }
    }
    #[doc = "Bit 10 - Port 10 output max speed bits"]
    #[inline(always)]
    pub fn spd10(&mut self) -> SPD10_W {
        SPD10_W { w: self }
    }
    #[doc = "Bit 9 - Port 9 output max speed bits"]
    #[inline(always)]
    pub fn spd9(&mut self) -> SPD9_W {
        SPD9_W { w: self }
    }
    #[doc = "Bit 8 - Port 8 output max speed bits"]
    #[inline(always)]
    pub fn spd8(&mut self) -> SPD8_W {
        SPD8_W { w: self }
    }
    #[doc = "Bit 7 - Port 7 output max speed bits"]
    #[inline(always)]
    pub fn spd7(&mut self) -> SPD7_W {
        SPD7_W { w: self }
    }
    #[doc = "Bit 6 - Port 6 output max speed bits"]
    #[inline(always)]
    pub fn spd6(&mut self) -> SPD6_W {
        SPD6_W { w: self }
    }
    #[doc = "Bit 5 - Port 5 output max speed bits"]
    #[inline(always)]
    pub fn spd5(&mut self) -> SPD5_W {
        SPD5_W { w: self }
    }
    #[doc = "Bit 4 - Port 4 output max speed bits"]
    #[inline(always)]
    pub fn spd4(&mut self) -> SPD4_W {
        SPD4_W { w: self }
    }
    #[doc = "Bit 3 - Port 3 output max speed bits"]
    #[inline(always)]
    pub fn spd3(&mut self) -> SPD3_W {
        SPD3_W { w: self }
    }
    #[doc = "Bit 2 - Port 2 output max speed bits"]
    #[inline(always)]
    pub fn spd2(&mut self) -> SPD2_W {
        SPD2_W { w: self }
    }
    #[doc = "Bit 1 - Port 1 output max speed bits"]
    #[inline(always)]
    pub fn spd1(&mut self) -> SPD1_W {
        SPD1_W { w: self }
    }
    #[doc = "Bit 0 - Port 0 output max speed bits"]
    #[inline(always)]
    pub fn spd0(&mut self) -> SPD0_W {
        SPD0_W { w: self }
    }
}
