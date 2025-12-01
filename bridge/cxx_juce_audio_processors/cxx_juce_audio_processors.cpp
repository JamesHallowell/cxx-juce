#include "cxx_juce_audio_processors.h"

#include <cxx-juce/src/juce_audio_processors/plugin_description.rs.h>
#include <cxx-juce/src/juce_audio_processors/plugin_formats.rs.h>
#include <cxx-juce/src/juce_audio_processors/plugin_instance.rs.h>
#include <cxx_juce_utils.h>

CXX_JUCE_ASSERT_SIZE_ALIGN (AudioPluginFormatManager)

CXX_JUCE_ASSERT_SIZE_ALIGN (PluginDescription)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, name, NameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, descriptiveName, DescriptiveNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, pluginFormatName, PluginFormatNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, category, CategoryOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, manufacturerName, ManufacturerNameOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, version, VersionOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, fileOrIdentifier, FileOrIdentifierOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, lastFileModTime, LastFileModTimeOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, uniqueId, UniqueIdOffset)
CXX_JUCE_ASSERT_FIELD_OFFSET (PluginDescription, isInstrument, IsInstrumentOffset)

namespace cxx_juce
{
juce::String audioProcessorGetName (const juce::AudioProcessor& processor) noexcept
{
    return processor.getName();
}

CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioPluginFormat)
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (AudioPlugin)

std::unique_ptr<juce::AudioPluginFormat> wrap (BoxDynAudioPluginFormat format) noexcept
{
    struct AudioPluginFormat : juce::AudioPluginFormat
    {
        explicit AudioPluginFormat (BoxDynAudioPluginFormat format) noexcept
            : _format { std::move (format) }
        {
        }

        juce::String getName() const override
        {
            return AudioPluginFormatImpl::name (_format);
        }

        void findAllTypesForFile (juce::OwnedArray<juce::PluginDescription>& results, const juce::String& file) override
        {
            return AudioPluginFormatImpl::find_all_types_for_file (_format, results, file);
        }

        bool fileMightContainThisPluginType (const juce::String&) override
        {
            return {};
        }

        juce::String getNameOfPluginFromIdentifier (const juce::String&) override
        {
            return {};
        }

        bool pluginNeedsRescanning (const juce::PluginDescription&) override
        {
            return {};
        }

        bool doesPluginStillExist (const juce::PluginDescription&) override
        {
            return {};
        }

        bool canScanForPlugins() const override
        {
            return {};
        }

        bool isTrivialToScan() const override
        {
            return {};
        }

        juce::StringArray searchPathsForPlugins (const juce::FileSearchPath&, bool, bool) override
        {
            return {};
        }

        juce::FileSearchPath getDefaultLocationsToSearch() override
        {
            return {};
        }

        bool requiresUnblockedMessageThreadDuringCreation (const juce::PluginDescription&) const override
        {
            return {};
        }

    protected:
        void createPluginInstance (const juce::PluginDescription& description, double sampleRate, int bufferSize, PluginCreationCallback callback) override
        {
            auto instance = AudioPluginFormatImpl::create_plugin_instance (_format, description, sampleRate, bufferSize);
            callback (std::move (instance), juce::String {});
        }

    public:
        BoxDynAudioPluginFormat _format;
    };

    return std::make_unique<AudioPluginFormat> (std::move (format));
}

std::unique_ptr<juce::AudioPluginInstance> wrap (BoxDynAudioPlugin plugin) noexcept
{
    struct AudioPlugin : juce::AudioPluginInstance
    {
        explicit AudioPlugin (BoxDynAudioPlugin plugin)
            : _plugin { std::move (plugin) }
        {
        }
        const juce::String getName() const override { return AudioPluginImpl::get_name (_plugin); }
        void prepareToPlay (double sampleRate, int maximumExpectedSamplesPerBlock) override
        {
            std::ignore = sampleRate;
            std::ignore = maximumExpectedSamplesPerBlock;
        }
        void releaseResources() override {}
        void processBlock (juce::AudioBuffer<float>& buffer, juce::MidiBuffer& midiMessages) override
        {
            std::ignore = buffer;
            std::ignore = midiMessages;
        }
        double getTailLengthSeconds() const override { return 0.0; }
        bool acceptsMidi() const override { return false; }
        bool producesMidi() const override { return false; }
        juce::AudioProcessorEditor* createEditor() override { return nullptr; }
        bool hasEditor() const override { return false; }
        int getNumPrograms() override { return 0; }
        int getCurrentProgram() override { return 0; }
        void setCurrentProgram (int index) override { std::ignore = index; }
        const juce::String getProgramName (int index) override
        {
            std::ignore = index;
            return {};
        }
        void changeProgramName (int index, const juce::String& newName) override
        {
            std::ignore = index;
            std::ignore = newName;
        }
        void getStateInformation (juce::MemoryBlock& destData) override { std::ignore = destData; }
        void setStateInformation (const void* data, int sizeInBytes) override
        {
            std::ignore = data;
            std::ignore = sizeInBytes;
        }
        void fillInPluginDescription (juce::PluginDescription&) const override {}

        BoxDynAudioPlugin _plugin;
    };

    return std::make_unique<AudioPlugin> (std::move (plugin));
}

} // namespace cxx_juce
