use enumflags2::BitFlags;
use num_derive::FromPrimitive;

pub type WINDOW = u32;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Request<'a> {
    Connect(Connect<'a>),
    ConnectReply(ConnectReply),
}

impl<'a> Request<'a> {
    pub fn opcode(&self) -> Opcode {
        match self {
            Request::Connect(..) => Opcode::Connect,
            Request::ConnectReply(..) => Opcode::ConnectReply,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[repr(transparent)]
pub struct XimString<'a>(pub &'a str);

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Connect<'a> {
    pub client_major_protocol_version: u16,
    pub client_minor_protocol_version: u16,
    pub client_auth_protocol_names: Vec<XimString<'a>>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct ConnectReply {
    pub server_major_protocol_version: u16,
    pub server_minor_protocol_version: u16,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct RequestHeader {
    pub major_opcode: Opcode,
    pub minor_opcode: u8,
    pub length: u16,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Opcode {
    Connect = 1,
    ConnectReply = 2,
    Disconnect = 3,
    DisconnectReply = 4,

    OpenReply = 31,
    CloseReply = 33,
    RegisterTriggerKeys = 34,
    TriggerNotifyReply = 36,

    SetEventMask = 37,
    EncodingNegotiationReply = 39,
    QueryExtensionReply = 41,
    SetImValuesReply = 43,
    GetImValuesReply = 45,

    CreateIcReply = 51,
    DestroyIcReply = 53,
    SetIcValuesReply = 55,
    GetIcValuesReply = 57,
    SyncReply = 62,
    Commit = 63,
    ResetIcReply = 65,

    Geometry = 70,
    StrConversion = 71,
    PreeditStart = 73,
    PreeditDraw = 75,
    PreeditCaret = 76,
    PreeditDone = 78,
    StatusStart = 79,
    StatusDraw = 80,
    StatusDone = 81,
    PreeditState = 82,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Attr<'a> {
    pub id: u16,
    pub type_: u16,
    pub name: XimString<'a>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Attribute<'a> {
    pub id: u16,
    pub name: XimString<'a>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct EncodingInfo<'a> {
    pub name: XimString<'a>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct StrConvText<'a> {
    pub type_: u16,
    pub text: XimString<'a>,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct TriggerKey {
    pub keysym: u32,
    pub modifier: u32,
    pub modifier_mask: u32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct PreeditCaret {
    pub method_id: u16,
    pub context_id: u16,
    pub position: i32,
    pub direction: CaretDirection,
    pub style: CaretStyle,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct PreeditCaretReply {
    pub method_id: u16,
    pub context_id: u16,
    pub position: i32,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct PreeditDone {
    pub method_id: u16,
    pub context_id: u16,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, FromPrimitive)]
#[repr(u32)]
pub enum CaretDirection {
    ForwardChar = 0,
    BackwardChar = 1,
    ForwardWord = 2,
    BackwardWord = 3,
    CaretUp = 4,
    CaretDown = 5,
    NextLine = 6,
    PreviousLine = 7,
    LineStart = 8,
    LineEnd = 9,
    AbsolutePosition = 10,
    DontChange = 11,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, FromPrimitive)]
#[repr(u32)]
pub enum CaretStyle {
    Invisible = 0,
    Primary = 1,
    Secondary = 2,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, FromPrimitive, BitFlags)]
#[repr(u16)]
pub enum StrConvFeedbackType {
    LeftEdge = 0x1,
    RightEdge = 0x2,
    TopEdge = 0x4,
    BottomEdge = 0x8,
    Convealed = 0x10,
    Wrapped = 0x20,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, FromPrimitive, BitFlags)]
#[repr(u32)]
pub enum Feedback {
    Reverse = 0x1,
    Underline = 0x2,
    Highlight = 0x4,
    Primary = 0x8,
    Secondary = 0x10,
    Tertiary = 0x20,
    VisibleToForward = 0x40,
    VisibleToBackward = 0x80,
    VisibleToCenter = 0x100,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, FromPrimitive)]
#[repr(u32)]
pub enum HotkeyState {
    On = 0x1,
    Off = 0x2,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, FromPrimitive)]
#[repr(u32)]
pub enum PreeditState {
    Enable = 0x1,
    Disable = 0x2,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, FromPrimitive)]
#[repr(u32)]
pub enum ResetState {
    Initial = 0x1,
    Preserve = 0x2,
}
