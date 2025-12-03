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

        bool fileMightContainThisPluginType (const juce::String& fileOrIdentifier) override
        {
            return AudioPluginFormatImpl::file_might_contain_this_plugin_type (_format, fileOrIdentifier);
        }

        juce::String getNameOfPluginFromIdentifier (const juce::String& fileOrIdentifier) override
        {
            return AudioPluginFormatImpl::get_name_of_plugin_from_identifier (_format, fileOrIdentifier);
        }

        bool pluginNeedsRescanning (const juce::PluginDescription& plugin) override
        {
            return AudioPluginFormatImpl::plugin_needs_rescanning (_format, plugin);
        }

        bool doesPluginStillExist (const juce::PluginDescription& plugin) override
        {
            return AudioPluginFormatImpl::does_plugin_still_exist (_format, plugin);
        }

        bool canScanForPlugins() const override
        {
            return AudioPluginFormatImpl::can_scan_for_plugins (_format);
        }

        bool isTrivialToScan() const override
        {
            return AudioPluginFormatImpl::is_trivial_to_scan (_format);
        }

        juce::StringArray searchPathsForPlugins (const juce::FileSearchPath& directories, bool recursive, bool allowAsync) override
        {
            return AudioPluginFormatImpl::search_paths_for_plugins (_format, directories, recursive, allowAsync);
        }

        juce::FileSearchPath getDefaultLocationsToSearch() override
        {
            return AudioPluginFormatImpl::get_default_locations_to_search (_format);
        }

        bool requiresUnblockedMessageThreadDuringCreation (const juce::PluginDescription& description) const override
        {
            return AudioPluginFormatImpl::requires_unblocked_message_thread_during_creation (_format, description);
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
            AudioPluginImpl::prepare_to_play (_plugin, sampleRate, maximumExpectedSamplesPerBlock);
        }

        void releaseResources() override
        {
            AudioPluginImpl::release_resources (_plugin);
        }

        void processBlock (juce::AudioBuffer<float>& buffer, juce::MidiBuffer& midiMessages) override
        {
            AudioPluginImpl::process_block (_plugin, buffer, midiMessages);
        }

        double getTailLengthSeconds() const override
        {
            return AudioPluginImpl::get_tail_length_seconds (_plugin);
        }

        bool acceptsMidi() const override
        {
            return AudioPluginImpl::accepts_midi (_plugin);
        }

        bool producesMidi() const override
        {
            return AudioPluginImpl::produces_midi (_plugin);
        }

        juce::AudioProcessorEditor* createEditor() override { return nullptr; }

        bool hasEditor() const override
        {
            return AudioPluginImpl::has_editor (_plugin);
        }

        int getNumPrograms() override
        {
            return AudioPluginImpl::get_num_programs (_plugin);
        }

        int getCurrentProgram() override
        {
            return AudioPluginImpl::get_current_program (_plugin);
        }

        void setCurrentProgram (int index) override
        {
            AudioPluginImpl::set_current_program (_plugin, index);
        }

        const juce::String getProgramName (int index) override
        {
            return AudioPluginImpl::get_program_name (_plugin, index);
        }

        void changeProgramName (int index, const juce::String& newName) override
        {
            AudioPluginImpl::change_program_name (_plugin, index, newName);
        }

        void getStateInformation (juce::MemoryBlock& destData) override { std::ignore = destData; }

        void setStateInformation (const void* data, int sizeInBytes) override
        {
            std::ignore = data;
            std::ignore = sizeInBytes;
        }

        void fillInPluginDescription (juce::PluginDescription& description) const override
        {
            AudioPluginImpl::fill_in_plugin_description (_plugin, description);
        }

        BoxDynAudioPlugin _plugin;
    };

    return std::make_unique<AudioPlugin> (std::move (plugin));
}

} // namespace cxx_juce
