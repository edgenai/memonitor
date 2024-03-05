cmake_minimum_required(VERSION 3.5)

project(memonitor-vk LANGUAGES C)

set(VULKAN_HEADERS_INSTALL_DIR "${CMAKE_CURRENT_SOURCE_DIR}/thirdparty/Vulkan-Headers")
add_subdirectory(thirdparty/volk)

add_library(memonitor-vk STATIC "include/memonitor.h" "src/memonitor.c")
target_include_directories(memonitor-vk PUBLIC "include")
target_link_libraries(memonitor-vk PUBLIC volk)

if (NOT VOLK_INSTALL)
    install(TARGETS volk
            LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
            ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
            RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
    )
endif ()

install(TARGETS memonitor-vk
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
        ARCHIVE DESTINATION ${CMAKE_INSTALL_LIBDIR}
        RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
