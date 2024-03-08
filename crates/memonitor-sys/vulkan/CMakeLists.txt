cmake_minimum_required(VERSION 3.5)

project(memonitor-vk LANGUAGES C)

if (WIN32)
    set(VOLK_STATIC_DEFINES VK_USE_PLATFORM_WIN32_KHR)
endif ()

set(VULKAN_HEADERS_INSTALL_DIR "${CMAKE_CURRENT_SOURCE_DIR}/thirdparty/Vulkan-Headers")
add_subdirectory(thirdparty/volk)

option(MEMONITOR_VALIDATE "Use Vulkan validation layers" OFF)

add_library(memonitor-vk STATIC "include/memonitor.h" "src/memonitor.c")
target_include_directories(memonitor-vk PUBLIC "include")
target_link_libraries(memonitor-vk PRIVATE volk)

if (MEMONITOR_VALIDATE)
    target_compile_definitions(memonitor-vk PUBLIC USE_VALIDATION_LAYERS)
endif ()

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
