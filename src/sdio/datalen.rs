#[doc = "Reader of register DATALEN"]
pub type R = crate::R<u32, super::DATALEN>;
#[doc = "Writer for register DATALEN"]
pub type W = crate::W<u32, super::DATALEN>;
#[doc = "Register DATALEN `reset()`'s with value 0"]
impl crate::ResetValue for super::DATALEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATALEN`"]
pub type DATALEN_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATALEN`"]
pub struct DATALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff_ffff) | ((value as u32) & 0x01ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:24 - Data transfer length"]
    #[inline(always)]
    pub fn datalen(&self) -> DATALEN_R {
        DATALEN_R::new((self.bits & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data transfer length"]
    #[inline(always)]
    pub fn datalen(&mut self) -> DATALEN_W {
        DATALEN_W { w: self }
    }
}
