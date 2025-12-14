#include <cxx_juce_events/cxx_juce_events.h>

#include <cxx-juce/src/juce_events/application.rs.h>
#include <cxx-juce/src/juce_events/message_manager.rs.h>
#include <cxx_juce_utils.h>

namespace cxx_juce
{
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (CallAsyncCallback)
CXX_JUCE_DEFINE_BOXED_TRAIT_TYPE (JuceApplicationBase)
} // namespace cxx_juce

namespace
{
struct JuceApplicationBase : juce::JUCEApplicationBase, juce::MultiTimer
{
    explicit JuceApplicationBase (cxx_juce::BoxDynJuceApplicationBase app)
        : _app { std::move (app) }
    {
    }

    static JuceApplicationBase* get()
    {
        return dynamic_cast<JuceApplicationBase*> (juce::JUCEApplicationBase::getInstance());
    }

    const juce::String getApplicationName() override
    {
        return cxx_juce::JuceApplicationBaseImpl::getApplicationName (_app);
    }

    const juce::String getApplicationVersion() override
    {
        return cxx_juce::JuceApplicationBaseImpl::getApplicationVersion (_app);
    }

    void initialise (const juce::String& commandLine) override
    {
        cxx_juce::JuceApplicationBaseImpl::initialise (_app, commandLine);
    }

    void shutdown() override
    {
        cxx_juce::JuceApplicationBaseImpl::shutdown (_app);
    }

    void systemRequestedQuit() override
    {
        cxx_juce::JuceApplicationBaseImpl::systemRequestedQuit (_app);
    }

    bool moreThanOneInstanceAllowed() override
    {
        return cxx_juce::JuceApplicationBaseImpl::moreThanOneInstanceAllowed (_app);
    }

    void anotherInstanceStarted (const juce::String& commandLine) override
    {
        cxx_juce::JuceApplicationBaseImpl::anotherInstanceStarted (_app, commandLine);
    }

    void suspended() override
    {
        cxx_juce::JuceApplicationBaseImpl::suspended (_app);
    }

    void resumed() override
    {
        cxx_juce::JuceApplicationBaseImpl::resumed (_app);
    }

    void unhandledException (const std::exception*, const juce::String&, int) override
    {
        cxx_juce::JuceApplicationBaseImpl::unhandledException (_app);
    }

    void timerCallback (int timerID) override
    {
        cxx_juce::JuceApplicationBaseImpl::timerCallback (_app, timerID);
    }

    cxx_juce::BoxDynJuceApplicationBase _app;
};
} // namespace

namespace cxx_juce
{
void callAsync (BoxDynCallAsyncCallback callback)
{
    auto callbackPtr = std::make_shared<BoxDynCallAsyncCallback> (std::move (callback));

    juce::MessageManager::callAsync ([callbackPtr]()
                                     { CallAsyncCallbackImpl::call (*callbackPtr); });
}

bool hasStopMessageBeenSent()
{
    const auto* instance = juce::MessageManager::getInstanceWithoutCreating();
    return instance ? instance->hasStopMessageBeenSent() : true;
}

std::unique_ptr<juce::JUCEApplicationBase> wrap (BoxDynJuceApplicationBase app) noexcept
{
    juce::JUCEApplicationBase::createInstance = []() -> juce::JUCEApplicationBase*
    {
        return nullptr;
    };
    return std::make_unique<JuceApplicationBase> (std::move (app));
}

int runApp()
{
    juce::JUCEApplicationBase::createInstance = []
    {
        return createInstance().release();
    };
    return juce::JUCEApplicationBase::main();
}

BoxDynJuceApplicationBase* castAppInstance()
{
    if (auto* ptr = JuceApplicationBase::get(); ptr != nullptr)
    {
        return std::addressof (ptr->_app);
    }

    return nullptr;
}

void startAppTimer (int timerId, int interval)
{
    if (auto* ptr = JuceApplicationBase::get(); ptr != nullptr)
    {
        return ptr->startTimer (timerId, interval);
    }
}

void stopAppTimer (int timerId)
{
    if (auto* ptr = JuceApplicationBase::get(); ptr != nullptr)
    {
        return ptr->stopTimer (timerId);
    }
}

void runDispatchLoop()
{
    if (auto* instance = juce::MessageManager::getInstanceWithoutCreating())
    {
        instance->runDispatchLoop();
    }
}

void stopDispatchLoop()
{
    if (auto* instance = juce::MessageManager::getInstanceWithoutCreating())
    {
        instance->stopDispatchLoop();
    }
}
} // namespace cxx_juce
