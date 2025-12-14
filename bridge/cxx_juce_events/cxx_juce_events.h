#pragma once

#include <juce_events/juce_events.h>

#include <cxx_juce_utils.h>

namespace cxx_juce
{
CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE_CUSTOM(CallAsyncCallback)
CXX_JUCE_DECLARE_BOXED_TRAIT_TYPE(JuceApplicationBase, juce::JUCEApplicationBase)

void callAsync(BoxDynCallAsyncCallback callback);
bool hasStopMessageBeenSent();
int runApp();
BoxDynJuceApplicationBase* castAppInstance();
void startAppTimer(int timerId, int interval);
void stopAppTimer(int timerId);
void runDispatchLoop();
void stopDispatchLoop();
} // namespace cxx_juce
