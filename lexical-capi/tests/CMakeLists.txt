# CONFIG
cmake_minimum_required(VERSION 2.6)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
project(lexical_test)
enable_testing()

# GTEST
find_package(GTest REQUIRED)
include_directories(${GTEST_INCLUDE_DIR})
# Gtest requires a threads library.
find_package(Threads REQUIRED)

# LEXICAL
set(RELEASE_DIR "${PROJECT_SOURCE_DIR}/../target/release")
set(LIBLEXICAL "${CMAKE_SHARED_LIBRARY_PREFIX}lexical_capi${CMAKE_SHARED_LIBRARY_SUFFIX}")
add_library(lexical SHARED IMPORTED)
set_target_properties(lexical PROPERTIES IMPORTED_LOCATION "${RELEASE_DIR}/${LIBLEXICAL}")

# Check for symbols within the lexical library.
# These aren't functions, but...
# check_symbol_exists fails, which check_function_exists works.
include(CheckFunctionExists)
list(APPEND CMAKE_REQUIRED_LIBRARIES lexical)
check_function_exists(LEXICAL_HAVE_FORMAT HAVE_FORMAT)
check_function_exists(LEXICAL_HAVE_RADIX HAVE_RADIX)
check_function_exists(LEXICAL_HAVE_ROUNDING HAVE_ROUNDING)

# EXECUTABLES
add_executable(lexical_c_test
    test_c.cc
)

add_executable(lexical_cpp_test
    test_cc.cc
)

# DEFINITIONS
if(HAVE_FORMAT)
    target_compile_definitions(lexical_c_test PRIVATE HAVE_FORMAT=1)
    target_compile_definitions(lexical_cpp_test PRIVATE HAVE_FORMAT=1)
endif()

if(HAVE_RADIX)
    target_compile_definitions(lexical_c_test PRIVATE HAVE_RADIX=1)
    target_compile_definitions(lexical_cpp_test PRIVATE HAVE_RADIX=1)
endif()

if(HAVE_ROUNDING)
    target_compile_definitions(lexical_c_test PRIVATE HAVE_ROUNDING=1)
    target_compile_definitions(lexical_cpp_test PRIVATE HAVE_ROUNDING=1)
endif()

# INCLUDE
target_include_directories(lexical_c_test PRIVATE "${PROJECT_SOURCE_DIR}/../include")
target_include_directories(lexical_cpp_test PRIVATE "${PROJECT_SOURCE_DIR}/../include")

# LINK
target_link_libraries(lexical_c_test lexical gtest gtest_main ${CMAKE_THREAD_LIBS_INIT})
target_link_libraries(lexical_cpp_test lexical gtest gtest_main ${CMAKE_THREAD_LIBS_INIT})

# RUN
add_test(
    NAME lexical_c_test
    COMMAND lexical_c_test
)

add_test(
    NAME lexical_cpp_test
    COMMAND lexical_cpp_test
)
