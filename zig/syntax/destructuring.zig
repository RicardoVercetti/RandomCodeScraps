const print = @import("std").debug.print;

pub fn main() void {
    print("destructuring things...\n", .{});

    var a: i32 = undefined;
    var b: i32 = undefined;
    var c: i32 = undefined;


    const tuple: struct {i32, i32, i32} = .{10, 20, 30 };

    a, b, c = tuple;
    print("a: {}, b: {}, c: {}\n", .{a, b, c});
}
