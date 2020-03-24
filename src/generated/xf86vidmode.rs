// This file contains generated code. Do not edit directly.
// To regenerate this, run 'make'.

#![allow(clippy::unreadable_literal)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::identity_op)]
#![allow(clippy::trivially_copy_pass_by_ref)]
#![allow(clippy::eq_op)]
use std::convert::TryFrom;
#[allow(unused_imports)]
use std::convert::TryInto;
use std::io::IoSlice;
#[allow(unused_imports)]
use crate::utils::RawFdContainer;
#[allow(unused_imports)]
use crate::x11_utils::Event as _;
use crate::x11_utils::{TryParse, Serialize};
use crate::connection::RequestConnection;
#[allow(unused_imports)]
use crate::cookie::{Cookie, CookieWithFds, VoidCookie};
use crate::errors::{ParseError, ConnectionError};
#[allow(unused_imports)]
use crate::x11_utils::GenericEvent;
#[allow(unused_imports)]
use crate::x11_utils::GenericError;

/// The X11 name of the extension for QueryExtension
pub const X11_EXTENSION_NAME: &str = "XFree86-VidModeExtension";

/// The version number of this extension that this client library supports.
///
/// This constant contains the version number of this extension that is supported
/// by this build of x11rb. For most things, it does not make sense to use this
/// information. If you need to send a `QueryVersion`, it is recommended to instead
/// send the maximum version of the extension that you need.
pub const X11_XML_VERSION: (u32, u32) = (2, 2);

pub type SYNCRANGE = u32;

pub type DOTCLOCK = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum ModeFlag {
    PositiveHSync = 1 << 0,
    NegativeHSync = 1 << 1,
    PositiveVSync = 1 << 2,
    NegativeVSync = 1 << 3,
    Interlace = 1 << 4,
    CompositeSync = 1 << 5,
    PositiveCSync = 1 << 6,
    NegativeCSync = 1 << 7,
    HSkew = 1 << 8,
    Broadcast = 1 << 9,
    Pixmux = 1 << 10,
    DoubleClock = 1 << 11,
    HalfClock = 1 << 12,
}
impl From<ModeFlag> for u16 {
    fn from(input: ModeFlag) -> Self {
        match input {
            ModeFlag::PositiveHSync => 1 << 0,
            ModeFlag::NegativeHSync => 1 << 1,
            ModeFlag::PositiveVSync => 1 << 2,
            ModeFlag::NegativeVSync => 1 << 3,
            ModeFlag::Interlace => 1 << 4,
            ModeFlag::CompositeSync => 1 << 5,
            ModeFlag::PositiveCSync => 1 << 6,
            ModeFlag::NegativeCSync => 1 << 7,
            ModeFlag::HSkew => 1 << 8,
            ModeFlag::Broadcast => 1 << 9,
            ModeFlag::Pixmux => 1 << 10,
            ModeFlag::DoubleClock => 1 << 11,
            ModeFlag::HalfClock => 1 << 12,
        }
    }
}
impl From<ModeFlag> for Option<u16> {
    fn from(input: ModeFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<ModeFlag> for u32 {
    fn from(input: ModeFlag) -> Self {
        Self::from(u16::from(input))
    }
}
impl From<ModeFlag> for Option<u32> {
    fn from(input: ModeFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u16> for ModeFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ModeFlag::PositiveHSync),
            2 => Ok(ModeFlag::NegativeHSync),
            4 => Ok(ModeFlag::PositiveVSync),
            8 => Ok(ModeFlag::NegativeVSync),
            16 => Ok(ModeFlag::Interlace),
            32 => Ok(ModeFlag::CompositeSync),
            64 => Ok(ModeFlag::PositiveCSync),
            128 => Ok(ModeFlag::NegativeCSync),
            256 => Ok(ModeFlag::HSkew),
            512 => Ok(ModeFlag::Broadcast),
            1024 => Ok(ModeFlag::Pixmux),
            2048 => Ok(ModeFlag::DoubleClock),
            4096 => Ok(ModeFlag::HalfClock),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u32> for ModeFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u16::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ModeFlag, u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClockFlag {
    Programable = 1 << 0,
}
impl From<ClockFlag> for u8 {
    fn from(input: ClockFlag) -> Self {
        match input {
            ClockFlag::Programable => 1 << 0,
        }
    }
}
impl From<ClockFlag> for Option<u8> {
    fn from(input: ClockFlag) -> Self {
        Some(u8::from(input))
    }
}
impl From<ClockFlag> for u16 {
    fn from(input: ClockFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ClockFlag> for Option<u16> {
    fn from(input: ClockFlag) -> Self {
        Some(u16::from(input))
    }
}
impl From<ClockFlag> for u32 {
    fn from(input: ClockFlag) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<ClockFlag> for Option<u32> {
    fn from(input: ClockFlag) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for ClockFlag {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ClockFlag::Programable),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for ClockFlag {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for ClockFlag {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(ClockFlag, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Permission {
    Read = 1 << 0,
    Write = 1 << 1,
}
impl From<Permission> for u8 {
    fn from(input: Permission) -> Self {
        match input {
            Permission::Read => 1 << 0,
            Permission::Write => 1 << 1,
        }
    }
}
impl From<Permission> for Option<u8> {
    fn from(input: Permission) -> Self {
        Some(u8::from(input))
    }
}
impl From<Permission> for u16 {
    fn from(input: Permission) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Permission> for Option<u16> {
    fn from(input: Permission) -> Self {
        Some(u16::from(input))
    }
}
impl From<Permission> for u32 {
    fn from(input: Permission) -> Self {
        Self::from(u8::from(input))
    }
}
impl From<Permission> for Option<u32> {
    fn from(input: Permission) -> Self {
        Some(u32::from(input))
    }
}
impl TryFrom<u8> for Permission {
    type Error = ParseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Permission::Read),
            2 => Ok(Permission::Write),
            _ => Err(ParseError::ParseError)
        }
    }
}
impl TryFrom<u16> for Permission {
    type Error = ParseError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
impl TryFrom<u32> for Permission {
    type Error = ParseError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::try_from(u8::try_from(value).or(Err(ParseError::ParseError))?)
    }
}
bitmask_binop!(Permission, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeInfo {
    pub dotclock: DOTCLOCK,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u32,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub privsize: u32,
}
impl TryParse for ModeInfo {
    fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (dotclock, remaining) = DOTCLOCK::try_parse(remaining)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u32::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(4..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let result = ModeInfo { dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, privsize };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ModeInfo {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl Serialize for ModeInfo {
    type Bytes = [u8; 48];
    fn serialize(&self) -> Self::Bytes {
        let dotclock_bytes = self.dotclock.serialize();
        let hdisplay_bytes = self.hdisplay.serialize();
        let hsyncstart_bytes = self.hsyncstart.serialize();
        let hsyncend_bytes = self.hsyncend.serialize();
        let htotal_bytes = self.htotal.serialize();
        let hskew_bytes = self.hskew.serialize();
        let vdisplay_bytes = self.vdisplay.serialize();
        let vsyncstart_bytes = self.vsyncstart.serialize();
        let vsyncend_bytes = self.vsyncend.serialize();
        let vtotal_bytes = self.vtotal.serialize();
        let flags_bytes = self.flags.serialize();
        let privsize_bytes = self.privsize.serialize();
        [
            dotclock_bytes[0],
            dotclock_bytes[1],
            dotclock_bytes[2],
            dotclock_bytes[3],
            hdisplay_bytes[0],
            hdisplay_bytes[1],
            hsyncstart_bytes[0],
            hsyncstart_bytes[1],
            hsyncend_bytes[0],
            hsyncend_bytes[1],
            htotal_bytes[0],
            htotal_bytes[1],
            hskew_bytes[0],
            hskew_bytes[1],
            hskew_bytes[2],
            hskew_bytes[3],
            vdisplay_bytes[0],
            vdisplay_bytes[1],
            vsyncstart_bytes[0],
            vsyncstart_bytes[1],
            vsyncend_bytes[0],
            vsyncend_bytes[1],
            vtotal_bytes[0],
            vtotal_bytes[1],
            0,
            0,
            0,
            0,
            flags_bytes[0],
            flags_bytes[1],
            flags_bytes[2],
            flags_bytes[3],
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            privsize_bytes[0],
            privsize_bytes[1],
            privsize_bytes[2],
            privsize_bytes[3],
        ]
    }
    fn serialize_into(&self, bytes: &mut Vec<u8>) {
        bytes.reserve(48);
        self.dotclock.serialize_into(bytes);
        self.hdisplay.serialize_into(bytes);
        self.hsyncstart.serialize_into(bytes);
        self.hsyncend.serialize_into(bytes);
        self.htotal.serialize_into(bytes);
        self.hskew.serialize_into(bytes);
        self.vdisplay.serialize_into(bytes);
        self.vsyncstart.serialize_into(bytes);
        self.vsyncend.serialize_into(bytes);
        self.vtotal.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 4]);
        self.flags.serialize_into(bytes);
        bytes.extend_from_slice(&[0; 12]);
        self.privsize.serialize_into(bytes);
    }
}

/// Opcode for the QueryVersion request
pub const QUERY_VERSION_REQUEST: u8 = 0;
pub fn query_version<Conn>(conn: &Conn) -> Result<Cookie<'_, Conn, QueryVersionReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (4) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let request0 = [
        extension_information.major_opcode,
        QUERY_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryVersionReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: u16,
    pub minor_version: u16,
}
impl QueryVersionReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (major_version, remaining) = u16::try_parse(remaining)?;
        let (minor_version, remaining) = u16::try_parse(remaining)?;
        let result = QueryVersionReply { response_type, sequence, length, major_version, minor_version };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for QueryVersionReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetModeLine request
pub const GET_MODE_LINE_REQUEST: u8 = 1;
pub fn get_mode_line<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetModeLineReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MODE_LINE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetModeLineReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub dotclock: DOTCLOCK,
    pub hdisplay: u16,
    pub hsyncstart: u16,
    pub hsyncend: u16,
    pub htotal: u16,
    pub hskew: u16,
    pub vdisplay: u16,
    pub vsyncstart: u16,
    pub vsyncend: u16,
    pub vtotal: u16,
    pub flags: u32,
    pub private: Vec<u8>,
}
impl GetModeLineReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (dotclock, remaining) = DOTCLOCK::try_parse(remaining)?;
        let (hdisplay, remaining) = u16::try_parse(remaining)?;
        let (hsyncstart, remaining) = u16::try_parse(remaining)?;
        let (hsyncend, remaining) = u16::try_parse(remaining)?;
        let (htotal, remaining) = u16::try_parse(remaining)?;
        let (hskew, remaining) = u16::try_parse(remaining)?;
        let (vdisplay, remaining) = u16::try_parse(remaining)?;
        let (vsyncstart, remaining) = u16::try_parse(remaining)?;
        let (vsyncend, remaining) = u16::try_parse(remaining)?;
        let (vtotal, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(2..).ok_or(ParseError::ParseError)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (privsize, remaining) = u32::try_parse(remaining)?;
        let (private, remaining) = crate::x11_utils::parse_list::<u8>(remaining, privsize as usize)?;
        let result = GetModeLineReply { response_type, sequence, length, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetModeLineReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the ModModeLine request
pub const MOD_MODE_LINE_REQUEST: u8 = 2;
pub fn mod_mode_line<'c, Conn>(conn: &'c Conn, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, private: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (48 + 1 * private.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let hdisplay_bytes = hdisplay.serialize();
    let hsyncstart_bytes = hsyncstart.serialize();
    let hsyncend_bytes = hsyncend.serialize();
    let htotal_bytes = htotal.serialize();
    let hskew_bytes = hskew.serialize();
    let vdisplay_bytes = vdisplay.serialize();
    let vsyncstart_bytes = vsyncstart.serialize();
    let vsyncend_bytes = vsyncend.serialize();
    let vtotal_bytes = vtotal.serialize();
    let flags_bytes = flags.serialize();
    let privsize: u32 = private.len().try_into()?;
    let privsize_bytes = privsize.serialize();
    let request0 = [
        extension_information.major_opcode,
        MOD_MODE_LINE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        hdisplay_bytes[0],
        hdisplay_bytes[1],
        hsyncstart_bytes[0],
        hsyncstart_bytes[1],
        hsyncend_bytes[0],
        hsyncend_bytes[1],
        htotal_bytes[0],
        htotal_bytes[1],
        hskew_bytes[0],
        hskew_bytes[1],
        vdisplay_bytes[0],
        vdisplay_bytes[1],
        vsyncstart_bytes[0],
        vsyncstart_bytes[1],
        vsyncend_bytes[0],
        vsyncend_bytes[1],
        vtotal_bytes[0],
        vtotal_bytes[1],
        0,
        0,
        flags_bytes[0],
        flags_bytes[1],
        flags_bytes[2],
        flags_bytes[3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        privsize_bytes[0],
        privsize_bytes[1],
        privsize_bytes[2],
        privsize_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (private).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(private), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the SwitchMode request
pub const SWITCH_MODE_REQUEST: u8 = 3;
pub fn switch_mode<Conn>(conn: &Conn, screen: u16, zoom: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let zoom_bytes = zoom.serialize();
    let request0 = [
        extension_information.major_opcode,
        SWITCH_MODE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        zoom_bytes[0],
        zoom_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetMonitor request
pub const GET_MONITOR_REQUEST: u8 = 4;
pub fn get_monitor<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetMonitorReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_MONITOR_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetMonitorReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub hsync: Vec<SYNCRANGE>,
    pub vsync: Vec<SYNCRANGE>,
    pub vendor: Vec<u8>,
    pub alignment_pad: Vec<u8>,
    pub model: Vec<u8>,
}
impl GetMonitorReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (vendor_length, remaining) = u8::try_parse(remaining)?;
        let (model_length, remaining) = u8::try_parse(remaining)?;
        let (num_hsync, remaining) = u8::try_parse(remaining)?;
        let (num_vsync, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (hsync, remaining) = crate::x11_utils::parse_list::<SYNCRANGE>(remaining, num_hsync as usize)?;
        let (vsync, remaining) = crate::x11_utils::parse_list::<SYNCRANGE>(remaining, num_vsync as usize)?;
        let (vendor, remaining) = crate::x11_utils::parse_list::<u8>(remaining, vendor_length as usize)?;
        let (alignment_pad, remaining) = crate::x11_utils::parse_list::<u8>(remaining, (((vendor_length as usize) + (3)) & (!(3))) - (vendor_length as usize))?;
        let (model, remaining) = crate::x11_utils::parse_list::<u8>(remaining, model_length as usize)?;
        let result = GetMonitorReply { response_type, sequence, length, hsync, vsync, vendor, alignment_pad, model };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetMonitorReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the LockModeSwitch request
pub const LOCK_MODE_SWITCH_REQUEST: u8 = 5;
pub fn lock_mode_switch<Conn>(conn: &Conn, screen: u16, lock: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let lock_bytes = lock.serialize();
    let request0 = [
        extension_information.major_opcode,
        LOCK_MODE_SWITCH_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        lock_bytes[0],
        lock_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetAllModeLines request
pub const GET_ALL_MODE_LINES_REQUEST: u8 = 6;
pub fn get_all_mode_lines<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetAllModeLinesReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_ALL_MODE_LINES_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAllModeLinesReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub modeinfo: Vec<ModeInfo>,
}
impl GetAllModeLinesReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (modecount, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let (modeinfo, remaining) = crate::x11_utils::parse_list::<ModeInfo>(remaining, modecount as usize)?;
        let result = GetAllModeLinesReply { response_type, sequence, length, modeinfo };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetAllModeLinesReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the AddModeLine request
pub const ADD_MODE_LINE_REQUEST: u8 = 7;
pub fn add_mode_line<'c, Conn>(conn: &'c Conn, screen: u32, dotclock: DOTCLOCK, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, after_dotclock: DOTCLOCK, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: u32, private: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (92 + 1 * private.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let dotclock_bytes = dotclock.serialize();
    let hdisplay_bytes = hdisplay.serialize();
    let hsyncstart_bytes = hsyncstart.serialize();
    let hsyncend_bytes = hsyncend.serialize();
    let htotal_bytes = htotal.serialize();
    let hskew_bytes = hskew.serialize();
    let vdisplay_bytes = vdisplay.serialize();
    let vsyncstart_bytes = vsyncstart.serialize();
    let vsyncend_bytes = vsyncend.serialize();
    let vtotal_bytes = vtotal.serialize();
    let flags_bytes = flags.serialize();
    let privsize: u32 = private.len().try_into()?;
    let privsize_bytes = privsize.serialize();
    let after_dotclock_bytes = after_dotclock.serialize();
    let after_hdisplay_bytes = after_hdisplay.serialize();
    let after_hsyncstart_bytes = after_hsyncstart.serialize();
    let after_hsyncend_bytes = after_hsyncend.serialize();
    let after_htotal_bytes = after_htotal.serialize();
    let after_hskew_bytes = after_hskew.serialize();
    let after_vdisplay_bytes = after_vdisplay.serialize();
    let after_vsyncstart_bytes = after_vsyncstart.serialize();
    let after_vsyncend_bytes = after_vsyncend.serialize();
    let after_vtotal_bytes = after_vtotal.serialize();
    let after_flags_bytes = after_flags.serialize();
    let request0 = [
        extension_information.major_opcode,
        ADD_MODE_LINE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        dotclock_bytes[0],
        dotclock_bytes[1],
        dotclock_bytes[2],
        dotclock_bytes[3],
        hdisplay_bytes[0],
        hdisplay_bytes[1],
        hsyncstart_bytes[0],
        hsyncstart_bytes[1],
        hsyncend_bytes[0],
        hsyncend_bytes[1],
        htotal_bytes[0],
        htotal_bytes[1],
        hskew_bytes[0],
        hskew_bytes[1],
        vdisplay_bytes[0],
        vdisplay_bytes[1],
        vsyncstart_bytes[0],
        vsyncstart_bytes[1],
        vsyncend_bytes[0],
        vsyncend_bytes[1],
        vtotal_bytes[0],
        vtotal_bytes[1],
        0,
        0,
        flags_bytes[0],
        flags_bytes[1],
        flags_bytes[2],
        flags_bytes[3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        privsize_bytes[0],
        privsize_bytes[1],
        privsize_bytes[2],
        privsize_bytes[3],
        after_dotclock_bytes[0],
        after_dotclock_bytes[1],
        after_dotclock_bytes[2],
        after_dotclock_bytes[3],
        after_hdisplay_bytes[0],
        after_hdisplay_bytes[1],
        after_hsyncstart_bytes[0],
        after_hsyncstart_bytes[1],
        after_hsyncend_bytes[0],
        after_hsyncend_bytes[1],
        after_htotal_bytes[0],
        after_htotal_bytes[1],
        after_hskew_bytes[0],
        after_hskew_bytes[1],
        after_vdisplay_bytes[0],
        after_vdisplay_bytes[1],
        after_vsyncstart_bytes[0],
        after_vsyncstart_bytes[1],
        after_vsyncend_bytes[0],
        after_vsyncend_bytes[1],
        after_vtotal_bytes[0],
        after_vtotal_bytes[1],
        0,
        0,
        after_flags_bytes[0],
        after_flags_bytes[1],
        after_flags_bytes[2],
        after_flags_bytes[3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (private).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(private), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the DeleteModeLine request
pub const DELETE_MODE_LINE_REQUEST: u8 = 8;
pub fn delete_mode_line<'c, Conn>(conn: &'c Conn, screen: u32, dotclock: DOTCLOCK, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, private: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (52 + 1 * private.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let dotclock_bytes = dotclock.serialize();
    let hdisplay_bytes = hdisplay.serialize();
    let hsyncstart_bytes = hsyncstart.serialize();
    let hsyncend_bytes = hsyncend.serialize();
    let htotal_bytes = htotal.serialize();
    let hskew_bytes = hskew.serialize();
    let vdisplay_bytes = vdisplay.serialize();
    let vsyncstart_bytes = vsyncstart.serialize();
    let vsyncend_bytes = vsyncend.serialize();
    let vtotal_bytes = vtotal.serialize();
    let flags_bytes = flags.serialize();
    let privsize: u32 = private.len().try_into()?;
    let privsize_bytes = privsize.serialize();
    let request0 = [
        extension_information.major_opcode,
        DELETE_MODE_LINE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        dotclock_bytes[0],
        dotclock_bytes[1],
        dotclock_bytes[2],
        dotclock_bytes[3],
        hdisplay_bytes[0],
        hdisplay_bytes[1],
        hsyncstart_bytes[0],
        hsyncstart_bytes[1],
        hsyncend_bytes[0],
        hsyncend_bytes[1],
        htotal_bytes[0],
        htotal_bytes[1],
        hskew_bytes[0],
        hskew_bytes[1],
        vdisplay_bytes[0],
        vdisplay_bytes[1],
        vsyncstart_bytes[0],
        vsyncstart_bytes[1],
        vsyncend_bytes[0],
        vsyncend_bytes[1],
        vtotal_bytes[0],
        vtotal_bytes[1],
        0,
        0,
        flags_bytes[0],
        flags_bytes[1],
        flags_bytes[2],
        flags_bytes[3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        privsize_bytes[0],
        privsize_bytes[1],
        privsize_bytes[2],
        privsize_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (private).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(private), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the ValidateModeLine request
pub const VALIDATE_MODE_LINE_REQUEST: u8 = 9;
pub fn validate_mode_line<'c, Conn>(conn: &'c Conn, screen: u32, dotclock: DOTCLOCK, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, private: &[u8]) -> Result<Cookie<'c, Conn, ValidateModeLineReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (52 + 1 * private.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let dotclock_bytes = dotclock.serialize();
    let hdisplay_bytes = hdisplay.serialize();
    let hsyncstart_bytes = hsyncstart.serialize();
    let hsyncend_bytes = hsyncend.serialize();
    let htotal_bytes = htotal.serialize();
    let hskew_bytes = hskew.serialize();
    let vdisplay_bytes = vdisplay.serialize();
    let vsyncstart_bytes = vsyncstart.serialize();
    let vsyncend_bytes = vsyncend.serialize();
    let vtotal_bytes = vtotal.serialize();
    let flags_bytes = flags.serialize();
    let privsize: u32 = private.len().try_into()?;
    let privsize_bytes = privsize.serialize();
    let request0 = [
        extension_information.major_opcode,
        VALIDATE_MODE_LINE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        dotclock_bytes[0],
        dotclock_bytes[1],
        dotclock_bytes[2],
        dotclock_bytes[3],
        hdisplay_bytes[0],
        hdisplay_bytes[1],
        hsyncstart_bytes[0],
        hsyncstart_bytes[1],
        hsyncend_bytes[0],
        hsyncend_bytes[1],
        htotal_bytes[0],
        htotal_bytes[1],
        hskew_bytes[0],
        hskew_bytes[1],
        vdisplay_bytes[0],
        vdisplay_bytes[1],
        vsyncstart_bytes[0],
        vsyncstart_bytes[1],
        vsyncend_bytes[0],
        vsyncend_bytes[1],
        vtotal_bytes[0],
        vtotal_bytes[1],
        0,
        0,
        flags_bytes[0],
        flags_bytes[1],
        flags_bytes[2],
        flags_bytes[3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        privsize_bytes[0],
        privsize_bytes[1],
        privsize_bytes[2],
        privsize_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (private).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0), IoSlice::new(private), IoSlice::new(&padding1)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidateModeLineReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: u32,
}
impl ValidateModeLineReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (status, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = ValidateModeLineReply { response_type, sequence, length, status };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ValidateModeLineReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SwitchToMode request
pub const SWITCH_TO_MODE_REQUEST: u8 = 10;
pub fn switch_to_mode<'c, Conn>(conn: &'c Conn, screen: u32, dotclock: DOTCLOCK, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, private: &[u8]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (52 + 1 * private.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let dotclock_bytes = dotclock.serialize();
    let hdisplay_bytes = hdisplay.serialize();
    let hsyncstart_bytes = hsyncstart.serialize();
    let hsyncend_bytes = hsyncend.serialize();
    let htotal_bytes = htotal.serialize();
    let hskew_bytes = hskew.serialize();
    let vdisplay_bytes = vdisplay.serialize();
    let vsyncstart_bytes = vsyncstart.serialize();
    let vsyncend_bytes = vsyncend.serialize();
    let vtotal_bytes = vtotal.serialize();
    let flags_bytes = flags.serialize();
    let privsize: u32 = private.len().try_into()?;
    let privsize_bytes = privsize.serialize();
    let request0 = [
        extension_information.major_opcode,
        SWITCH_TO_MODE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        screen_bytes[2],
        screen_bytes[3],
        dotclock_bytes[0],
        dotclock_bytes[1],
        dotclock_bytes[2],
        dotclock_bytes[3],
        hdisplay_bytes[0],
        hdisplay_bytes[1],
        hsyncstart_bytes[0],
        hsyncstart_bytes[1],
        hsyncend_bytes[0],
        hsyncend_bytes[1],
        htotal_bytes[0],
        htotal_bytes[1],
        hskew_bytes[0],
        hskew_bytes[1],
        vdisplay_bytes[0],
        vdisplay_bytes[1],
        vsyncstart_bytes[0],
        vsyncstart_bytes[1],
        vsyncend_bytes[0],
        vsyncend_bytes[1],
        vtotal_bytes[0],
        vtotal_bytes[1],
        0,
        0,
        flags_bytes[0],
        flags_bytes[1],
        flags_bytes[2],
        flags_bytes[3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        privsize_bytes[0],
        privsize_bytes[1],
        privsize_bytes[2],
        privsize_bytes[3],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (private).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(private), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetViewPort request
pub const GET_VIEW_PORT_REQUEST: u8 = 11;
pub fn get_view_port<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetViewPortReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_VIEW_PORT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetViewPortReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub x: u32,
    pub y: u32,
}
impl GetViewPortReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (x, remaining) = u32::try_parse(remaining)?;
        let (y, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(16..).ok_or(ParseError::ParseError)?;
        let result = GetViewPortReply { response_type, sequence, length, x, y };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetViewPortReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetViewPort request
pub const SET_VIEW_PORT_REQUEST: u8 = 12;
pub fn set_view_port<Conn>(conn: &Conn, screen: u16, x: u32, y: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (16) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let x_bytes = x.serialize();
    let y_bytes = y.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_VIEW_PORT_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
        x_bytes[0],
        x_bytes[1],
        x_bytes[2],
        x_bytes[3],
        y_bytes[0],
        y_bytes[1],
        y_bytes[2],
        y_bytes[3],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetDotClocks request
pub const GET_DOT_CLOCKS_REQUEST: u8 = 13;
pub fn get_dot_clocks<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetDotClocksReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_DOT_CLOCKS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetDotClocksReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub flags: u32,
    pub clocks: u32,
    pub maxclocks: u32,
    pub clock: Vec<u32>,
}
impl GetDotClocksReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (flags, remaining) = u32::try_parse(remaining)?;
        let (clocks, remaining) = u32::try_parse(remaining)?;
        let (maxclocks, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let (clock, remaining) = crate::x11_utils::parse_list::<u32>(remaining, ((1) - ((flags as usize) & (1))) * (clocks as usize))?;
        let result = GetDotClocksReply { response_type, sequence, length, flags, clocks, maxclocks, clock };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetDotClocksReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetClientVersion request
pub const SET_CLIENT_VERSION_REQUEST: u8 = 14;
pub fn set_client_version<Conn>(conn: &Conn, major: u16, minor: u16) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let major_bytes = major.serialize();
    let minor_bytes = minor.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_CLIENT_VERSION_REQUEST,
        length_bytes[0],
        length_bytes[1],
        major_bytes[0],
        major_bytes[1],
        minor_bytes[0],
        minor_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the SetGamma request
pub const SET_GAMMA_REQUEST: u8 = 15;
pub fn set_gamma<Conn>(conn: &Conn, screen: u16, red: u32, green: u32, blue: u32) -> Result<VoidCookie<'_, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let red_bytes = red.serialize();
    let green_bytes = green.serialize();
    let blue_bytes = blue.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_GAMMA_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
        red_bytes[0],
        red_bytes[1],
        red_bytes[2],
        red_bytes[3],
        green_bytes[0],
        green_bytes[1],
        green_bytes[2],
        green_bytes[3],
        blue_bytes[0],
        blue_bytes[1],
        blue_bytes[2],
        blue_bytes[3],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0)], Vec::new())?)
}

/// Opcode for the GetGamma request
pub const GET_GAMMA_REQUEST: u8 = 16;
pub fn get_gamma<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetGammaReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (32) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_GAMMA_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetGammaReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
impl GetGammaReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (red, remaining) = u32::try_parse(remaining)?;
        let (green, remaining) = u32::try_parse(remaining)?;
        let (blue, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(12..).ok_or(ParseError::ParseError)?;
        let result = GetGammaReply { response_type, sequence, length, red, green, blue };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetGammaReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetGammaRamp request
pub const GET_GAMMA_RAMP_REQUEST: u8 = 17;
pub fn get_gamma_ramp<Conn>(conn: &Conn, screen: u16, size: u16) -> Result<Cookie<'_, Conn, GetGammaRampReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let size_bytes = size.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_GAMMA_RAMP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        size_bytes[0],
        size_bytes[1],
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetGammaRampReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
    pub red: Vec<u16>,
    pub green: Vec<u16>,
    pub blue: Vec<u16>,
}
impl GetGammaRampReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let (red, remaining) = crate::x11_utils::parse_list::<u16>(remaining, ((size as usize) + (1)) & (!(1)))?;
        let (green, remaining) = crate::x11_utils::parse_list::<u16>(remaining, ((size as usize) + (1)) & (!(1)))?;
        let (blue, remaining) = crate::x11_utils::parse_list::<u16>(remaining, ((size as usize) + (1)) & (!(1)))?;
        let result = GetGammaRampReply { response_type, sequence, length, size, red, green, blue };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetGammaRampReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the SetGammaRamp request
pub const SET_GAMMA_RAMP_REQUEST: u8 = 18;
pub fn set_gamma_ramp<'c, Conn>(conn: &'c Conn, screen: u16, size: u16, red: &[u16], green: &[u16], blue: &[u16]) -> Result<VoidCookie<'c, Conn>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8 + 2 * red.len() + 2 * green.len() + 2 * blue.len() + 3) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let size_bytes = size.serialize();
    assert_eq!(red.len(), ((size as usize) + (1)) & (!(1)), "Argument red has an incorrect length");
    let red_bytes = red.serialize();
    let request0 = [
        extension_information.major_opcode,
        SET_GAMMA_RAMP_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        size_bytes[0],
        size_bytes[1],
    ];
    let length_so_far = (&request0).len();
    let length_so_far = length_so_far + (&red_bytes).len();
    assert_eq!(green.len(), ((size as usize) + (1)) & (!(1)), "Argument green has an incorrect length");
    let green_bytes = green.serialize();
    let length_so_far = length_so_far + (&green_bytes).len();
    assert_eq!(blue.len(), ((size as usize) + (1)) & (!(1)), "Argument blue has an incorrect length");
    let blue_bytes = blue.serialize();
    let length_so_far = length_so_far + (&blue_bytes).len();
    let padding1 = &[0; 3][..(4 - (length_so_far % 4)) % 4];
    let length_so_far = length_so_far + (&padding1).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_without_reply(&[IoSlice::new(&request0), IoSlice::new(&red_bytes), IoSlice::new(&green_bytes), IoSlice::new(&blue_bytes), IoSlice::new(&padding1)], Vec::new())?)
}

/// Opcode for the GetGammaRampSize request
pub const GET_GAMMA_RAMP_SIZE_REQUEST: u8 = 19;
pub fn get_gamma_ramp_size<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetGammaRampSizeReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_GAMMA_RAMP_SIZE_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetGammaRampSizeReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub size: u16,
}
impl GetGammaRampSizeReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (size, remaining) = u16::try_parse(remaining)?;
        let remaining = remaining.get(22..).ok_or(ParseError::ParseError)?;
        let result = GetGammaRampSizeReply { response_type, sequence, length, size };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetGammaRampSizeReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the GetPermissions request
pub const GET_PERMISSIONS_REQUEST: u8 = 20;
pub fn get_permissions<Conn>(conn: &Conn, screen: u16) -> Result<Cookie<'_, Conn, GetPermissionsReply>, ConnectionError>
where Conn: RequestConnection + ?Sized
{
    let extension_information = conn.extension_information(X11_EXTENSION_NAME)?
        .ok_or(ConnectionError::UnsupportedExtension)?;
    let length: usize = (8) / 4;
    let length_bytes = TryInto::<u16>::try_into(length).unwrap_or(0).serialize();
    let screen_bytes = screen.serialize();
    let request0 = [
        extension_information.major_opcode,
        GET_PERMISSIONS_REQUEST,
        length_bytes[0],
        length_bytes[1],
        screen_bytes[0],
        screen_bytes[1],
        0,
        0,
    ];
    let length_so_far = (&request0).len();
    assert_eq!(length_so_far, length * 4);
    Ok(conn.send_request_with_reply(&[IoSlice::new(&request0)], Vec::new())?)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GetPermissionsReply {
    pub response_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub permissions: u32,
}
impl GetPermissionsReply {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let remaining = remaining.get(1..).ok_or(ParseError::ParseError)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let (length, remaining) = u32::try_parse(remaining)?;
        let (permissions, remaining) = u32::try_parse(remaining)?;
        let remaining = remaining.get(20..).ok_or(ParseError::ParseError)?;
        let result = GetPermissionsReply { response_type, sequence, length, permissions };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for GetPermissionsReply {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}

/// Opcode for the BadClock error
pub const BAD_CLOCK_ERROR: u8 = 0;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadClockError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadClockError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadClockError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadClockError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadClockError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadClockError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadClockError> for [u8; 32] {
    fn from(input: &BadClockError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadClockError> for [u8; 32] {
    fn from(input: BadClockError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadHTimings error
pub const BAD_H_TIMINGS_ERROR: u8 = 1;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadHTimingsError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadHTimingsError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadHTimingsError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadHTimingsError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadHTimingsError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadHTimingsError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadHTimingsError> for [u8; 32] {
    fn from(input: &BadHTimingsError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadHTimingsError> for [u8; 32] {
    fn from(input: BadHTimingsError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the BadVTimings error
pub const BAD_V_TIMINGS_ERROR: u8 = 2;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadVTimingsError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl BadVTimingsError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = BadVTimingsError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for BadVTimingsError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for BadVTimingsError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for BadVTimingsError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&BadVTimingsError> for [u8; 32] {
    fn from(input: &BadVTimingsError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<BadVTimingsError> for [u8; 32] {
    fn from(input: BadVTimingsError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ModeUnsuitable error
pub const MODE_UNSUITABLE_ERROR: u8 = 3;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeUnsuitableError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl ModeUnsuitableError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = ModeUnsuitableError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ModeUnsuitableError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ModeUnsuitableError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ModeUnsuitableError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ModeUnsuitableError> for [u8; 32] {
    fn from(input: &ModeUnsuitableError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ModeUnsuitableError> for [u8; 32] {
    fn from(input: ModeUnsuitableError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ExtensionDisabled error
pub const EXTENSION_DISABLED_ERROR: u8 = 4;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtensionDisabledError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl ExtensionDisabledError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = ExtensionDisabledError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ExtensionDisabledError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ExtensionDisabledError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ExtensionDisabledError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ExtensionDisabledError> for [u8; 32] {
    fn from(input: &ExtensionDisabledError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ExtensionDisabledError> for [u8; 32] {
    fn from(input: ExtensionDisabledError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ClientNotLocal error
pub const CLIENT_NOT_LOCAL_ERROR: u8 = 5;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientNotLocalError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl ClientNotLocalError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = ClientNotLocalError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ClientNotLocalError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ClientNotLocalError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ClientNotLocalError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ClientNotLocalError> for [u8; 32] {
    fn from(input: &ClientNotLocalError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ClientNotLocalError> for [u8; 32] {
    fn from(input: ClientNotLocalError) -> Self {
        Self::from(&input)
    }
}

/// Opcode for the ZoomLocked error
pub const ZOOM_LOCKED_ERROR: u8 = 6;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoomLockedError {
    pub response_type: u8,
    pub error_code: u8,
    pub sequence: u16,
}
impl ZoomLockedError {
    pub(crate) fn try_parse(remaining: &[u8]) -> Result<(Self, &[u8]), ParseError> {
        let (response_type, remaining) = u8::try_parse(remaining)?;
        let (error_code, remaining) = u8::try_parse(remaining)?;
        let (sequence, remaining) = u16::try_parse(remaining)?;
        let result = ZoomLockedError { response_type, error_code, sequence };
        Ok((result, remaining))
    }
}
impl TryFrom<&[u8]> for ZoomLockedError {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(Self::try_parse(value)?.0)
    }
}
impl<B: AsRef<[u8]>> From<GenericError<B>> for ZoomLockedError {
    fn from(value: GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl<B: AsRef<[u8]>> From<&GenericError<B>> for ZoomLockedError {
    fn from(value: &GenericError<B>) -> Self {
        Self::try_from(value.raw_bytes()).expect("Buffer should be large enough so that parsing cannot fail")
    }
}
impl From<&ZoomLockedError> for [u8; 32] {
    fn from(input: &ZoomLockedError) -> Self {
        let sequence = input.sequence.serialize();
        [
            input.response_type, input.error_code, sequence[0], sequence[1], /* trailing padding */ 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0
        ]
    }
}
impl From<ZoomLockedError> for [u8; 32] {
    fn from(input: ZoomLockedError) -> Self {
        Self::from(&input)
    }
}

/// Extension trait defining the requests of this extension.
pub trait ConnectionExt: RequestConnection {
    fn xf86vidmode_query_version(&self) -> Result<Cookie<'_, Self, QueryVersionReply>, ConnectionError>
    {
        query_version(self)
    }

    fn xf86vidmode_get_mode_line(&self, screen: u16) -> Result<Cookie<'_, Self, GetModeLineReply>, ConnectionError>
    {
        get_mode_line(self, screen)
    }

    fn xf86vidmode_mod_mode_line<'c>(&'c self, screen: u32, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, private: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        mod_mode_line(self, screen, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }

    fn xf86vidmode_switch_mode(&self, screen: u16, zoom: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        switch_mode(self, screen, zoom)
    }

    fn xf86vidmode_get_monitor(&self, screen: u16) -> Result<Cookie<'_, Self, GetMonitorReply>, ConnectionError>
    {
        get_monitor(self, screen)
    }

    fn xf86vidmode_lock_mode_switch(&self, screen: u16, lock: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        lock_mode_switch(self, screen, lock)
    }

    fn xf86vidmode_get_all_mode_lines(&self, screen: u16) -> Result<Cookie<'_, Self, GetAllModeLinesReply>, ConnectionError>
    {
        get_all_mode_lines(self, screen)
    }

    fn xf86vidmode_add_mode_line<'c>(&'c self, screen: u32, dotclock: DOTCLOCK, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, after_dotclock: DOTCLOCK, after_hdisplay: u16, after_hsyncstart: u16, after_hsyncend: u16, after_htotal: u16, after_hskew: u16, after_vdisplay: u16, after_vsyncstart: u16, after_vsyncend: u16, after_vtotal: u16, after_flags: u32, private: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        add_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, after_dotclock, after_hdisplay, after_hsyncstart, after_hsyncend, after_htotal, after_hskew, after_vdisplay, after_vsyncstart, after_vsyncend, after_vtotal, after_flags, private)
    }

    fn xf86vidmode_delete_mode_line<'c>(&'c self, screen: u32, dotclock: DOTCLOCK, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, private: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        delete_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }

    fn xf86vidmode_validate_mode_line<'c>(&'c self, screen: u32, dotclock: DOTCLOCK, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, private: &[u8]) -> Result<Cookie<'c, Self, ValidateModeLineReply>, ConnectionError>
    {
        validate_mode_line(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }

    fn xf86vidmode_switch_to_mode<'c>(&'c self, screen: u32, dotclock: DOTCLOCK, hdisplay: u16, hsyncstart: u16, hsyncend: u16, htotal: u16, hskew: u16, vdisplay: u16, vsyncstart: u16, vsyncend: u16, vtotal: u16, flags: u32, private: &[u8]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        switch_to_mode(self, screen, dotclock, hdisplay, hsyncstart, hsyncend, htotal, hskew, vdisplay, vsyncstart, vsyncend, vtotal, flags, private)
    }

    fn xf86vidmode_get_view_port(&self, screen: u16) -> Result<Cookie<'_, Self, GetViewPortReply>, ConnectionError>
    {
        get_view_port(self, screen)
    }

    fn xf86vidmode_set_view_port(&self, screen: u16, x: u32, y: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_view_port(self, screen, x, y)
    }

    fn xf86vidmode_get_dot_clocks(&self, screen: u16) -> Result<Cookie<'_, Self, GetDotClocksReply>, ConnectionError>
    {
        get_dot_clocks(self, screen)
    }

    fn xf86vidmode_set_client_version(&self, major: u16, minor: u16) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_client_version(self, major, minor)
    }

    fn xf86vidmode_set_gamma(&self, screen: u16, red: u32, green: u32, blue: u32) -> Result<VoidCookie<'_, Self>, ConnectionError>
    {
        set_gamma(self, screen, red, green, blue)
    }

    fn xf86vidmode_get_gamma(&self, screen: u16) -> Result<Cookie<'_, Self, GetGammaReply>, ConnectionError>
    {
        get_gamma(self, screen)
    }

    fn xf86vidmode_get_gamma_ramp(&self, screen: u16, size: u16) -> Result<Cookie<'_, Self, GetGammaRampReply>, ConnectionError>
    {
        get_gamma_ramp(self, screen, size)
    }

    fn xf86vidmode_set_gamma_ramp<'c>(&'c self, screen: u16, size: u16, red: &[u16], green: &[u16], blue: &[u16]) -> Result<VoidCookie<'c, Self>, ConnectionError>
    {
        set_gamma_ramp(self, screen, size, red, green, blue)
    }

    fn xf86vidmode_get_gamma_ramp_size(&self, screen: u16) -> Result<Cookie<'_, Self, GetGammaRampSizeReply>, ConnectionError>
    {
        get_gamma_ramp_size(self, screen)
    }

    fn xf86vidmode_get_permissions(&self, screen: u16) -> Result<Cookie<'_, Self, GetPermissionsReply>, ConnectionError>
    {
        get_permissions(self, screen)
    }

}
impl<C: RequestConnection + ?Sized> ConnectionExt for C {}