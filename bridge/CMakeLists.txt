cmake_minimum_required(VERSION 3.15)

project(cxx-juce VERSION 0.1)

set(CXX_JUCE_BINDINGS_DIR "" CACHE PATH "Path to the bindings directory")
set(CXX_JUCE_USE_ASIO OFF CACHE BOOL "Use ASIO")
set(CXX_JUCE_ASIO_SDK_DIR "" CACHE PATH "Path to the ASIO SDK directory")

include(FetchContent)

set(CXX_JUCE_VERSION_OF_JUCE 7.0.12)

FetchContent_Declare(
    JUCE
    GIT_REPOSITORY https://github.com/juce-framework/JUCE.git
    GIT_TAG ${CXX_JUCE_VERSION_OF_JUCE}
)

message(STATUS "Fetching JUCE ${CXX_JUCE_VERSION_OF_JUCE}...")
FetchContent_MakeAvailable(JUCE)

add_library(cxx-juce STATIC)

_juce_initialise_target(cxx-juce)

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
        cxx_juce_audio_devices/cxx_juce_audio_callback_handle.h
        cxx_juce_audio_devices/cxx_juce_audio_callback_handle.cpp
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
