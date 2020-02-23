#[doc = "Reader of register CRCPOLY"]
pub type R = crate::R<u32, super::CRCPOLY>;
#[doc = "Writer for register CRCPOLY"]
pub type W = crate::W<u32, super::CRCPOLY>;
#[doc = "Register CRCPOLY `reset()`'s with value 0x07"]
impl crate::ResetValue for super::CRCPOLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07
    }
}
#[doc = "Reader of field `CPR`"]
pub type CPR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `CPR`"]
pub struct CPR_W<'a> {
    w: &'a mut W,
}
impl<'a> CPR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn cpr(&self) -> CPR_R {
        CPR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC polynomial register"]
    #[inline(always)]
    pub fn cpr(&mut self) -> CPR_W {
        CPR_W { w: self }
    }
}
