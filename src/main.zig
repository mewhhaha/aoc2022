const std = @import("std");

pub fn main() !void {
    const allocator = std.heap.page_allocator;
    const stdin = std.io.getStdIn().reader();

    var buffer: [20]u8 = undefined;
    var numbers: std.ArrayList(u32) = std.ArrayList(u32).init(allocator);

    while (try stdin.readUntilDelimiterOrEof(&buffer, '\n')) |line| {
        std.log.info("why does this come out like this {s}", .{line});
        var i: u32 = try std.fmt.parseUnsigned(u32, line, 10);
        try numbers.append(i);
    }
}

test "simple test" {}
