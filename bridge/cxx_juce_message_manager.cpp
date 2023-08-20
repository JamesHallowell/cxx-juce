#include "cxx_juce_bindings.h"

namespace cxx_juce::message_manager
{
juce::MessageManager* getInstanceWithoutCreating()
{
    return juce::MessageManager::getInstanceWithoutCreating();
}
} // namespace cxx_juce::message_manager