cmake_minimum_required(VERSION 3.22)
project(loadlibrary VERSION 1.0)

add_subdirectory(peloader)
add_subdirectory(intercept)

add_executable(mpclient mpclient.c)

target_compile_options(mpclient PRIVATE
  "-march=native"
  "-ggdb3"
  "-m32"
  "-std=gnu99"
  "-fshort-wchar"
  "-Wno-multichar"
  "-mstackrealign"
)

target_link_options(mpclient PRIVATE
  "-m32"
  "-lm"
)

target_compile_definitions(mpclient PRIVATE -D_GNU_SOURCE)

target_include_directories(mpclient PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/include)
target_link_libraries(mpclient peloader intercept)
