#![recursion_limit="256"]

use std::future::Future;

enum Operation {
    Has(),
    HasLabel(),
}

struct Has {

}

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

use cpp::cpp;

cpp!{{
    #include <iostream>
    #include "mlir/IR/Dialect.h"
    #include "mlir/IR/BuiltinOps.h"
    #include "mlir/IR/MLIRContext.h"
    #include "mlir/IR/OpDefinition.h"
    #include "mlir/IR/BuiltinTypes.h"
    #include "mlir/Support/LogicalResult.h"

    class RepeatOp;

    class MMADTDialect : public mlir::Dialect {
    public:
        explicit MMADTDialect(mlir::MLIRContext *ctx);

        static  llvm::StringRef getDialectNamespace() {
            return "mmadt";
        }

        void initialize() {
            addOperations<RepeatOp>();
        }
    };

    class RepeatOp : public mlir::Op<
                        RepeatOp,
                        mlir::OpTrait::ZeroOperands,
                        mlir::OpTrait::OneResult,
                        mlir::OpTrait::OneTypedResult<mlir::TensorType>::Impl> {
    public:
        using Op::Op;
        static llvm::StringRef getOperationName() { return "toy.constant"; }
        mlir::DenseElementsAttr getValue();
        mlir::LogicalResult verify();
        static void build(mlir::OpBuilder &builder, mlir::OperationState &state, mlir::Type result, mlir::DenseElementsAttr value);
        static void build(mlir::OpBuilder &builder, mlir::OperationState &state, mlir::DenseElementsAttr value);
        static void build(mlir::OpBuilder &builder, mlir::OperationState &state, double value);
    };
}}

fn main() {
    let name = std::ffi::CString::new("World").unwrap();
    let name_ptr = name.as_ptr();
    let r = unsafe {
        cpp!([name_ptr as "const char *"] -> u32 as "int32_t" {
            mlir::MLIRContext context;
            // context.getOrLoadDialect<MMADTDialect>();
            // mlir::OwningModuleRef module;

            std::cout << "Hello, 12" << name_ptr << std::endl;
            return 42;
        })
    };
    assert_eq!(r, 42)
}
