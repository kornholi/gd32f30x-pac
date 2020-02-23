#[doc = "Reader of register DATATO"]
pub type R = crate::R<u32, super::DATATO>;
#[doc = "Writer for register DATATO"]
pub type W = crate::W<u32, super::DATATO>;
#[doc = "Register DATATO `reset()`'s with value 0"]
impl crate::ResetValue for super::DATATO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DATATO`"]
pub type DATATO_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DATATO`"]
pub struct DATATO_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn datato(&self) -> DATATO_R {
        DATATO_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn datato(&mut self) -> DATATO_W {
        DATATO_W { w: self }
    }
}
