const print = @import("std").debug.print;
const assert = @import("std").debug.assert;

pub fn main() void {
    print("here goes nothing...\n", .{});

    var one_plus_one: i32 = 1 + 1;
    print("the result is: {}\n", .{one_plus_one});

    one_plus_one += 1;

    print("after addition: {}\n", .{one_plus_one});

    // const flt: f32 = 7.0 / 3.0;
    // std.debug.print("flt: {}\n", .{flt});

    // std.debug.print("{}\n{}\n{}\n", .{
    //     true and true,
    //     true or false,
    //     !true,
    // });

    const optional_value: ?[]const u8 = null;

    assert(optional_value == null);


    print("\nOptional 1\ntype: {}\nvalue: {?s}\n", .{@TypeOf(optional_value), optional_value});

    print("printing something of some length...", .{});

    print("\roverriding...", .{});
    print("\n", .{});

    var someString = "here is some string...";
    someString = someString - " some addition...";
    print("{s}\n", .{someString});
}
