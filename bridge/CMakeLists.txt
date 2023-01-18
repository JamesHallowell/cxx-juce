cmake_minimum_required(VERSION 3.15)

project(cxx-juce VERSION 0.1)

set(CXX_JUCE_BINDINGS_DIR "" CACHE PATH "Path to the bindings directory")
set(CXX_JUCE_USE_ASIO OFF CACHE BOOL "Use ASIO")
set(CXX_JUCE_ASIO_SDK_DIR "" CACHE PATH "Path to the ASIO SDK directory")

include(FetchContent)

set(CXX_JUCE_VERSION_OF_JUCE 7.0.5)

FetchContent_Declare(
    JUCE
    GIT_REPOSITORY https://github.com/juce-framework/JUCE.git
    GIT_TAG ${CXX_JUCE_VERSION_OF_JUCE}
)

message(STATUS "Fetching JUCE ${CXX_JUCE_VERSION_OF_JUCE}...")
FetchContent_MakeAvailable(JUCE)

add_library(cxx-juce STATIC
    cxx_juce.cpp
)

_juce_initialise_target(cxx-juce)

target_compile_features(cxx-juce
    PUBLIC
        cxx_std_17
)

message(STATUS "Using bindings at ${CXX_JUCE_BINDINGS_DIR}")

target_include_directories(cxx-juce
    PRIVATE
        ${CXX_JUCE_BINDINGS_DIR}/include
        ${CXX_JUCE_BINDINGS_DIR}/crate
        ${CXX_JUCE_BINDINGS_DIR}/sources
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