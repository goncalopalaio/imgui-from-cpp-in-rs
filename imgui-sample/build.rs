use std::path::Path;
use std::env;

fn main() {
	let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // Small example for testing
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");

    // Compile the whole example. The only difference is that main was renamed to imgui_example_main
	let files = vec!("src/gui/main.cpp",
						"src/gui/imgui_impl/imgui_impl_glfw.cpp",
						"src/gui/imgui_impl/imgui_impl_opengl3.cpp",
						"src/gui/imgui/imgui.cpp",
						"src/gui/imgui/imgui_demo.cpp",
						"src/gui/imgui/imgui_draw.cpp",
						"src/gui/imgui/imgui_widgets.cpp",
						"src/gui/gl_libs/gl3w/GL/gl3w.c");

	let imgui = Path::new("src/gui/imgui/");
    let imgui_impl = Path::new("src/gui/imgui_impl/");
    let gl_libs = Path::new("src/gui/gl_libs/gl3w");

    // Unused but included in the original makefile
    // let usr_local_include = Path::new("/usr/local/include");
    // let opt_local_include = Path::new("/opt/local/include");
	//.include(usr_local_include)
	//.include(opt_local_include)
    // .flag("-L/usr/local/lib")
    // .flag("-L/opt/local/lib")

    cc::Build::new()
    	.cpp(true)
        .include(imgui_impl)
        .include(imgui)
        .include(gl_libs)
        .files(files)
        .define("IMGUI_IMPL_OPENGL_LOADER_GL3W", "")
        .compile("gui");

    if target_os == "macos" {
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=dylib=glfw");
    } else if target_os == "linux" {
        println!("cargo:rustc-link-lib=dylib=rt");
        println!("cargo:rustc-link-lib=dylib=m");
        println!("cargo:rustc-link-lib=dylib=Xrandr");
        println!("cargo:rustc-link-lib=dylib=Xinerama");
        println!("cargo:rustc-link-lib=dylib=Xxf86vm");
        println!("cargo:rustc-link-lib=dylib=Xcursor");
        println!("cargo:rustc-link-lib=dylib=X11");
    }
}
