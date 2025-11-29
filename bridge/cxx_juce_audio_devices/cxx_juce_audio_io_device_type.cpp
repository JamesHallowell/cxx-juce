#include "cxx_juce_audio_io_device_type.h"

#include <cxx-juce/src/juce_audio_devices/device_type.rs.h>
#include <cxx-juce/src/juce_audio_devices/mod.rs.h>

namespace cxx_juce
{
void DropBoxDynAudioIODeviceType::operator() (BoxDynAudioIODeviceType* deviceType) const
{
    BoxDynAudioIODeviceTypeImpl::drop (deviceType);
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
                return BoxDynAudioIODeviceTypeImpl::create_device (_deviceType, inputDeviceName, outputDeviceName).release();
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
