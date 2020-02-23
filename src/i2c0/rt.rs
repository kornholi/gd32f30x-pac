#[doc = "Reader of register RT"]
pub type R = crate::R<u32, super::RT>;
#[doc = "Writer for register RT"]
pub type W = crate::W<u32, super::RT>;
#[doc = "Register RT `reset()`'s with value 0x02"]
impl crate::ResetValue for super::RT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `RISETIME`"]
pub type RISETIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RISETIME`"]
pub struct RISETIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RISETIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Maximum rise time in master mode"]
    #[inline(always)]
    pub fn risetime(&self) -> RISETIME_R {
        RISETIME_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Maximum rise time in master mode"]
    #[inline(always)]
    pub fn risetime(&mut self) -> RISETIME_W {
        RISETIME_W { w: self }
    }
}
