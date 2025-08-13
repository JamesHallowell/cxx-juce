cmake_minimum_required(VERSION 3.15)

project(cxx-juce VERSION 0.1)

set(CXX_JUCE_BINDINGS_DIR "" CACHE PATH "Path to the bindings directory")
set(CXX_JUCE_USE_ASIO OFF CACHE BOOL "Use ASIO")
set(CXX_JUCE_ASIO_SDK_DIR "" CACHE PATH "Path to the ASIO SDK directory")
set(CXX_JUCE_VERSION_OF_JUCE 7.0.12)

# Add option to use system-provided JUCE source
set(CXX_JUCE_SYSTEM_JUCE_SOURCE "" CACHE PATH "Path to pre-fetched JUCE source directory")

set(JUCE_MODULES_ONLY ON)

# First try to find installed JUCE
find_package(JUCE QUIET COMPONENTS 
    juce_audio_basics
    juce_audio_devices
    juce_core
    juce_events
)

if(JUCE_FOUND)
    message(STATUS "Found installed JUCE")
elseif(CXX_JUCE_SYSTEM_JUCE_SOURCE)
    # Use system-provided JUCE source
    message(STATUS "Using system-provided JUCE source at ${CXX_JUCE_SYSTEM_JUCE_SOURCE}")
    add_subdirectory(${CXX_JUCE_SYSTEM_JUCE_SOURCE} ${CMAKE_BINARY_DIR}/juce-build)
else()
    # Fetch JUCE from GitHub (original behavior)
    include(FetchContent)
    set(JUCE_URL https://github.com/juce-framework/JUCE/archive/refs/tags/${CXX_JUCE_VERSION_OF_JUCE}.tar.gz)
    FetchContent_Declare(
        JUCE
        URL ${JUCE_URL}
        URL_HASH SHA256=94e3b35e1990cd67f59736bb5e1eff1cfd9590b16fea82696f309a8f59452434
    )
    message(STATUS "Fetching JUCE ${CXX_JUCE_VERSION_OF_JUCE}...")
    FetchContent_MakeAvailable(JUCE)
endif()

add_library(cxx-juce STATIC)

target_compile_features(cxx-juce
    PUBLIC
        cxx_std_17
)

target_compile_definitions(cxx-juce
    PUBLIC
        JUCE_USE_CURL=0
)

message(STATUS "Using bindings at ${CXX_JUCE_BINDINGS_DIR}")

target_include_directories(cxx-juce
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CXX_JUCE_BINDINGS_DIR}/crate
        ${CXX_JUCE_BINDINGS_DIR}/include
        ${CXX_JUCE_BINDINGS_DIR}/sources
)

target_sources(cxx-juce
    PRIVATE
        cxx_juce.h
        cxx_juce.cpp
        cxx_juce_bindings.h
        cxx_juce_bindings.cpp
        cxx_juce_audio_basics/cxx_juce_iir_filter.h
        cxx_juce_audio_basics/cxx_juce_iir_filter.cpp
        cxx_juce_audio_devices/cxx_juce_audio_callback_wrapper.h
        cxx_juce_audio_devices/cxx_juce_audio_callback_wrapper.cpp
        cxx_juce_audio_devices/cxx_juce_audio_device_setup.h
        cxx_juce_audio_devices/cxx_juce_audio_device_setup.cpp
        cxx_juce_audio_devices/cxx_juce_audio_device_manager.h
        cxx_juce_audio_devices/cxx_juce_audio_device_manager.cpp
        cxx_juce_audio_devices/cxx_juce_system_audio_volume.h
        cxx_juce_audio_devices/cxx_juce_system_audio_volume.cpp
        cxx_juce_audio_devices/cxx_juce_audio_io_device_type.h
        cxx_juce_audio_devices/cxx_juce_audio_io_device_type.cpp
        cxx_juce_audio_devices/cxx_juce_audio_io_device.h
        cxx_juce_audio_devices/cxx_juce_audio_io_device.cpp
)

target_link_libraries(cxx-juce
    PUBLIC
        juce::juce_audio_basics
        juce::juce_audio_devices
        juce::juce_core
        juce::juce_events
        juce::juce_recommended_config_flags
        juce::juce_recommended_warning_flags
)

if (CXX_JUCE_USE_ASIO)
    message(STATUS "Using ASIO SDK at ${CXX_JUCE_ASIO_SDK_DIR}")

    target_compile_definitions(cxx-juce
    PUBLIC
        JUCE_ASIO=1
    )

    target_include_directories(cxx-juce
    PRIVATE
        ${CXX_JUCE_ASIO_SDK_DIR}/common
    )
endif()