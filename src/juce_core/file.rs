use crate::{define_array_type, define_juce_type, JuceString};

define_juce_type! {
    File,
    layout = juce::FileLayout,
    cxx_name = "juce::File",
    default = juce::file_new,
    drop = juce::file_drop,
    equality = juce::file_equality,
}

impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.get_full_path_name())
    }
}

impl File {
    pub fn from_absolute_path(path: impl Into<JuceString>) -> Self {
        juce::from_absolute_path(&path.into())
    }
}

define_juce_type! {
    FileSearchPath,
    layout = juce::FileSearchPathLayout,
    cxx_name = "juce::FileSearchPath",
    default = juce::file_search_path_new,
    drop = juce::file_search_path_drop,
}

define_array_type! {
    FileSearchPath,
    File,
    index = juce::file_search_path_get,
}

#[cxx::bridge(namespace = "juce")]
mod juce {
    enum FileLayout {
        Size = 8,
        Alignment = 8,
    }

    enum FileSearchPathLayout {
        Size = 16,
        Alignment = 8,
    }

    unsafe extern "C++" {
        include!("cxx_juce.h");

        type File = super::File;
        type FileSearchPath = super::FileSearchPath;
        type JuceString = crate::JuceString;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn file_new() -> File;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn file_drop(self_: &mut File);

        #[namespace = "cxx_juce"]
        #[cxx_name = "eq"]
        fn file_equality(self_: &File, other: &File) -> bool;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn from_absolute_path(path: &JuceString) -> File;

        #[cxx_name = "getFullPathName"]
        fn get_full_path_name(self: &File) -> &JuceString;

        #[namespace = "cxx_juce"]
        #[cxx_name = "construct"]
        fn file_search_path_new() -> FileSearchPath;

        #[namespace = "cxx_juce"]
        #[cxx_name = "drop"]
        fn file_search_path_drop(self_: &mut FileSearchPath);

        #[cxx_name = "getNumPaths"]
        fn size(self: &FileSearchPath) -> i32;

        #[namespace = "cxx_juce"]
        #[cxx_name = "index"]
        fn file_search_path_get(self_: &FileSearchPath, index: i32) -> File;

        #[cxx_name = "addIfNotAlreadyThere"]
        fn add(self: &mut FileSearchPath, file: &File) -> bool;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_file_search_path() {
        let mut search_path = FileSearchPath::default();
        search_path.add(&File::from_absolute_path("/foo/bar"));
        search_path.add(&File::from_absolute_path("/bar/foo"));

        assert_eq!(search_path.size(), 2);

        assert_eq!(search_path.get(0), File::from_absolute_path("/foo/bar"));
    }
}
