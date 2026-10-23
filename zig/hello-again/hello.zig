const std = @import("std");

pub fn main() void {
    // try std.Io.File.stdout().writeStreamingAll(init.io, "Hello world\n");
    std.debug.print("Hello, {s}!\n", .{"Omiverse"});
}