const std = @import("std");
const fmt = std.fmt;
const log = std.log;
const io = std.io;

pub fn main() !void {
    const stdin = io.getStdIn().reader();

    var buffer: [20]u8 = undefined;
    var current: u32 = 0;
    var max: u32 = 0;

    while (try stdin.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        if (line.len == 0) {
            if (current > max) max = current;
            current = 0;
        } else {
            var n: u32 = try fmt.parseUnsigned(u32, line, 10);
            current += n;
        }
    }
    if (current > max) max = current;

    log.info("{d}", .{max});
}
