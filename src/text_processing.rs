use std::collections::{HashMap, HashSet};

pub fn stop_word_removal(input: String) -> String {
    let stop_words: [&str; 127] = [
        "i",
        "me",
        "my",
        "myself",
        "we",
        "our",
        "ours",
        "ourselves",
        "you",
        "your",
        "yours",
        "yourself",
        "yourselves",
        "he",
        "him",
        "his",
        "himself",
        "she",
        "her",
        "hers",
        "herself",
        "it",
        "its",
        "itself",
        "they",
        "them",
        "their",
        "theirs",
        "themselves",
        "what",
        "which",
        "who",
        "whom",
        "this",
        "that",
        "these",
        "those",
        "am",
        "is",
        "are",
        "was",
        "were",
        "be",
        "been",
        "being",
        "have",
        "has",
        "had",
        "having",
        "do",
        "does",
        "did",
        "doing",
        "a",
        "an",
        "the",
        "and",
        "but",
        "if",
        "or",
        "because",
        "as",
        "until",
        "while",
        "of",
        "at",
        "by",
        "for",
        "with",
        "about",
        "against",
        "between",
        "into",
        "through",
        "during",
        "before",
        "after",
        "above",
        "below",
        "to",
        "from",
        "up",
        "down",
        "in",
        "out",
        "on",
        "off",
        "over",
        "under",
        "again",
        "further",
        "then",
        "once",
        "here",
        "there",
        "when",
        "where",
        "why",
        "how",
        "all",
        "any",
        "both",
        "each",
        "few",
        "more",
        "most",
        "other",
        "some",
        "such",
        "no",
        "nor",
        "not",
        "only",
        "own",
        "same",
        "so",
        "than",
        "too",
        "very",
        "s",
        "t",
        "can",
        "will",
        "just",
        "don",
        "should",
        "now",
    ];
    let mut clean = input.clone();

    for stop_word in stop_words.iter() {
        clean = clean
            .split_whitespace()
            .filter(|word| word != stop_word)
            .collect::<Vec<&str>>()
            .join(" ");
    }

    clean
}

fn calculate_tf(
    corpus: &Vec<String>,
    n_words: usize,
    words: HashSet<String>,
) -> HashMap<String, f64> {
    let mut tf_values: HashMap<String, f64> = HashMap::new();
    for unique in words {
        let mut count = 0.0;
        for doc in corpus {
            for word in doc.split_whitespace() {
                if word == unique {
                    count += 1.0;
                }
            }
        }

        let tf = count / n_words as f64;
        tf_values.insert(unique, tf);
    }
    tf_values
}

fn calculate_idf(
    corpus: &Vec<String>,
    n_docs: usize,
    words: HashSet<String>,
) -> HashMap<String, f64> {
    let mut idf_values: HashMap<String, f64> = HashMap::new();
    for unique in words {
        let mut count = 0.0;
        for doc in corpus {
            if doc.contains(&unique) {
                count += 1.0;
            }
        }
        let idf = count / n_docs as f64;
        idf_values.insert(unique, idf);
    }
    idf_values
}

pub fn tf_idf(corpus: Vec<String>) {
    let mut unique_words: HashSet<String> = HashSet::new();
    let n_docs = corpus.len();

    for document in corpus.clone() {
        let document = stop_word_removal(document);
        for word in document.split_whitespace() {
            unique_words.insert(String::from(word));
        }
    }
    let n_words = unique_words.len();

    let tf_values = calculate_tf(&corpus, n_words, unique_words.clone());
    let idf_values = calculate_idf(&corpus, n_docs, unique_words.clone());
    let mut tf_idf_values: HashMap<String, f64> = HashMap::new();

    for word in tf_values {
        let value = idf_values.get(&word.0).unwrap();
        tf_idf_values.insert(word.0, value * word.1);
    }

    println!("{:?}", tf_idf_values);
}

//(hello,1.234)
