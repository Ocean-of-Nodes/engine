#![recursion_limit = "256"]
#![feature(trait_alias)]

use std::future::Future;

enum Operation {
    Has(),
    HasLabel(),
}

struct Has {}

struct Traversal {
    ops: Vec<Operation>,
}

impl Traversal {
    fn has(self, key: &str, value: &str) -> Self {
        todo!()
    }

    fn has_label() -> Self {
        todo!()
    }

    fn faz_has(self) -> Self {
        todo!()
    }

    fn exec(self) -> impl Future {
        std::future::ready(1)
    }
}

use cpp::{cpp, cpp_class};

cpp! {{
    #include <iostream>
    #include "mlir/IR/Dialect.h"
    #include "mlir/IR/BuiltinOps.h"
    #include "mlir/IR/MLIRContext.h"
    #include "mlir/IR/OpDefinition.h"
    #include "mlir/IR/BuiltinTypes.h"
    #include "mlir/IR/AsmState.h"
    #include "mlir/IR/Builders.h"

    #include "mlir/Support/LogicalResult.h"
    #include "mlir/Support/TypeID.h"
    #include "mlir/ExecutionEngine/ExecutionEngine.h"
    #include "mlir/ExecutionEngine/OptUtils.h"
    #include "mlir/Target/LLVMIR/Dialect/LLVMIR/LLVMToLLVMIRTranslation.h"
    #include "mlir/Target/LLVMIR/Export.h"
    #include "llvm/Support/TargetSelect.h"

    class RepeatOp;

    class MMADTDialect : public mlir::Dialect {
    public:
        explicit MMADTDialect(mlir::MLIRContext *ctx): mlir::Dialect("mmadt", ctx, mlir::TypeID::get<MMADTDialect>()) {
            addOperations<RepeatOp>();
        }

        static  llvm::StringRef getDialectNamespace() {
            return "mmadt";
        }
    };

    class RepeatOp : public mlir::Op<
                        RepeatOp,
                        mlir::OpTrait::ZeroOperands,
                        mlir::OpTrait::OneResult,
                        mlir::OpTrait::OneTypedResult<mlir::TensorType>::Impl> {
    public:
        using Op::Op;

        static llvm::StringRef getOperationName() { return "mmadt.repeat"; }

        // mlir::DenseElementsAttr getValue();

        // mlir::LogicalResult verify();

        static void build(
            mlir::OpBuilder &builder,
            mlir::OperationState &state
            // mlir::Type result,
            // mlir::DenseElementsAttr value
        ) {
            // mlir::OpBuilder::create<ConstantOp>(...);
        }
    };
}}

cpp! {{
    // int runJit(mlir::ModuleOp module) {
    //     // Initialize LLVM targets.
    //     llvm::InitializeNativeTarget();
    //     llvm::InitializeNativeTargetAsmPrinter();

    //     // Register the translation from MLIR to LLVM IR, which must happen before we
    //     // can JIT-compile.
    //     mlir::registerLLVMDialectTranslation(*module->getContext());

    //     // An optimization pipeline to use within the execution engine.
    //     auto optPipeline = mlir::makeOptimizingTransformer(
    //         /*optLevel=*/enableOpt ? 3 : 0, /*sizeLevel=*/0,
    //         /*targetMachine=*/nullptr);

    //     // Create an MLIR execution engine. The execution engine eagerly JIT-compiles
    //     // the module.
    //     auto maybeEngine = mlir::ExecutionEngine::create(module, nullptr, optPipeline);
    //     assert(maybeEngine && "failed to construct an execution engine");
    //     auto &engine = maybeEngine.get();

    //     // Invoke the JIT-compiled function.
    //     auto invocationResult = engine->invokePacked("main");
    //     if (invocationResult) {
    //       llvm::errs() << "JIT invocation failed\n";
    //       return -1;
    //     }

    //     return 0;
    //   }
}}

enum Instruction {
    Branch,
    Combine,
    Lift,
    Merge,
    Repeat,
    Split,
    Swap,
    Trace,
}

trait Instructions = IntoIterator<Item = Instruction>;

/// Implements of a simple MLIR emission.
cpp! {{
    struct MLIRGen {
        mlir::ModuleOp module;
    };
}}
cpp_class!(pub unsafe struct MLIRGen as "MLIRGen");

impl MLIRGen {
    fn new() -> Self {
        unsafe {
            cpp!([] -> MLIRGen as "MLIRGen" {
                auto context = new mlir::MLIRContext;
                context->loadDialect<MMADTDialect>();
                mlir::OpBuilder builder(context);
                auto module = mlir::ModuleOp::create(builder.getUnknownLoc());

                module.push_back(builder.create<RepeatOp>(builder.getUnknownLoc()));
                // mlir::OwningModuleRef module;
                // if (int error = runJit(module)) {
                //     std::cout << "error: " << error << std::endl;
                // }

                // std::cout << "Hello, 1212" << std::endl;
                return MLIRGen{module};
            })
        }
    }

    // Translate `mmadt`
    fn translate(&mut self, insts: impl Instructions) {
        for inst in insts {

        }
    }

    fn dump(&self) { 
        unsafe {
            cpp!([self as "MLIRGen*"] {
                self->module.dump();
            })
        }
    }
}

fn main() {
    let mut gen = MLIRGen::new();
    gen.dump();
}
