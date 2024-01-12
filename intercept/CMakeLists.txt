cmake_minimum_required(VERSION 3.18)
project(intercept VERSION 1.0 LANGUAGES C)

add_subdirectory(libdisasm)

add_library(intercept STATIC)

target_compile_options(intercept PRIVATE
  "-O0"
  "-march=native"
  "-ggdb3"
  "-m32"
  "-std=gnu99"
  "-fshort-wchar"
  "-Wno-multichar"
)

target_link_options(intercept PRIVATE
  "-m32"
)

target_compile_definitions(intercept PRIVATE -DHAVE_CONFIG_H)

target_include_directories(intercept PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

target_sources(intercept
    PRIVATE
    hook.c
)

target_link_libraries(intercept disasm)
install (TARGETS intercept DESTINATION .)
