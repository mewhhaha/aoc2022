const std = @import("std");
const fmt = std.fmt;
const log = std.log;
const io = std.io;
const math = std.math;

pub fn main() !void {
    const stdin = io.getStdIn().reader();

    var buffer: [20]u8 = undefined;
    var current: u32 = 0;
    var max: u32 = 0;

    while (try stdin.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        if (line.len == 0) {
            max = math.max(current, max);
            current = 0;
        } else {
            current += try fmt.parseUnsigned(u32, line, 10);
        }
    }
    max = math.max(current, max);

    log.info("{d}", .{max});
}
