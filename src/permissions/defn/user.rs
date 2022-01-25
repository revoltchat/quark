use num_enum::TryFromPrimitive;
use std::ops;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Copy, Clone)]
#[repr(u32)]
pub enum UserPermission {
    Access = 0b00000000000000000000000000000001,      // 1
    ViewProfile = 0b00000000000000000000000000000010, // 2
    SendMessage = 0b00000000000000000000000000000100, // 4
    Invite = 0b00000000000000000000000000001000,      // 8
}

impl_op_ex!(+ |a: &UserPermission, b: &UserPermission| -> u32 { *a as u32 | *b as u32 });
impl_op_ex_commutative!(+ |a: &u32, b: &UserPermission| -> u32 { *a | *b as u32 });

pub struct UserPermissions();
