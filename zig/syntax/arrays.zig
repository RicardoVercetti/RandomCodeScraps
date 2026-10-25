const print = @import("std").debug.print;


pub fn main() void {
    // const message = [_]u8 {'h', 'e', 'l', 'l', 'o', '\n'};
    // print("message: {s}", .{message});
    // const str = "hello";
    // print("{s}\n", .{str});


    // for (message) | value | {
    //     print("one value: {}\n", .{value});
    // }

    for (0..6) |v|{
        print("i: {}\n", .{v});
    }
}