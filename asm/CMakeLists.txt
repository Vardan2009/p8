cmake_minimum_required(VERSION 3.10)
project(p8asm)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

add_executable(p8asm
	src/main.cpp
	src/def.h
	src/def.cpp
	src/util.h
	src/util.cpp
	src/symtable.h
	src/symtable.cpp
	src/expr.h
	src/expr.cpp
	src/encoder.h
	src/preprocessor.h
	src/preprocessor.cpp
	src/assembler.h
	src/assembler.cpp
)