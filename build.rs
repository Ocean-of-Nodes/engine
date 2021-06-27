//! Parse and collect inserts into inserts file.
//! After, uses library cmake for building result file as library.
//!
//! NOTE: Uses a modified version cpp_build in which the build was removed.

extern crate cpp_build;
use std::process::Command;

use cpp_build::{create_lib, CPP_DIR};
use std::array;

fn link_libs(libs: &[&str]) {
    for lib in libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }
}

fn main() {
    let inserts_path = create_lib("src/main.rs");
    let status = Command::new("cmake")
        .args(&[
            "-S",
            "./",
            "-B",
            &CPP_DIR.join("build").as_path().to_string_lossy(),
            "-G",
            "Ninja",
        ])
        .envs(array::IntoIter::new([
            ("LLVM_DIR", env!("LLVM_DIR")),
            ("MLIR_DIR", env!("MLIR_DIR")),
        ]))
        .status()
        .expect("failed to execute cmake");

    if !status.success() {
        panic!("cmake execution error: {}", status);
    }

    let status = Command::new("cmake")
        .current_dir(CPP_DIR.join("build").as_path())
        .args(&["--build", "."])
        .status()
        .expect("failed to execute cmake");

    if !status.success() {
        panic!("cmake execution error: {}", status);
    }

    let OUT_DIR = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    println!("OUT_DIR: {}", OUT_DIR.to_string_lossy());
    println!(
        "cargo:rustc-link-search=native={}",
        OUT_DIR.to_string_lossy()
    );

    println!(
        "cargo:rustc-link-search=native={}",
        "/home/mrsmith/Desktop/code_experiments/llvm-project/build/lib/"
    );

    println!("cargo:rustc-link-lib=dylib=dl");
    println!("cargo:rustc-link-lib=dylib=m");
    println!("cargo:rustc-link-lib=dylib=pthread");
    link_libs(&[
        "stdc++",
        // llvm-config --system-libs
        "rt",
        "z",
        "tinfo",
        // llvm-config --libs
        "LLVMWindowsManifest",
        "LLVMXRay",
        "LLVMLibDriver",
        "LLVMDlltoolDriver",
        "LLVMCoverage",
        "LLVMLineEditor",
        "LLVMAMDGPUDisassembler",
        "LLVMAMDGPUAsmParser",
        "LLVMAMDGPUCodeGen",
        "LLVMAMDGPUDesc",
        "LLVMAMDGPUUtils",
        "LLVMAMDGPUInfo",
        "LLVMNVPTXCodeGen",
        "LLVMNVPTXDesc",
        "LLVMNVPTXInfo",
        "LLVMX86Disassembler",
        "LLVMX86AsmParser",
        "LLVMX86CodeGen",
        "LLVMX86Desc",
        "LLVMX86Info",
        "LLVMOrcJIT",
        "LLVMMCJIT",
        "LLVMJITLink",
        "LLVMInterpreter",
        "LLVMExecutionEngine",
        "LLVMRuntimeDyld",
        "LLVMOrcTargetProcess",
        "LLVMOrcShared",
        "LLVMSymbolize",
        "LLVMDebugInfoPDB",
        "LLVMDebugInfoGSYM",
        "LLVMOption",
        "LLVMObjectYAML",
        "LLVMMCA",
        "LLVMMCDisassembler",
        "LLVMLTO",
        "LLVMPasses",
        "LLVMCFGuard",
        "LLVMCoroutines",
        "LLVMObjCARCOpts",
        "LLVMipo",
        "LLVMVectorize",
        "LLVMLinker",
        "LLVMInstrumentation",
        "LLVMFrontendOpenMP",
        "LLVMFrontendOpenACC",
        "LLVMExtensions",
        "LLVMDWARFLinker",
        "LLVMGlobalISel",
        "LLVMMIRParser",
        "LLVMAsmPrinter",
        "LLVMDebugInfoMSF",
        "LLVMDebugInfoDWARF",
        "LLVMSelectionDAG",
        "LLVMCodeGen",
        "LLVMIRReader",
        "LLVMAsmParser",
        "LLVMInterfaceStub",
        "LLVMFileCheck",
        "LLVMFuzzMutate",
        "LLVMTarget",
        "LLVMScalarOpts",
        "LLVMInstCombine",
        "LLVMAggressiveInstCombine",
        "LLVMTransformUtils",
        "LLVMBitWriter",
        "LLVMAnalysis",
        "LLVMProfileData",
        "LLVMObject",
        "LLVMTextAPI",
        "LLVMMCParser",
        "LLVMMC",
        "LLVMDebugInfoCodeView",
        "LLVMBitReader",
        "LLVMCore",
        "LLVMRemarks",
        "LLVMBitstreamReader",
        "LLVMBinaryFormat",
        "LLVMTableGen",
        "LLVMSupport",
        "LLVMDemangle",
        //dialect_libs
        "MLIRAffine",
        "MLIRAffineEDSC",
        "MLIRAffineTransforms",
        "MLIRAffineUtils",
        "MLIRArmNeon",
        "MLIRArmSVE",
        "MLIRArmSVETransforms",
        "MLIRAsync",
        "MLIRAsyncTransforms",
        "MLIRAMX",
        "MLIRAMXTransforms",
        "MLIRComplex",
        "MLIRDLTI",
        "MLIRGPU",
        "MLIRLinalgAnalysis",
        "MLIRLinalgEDSC",
        "MLIRLinalg",
        "MLIRLinalgTransforms",
        "MLIRLinalgUtils",
        "MLIRLLVMIRTransforms",
        "MLIRLLVMIR",
        "MLIRNVVMIR",
        "MLIRROCDLIR",
        "MLIRMath",
        "MLIRMathTransforms",
        "MLIRMemRef",
        "MLIRMemRefTransforms",
        "MLIRMemRefUtils",
        "MLIROpenACC",
        "MLIROpenMP",
        "MLIRPDL",
        "MLIRPDLInterp",
        "MLIRQuant",
        "MLIRSCF",
        "MLIRSCFTransforms",
        "MLIRSDBM",
        "MLIRShape",
        "MLIRShapeOpsTransforms",
        "MLIRSPIRV",
        "MLIRSPIRVModuleCombiner",
        "MLIRSPIRVConversion",
        "MLIRSPIRVTransforms",
        "MLIRSPIRVUtils",
        "MLIRStandard",
        "MLIRStandardOpsTransforms",
        "MLIRTensor",
        "MLIRTensorTransforms",
        "MLIRTosa",
        "MLIRTosaTransforms",
        "MLIRVector",
        "MLIRX86Vector",
        "MLIRX86VectorTransforms",
        "MLIRTosaTestPasses",
        //conversion_libs
        "MLIRAffineToStandard",
        "MLIRAsyncToLLVM",
        "MLIRComplexToLLVM",
        "MLIRGPUToGPURuntimeTransforms",
        "MLIRGPUToNVVMTransforms",
        "MLIRGPUToROCDLTransforms",
        "MLIRGPUToSPIRV",
        "MLIRGPUToVulkanTransforms",
        "MLIRLinalgToLLVM",
        "MLIRLinalgToSPIRV",
        "MLIRLinalgToStandard",
        "MLIRMathToLibm",
        "MLIROpenMPToLLVM",
        "MLIRPDLToPDLInterp",
        "MLIRSCFToGPU",
        "MLIRSCFToOpenMP",
        "MLIRSCFToSPIRV",
        "MLIRSCFToStandard",
        "MLIRShapeToStandard",
        "MLIRSPIRVToLLVM",
        "MLIRStandardToLLVM",
        "MLIRStandardToSPIRV",
        "MLIRTosaToLinalg",
        "MLIRTosaToSCF",
        "MLIRTosaToStandard",
        "MLIRVectorToROCDL",
        "MLIRVectorToLLVM",
        "MLIRVectorToSCF",
        "MLIRVectorToSPIRV",
        //translation_libs
        "MLIRAffineToStandard",
        "MLIRAsyncToLLVM",
        "MLIRComplexToLLVM",
        "MLIRGPUToGPURuntimeTransforms",
        "MLIRGPUToNVVMTransforms",
        "MLIRGPUToROCDLTransforms",
        "MLIRGPUToSPIRV",
        "MLIRGPUToVulkanTransforms",
        "MLIRLinalgToLLVM",
        "MLIRLinalgToSPIRV",
        "MLIRLinalgToStandard",
        "MLIRMathToLibm",
        "MLIROpenMPToLLVM",
        "MLIRPDLToPDLInterp",
        "MLIRSCFToGPU",
        "MLIRSCFToOpenMP",
        "MLIRSCFToSPIRV",
        "MLIRSCFToStandard",
        "MLIRShapeToStandard",
        "MLIRSPIRVToLLVM",
        "MLIRStandardToLLVM",
        "MLIRStandardToSPIRV",
        "MLIRTosaToLinalg",
        "MLIRTosaToSCF",
        "MLIRTosaToStandard",
        "MLIRVectorToROCDL",
        "MLIRVectorToLLVM",
        "MLIRVectorToSCF",
        "MLIRVectorToSPIRV",
        "MLIRAnalysis",
        "MLIRCallInterfaces",
        "MLIRExecutionEngine",
        "MLIRIR",
        "MLIRLLVMIR",
        "MLIRParser",
        "MLIRPass",
        "MLIRSideEffectInterfaces",
        "MLIRSupport",
        "MLIRTargetLLVMIRExport",
        "MLIRTransforms",
        "MLIROptLib",
        "rust_cpp_generated",
    ]);
}
