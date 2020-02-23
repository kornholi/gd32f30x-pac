#[doc = "Reader of register BADDR"]
pub type R = crate::R<u32, super::BADDR>;
#[doc = "Writer for register BADDR"]
pub type W = crate::W<u32, super::BADDR>;
#[doc = "Register BADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::BADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BAR`"]
pub type BAR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BAR`"]
pub struct BAR_W<'a> {
    w: &'a mut W,
}
impl<'a> BAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1fff << 3)) | (((value as u32) & 0x1fff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:15 - Buffer address"]
    #[inline(always)]
    pub fn bar(&self) -> BAR_R {
        BAR_R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 3:15 - Buffer address"]
    #[inline(always)]
    pub fn bar(&mut self) -> BAR_W {
        BAR_W { w: self }
    }
}
