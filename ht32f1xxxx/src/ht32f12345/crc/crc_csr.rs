#[doc = "Reader of register CRC_CSR"]
pub type R = crate::R<u32, super::CRC_CSR>;
#[doc = "Writer for register CRC_CSR"]
pub type W = crate::W<u32, super::CRC_CSR>;
#[doc = "Register CRC_CSR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_CSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CHKSUM`"]
pub type CHKSUM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `CHKSUM`"]
pub struct CHKSUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CHKSUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CHKSUM"]
    #[inline(always)]
    pub fn chksum(&self) -> CHKSUM_R {
        CHKSUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CHKSUM"]
    #[inline(always)]
    pub fn chksum(&mut self) -> CHKSUM_W {
        CHKSUM_W { w: self }
    }
}
