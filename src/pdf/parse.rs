use regex::Regex;
use std::collections::{BTreeMap, HashMap};


// Returns an unsorted HashMap containing the word (string) and number of instances (usize).
#[deprecated = "The function better_parse_for_words has a better word detection algorithim. It is recommended to use that function instead of this one."]
pub fn parse_for_words(data: &String) -> HashMap<String, usize> {
    // make the string all lowercase for better word detection
    let fixed_data = data.to_ascii_lowercase();

    let mut word_counts: HashMap<String, usize> = HashMap::new();

    let words: Vec<&str> = fixed_data.split_whitespace().collect();

    for word in words {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }

    word_counts
}

// Uses regex and splits by whitespace to find words.
// Returns an unsorted HashMap containing the word (string) and number of isntances (usize)
// you know, when i first started i didnt realize a tuple vector could also work... oh well!
pub fn regex_parse_for_words(data: &str) -> HashMap<String, usize> {
    let fixed_data = data.to_ascii_lowercase();
    let re = Regex::new(r"[^\w-]+").unwrap(); 
    let cleaned_data = re.replace_all(&fixed_data, " "); 

    let mut word_counts: HashMap<String, usize> = HashMap::new();

    let words: Vec<&str> = cleaned_data.split_whitespace().collect();

    for word in words {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }
    word_counts
}

// Returns a sorted vector from most instances to least instances for the words.
pub fn sort_by_instances(data: HashMap<String, usize>) -> Vec<(String, usize)> {
    let mut sorted_entries: Vec<(String, usize)> = data.into_iter().collect();
    sorted_entries.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_entries
}

// the order of the vector is not the same as the hashmap, but it's not sorted so it's negligible
// FIXME: does NOT work as intended.
pub fn to_btreemap(data: Vec<(String, usize)>) -> BTreeMap<usize, String> {
    let mut entries: BTreeMap<usize, String> = BTreeMap::new();
    for (word, count) in data {
        entries.insert(count, word);
    }
    entries
}

