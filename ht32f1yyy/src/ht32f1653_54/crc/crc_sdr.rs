#[doc = "Reader of register CRC_SDR"]
pub type R = crate::R<u32, super::CRC_SDR>;
#[doc = "Writer for register CRC_SDR"]
pub type W = crate::W<u32, super::CRC_SDR>;
#[doc = "Register CRC_SDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CRC_SDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEED`"]
pub type SEED_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SEED`"]
pub struct SEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SEED"]
    #[inline(always)]
    pub fn seed(&self) -> SEED_R {
        SEED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SEED"]
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W {
        SEED_W { w: self }
    }
}
