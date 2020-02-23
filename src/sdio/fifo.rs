#[doc = "Reader of register FIFO"]
pub type R = crate::R<u32, super::FIFO>;
#[doc = "Writer for register FIFO"]
pub type W = crate::W<u32, super::FIFO>;
#[doc = "Register FIFO `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIFODT`"]
pub type FIFODT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `FIFODT`"]
pub struct FIFODT_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFODT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive FIFO data or transmit FIFO data"]
    #[inline(always)]
    pub fn fifodt(&self) -> FIFODT_R {
        FIFODT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive FIFO data or transmit FIFO data"]
    #[inline(always)]
    pub fn fifodt(&mut self) -> FIFODT_W {
        FIFODT_W { w: self }
    }
}
