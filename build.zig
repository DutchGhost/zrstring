const Builder = @import("std").build.Builder;

pub fn build(b: *Builder) !void {
    const mode = b.standardReleaseOptions();

    const rustbuild = b.addSystemCommand(&[_][]const u8{
        "cargo",
        "+nightly",
        "build",
        "--release",
        "--manifest-path",
        "rstring/Cargo.toml",
    });
    try rustbuild.step.make();

    const lib = b.addStaticLibrary("zrstring", "src/main.zig");
    lib.setBuildMode(mode);

    lib.addLibPath("./rstring/target/release");
    lib.linkSystemLibrary("rstring");
    lib.linkLibC();
    lib.install();

    var main_tests = b.addTest("src/main.zig");
    main_tests.setBuildMode(mode);
    main_tests.addLibPath("./rstring/target/release");
    main_tests.linkSystemLibrary("rstring");
    main_tests.linkLibC();

    const test_step = b.step("test", "Run library tests");
    test_step.dependOn(&main_tests.step);
}
