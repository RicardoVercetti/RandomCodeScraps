const print = @import("std").debug.print;
const assert = @import("std").testing.expectEqual;

fn max(comptime T: type, a: T, b: T) T {
    return if (a > b) a else b;
}

fn gimmeBiggerFloat(a: f32, b: f32) f32 {
    return max(f32, a, b);
}

fn gimmeBiggerInt(a: i32, b: i32) i32 {
    return max(i32, a, b);
}

pub fn main() void {
    print("here is the sample string...\n", .{});

    const a: f32 = 10.1;
    const b: f32 = 20.3;

    const c = gimmeBiggerFloat(a, b);

    print("c: {}\n", .{c});
}

test "some random test module" {
    const a = 20;
    const b = 10;
    const c = gimmeBiggerInt(a, b);
    try assert(10, c);
}

test "some other test module" {
    const a = 30.10;
    const b = 40.10;
    const c = gimmeBiggerFloat(a, b);
    try assert(40.10, c);
}