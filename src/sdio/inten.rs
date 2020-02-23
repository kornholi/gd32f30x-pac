#[doc = "Reader of register INTEN"]
pub type R = crate::R<u32, super::INTEN>;
#[doc = "Writer for register INTEN"]
pub type W = crate::W<u32, super::INTEN>;
#[doc = "Register INTEN `reset()`'s with value 0"]
impl crate::ResetValue for super::INTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CCRCERRIE`"]
pub type CCRCERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CCRCERRIE`"]
pub struct CCRCERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCRCERRIE_W<'a> {
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
#[doc = "Reader of field `DTCRCERRIE`"]
pub type DTCRCERRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTCRCERRIE`"]
pub struct DTCRCERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCRCERRIE_W<'a> {
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
#[doc = "Reader of field `CMDTMOUTIE`"]
pub type CMDTMOUTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDTMOUTIE`"]
pub struct CMDTMOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDTMOUTIE_W<'a> {
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
#[doc = "Reader of field `DTTMOUTIE`"]
pub type DTTMOUTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTTMOUTIE`"]
pub struct DTTMOUTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTTMOUTIE_W<'a> {
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
#[doc = "Reader of field `TXUREIE`"]
pub type TXUREIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUREIE`"]
pub struct TXUREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUREIE_W<'a> {
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
#[doc = "Reader of field `RXOREIE`"]
pub type RXOREIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOREIE`"]
pub struct RXOREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOREIE_W<'a> {
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
#[doc = "Reader of field `CMDRECVIE`"]
pub type CMDRECVIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDRECVIE`"]
pub struct CMDRECVIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRECVIE_W<'a> {
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
#[doc = "Reader of field `CMDSENDIE`"]
pub type CMDSENDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDSENDIE`"]
pub struct CMDSENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSENDIE_W<'a> {
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
#[doc = "Reader of field `DTENDIE`"]
pub type DTENDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTENDIE`"]
pub struct DTENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTENDIE_W<'a> {
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
#[doc = "Reader of field `STBITEIE`"]
pub type STBITEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STBITEIE`"]
pub struct STBITEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> STBITEIE_W<'a> {
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
#[doc = "Reader of field `DTBLKENDIE`"]
pub type DTBLKENDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTBLKENDIE`"]
pub struct DTBLKENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBLKENDIE_W<'a> {
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
#[doc = "Reader of field `CMDRUNIE`"]
pub type CMDRUNIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDRUNIE`"]
pub struct CMDRUNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDRUNIE_W<'a> {
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
#[doc = "Reader of field `TXRUNIE`"]
pub type TXRUNIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXRUNIE`"]
pub struct TXRUNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRUNIE_W<'a> {
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
#[doc = "Reader of field `RXRUNIE`"]
pub type RXRUNIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXRUNIE`"]
pub struct RXRUNIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRUNIE_W<'a> {
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
#[doc = "Reader of field `TFHIE`"]
pub type TFHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFHIE`"]
pub struct TFHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFHIE_W<'a> {
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
#[doc = "Reader of field `RFHIE`"]
pub type RFHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFHIE`"]
pub struct RFHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFHIE_W<'a> {
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
#[doc = "Reader of field `TFFIE`"]
pub type TFFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFFIE`"]
pub struct TFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFFIE_W<'a> {
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
#[doc = "Reader of field `RFFIE`"]
pub type RFFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFFIE`"]
pub struct RFFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFFIE_W<'a> {
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
#[doc = "Reader of field `TFEIE`"]
pub type TFEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFEIE`"]
pub struct TFEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TFEIE_W<'a> {
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
#[doc = "Reader of field `RFEIE`"]
pub type RFEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RFEIE`"]
pub struct RFEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RFEIE_W<'a> {
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
#[doc = "Reader of field `TXDTVALIE`"]
pub type TXDTVALIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDTVALIE`"]
pub struct TXDTVALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDTVALIE_W<'a> {
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
#[doc = "Reader of field `RXDTVALIE`"]
pub type RXDTVALIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDTVALIE`"]
pub struct RXDTVALIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDTVALIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SDIOINTIE`"]
pub type SDIOINTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SDIOINTIE`"]
pub struct SDIOINTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIOINTIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `ATAENDIE`"]
pub type ATAENDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ATAENDIE`"]
pub struct ATAENDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATAENDIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Command response CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcerrie(&self) -> CCRCERRIE_R {
        CCRCERRIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dtcrcerrie(&self) -> DTCRCERRIE_R {
        DTCRCERRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command response timeout interrupt enable"]
    #[inline(always)]
    pub fn cmdtmoutie(&self) -> CMDTMOUTIE_R {
        CMDTMOUTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dttmoutie(&self) -> DTTMOUTIE_R {
        DTTMOUTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txureie(&self) -> TXUREIE_R {
        TXUREIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Received FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoreie(&self) -> RXOREIE_R {
        RXOREIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrecvie(&self) -> CMDRECVIE_R {
        CMDRECVIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsendie(&self) -> CMDSENDIE_R {
        CMDSENDIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dtendie(&self) -> DTENDIE_R {
        DTENDIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiteie(&self) -> STBITEIE_R {
        STBITEIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dtblkendie(&self) -> DTBLKENDIE_R {
        DTBLKENDIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command transmission interrupt enable"]
    #[inline(always)]
    pub fn cmdrunie(&self) -> CMDRUNIE_R {
        CMDRUNIE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data transmission interrupt enable"]
    #[inline(always)]
    pub fn txrunie(&self) -> TXRUNIE_R {
        TXRUNIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Data reception interrupt enable"]
    #[inline(always)]
    pub fn rxrunie(&self) -> RXRUNIE_R {
        RXRUNIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn tfhie(&self) -> TFHIE_R {
        TFHIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rfhie(&self) -> RFHIE_R {
        RFHIE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Transmit FIFO full interrupt enable"]
    #[inline(always)]
    pub fn tffie(&self) -> TFFIE_R {
        TFFIE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&self) -> RFFIE_R {
        RFFIE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Transmit FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn tfeie(&self) -> TFEIE_R {
        TFEIE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rfeie(&self) -> RFEIE_R {
        RFEIE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data valid in transmit FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdtvalie(&self) -> TXDTVALIE_R {
        TXDTVALIE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Data valid in receive FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdtvalie(&self) -> RXDTVALIE_R {
        RXDTVALIE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdiointie(&self) -> SDIOINTIE_R {
        SDIOINTIE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ataendie(&self) -> ATAENDIE_R {
        ATAENDIE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command response CRC fail interrupt enable"]
    #[inline(always)]
    pub fn ccrcerrie(&mut self) -> CCRCERRIE_W {
        CCRCERRIE_W { w: self }
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable"]
    #[inline(always)]
    pub fn dtcrcerrie(&mut self) -> DTCRCERRIE_W {
        DTCRCERRIE_W { w: self }
    }
    #[doc = "Bit 2 - Command response timeout interrupt enable"]
    #[inline(always)]
    pub fn cmdtmoutie(&mut self) -> CMDTMOUTIE_W {
        CMDTMOUTIE_W { w: self }
    }
    #[doc = "Bit 3 - Data timeout interrupt enable"]
    #[inline(always)]
    pub fn dttmoutie(&mut self) -> DTTMOUTIE_W {
        DTTMOUTIE_W { w: self }
    }
    #[doc = "Bit 4 - Transmit FIFO underrun error interrupt enable"]
    #[inline(always)]
    pub fn txureie(&mut self) -> TXUREIE_W {
        TXUREIE_W { w: self }
    }
    #[doc = "Bit 5 - Received FIFO overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxoreie(&mut self) -> RXOREIE_W {
        RXOREIE_W { w: self }
    }
    #[doc = "Bit 6 - Command response received interrupt enable"]
    #[inline(always)]
    pub fn cmdrecvie(&mut self) -> CMDRECVIE_W {
        CMDRECVIE_W { w: self }
    }
    #[doc = "Bit 7 - Command sent interrupt enable"]
    #[inline(always)]
    pub fn cmdsendie(&mut self) -> CMDSENDIE_W {
        CMDSENDIE_W { w: self }
    }
    #[doc = "Bit 8 - Data end interrupt enable"]
    #[inline(always)]
    pub fn dtendie(&mut self) -> DTENDIE_W {
        DTENDIE_W { w: self }
    }
    #[doc = "Bit 9 - Start bit error interrupt enable"]
    #[inline(always)]
    pub fn stbiteie(&mut self) -> STBITEIE_W {
        STBITEIE_W { w: self }
    }
    #[doc = "Bit 10 - Data block end interrupt enable"]
    #[inline(always)]
    pub fn dtblkendie(&mut self) -> DTBLKENDIE_W {
        DTBLKENDIE_W { w: self }
    }
    #[doc = "Bit 11 - Command transmission interrupt enable"]
    #[inline(always)]
    pub fn cmdrunie(&mut self) -> CMDRUNIE_W {
        CMDRUNIE_W { w: self }
    }
    #[doc = "Bit 12 - Data transmission interrupt enable"]
    #[inline(always)]
    pub fn txrunie(&mut self) -> TXRUNIE_W {
        TXRUNIE_W { w: self }
    }
    #[doc = "Bit 13 - Data reception interrupt enable"]
    #[inline(always)]
    pub fn rxrunie(&mut self) -> RXRUNIE_W {
        RXRUNIE_W { w: self }
    }
    #[doc = "Bit 14 - Transmit FIFO half empty interrupt enable"]
    #[inline(always)]
    pub fn tfhie(&mut self) -> TFHIE_W {
        TFHIE_W { w: self }
    }
    #[doc = "Bit 15 - Receive FIFO half full interrupt enable"]
    #[inline(always)]
    pub fn rfhie(&mut self) -> RFHIE_W {
        RFHIE_W { w: self }
    }
    #[doc = "Bit 16 - Transmit FIFO full interrupt enable"]
    #[inline(always)]
    pub fn tffie(&mut self) -> TFFIE_W {
        TFFIE_W { w: self }
    }
    #[doc = "Bit 17 - Receive FIFO full interrupt enable"]
    #[inline(always)]
    pub fn rffie(&mut self) -> RFFIE_W {
        RFFIE_W { w: self }
    }
    #[doc = "Bit 18 - Transmit FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn tfeie(&mut self) -> TFEIE_W {
        TFEIE_W { w: self }
    }
    #[doc = "Bit 19 - Receive FIFO empty interrupt enable"]
    #[inline(always)]
    pub fn rfeie(&mut self) -> RFEIE_W {
        RFEIE_W { w: self }
    }
    #[doc = "Bit 20 - Data valid in transmit FIFO interrupt enable"]
    #[inline(always)]
    pub fn txdtvalie(&mut self) -> TXDTVALIE_W {
        TXDTVALIE_W { w: self }
    }
    #[doc = "Bit 21 - Data valid in receive FIFO interrupt enable"]
    #[inline(always)]
    pub fn rxdtvalie(&mut self) -> RXDTVALIE_W {
        RXDTVALIE_W { w: self }
    }
    #[doc = "Bit 22 - SD I/O interrupt received interrupt enable"]
    #[inline(always)]
    pub fn sdiointie(&mut self) -> SDIOINTIE_W {
        SDIOINTIE_W { w: self }
    }
    #[doc = "Bit 23 - CE-ATA command completion signal received interrupt enable"]
    #[inline(always)]
    pub fn ataendie(&mut self) -> ATAENDIE_W {
        ATAENDIE_W { w: self }
    }
}
