cmake_minimum_required(VERSION 3.15)

project(cxx-juce VERSION 0.1)

set(CXX_JUCE_BINDINGS_DIR "${CMAKE_CURRENT_SOURCE_DIR}/../target/cxxbridge" CACHE PATH "Path to the bindings directory")
set(CXX_JUCE_BRIDGE_FILES "" CACHE STRING "A list of bridge files to build")

set(CXX_JUCE_USE_ASIO OFF CACHE BOOL "Use ASIO")
set(CXX_JUCE_ASIO_SDK_DIR "" CACHE PATH "Path to the ASIO SDK directory")
set(CXX_JUCE_VERSION_OF_JUCE 7.0.12)

set(JUCE_MODULES_ONLY ON)

include(FetchContent)
set(JUCE_URL https://github.com/juce-framework/JUCE/archive/refs/tags/${CXX_JUCE_VERSION_OF_JUCE}.tar.gz)
FetchContent_Declare(
    JUCE
    URL ${JUCE_URL}
    URL_HASH SHA256=94e3b35e1990cd67f59736bb5e1eff1cfd9590b16fea82696f309a8f59452434
)
message(STATUS "Fetching JUCE ${CXX_JUCE_VERSION_OF_JUCE}...")
FetchContent_MakeAvailable(JUCE)

add_library(cxx-juce STATIC)

target_compile_features(cxx-juce
    PUBLIC
        cxx_std_20
)

target_compile_definitions(cxx-juce
    PUBLIC
        JUCE_USE_CURL=0
        JUCE_WEB_BROWSER=0
        JUCE_CHECK_MEMORY_LEAKS=0
        JUCE_PLUGINHOST_AU=1
)

message(STATUS "Using bindings at ${CXX_JUCE_BINDINGS_DIR}")

target_include_directories(cxx-juce
    PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CXX_JUCE_BINDINGS_DIR}
)

target_sources(cxx-juce
    PRIVATE
        cxx_juce.h
        cxx_juce_audio_basics/cxx_juce_audio_basics.cpp
        cxx_juce_audio_basics/cxx_juce_audio_basics.h
        cxx_juce_audio_devices/cxx_juce_audio_devices.cpp
        cxx_juce_audio_devices/cxx_juce_audio_devices.h
        cxx_juce_audio_processors/cxx_juce_audio_processors.cpp
        cxx_juce_audio_processors/cxx_juce_audio_processors.h
        cxx_juce_core/cxx_juce_core.cpp
        cxx_juce_core/cxx_juce_core.h
        cxx_juce_events/cxx_juce_events.cpp
        cxx_juce_events/cxx_juce_events.h
        cxx_juce_utils.h
        ${CXX_JUCE_BRIDGE_FILES}

)

target_link_libraries(cxx-juce
    PUBLIC
        juce::juce_audio_basics
        juce::juce_audio_devices
        juce::juce_core
        juce::juce_events
        juce::juce_audio_processors
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

if (MSVC)
    set_source_files_properties(${CXX_JUCE_BRIDGE_FILES} PROPERTIES COMPILE_FLAGS "/w")
else()
    set_source_files_properties(${CXX_JUCE_BRIDGE_FILES} PROPERTIES COMPILE_FLAGS "-w")
endif()
