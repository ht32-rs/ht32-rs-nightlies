#[doc = "Reader of register CKCU_MCUDBGCR"]
pub type R = crate::R<u32, super::CKCU_MCUDBGCR>;
#[doc = "Writer for register CKCU_MCUDBGCR"]
pub type W = crate::W<u32, super::CKCU_MCUDBGCR>;
#[doc = "Register CKCU_MCUDBGCR `reset()`'s with value 0"]
impl crate::ResetValue for super::CKCU_MCUDBGCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBSLP`"]
pub type DBSLP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSLP`"]
pub struct DBSLP_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSLP_W<'a> {
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
#[doc = "Reader of field `DBDSLP1`"]
pub type DBDSLP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBDSLP1`"]
pub struct DBDSLP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBDSLP1_W<'a> {
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
#[doc = "Reader of field `DBPD`"]
pub type DBPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBPD`"]
pub struct DBPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DBPD_W<'a> {
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
#[doc = "Reader of field `DBWDT`"]
pub type DBWDT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBWDT`"]
pub struct DBWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBWDT_W<'a> {
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
#[doc = "Reader of field `DBMCTM`"]
pub type DBMCTM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBMCTM`"]
pub struct DBMCTM_W<'a> {
    w: &'a mut W,
}
impl<'a> DBMCTM_W<'a> {
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
#[doc = "Reader of field `DBGPTM0`"]
pub type DBGPTM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGPTM0`"]
pub struct DBGPTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGPTM0_W<'a> {
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
#[doc = "Reader of field `DBGPTM1`"]
pub type DBGPTM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGPTM1`"]
pub struct DBGPTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGPTM1_W<'a> {
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
#[doc = "Reader of field `DBUR0`"]
pub type DBUR0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBUR0`"]
pub struct DBUR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUR0_W<'a> {
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
#[doc = "Reader of field `DBUR1`"]
pub type DBUR1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBUR1`"]
pub struct DBUR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBUR1_W<'a> {
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
#[doc = "Reader of field `DBSPI0`"]
pub type DBSPI0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSPI0`"]
pub struct DBSPI0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSPI0_W<'a> {
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
#[doc = "Reader of field `DBSPI1`"]
pub type DBSPI1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBSPI1`"]
pub struct DBSPI1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSPI1_W<'a> {
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
#[doc = "Reader of field `DBI2C0`"]
pub type DBI2C0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBI2C0`"]
pub struct DBI2C0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI2C0_W<'a> {
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
#[doc = "Reader of field `DBI2C1`"]
pub type DBI2C1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBI2C1`"]
pub struct DBI2C1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI2C1_W<'a> {
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
#[doc = "Reader of field `DBDSLP2`"]
pub type DBDSLP2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBDSLP2`"]
pub struct DBDSLP2_W<'a> {
    w: &'a mut W,
}
impl<'a> DBDSLP2_W<'a> {
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
#[doc = "Reader of field `DBDSCI`"]
pub type DBDSCI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBDSCI`"]
pub struct DBDSCI_W<'a> {
    w: &'a mut W,
}
impl<'a> DBDSCI_W<'a> {
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
#[doc = "Reader of field `DBBFTM0`"]
pub type DBBFTM0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBBFTM0`"]
pub struct DBBFTM0_W<'a> {
    w: &'a mut W,
}
impl<'a> DBBFTM0_W<'a> {
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
#[doc = "Reader of field `DBBFTM1`"]
pub type DBBFTM1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBBFTM1`"]
pub struct DBBFTM1_W<'a> {
    w: &'a mut W,
}
impl<'a> DBBFTM1_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&self) -> DBSLP_R {
        DBSLP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&self) -> DBDSLP1_R {
        DBDSLP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    pub fn dbpd(&self) -> DBPD_R {
        DBPD_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&self) -> DBWDT_R {
        DBWDT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DBMCTM"]
    #[inline(always)]
    pub fn dbmctm(&self) -> DBMCTM_R {
        DBMCTM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    pub fn dbgptm0(&self) -> DBGPTM0_R {
        DBGPTM0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    pub fn dbgptm1(&self) -> DBGPTM1_R {
        DBGPTM1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DBUR0"]
    #[inline(always)]
    pub fn dbur0(&self) -> DBUR0_R {
        DBUR0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DBUR1"]
    #[inline(always)]
    pub fn dbur1(&self) -> DBUR1_R {
        DBUR1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - DBSPI0"]
    #[inline(always)]
    pub fn dbspi0(&self) -> DBSPI0_R {
        DBSPI0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - DBSPI1"]
    #[inline(always)]
    pub fn dbspi1(&self) -> DBSPI1_R {
        DBSPI1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - DBI2C0"]
    #[inline(always)]
    pub fn dbi2c0(&self) -> DBI2C0_R {
        DBI2C0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - DBI2C1"]
    #[inline(always)]
    pub fn dbi2c1(&self) -> DBI2C1_R {
        DBI2C1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&self) -> DBDSLP2_R {
        DBDSLP2_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - DBDSCI"]
    #[inline(always)]
    pub fn dbdsci(&self) -> DBDSCI_R {
        DBDSCI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - DBBFTM0"]
    #[inline(always)]
    pub fn dbbftm0(&self) -> DBBFTM0_R {
        DBBFTM0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - DBBFTM1"]
    #[inline(always)]
    pub fn dbbftm1(&self) -> DBBFTM1_R {
        DBBFTM1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DBSLP"]
    #[inline(always)]
    pub fn dbslp(&mut self) -> DBSLP_W {
        DBSLP_W { w: self }
    }
    #[doc = "Bit 1 - DBDSLP1"]
    #[inline(always)]
    pub fn dbdslp1(&mut self) -> DBDSLP1_W {
        DBDSLP1_W { w: self }
    }
    #[doc = "Bit 2 - DBPD"]
    #[inline(always)]
    pub fn dbpd(&mut self) -> DBPD_W {
        DBPD_W { w: self }
    }
    #[doc = "Bit 3 - DBWDT"]
    #[inline(always)]
    pub fn dbwdt(&mut self) -> DBWDT_W {
        DBWDT_W { w: self }
    }
    #[doc = "Bit 4 - DBMCTM"]
    #[inline(always)]
    pub fn dbmctm(&mut self) -> DBMCTM_W {
        DBMCTM_W { w: self }
    }
    #[doc = "Bit 6 - DBGPTM0"]
    #[inline(always)]
    pub fn dbgptm0(&mut self) -> DBGPTM0_W {
        DBGPTM0_W { w: self }
    }
    #[doc = "Bit 7 - DBGPTM1"]
    #[inline(always)]
    pub fn dbgptm1(&mut self) -> DBGPTM1_W {
        DBGPTM1_W { w: self }
    }
    #[doc = "Bit 8 - DBUR0"]
    #[inline(always)]
    pub fn dbur0(&mut self) -> DBUR0_W {
        DBUR0_W { w: self }
    }
    #[doc = "Bit 9 - DBUR1"]
    #[inline(always)]
    pub fn dbur1(&mut self) -> DBUR1_W {
        DBUR1_W { w: self }
    }
    #[doc = "Bit 10 - DBSPI0"]
    #[inline(always)]
    pub fn dbspi0(&mut self) -> DBSPI0_W {
        DBSPI0_W { w: self }
    }
    #[doc = "Bit 11 - DBSPI1"]
    #[inline(always)]
    pub fn dbspi1(&mut self) -> DBSPI1_W {
        DBSPI1_W { w: self }
    }
    #[doc = "Bit 12 - DBI2C0"]
    #[inline(always)]
    pub fn dbi2c0(&mut self) -> DBI2C0_W {
        DBI2C0_W { w: self }
    }
    #[doc = "Bit 13 - DBI2C1"]
    #[inline(always)]
    pub fn dbi2c1(&mut self) -> DBI2C1_W {
        DBI2C1_W { w: self }
    }
    #[doc = "Bit 14 - DBDSLP2"]
    #[inline(always)]
    pub fn dbdslp2(&mut self) -> DBDSLP2_W {
        DBDSLP2_W { w: self }
    }
    #[doc = "Bit 15 - DBDSCI"]
    #[inline(always)]
    pub fn dbdsci(&mut self) -> DBDSCI_W {
        DBDSCI_W { w: self }
    }
    #[doc = "Bit 16 - DBBFTM0"]
    #[inline(always)]
    pub fn dbbftm0(&mut self) -> DBBFTM0_W {
        DBBFTM0_W { w: self }
    }
    #[doc = "Bit 17 - DBBFTM1"]
    #[inline(always)]
    pub fn dbbftm1(&mut self) -> DBBFTM1_W {
        DBBFTM1_W { w: self }
    }
}
