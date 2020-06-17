const ffi = struct {
    const FFIString = extern struct {
        ptr: *u8,
        len: usize,
        cap: usize,
    };

    extern fn string_new() FFIString;
    extern fn string_drop(noalias s: *FFIString) void;
    extern fn string_push(noalias s: *FFIString, ch: u32) void;
    extern fn string_len(s: *const FFIString) usize;
    extern fn string_ptr(s: *const FFIString) *const u8;
    extern fn string_push_str(noalias s: *FFIString, ptr: *const u8, len: usize) void;
};

pub const String = struct {
    inner: ffi.FFIString,

    const Self = @This();

    fn asPtr(self: *const Self) *const u8 {
        return ffi.string_ptr(&self.inner);
    }

    pub fn init() Self {
        const ffistring = ffi.string_new();

        return .{ .inner = ffistring };
    }

    pub fn deinit(noalias self: *Self) void {
        ffi.string_drop(&self.inner);
    }

    pub fn push(noalias self: *Self, ch: u32) void {
        ffi.string_push(&self.inner, ch);
    }

    pub fn len(self: *const Self) usize {
        return ffi.string_len(&self.inner);
    }

    pub fn asSlice(self: *const Self) []const u8 {
        const bytes_ptr = self.asPtr();
        const ptr = @ptrCast([*]const u8, bytes_ptr);
        const self_len = self.len();

        return ptr[0..self_len];
    }

    pub fn pushStr(noalias self: *Self, str: []const u8) void {
        ffi.string_push_str(&self.inner, @ptrCast(*const u8, str.ptr), str.len);
    }
};

const std = @import("std");
const testing = std.testing;

test "test string" {
    var s = String.init();
    defer s.deinit();

    s.push('h');
    s.push('e');

    s.pushStr("llo world!");
    s.pushStr("τεσ");

    std.debug.assert(std.mem.eql(u8, s.asSlice(), "hello world!τεσ"));
}
