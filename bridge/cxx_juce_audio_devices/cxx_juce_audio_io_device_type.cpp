#include "cxx_juce_audio_io_device_type.h"

#include <cxx-juce/src/juce_audio_devices/device_type.rs.h>
#include <cxx-juce/src/juce_audio_devices/mod.rs.h>

namespace cxx_juce
{
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioDeviceType)

std::unique_ptr<juce::AudioIODeviceType> wrap (BoxDynAudioDeviceType deviceType) noexcept
{
    struct AudioIODeviceType : juce::AudioIODeviceType
    {
        explicit AudioIODeviceType (BoxDynAudioDeviceType deviceType)
            : juce::AudioIODeviceType (
                  static_cast<std::string> (AudioDeviceTypeImpl::name (deviceType)))
            , _deviceType { std::move (deviceType) }
        {
        }

        void scanForDevices() override
        {
            AudioDeviceTypeImpl::scan_for_devices (_deviceType);
        }

        [[nodiscard]] juce::StringArray getDeviceNames (bool wantInputNames) const override
        {
            const auto names = wantInputNames ? AudioDeviceTypeImpl::input_devices (_deviceType) : AudioDeviceTypeImpl::output_devices (_deviceType);

            juce::StringArray stringArray;
            for (const auto& name : names)
            {
                stringArray.add (static_cast<std::string> (name));
            }
            return stringArray;
        }

        [[nodiscard]] int getDefaultDeviceIndex (bool /*forInput*/) const override
        {
            return 0;
        }

        int getIndexOfDevice (juce::AudioIODevice* device,
                              bool asInput) const override
        {
            return getDeviceNames (asInput).indexOf (device->getName());
        }

        [[nodiscard]] bool hasSeparateInputsAndOutputs() const override
        {
            return true;
        }

        juce::AudioIODevice* createDevice (const juce::String& inputDeviceName,
                                           const juce::String& outputDeviceName) override
        {
            try
            {
                return AudioDeviceTypeImpl::create_device (_deviceType, inputDeviceName, outputDeviceName).release();
            }
            catch (const rust::Error&)
            {
                return nullptr;
            }
        }

        BoxDynAudioDeviceType _deviceType;
    };

    return std::make_unique<AudioIODeviceType> (std::move (deviceType));
}
} // namespace cxx_juce
