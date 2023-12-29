cmake_minimum_required(VERSION 3.22)
project(disasm VERSION 1.0 LANGUAGES C)

add_library(disasm STATIC)

target_compile_options(disasm PRIVATE
  "-march=native"
  "-ggdb3"
  "-m32"
  "-std=gnu99"
  "-fshort-wchar"
  "-Wno-multichar"
)

target_link_options(disasm PRIVATE
  "-m32"
)

target_compile_definitions(disasm PRIVATE -DHAVE_CONFIG_H)

target_include_directories(disasm PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})


file(GLOB Main ${CMAKE_CURRENT_SOURCE_DIR}/*.c)

target_sources(disasm
    PRIVATE
    ${Main}
)
