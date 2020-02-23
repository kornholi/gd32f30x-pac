#[doc = "Reader of register RT"]
pub type R = crate::R<u32, super::RT>;
#[doc = "Writer for register RT"]
pub type W = crate::W<u32, super::RT>;
#[doc = "Register RT `reset()`'s with value 0"]
impl crate::ResetValue for super::RT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BL`"]
pub type BL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BL`"]
pub struct BL_W<'a> {
    w: &'a mut W,
}
impl<'a> BL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `RT`"]
pub type RT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RT`"]
pub struct RT_W<'a> {
    w: &'a mut W,
}
impl<'a> RT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 0:23 - Receiver timeout threshold"]
    #[inline(always)]
    pub fn rt(&self) -> RT_R {
        RT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn bl(&mut self) -> BL_W {
        BL_W { w: self }
    }
    #[doc = "Bits 0:23 - Receiver timeout threshold"]
    #[inline(always)]
    pub fn rt(&mut self) -> RT_W {
        RT_W { w: self }
    }
}
