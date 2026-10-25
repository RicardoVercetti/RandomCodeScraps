const print = @import("std").debug.print;
const std = @import("std");

pub fn main(init: std.process.Init) !void {
    print("An attempt to open a file and print its binary contents in hex...\n", .{});

    // const file = std.Io.Dir.cwd().createFile("junk_file.txt", .{ .read = true },) catch |err| {
    //     print("error occured: {}\n", .{err});
    // };
    // file.writeStreamingAll(.{}, "hello");
    // file.close();

    const io = init.io;

    var buf_out: [128]u8 = undefined;
    var stdout = std.Io.File.stdout().writerStreaming(io, &buf_out);
    const out = &stdout.interface;
    
    const returned = try out.write("trying to write some string to the writer btw..\n");
    print("written: {}\n", .{returned});

    var dir = std.Io.Dir.cwd();
    print("current directory: {}\n", .{dir});
    

    const file = try dir.createFile(io, "zig_example.txt", .{});
    defer file.close(io);

    // var buf: [128]u8 = undefined;
    // var writer = file.writeStreaming(io, &buf);
    // try writer.interface.writeAll("written by zig\n");
    // try writer.interface.flush();

    print("all done buddy...\n", .{});
}
