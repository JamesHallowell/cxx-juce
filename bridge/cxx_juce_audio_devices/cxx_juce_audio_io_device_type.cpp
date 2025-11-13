#include "cxx_juce_audio_io_device_type.h"

#include <cxx-juce/src/juce_audio_devices/device_type.rs.h>
#include <cxx-juce/src/juce_audio_devices/mod.rs.h>

namespace cxx_juce
{
BoxDynAudioIODeviceType::BoxDynAudioIODeviceType (BoxDynAudioIODeviceType&& other) noexcept
    : _repr { other._repr }
{
    other._repr = { 0, 0 };
}

BoxDynAudioIODeviceType::~BoxDynAudioIODeviceType() noexcept
{
    if (_repr != FatPtr { 0, 0 })
    {
        BoxDynAudioIODeviceTypeImpl::drop (this);
    }
}

std::unique_ptr<juce::AudioIODeviceType> wrapAudioDeviceType (BoxDynAudioIODeviceType deviceType)
{
    struct AudioIODeviceType : juce::AudioIODeviceType
    {
        explicit AudioIODeviceType (BoxDynAudioIODeviceType deviceType)
            : juce::AudioIODeviceType (
                  static_cast<std::string> (BoxDynAudioIODeviceTypeImpl::name (deviceType)))
            , _deviceType { std::move (deviceType) }
        {
        }

        void scanForDevices() override
        {
            BoxDynAudioIODeviceTypeImpl::scan_for_devices (_deviceType);
        }

        [[nodiscard]] juce::StringArray getDeviceNames (bool wantInputNames) const override
        {
            const auto names = BoxDynAudioIODeviceTypeImpl::get_device_names (_deviceType, wantInputNames);

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
                auto box = BoxDynAudioIODeviceTypeImpl::create_device (_deviceType, inputDeviceName, outputDeviceName);
                return wrapAudioDevice (std::move (box)).release();
            }
            catch (const rust::Error&)
            {
                return nullptr;
            }
        }

        BoxDynAudioIODeviceType _deviceType;
    };

    return std::make_unique<AudioIODeviceType> (std::move (deviceType));
}
} // namespace cxx_juce
