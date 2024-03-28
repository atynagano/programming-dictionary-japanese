use programming_dictionary_japanese::Vocabulary;

fn main() {
    println!("{:?}", std::env::current_dir());
    std::fs::copy("public/dict.toml", "public/cache.toml").unwrap();
    let data = std::fs::read_to_string("public/dict.toml").unwrap();
    let mut data = toml::from_str::<Vocabulary>(&data).map_err(|e| panic!("{}", e)).unwrap();

    if false {
        let cloned = data.dictionary.clone();
        for (key, word) in &cloned {
            if word.enabled() {
                if let Some(w) = data.dictionary.get_mut(&format!("{key}s")) {
                    if w.enabled() {
                        w.singular.get_or_insert(key.clone());
                    }
                }
                if let Some(w) = data.dictionary.get_mut(&format!("{key}es")) {
                    if w.enabled() {
                        w.singular.get_or_insert(key.clone());
                    }
                }
            }
        }
    }
    {
        let cloned = data.dictionary.clone();
        for (key, word) in &cloned {
            for unshortened in word.unshortened.iter().flatten() {
                if let Some(w) = data.dictionary.get_mut(unshortened) {
                    let shortened = w.shortened.get_or_insert(vec![]);
                    if !shortened.contains(key) {
                        shortened.push(key.clone());
                    }
                }
            }
        }
    }
    {
        let cloned = data.dictionary.clone();
        for (key, word) in &cloned {
            for shortened in word.shortened.iter().flatten() {
                if let Some(w) = data.dictionary.get_mut(shortened) {
                    let unshortened = w.unshortened.get_or_insert(vec![]);
                    if !unshortened.contains(key) {
                        unshortened.push(key.clone());
                    }
                }
            }
        }
    }
    {
        let cloned = data.dictionary.clone();
        for (key, word) in &cloned {
            if let Some(singular) = &word.singular {
                if let Some(w) = data.dictionary.get_mut(singular) {
                    w.plural.get_or_insert_with(|| key.clone());
                }
            }
        }
    }
    {
        let cloned = data.dictionary.clone();
        for (key, word) in &cloned {
            if let Some(plural) = &word.plural {
                if let Some(w) = data.dictionary.get_mut(plural) {
                    w.singular.get_or_insert_with(|| key.clone());
                }
            }
        }
    }
    {
        let cloned = data.dictionary.clone();
        for (key, word) in &cloned {
            for antonym in word.antonym.iter().flatten() {
                if let Some(w) = data.dictionary.get_mut(antonym) {
                    let others = w.antonym.get_or_insert(vec![]);
                    if !others.contains(key) {
                        others.push(key.clone());
                    }
                }
            }
        }
    }
    {
        let data = toml::to_string_pretty(&data).unwrap();
        std::fs::write("public/dict.toml", data).unwrap();
    }
    {
        data.dictionary.retain(|_, w| !w.disabled.unwrap_or_default());
        let data_min = toml::to_string(&data).unwrap();
        std::fs::write("public/dict.min.toml", data_min).unwrap();
    }
}