cmake_minimum_required(VERSION 3.5)

project(memonitor-cuda LANGUAGES C)

add_library(memonitor-cuda STATIC "include/memonitor.h" "src/memonitor.c")
target_include_directories(memonitor-cuda PUBLIC "include")

install(TARGETS memonitor-cuda
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
        ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
        RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
