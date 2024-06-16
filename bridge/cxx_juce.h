#pragma once

#include "cxx_juce_audio_basics/cxx_juce_iir_filter.h"
#include "cxx_juce_audio_devices/cxx_juce_audio_callback_wrapper.h"
#include "cxx_juce_audio_devices/cxx_juce_audio_device_manager.h"
#include "cxx_juce_audio_devices/cxx_juce_audio_device_setup.h"
#include "cxx_juce_audio_devices/cxx_juce_audio_io_device.h"
#include "cxx_juce_audio_devices/cxx_juce_audio_io_device_type.h"
#include "cxx_juce_audio_devices/cxx_juce_system_audio_volume.h"

#include <juce_audio_devices/juce_audio_devices.h>
#include <juce_core/juce_core.h>
#include <juce_events/juce_events.h>
#include <rust/cxx.h>

namespace juce
{
using AudioIODeviceTypeArray = OwnedArray<juce::AudioIODeviceType>;
void initialiseNSApplication();
} // namespace juce

namespace cxx_juce
{
struct BoxedAudioIODeviceCallback;
struct BoxedAudioIODeviceType;

rust::String juceVersion();

void initialiseJuce();
void shutdownJuce();

rust::Str toStr (const juce::String& string);

} // namespace cxx_juce
