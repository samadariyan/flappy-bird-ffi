//
// we have to take the help of CC crate to compile C code
//

use std::fs;

fn main() {
    let mut builder = cc::Build::new();

    // 1. set the cross compiler
    builder.compiler("arm-none-eabi-gcc");

    // 2. Add all C source files (.c files)
    // 2.a Add all C files from the HAL diver folder
    // builder.file("c_src/lcd_tsc_mpu_drivers/Drivers/STM32F3xx_HAL_Driver/Src/file.c");
    let hal_src_path = "c_src/lcd_tsc_mpu_drivers/Drivers/STM32F3xx_HAL_Driver/Src";

    for entry in fs::read_dir(hal_src_path).expect("can not read hal driver from src folder") {
        let path = entry.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            println!("Compiling {:?}", path);
            println!("cargo::rerun-if-changed={}", path.display());
            builder.file(&path);
        }
    }

    // 2.b Add all .c files from core/src
    let core_src_path = "c_src/lcd_tsc_mpu_drivers/Core/Src";

    for entry in fs::read_dir(core_src_path).expect("can not read core file from src folder") {
        let path = entry.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) == Some("c") {
            println!("Compiling {:?}", path);
            println!("cargo::rerun-if-changed={}", path.display());
            builder.file(&path);
        }
    }

    // 3. Add all C include files (.h files)
    builder.include("c_src/lcd_tsc_mpu_drivers/Core/Inc");
    builder.include("c_src/lcd_tsc_mpu_drivers/Drivers/STM32F3xx_HAL_Driver/Inc");
    builder.include("c_src/lcd_tsc_mpu_drivers/Drivers/CMSIS/Include");
    builder.include("c_src/lcd_tsc_mpu_drivers/Drivers/CMSIS/Device/ST/STM32F3xx/Include");

    // 4. Add Define macros, -D (optional)
    builder.define("DEBUG", None);
    builder.define("USE_HAL_DRIVER", None);
    builder.define("STM32F303xC", None);

    // 5. Add os or .asm files (optional)
    builder.file("c_src/lcd_tsc_mpu_drivers/Core/Startup/startup_stm32f303cctx.s");
    println!(
        "cargo::rerun-if-changed=c_src/lcd_tsc_mpu_drivers/Core/Startup/startup_stm32f303cctx.s"
    );

    // 6. Add compiler flags
    builder
        .flag("-mcpu=cortex-m4")
        .flag("-mthumb")
        .flag("-mfpu=fpv4-sp-d16")
        .flag("-mfloat-abi=hard")
        .flag("-std=gnu11")
        .flag("-g3")
        .flag("-O0")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-Wall")
        .flag("-fstack-usage")
        .flag("-fcyclomatic-complexity");

    // 7. Add linker flags
    println!("cargo::rustc-link-arg=--specs=nano.specs");
    println!("cargo::rustc-link-arg=--specs=nosys.specs");
    println!("cargo::rustc-link-arg=-Wl,--gc-sections");
    println!("cargo::rustc-link-arg=-Wl,--start-group");
    println!("cargo::rustc-link-arg=-lc");
    println!("cargo::rustc-link-arg=-lm");
    println!("cargo::rustc-link-arg=-Wl,--end-group");

    // 8. generate the static library
    builder.compile("stm32_c_drivers");

    // 9. you may communicate with the cargo from build script using println!() statements.
    println!("cargo::rustc-link-lib=static=stm32_c_drivers")
}
