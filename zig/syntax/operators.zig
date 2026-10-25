const print = @import("std").debug.print;

pub fn main() void {
    print("here is the empty string...\n", .{});

    const a = 23;
    // const b = 32;
    // const result = a += b;
    var c: i64 = 10;
    c += a;
    print("result: {}\n", .{c});
}