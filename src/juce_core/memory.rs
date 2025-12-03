pub(crate) use juce::LeakedObjectDetectorLayout;

#[doc(hidden)]
#[macro_export]
macro_rules! define_leak_detector {
    (
        $name:ident,
        cxx_name = $cxx_name:literal,
        drop = $drop:expr,
    ) => {
        $crate::define_juce_type! {
            $name,
            layout = $crate::juce_core::LeakedObjectDetectorLayout,
            cxx_name = $cxx_name,
            drop = $drop,
        }
    };
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum LeakedObjectDetectorLayout {
        Size = 1,
        Alignment = 1,
    }
}
