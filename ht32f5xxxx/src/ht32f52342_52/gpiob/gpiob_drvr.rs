#[doc = "Reader of register GPIOB_DRVR"]
pub type R = crate::R<u32, super::GPIOB_DRVR>;
#[doc = "Writer for register GPIOB_DRVR"]
pub type W = crate::W<u32, super::GPIOB_DRVR>;
#[doc = "Register GPIOB_DRVR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIOB_DRVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DV0`"]
pub type DV0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV0`"]
pub struct DV0_W<'a> {
    w: &'a mut W,
}
impl<'a> DV0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `DV1`"]
pub type DV1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV1`"]
pub struct DV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DV1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DV2`"]
pub type DV2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV2`"]
pub struct DV2_W<'a> {
    w: &'a mut W,
}
impl<'a> DV2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `DV3`"]
pub type DV3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV3`"]
pub struct DV3_W<'a> {
    w: &'a mut W,
}
impl<'a> DV3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `DV4`"]
pub type DV4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV4`"]
pub struct DV4_W<'a> {
    w: &'a mut W,
}
impl<'a> DV4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `DV5`"]
pub type DV5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV5`"]
pub struct DV5_W<'a> {
    w: &'a mut W,
}
impl<'a> DV5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Reader of field `DV6`"]
pub type DV6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV6`"]
pub struct DV6_W<'a> {
    w: &'a mut W,
}
impl<'a> DV6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `DV7`"]
pub type DV7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV7`"]
pub struct DV7_W<'a> {
    w: &'a mut W,
}
impl<'a> DV7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `DV8`"]
pub type DV8_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV8`"]
pub struct DV8_W<'a> {
    w: &'a mut W,
}
impl<'a> DV8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `DV9`"]
pub type DV9_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV9`"]
pub struct DV9_W<'a> {
    w: &'a mut W,
}
impl<'a> DV9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Reader of field `DV10`"]
pub type DV10_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV10`"]
pub struct DV10_W<'a> {
    w: &'a mut W,
}
impl<'a> DV10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Reader of field `DV11`"]
pub type DV11_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV11`"]
pub struct DV11_W<'a> {
    w: &'a mut W,
}
impl<'a> DV11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `DV12`"]
pub type DV12_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV12`"]
pub struct DV12_W<'a> {
    w: &'a mut W,
}
impl<'a> DV12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `DV13`"]
pub type DV13_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV13`"]
pub struct DV13_W<'a> {
    w: &'a mut W,
}
impl<'a> DV13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `DV14`"]
pub type DV14_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV14`"]
pub struct DV14_W<'a> {
    w: &'a mut W,
}
impl<'a> DV14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `DV15`"]
pub type DV15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DV15`"]
pub struct DV15_W<'a> {
    w: &'a mut W,
}
impl<'a> DV15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - DV0"]
    #[inline(always)]
    pub fn dv0(&self) -> DV0_R {
        DV0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - DV1"]
    #[inline(always)]
    pub fn dv1(&self) -> DV1_R {
        DV1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - DV2"]
    #[inline(always)]
    pub fn dv2(&self) -> DV2_R {
        DV2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - DV3"]
    #[inline(always)]
    pub fn dv3(&self) -> DV3_R {
        DV3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - DV4"]
    #[inline(always)]
    pub fn dv4(&self) -> DV4_R {
        DV4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - DV5"]
    #[inline(always)]
    pub fn dv5(&self) -> DV5_R {
        DV5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - DV6"]
    #[inline(always)]
    pub fn dv6(&self) -> DV6_R {
        DV6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - DV7"]
    #[inline(always)]
    pub fn dv7(&self) -> DV7_R {
        DV7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - DV8"]
    #[inline(always)]
    pub fn dv8(&self) -> DV8_R {
        DV8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - DV9"]
    #[inline(always)]
    pub fn dv9(&self) -> DV9_R {
        DV9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - DV10"]
    #[inline(always)]
    pub fn dv10(&self) -> DV10_R {
        DV10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - DV11"]
    #[inline(always)]
    pub fn dv11(&self) -> DV11_R {
        DV11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DV11"]
    #[inline(always)]
    pub fn dv12(&self) -> DV12_R {
        DV12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - DV11"]
    #[inline(always)]
    pub fn dv13(&self) -> DV13_R {
        DV13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - DV14"]
    #[inline(always)]
    pub fn dv14(&self) -> DV14_R {
        DV14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - DV15"]
    #[inline(always)]
    pub fn dv15(&self) -> DV15_R {
        DV15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DV0"]
    #[inline(always)]
    pub fn dv0(&mut self) -> DV0_W {
        DV0_W { w: self }
    }
    #[doc = "Bits 2:3 - DV1"]
    #[inline(always)]
    pub fn dv1(&mut self) -> DV1_W {
        DV1_W { w: self }
    }
    #[doc = "Bits 4:5 - DV2"]
    #[inline(always)]
    pub fn dv2(&mut self) -> DV2_W {
        DV2_W { w: self }
    }
    #[doc = "Bits 6:7 - DV3"]
    #[inline(always)]
    pub fn dv3(&mut self) -> DV3_W {
        DV3_W { w: self }
    }
    #[doc = "Bits 8:9 - DV4"]
    #[inline(always)]
    pub fn dv4(&mut self) -> DV4_W {
        DV4_W { w: self }
    }
    #[doc = "Bits 10:11 - DV5"]
    #[inline(always)]
    pub fn dv5(&mut self) -> DV5_W {
        DV5_W { w: self }
    }
    #[doc = "Bits 12:13 - DV6"]
    #[inline(always)]
    pub fn dv6(&mut self) -> DV6_W {
        DV6_W { w: self }
    }
    #[doc = "Bits 14:15 - DV7"]
    #[inline(always)]
    pub fn dv7(&mut self) -> DV7_W {
        DV7_W { w: self }
    }
    #[doc = "Bits 16:17 - DV8"]
    #[inline(always)]
    pub fn dv8(&mut self) -> DV8_W {
        DV8_W { w: self }
    }
    #[doc = "Bits 18:19 - DV9"]
    #[inline(always)]
    pub fn dv9(&mut self) -> DV9_W {
        DV9_W { w: self }
    }
    #[doc = "Bits 20:21 - DV10"]
    #[inline(always)]
    pub fn dv10(&mut self) -> DV10_W {
        DV10_W { w: self }
    }
    #[doc = "Bits 22:23 - DV11"]
    #[inline(always)]
    pub fn dv11(&mut self) -> DV11_W {
        DV11_W { w: self }
    }
    #[doc = "Bits 24:25 - DV11"]
    #[inline(always)]
    pub fn dv12(&mut self) -> DV12_W {
        DV12_W { w: self }
    }
    #[doc = "Bits 26:27 - DV11"]
    #[inline(always)]
    pub fn dv13(&mut self) -> DV13_W {
        DV13_W { w: self }
    }
    #[doc = "Bits 28:29 - DV14"]
    #[inline(always)]
    pub fn dv14(&mut self) -> DV14_W {
        DV14_W { w: self }
    }
    #[doc = "Bits 30:31 - DV15"]
    #[inline(always)]
    pub fn dv15(&mut self) -> DV15_W {
        DV15_W { w: self }
    }
}
