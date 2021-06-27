cmake_minimum_required(VERSION 3.13.4)

set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY "../../")
# set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
# set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

if(POLICY CMP0068)
  cmake_policy(SET CMP0068 NEW)
  set(CMAKE_BUILD_WITH_INSTALL_NAME_DIR ON)
endif()

if(POLICY CMP0075)
  cmake_policy(SET CMP0075 NEW)
endif()

if(POLICY CMP0077)
  cmake_policy(SET CMP0077 NEW)
endif()

project(rust_cpp_generated LANGUAGES CXX C)
set(CMAKE_CXX_STANDARD 14 CACHE STRING "C++ standard to conform to")

find_package(MLIR REQUIRED CONFIG)

message(STATUS "Using MLIRConfig.cmake in: ${MLIR_DIR}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")

list(APPEND CMAKE_MODULE_PATH "${MLIR_CMAKE_DIR}")
list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}")

include(TableGen)
include(AddLLVM)
include(AddMLIR)
include(HandleLLVMOptions)

include_directories(${LLVM_INCLUDE_DIRS})
include_directories(${MLIR_INCLUDE_DIRS})

link_directories(${LLVM_BUILD_LIBRARY_DIR})
add_definitions(${LLVM_DEFINITIONS})

get_property(dialect_libs GLOBAL PROPERTY MLIR_DIALECT_LIBS)
get_property(conversion_libs GLOBAL PROPERTY MLIR_CONVERSION_LIBS)
get_property(translation_libs GLOBAL PROPERTY MLIR_CONVERSION_LIBS)
set(LIBS
        ${dialect_libs}
        ${conversion_libs}
        ${translation_libs}
        MLIRAnalysis
        MLIRCallInterfaces
        MLIRExecutionEngine
        MLIRIR
        MLIRLLVMIR
        MLIRParser
        MLIRPass
        MLIRSideEffectInterfaces
        MLIRSupport
        MLIRTargetLLVMIRExport
        MLIRTransforms
        MLIROptLib
        MLIRHello
        )
message(STATUS "MLIR_ALL_LIBS: ${dialect_libs}")
add_llvm_library(rust_cpp_generated "/home/mrsmith/Desktop/code_experiments/engine/target/debug/build/engine-4cd3ae83113637c8/out/rust_cpp/cpp_closures.cpp")

llvm_update_compile_flags(rust_cpp_generated)
target_link_libraries(rust_cpp_generated PRIVATE ${LIBS})

mlir_check_all_link_libraries(rust_cpp_generated)
