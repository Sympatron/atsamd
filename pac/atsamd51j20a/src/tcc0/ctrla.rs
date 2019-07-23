#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRLA {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r" Reset value of the register"]
    #[inline]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r" Value of the field"]
pub struct SWRSTR {
    bits: bool,
}
impl SWRSTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ENABLER {
    bits: bool,
}
impl ENABLER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `RESOLUTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLUTIONR {
    #[doc = "Dithering is disabled"]
    NONE,
    #[doc = "Dithering is done every 16 PWM frames"]
    DITH4,
    #[doc = "Dithering is done every 32 PWM frames"]
    DITH5,
    #[doc = "Dithering is done every 64 PWM frames"]
    DITH6,
}
impl RESOLUTIONR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESOLUTIONR::NONE => 0,
            RESOLUTIONR::DITH4 => 0x01,
            RESOLUTIONR::DITH5 => 0x02,
            RESOLUTIONR::DITH6 => 0x03,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESOLUTIONR {
        match value {
            0 => RESOLUTIONR::NONE,
            1 => RESOLUTIONR::DITH4,
            2 => RESOLUTIONR::DITH5,
            3 => RESOLUTIONR::DITH6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == RESOLUTIONR::NONE
    }
    #[doc = "Checks if the value of the field is `DITH4`"]
    #[inline]
    pub fn is_dith4(&self) -> bool {
        *self == RESOLUTIONR::DITH4
    }
    #[doc = "Checks if the value of the field is `DITH5`"]
    #[inline]
    pub fn is_dith5(&self) -> bool {
        *self == RESOLUTIONR::DITH5
    }
    #[doc = "Checks if the value of the field is `DITH6`"]
    #[inline]
    pub fn is_dith6(&self) -> bool {
        *self == RESOLUTIONR::DITH6
    }
}
#[doc = "Possible values of the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERR {
    #[doc = "No division"]
    DIV1,
    #[doc = "Divide by 2"]
    DIV2,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 256"]
    DIV256,
    #[doc = "Divide by 1024"]
    DIV1024,
}
impl PRESCALERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCALERR::DIV1 => 0,
            PRESCALERR::DIV2 => 0x01,
            PRESCALERR::DIV4 => 0x02,
            PRESCALERR::DIV8 => 0x03,
            PRESCALERR::DIV16 => 0x04,
            PRESCALERR::DIV64 => 0x05,
            PRESCALERR::DIV256 => 0x06,
            PRESCALERR::DIV1024 => 0x07,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCALERR {
        match value {
            0 => PRESCALERR::DIV1,
            1 => PRESCALERR::DIV2,
            2 => PRESCALERR::DIV4,
            3 => PRESCALERR::DIV8,
            4 => PRESCALERR::DIV16,
            5 => PRESCALERR::DIV64,
            6 => PRESCALERR::DIV256,
            7 => PRESCALERR::DIV1024,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline]
    pub fn is_div1(&self) -> bool {
        *self == PRESCALERR::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline]
    pub fn is_div2(&self) -> bool {
        *self == PRESCALERR::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline]
    pub fn is_div4(&self) -> bool {
        *self == PRESCALERR::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline]
    pub fn is_div8(&self) -> bool {
        *self == PRESCALERR::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline]
    pub fn is_div16(&self) -> bool {
        *self == PRESCALERR::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline]
    pub fn is_div64(&self) -> bool {
        *self == PRESCALERR::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline]
    pub fn is_div256(&self) -> bool {
        *self == PRESCALERR::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCALERR::DIV1024
    }
}
#[doc = r" Value of the field"]
pub struct RUNSTDBYR {
    bits: bool,
}
impl RUNSTDBYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `PRESCSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCSYNCR {
    #[doc = "Reload or reset counter on next GCLK"]
    GCLK,
    #[doc = "Reload or reset counter on next prescaler clock"]
    PRESC,
    #[doc = "Reload or reset counter on next GCLK and reset prescaler counter"]
    RESYNC,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl PRESCSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            PRESCSYNCR::GCLK => 0,
            PRESCSYNCR::PRESC => 0x01,
            PRESCSYNCR::RESYNC => 0x02,
            PRESCSYNCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> PRESCSYNCR {
        match value {
            0 => PRESCSYNCR::GCLK,
            1 => PRESCSYNCR::PRESC,
            2 => PRESCSYNCR::RESYNC,
            i => PRESCSYNCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `GCLK`"]
    #[inline]
    pub fn is_gclk(&self) -> bool {
        *self == PRESCSYNCR::GCLK
    }
    #[doc = "Checks if the value of the field is `PRESC`"]
    #[inline]
    pub fn is_presc(&self) -> bool {
        *self == PRESCSYNCR::PRESC
    }
    #[doc = "Checks if the value of the field is `RESYNC`"]
    #[inline]
    pub fn is_resync(&self) -> bool {
        *self == PRESCSYNCR::RESYNC
    }
}
#[doc = r" Value of the field"]
pub struct ALOCKR {
    bits: bool,
}
impl ALOCKR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct MSYNCR {
    bits: bool,
}
impl MSYNCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DMAOSR {
    bits: bool,
}
impl DMAOSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CPTEN0R {
    bits: bool,
}
impl CPTEN0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CPTEN1R {
    bits: bool,
}
impl CPTEN1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CPTEN2R {
    bits: bool,
}
impl CPTEN2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CPTEN3R {
    bits: bool,
}
impl CPTEN3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CPTEN4R {
    bits: bool,
}
impl CPTEN4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct CPTEN5R {
    bits: bool,
}
impl CPTEN5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _SWRSTW<'a> {
    w: &'a mut W,
}
impl<'a> _SWRSTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 0);
        self.w.bits |= ((value as u32) & 0x01) << 0;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _ENABLEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 1);
        self.w.bits |= ((value as u32) & 0x01) << 1;
        self.w
    }
}
#[doc = "Values that can be written to the field `RESOLUTION`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESOLUTIONW {
    #[doc = "Dithering is disabled"]
    NONE,
    #[doc = "Dithering is done every 16 PWM frames"]
    DITH4,
    #[doc = "Dithering is done every 32 PWM frames"]
    DITH5,
    #[doc = "Dithering is done every 64 PWM frames"]
    DITH6,
}
impl RESOLUTIONW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESOLUTIONW::NONE => 0,
            RESOLUTIONW::DITH4 => 1,
            RESOLUTIONW::DITH5 => 2,
            RESOLUTIONW::DITH6 => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESOLUTIONW<'a> {
    w: &'a mut W,
}
impl<'a> _RESOLUTIONW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESOLUTIONW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Dithering is disabled"]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(RESOLUTIONW::NONE)
    }
    #[doc = "Dithering is done every 16 PWM frames"]
    #[inline]
    pub fn dith4(self) -> &'a mut W {
        self.variant(RESOLUTIONW::DITH4)
    }
    #[doc = "Dithering is done every 32 PWM frames"]
    #[inline]
    pub fn dith5(self) -> &'a mut W {
        self.variant(RESOLUTIONW::DITH5)
    }
    #[doc = "Dithering is done every 64 PWM frames"]
    #[inline]
    pub fn dith6(self) -> &'a mut W {
        self.variant(RESOLUTIONW::DITH6)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 5);
        self.w.bits |= ((value as u32) & 0x03) << 5;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCALER`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCALERW {
    #[doc = "No division"]
    DIV1,
    #[doc = "Divide by 2"]
    DIV2,
    #[doc = "Divide by 4"]
    DIV4,
    #[doc = "Divide by 8"]
    DIV8,
    #[doc = "Divide by 16"]
    DIV16,
    #[doc = "Divide by 64"]
    DIV64,
    #[doc = "Divide by 256"]
    DIV256,
    #[doc = "Divide by 1024"]
    DIV1024,
}
impl PRESCALERW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCALERW::DIV1 => 0,
            PRESCALERW::DIV2 => 1,
            PRESCALERW::DIV4 => 2,
            PRESCALERW::DIV8 => 3,
            PRESCALERW::DIV16 => 4,
            PRESCALERW::DIV64 => 5,
            PRESCALERW::DIV256 => 6,
            PRESCALERW::DIV1024 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCALERW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCALERW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCALERW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No division"]
    #[inline]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV16)
    }
    #[doc = "Divide by 64"]
    #[inline]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV64)
    }
    #[doc = "Divide by 256"]
    #[inline]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV256)
    }
    #[doc = "Divide by 1024"]
    #[inline]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCALERW::DIV1024)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x07 << 8);
        self.w.bits |= ((value as u32) & 0x07) << 8;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RUNSTDBYW<'a> {
    w: &'a mut W,
}
impl<'a> _RUNSTDBYW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 11);
        self.w.bits |= ((value as u32) & 0x01) << 11;
        self.w
    }
}
#[doc = "Values that can be written to the field `PRESCSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCSYNCW {
    #[doc = "Reload or reset counter on next GCLK"]
    GCLK,
    #[doc = "Reload or reset counter on next prescaler clock"]
    PRESC,
    #[doc = "Reload or reset counter on next GCLK and reset prescaler counter"]
    RESYNC,
}
impl PRESCSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            PRESCSYNCW::GCLK => 0,
            PRESCSYNCW::PRESC => 1,
            PRESCSYNCW::RESYNC => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _PRESCSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _PRESCSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: PRESCSYNCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Reload or reset counter on next GCLK"]
    #[inline]
    pub fn gclk(self) -> &'a mut W {
        self.variant(PRESCSYNCW::GCLK)
    }
    #[doc = "Reload or reset counter on next prescaler clock"]
    #[inline]
    pub fn presc(self) -> &'a mut W {
        self.variant(PRESCSYNCW::PRESC)
    }
    #[doc = "Reload or reset counter on next GCLK and reset prescaler counter"]
    #[inline]
    pub fn resync(self) -> &'a mut W {
        self.variant(PRESCSYNCW::RESYNC)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(0x03 << 12);
        self.w.bits |= ((value as u32) & 0x03) << 12;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ALOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _ALOCKW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 14);
        self.w.bits |= ((value as u32) & 0x01) << 14;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _MSYNCW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 15);
        self.w.bits |= ((value as u32) & 0x01) << 15;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DMAOSW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAOSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 23);
        self.w.bits |= ((value as u32) & 0x01) << 23;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPTEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _CPTEN0W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 24);
        self.w.bits |= ((value as u32) & 0x01) << 24;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPTEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _CPTEN1W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 25);
        self.w.bits |= ((value as u32) & 0x01) << 25;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPTEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _CPTEN2W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 26);
        self.w.bits |= ((value as u32) & 0x01) << 26;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPTEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _CPTEN3W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 27);
        self.w.bits |= ((value as u32) & 0x01) << 27;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPTEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _CPTEN4W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 28);
        self.w.bits |= ((value as u32) & 0x01) << 28;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPTEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _CPTEN5W<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(0x01 << 29);
        self.w.bits |= ((value as u32) & 0x01) << 29;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&self) -> SWRSTR {
        let bits = ((self.bits >> 0) & 0x01) != 0;
        SWRSTR { bits }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&self) -> ENABLER {
        let bits = ((self.bits >> 1) & 0x01) != 0;
        ENABLER { bits }
    }
    #[doc = "Bits 5:6 - Enhanced Resolution"]
    #[inline]
    pub fn resolution(&self) -> RESOLUTIONR {
        RESOLUTIONR::_from(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline]
    pub fn prescaler(&self) -> PRESCALERR {
        PRESCALERR::_from(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline]
    pub fn runstdby(&self) -> RUNSTDBYR {
        let bits = ((self.bits >> 11) & 0x01) != 0;
        RUNSTDBYR { bits }
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
    #[inline]
    pub fn prescsync(&self) -> PRESCSYNCR {
        PRESCSYNCR::_from(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Auto Lock"]
    #[inline]
    pub fn alock(&self) -> ALOCKR {
        let bits = ((self.bits >> 14) & 0x01) != 0;
        ALOCKR { bits }
    }
    #[doc = "Bit 15 - Master Synchronization (only for TCC Slave Instance)"]
    #[inline]
    pub fn msync(&self) -> MSYNCR {
        let bits = ((self.bits >> 15) & 0x01) != 0;
        MSYNCR { bits }
    }
    #[doc = "Bit 23 - DMA One-shot Trigger Mode"]
    #[inline]
    pub fn dmaos(&self) -> DMAOSR {
        let bits = ((self.bits >> 23) & 0x01) != 0;
        DMAOSR { bits }
    }
    #[doc = "Bit 24 - Capture Channel 0 Enable"]
    #[inline]
    pub fn cpten0(&self) -> CPTEN0R {
        let bits = ((self.bits >> 24) & 0x01) != 0;
        CPTEN0R { bits }
    }
    #[doc = "Bit 25 - Capture Channel 1 Enable"]
    #[inline]
    pub fn cpten1(&self) -> CPTEN1R {
        let bits = ((self.bits >> 25) & 0x01) != 0;
        CPTEN1R { bits }
    }
    #[doc = "Bit 26 - Capture Channel 2 Enable"]
    #[inline]
    pub fn cpten2(&self) -> CPTEN2R {
        let bits = ((self.bits >> 26) & 0x01) != 0;
        CPTEN2R { bits }
    }
    #[doc = "Bit 27 - Capture Channel 3 Enable"]
    #[inline]
    pub fn cpten3(&self) -> CPTEN3R {
        let bits = ((self.bits >> 27) & 0x01) != 0;
        CPTEN3R { bits }
    }
    #[doc = "Bit 28 - Capture Channel 4 Enable"]
    #[inline]
    pub fn cpten4(&self) -> CPTEN4R {
        let bits = ((self.bits >> 28) & 0x01) != 0;
        CPTEN4R { bits }
    }
    #[doc = "Bit 29 - Capture Channel 5 Enable"]
    #[inline]
    pub fn cpten5(&self) -> CPTEN5R {
        let bits = ((self.bits >> 29) & 0x01) != 0;
        CPTEN5R { bits }
    }
}
impl W {
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Software Reset"]
    #[inline]
    pub fn swrst(&mut self) -> _SWRSTW {
        _SWRSTW { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline]
    pub fn enable(&mut self) -> _ENABLEW {
        _ENABLEW { w: self }
    }
    #[doc = "Bits 5:6 - Enhanced Resolution"]
    #[inline]
    pub fn resolution(&mut self) -> _RESOLUTIONW {
        _RESOLUTIONW { w: self }
    }
    #[doc = "Bits 8:10 - Prescaler"]
    #[inline]
    pub fn prescaler(&mut self) -> _PRESCALERW {
        _PRESCALERW { w: self }
    }
    #[doc = "Bit 11 - Run in Standby"]
    #[inline]
    pub fn runstdby(&mut self) -> _RUNSTDBYW {
        _RUNSTDBYW { w: self }
    }
    #[doc = "Bits 12:13 - Prescaler and Counter Synchronization Selection"]
    #[inline]
    pub fn prescsync(&mut self) -> _PRESCSYNCW {
        _PRESCSYNCW { w: self }
    }
    #[doc = "Bit 14 - Auto Lock"]
    #[inline]
    pub fn alock(&mut self) -> _ALOCKW {
        _ALOCKW { w: self }
    }
    #[doc = "Bit 15 - Master Synchronization (only for TCC Slave Instance)"]
    #[inline]
    pub fn msync(&mut self) -> _MSYNCW {
        _MSYNCW { w: self }
    }
    #[doc = "Bit 23 - DMA One-shot Trigger Mode"]
    #[inline]
    pub fn dmaos(&mut self) -> _DMAOSW {
        _DMAOSW { w: self }
    }
    #[doc = "Bit 24 - Capture Channel 0 Enable"]
    #[inline]
    pub fn cpten0(&mut self) -> _CPTEN0W {
        _CPTEN0W { w: self }
    }
    #[doc = "Bit 25 - Capture Channel 1 Enable"]
    #[inline]
    pub fn cpten1(&mut self) -> _CPTEN1W {
        _CPTEN1W { w: self }
    }
    #[doc = "Bit 26 - Capture Channel 2 Enable"]
    #[inline]
    pub fn cpten2(&mut self) -> _CPTEN2W {
        _CPTEN2W { w: self }
    }
    #[doc = "Bit 27 - Capture Channel 3 Enable"]
    #[inline]
    pub fn cpten3(&mut self) -> _CPTEN3W {
        _CPTEN3W { w: self }
    }
    #[doc = "Bit 28 - Capture Channel 4 Enable"]
    #[inline]
    pub fn cpten4(&mut self) -> _CPTEN4W {
        _CPTEN4W { w: self }
    }
    #[doc = "Bit 29 - Capture Channel 5 Enable"]
    #[inline]
    pub fn cpten5(&mut self) -> _CPTEN5W {
        _CPTEN5W { w: self }
    }
}