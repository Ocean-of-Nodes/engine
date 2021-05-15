# Engine

The main current goal of the project development of Universal General Representation. 
For replacement dissimilar internal api of rust analyzer. 

### Build 
First, install and build MLIR following the [instruction](https://mlir.llvm.org/getting_started/). 
Next, set env vars that containing path to libs and call cargo  
```
export LLVM_DIR=/home/mrsmith/Desktop/code_experiments/llvm-project/build/lib/cmake/llvm/ MLIR_DIR=/home/mrsmith/Desktop/code_experiments/llvm-project/build/lib/cmake/mlir/ & cargo run
```

### Motivation 
You may notice that the project is similar to a graph database. 
In general, it is so. Ok, but why we need new graph database? 

### Development agreements

We must strive to cover the current capabilities of the rast analyzer.
The main goal is to get a convenient external API and to satisfy the functionality requirements. The internals here and there can be ugly and bizarre.
This is required to move to the stage of integration and discussion.

### Road map
- Top level query builder for Gremlin dialect 
- mm-adt as internal language 
- MLIR backend
- Integration with RA 

### Long-term goals
- GPU acceleration
- Pluggable storages
- Materialized view

### Long term prospects
- Using as a starting point as a single uniform representation for unioning rust compiler and analyzer
- As an external api for any kind of RA or compiler plugins

### Interesting opening opportunities
- User queries - user can write is own queries, they can replace regular text search or regular expression search 
- Now the language is present function like macros, they have some limitations as they work at the token level. We could pass to them on the direct handle of the compiler UGR engine. 
In this case, they would turn into something similar to compiler plugins, for example, they would have access to types, etc. It seems really like new level of metaprogramming but that requires discussion and elaboration.

### Areas that can be explored separately from the project
- Compilation as service - compilation happens continuously while you write code
- Various type of visualization
- Run partially valid code (not finished code) and step by step add pieces and continue execution
- Universal schema and convertor. [Dragon](https://eng.uber.com/dragon-schema-integration-at-uber-scale/) makes an effort in this direction, but we don't know, will Uber make the project public or not. This would be a major improvement for people who engaged in date mining or wont extract data in a special format



