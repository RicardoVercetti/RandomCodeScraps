const print = @import("std").debug.print;


pub fn main() void {
    print("some conditional syntax testing...\n", .{});

    const a: f32 = 24;
    const b: f32 = 55;
    const c: f32 = a - b;
    const cond: bool = c > 0;

    if (!cond) {
        print("It's grater than 0\n", .{});
        print("but this is a negative condition...\n", .{});
    } else {
        print("It's a negative number: {}\n", .{c});
    }

    if (c == -31) {
        print("Its -31 though...\n", .{});
    }

    // const result: Err!f32 = foo(a, b);

    // if (result) |value|{
    //     print("this is a result: {}\n", .{value});
    // } else {
    //     print("the error: {}\n", .{result});
    // }


    // seems like optionals and error values are different
    const result = opt(a, b);
    if (result) |value| {
        print("we got value: {}\n", .{value});
    } else {
        print("We got no value...\n", .{});
    }

    // some straight forward way of doing things...
    const res: ?i32 = null;
    if (res != null) {
        print("It's directly not null....\n", .{});
    } else {
        print("It's direcly null\n", .{});
    }

    const struc = .{
        232,
        263,
        2.49,
        "some string",
    };

    print("struct values: {}\n", .{struc});

    // struc.@"2" = @as(comptime_int, 22);

    // print("changed value: {}\n", .{struc.@"0"});

    var opt_value: ?i32 = null;
    opt_value = 32;

    if (opt_value) |value| {
        print("opt value: {}\n", .{value});
    } else {
        print("there is nothing in there you idiot...\n", .{});
    }

    

    print("Exiting the code...\n", .{});
}

const Err = error {
    FirstError,
    SecondError,
    DivisionByZeroError,
};

fn foo(a: f32, b: f32) Err!f32 {
    if (a == 0 or b == 0) return Err.DivisionByZeroError;

    return a / b;
}

fn opt(a: f32, b: f32) ?f32 {
    if (a == 0 or b == 0) return null;
    return a / b;
}