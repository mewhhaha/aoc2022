const std = @import("std");
const sort = std.sort;
const fmt = std.fmt;
const log = std.log;
const io = std.io;
const mem = std.mem;

pub fn main() !void {
    const stdin = io.getStdIn().reader();

    var buffer: [20]u8 = undefined;
    var current: u32 = 0;
    var top = mem.zeroes([3]u32);

    while (try stdin.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        if (line.len == 0) {
            sort.sort(u32, &top, {}, sort.asc(u32));
            if (current > top[0]) top[0] = current;
            current = 0;
        } else {
            var n: u32 = try fmt.parseUnsigned(u32, line, 10);
            current += n;
        }
    }

    log.info("{d}", .{sum(u32, &top)});
}

pub fn sum(comptime T: type, numbers: []T) T {
    var s: T = 0;
    for (numbers) |n| {
        s += n;
    }
    return s;
}
