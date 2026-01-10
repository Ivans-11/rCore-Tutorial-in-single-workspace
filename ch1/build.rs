fn main() {
    use std::{env, fs, path::PathBuf};

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_NOBIOS");

    let nobios = env::var("CARGO_FEATURE_NOBIOS").is_ok();

    let linker_script = if nobios {
        // nobios mode: M-Mode entry at 0x80000000, then jump to S-Mode at 0x80200000
        r#"
OUTPUT_ARCH(riscv)
ENTRY(_m_start)
M_BASE_ADDRESS = 0x80000000;
S_BASE_ADDRESS = 0x80200000;

SECTIONS {
    . = M_BASE_ADDRESS;

    .text.m_entry : {
        *(.text.m_entry)
    }

    .text.m_trap : {
        *(.text.m_trap)
    }

    .bss.m_stack : {
        *(.bss.m_stack)
    }

    .bss.m_data : {
        *(.bss.m_data)
    }

    . = S_BASE_ADDRESS;

    .text : {
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    .bss : {
        *(.bss.uninit)
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
}
"#
    } else {
        // SBI mode: kernel at 0x80200000
        r#"
OUTPUT_ARCH(riscv)
SECTIONS {
    .text 0x80200000 : {
        *(.text.entry)
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }
    .bss : {
        *(.bss.uninit)
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }
}
"#
    };

    let ld = &PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("linker.ld");
    fs::write(ld, linker_script).unwrap();
    println!("cargo:rustc-link-arg=-T{}", ld.display());
}
