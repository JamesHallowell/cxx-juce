#include <cxx_juce_bindings.h>

namespace cxx_juce::audio_io_device_type
{
rust::String getTypeName (const juce::AudioIODeviceType& audioIoDeviceType)
{
    return audioIoDeviceType.getTypeName().toStdString();
}

rust::Vec<rust::String> getInputDeviceNames (
    const juce::AudioIODeviceType& audioIoDeviceType)
{
    const auto deviceNames = audioIoDeviceType.getDeviceNames (true);

    rust::Vec<rust::String> result;
    std::transform (
        std::begin (deviceNames),
        std::end (deviceNames),
        std::back_inserter (result),
        [] (const auto& deviceName)
        { return deviceName.toStdString(); });
    return result;
}

rust::Vec<rust::String> getOutputDeviceNames (
    const juce::AudioIODeviceType& audioIoDeviceType)
{
    const auto deviceNames = audioIoDeviceType.getDeviceNames (false);

    rust::Vec<rust::String> result;
    result.reserve (static_cast<size_t> (deviceNames.size()));
    std::transform (
        std::begin (deviceNames),
        std::end (deviceNames),
        std::back_inserter (result),
        [] (const auto& deviceName)
        { return deviceName.toStdString(); });
    return result;
}

std::unique_ptr<juce::AudioIODevice> createDevice (
    juce::AudioIODeviceType& audioIoDeviceType,
    rust::Str inputDeviceName,
    rust::Str outputDeviceName)
{
    if (auto* device = audioIoDeviceType.createDevice (
            static_cast<std::string> (inputDeviceName),
            static_cast<std::string> (outputDeviceName)))
    {
        return std::unique_ptr<juce::AudioIODevice> (device);
    }

    return nullptr;
}
} // namespace cxx_juce::audio_io_device_type
