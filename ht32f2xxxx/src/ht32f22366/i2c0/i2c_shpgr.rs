#[doc = "Reader of register I2C_SHPGR"]
pub type R = crate::R<u32, super::I2C_SHPGR>;
#[doc = "Writer for register I2C_SHPGR"]
pub type W = crate::W<u32, super::I2C_SHPGR>;
#[doc = "Register I2C_SHPGR `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_SHPGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SHPG`"]
pub type SHPG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SHPG`"]
pub struct SHPG_W<'a> {
    w: &'a mut W,
}
impl<'a> SHPG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SHPG"]
    #[inline(always)]
    pub fn shpg(&self) -> SHPG_R {
        SHPG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SHPG"]
    #[inline(always)]
    pub fn shpg(&mut self) -> SHPG_W {
        SHPG_W { w: self }
    }
}
