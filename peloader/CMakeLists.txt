cmake_minimum_required(VERSION 3.22)
project(peloader VERSION 1.0 LANGUAGES C)

add_library(peloader STATIC)

target_compile_options(peloader PRIVATE
  "-march=native"
  "-ggdb3"
  "-m32"
  "-std=gnu99"
  "-fshort-wchar"
  "-Wno-multichar"
  "-mstackrealign"
)

target_link_options(peloader PRIVATE
  "-m32"
  "-lm"
)

target_compile_definitions(peloader PRIVATE -D_GNU_SOURCE)

target_include_directories(peloader PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/winapi ${CMAKE_CURRENT_SOURCE_DIR})


file(GLOB Winapi ${CMAKE_CURRENT_SOURCE_DIR}/winapi/*.c)
file(GLOB Main ${CMAKE_CURRENT_SOURCE_DIR}/*.c)

target_sources(peloader
    PRIVATE
    ${Main}
    ${Winapi}
)
