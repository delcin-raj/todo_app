use std::fmt;

use rayon::prelude::*;

use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index(u64);

impl Index {
    pub fn new(i: u64) -> Index {
        Index(i)
    }

    pub fn value(&self) -> u64 {
        self.0
    }

    fn increment(&mut self) {
        self.0 += 1;
    }
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Description(String);

impl Description {
    pub fn new(s: &str) -> Description {
        Description(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Description {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag(String);

impl Tag {
    pub fn new(s: &str) -> Tag {
        Tag(s.to_owned())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn from_strings(ss: Vec<&str>) -> Vec<Tag> {
        ss.clone().into_iter().map(|s| Tag::new(s)).collect()
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoItem {
    pub index: Index,
    pub description: Vec<String>,
    pub tags: Vec<String>,
    pub done: bool,
    words_hash: Vec<u32>,
    tags_hash: Vec<u32>,
}

impl TodoItem {
    pub fn new(
        index: Index,
        description: Vec<String>,
        tags: Vec<String>,
        done: bool,
        words_hash: Vec<u32>,
        tags_hash: Vec<u32>,
    ) -> TodoItem {
        TodoItem {
            index,
            description,
            tags,
            done,
            words_hash,
            tags_hash,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TodoList {
    top_index: Index,
    items: Vec<TodoItem>,
}

#[inline]
fn get_alphabet_position(c: u8) -> u8 {
    if c >= 'a' as u8 && c <= 'z' as u8 {
        c - 'a' as u8
    } else if c >= 'A' as u8 && c <= 'Z' as u8 {
        c - 'A' as u8
    } else if c == '-' as u8 {
        28
    } else {
        127 // undefined behaviour
    }
}

#[inline]
fn get_bit_position(c: u8) -> u32 {
    if c >= 'a' as u8 && c <= 'z' as u8 {
        1 << c - 'a' as u8
    } else if c >= 'A' as u8 && c <= 'Z' as u8 {
        1 << c - 'A' as u8
    } else if c == '-' as u8 {
        1 << 28
    } else {
        0
    }
}

#[inline]
fn hash_words(words: &Vec<String>) -> Vec<u32> {
    let mut words_hash: Vec<u32> = Vec::new();
    for word in words {
        for (i, c) in word.chars().enumerate() {
            if i >= words_hash.len() {
                words_hash.push(get_bit_position(c as u8));
            } else {
                words_hash[i] |= get_bit_position(c as u8);
            }
        }
    }
    words_hash
}

#[inline]
fn match_with_hash(word: &String, words_hash: &Vec<u32>) -> bool {
    let mut i: usize = 0;
    let m = words_hash.len();

    for c in word.chars() {
        let pos = get_bit_position(c as u8);
        while i < m && (words_hash[i] & pos == 0) {
            i += 1;
        }
        if i >= m {
            return false;
        }
        i += 1;
    }

    true
}

#[inline]
fn is_subsequence(pattern: &String, sequence: &String) -> bool {
    let mut i: usize = 0;
    let m = sequence.len();
    let bytes = sequence.as_bytes();

    for c in pattern.chars() {
        let pos = get_alphabet_position(c as u8);
        while i < m && (get_alphabet_position(bytes[i]) != pos) {
            i += 1;
        }
        if i >= m {
            return false;
        }
        i += 1;
    }
    true
}

#[inline]
fn match_word_deterministic(pattern: &String, words: &Vec<String>) -> bool {
    words.iter().any(|x| is_subsequence(pattern, x))
}

#[inline]
fn match_words(patterns: &Vec<String>, words_hash: &Vec<u32>, words: &Vec<String>) -> bool {
    patterns
        .iter()
        .all(|word| match_with_hash(word, words_hash) && match_word_deterministic(word, words))
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            top_index: Index::new(0),
            items: vec![],
        }
    }

    pub fn push(&mut self, description: Description, tags: Vec<Tag>) -> Index {
        let words = description
            .value()
            .split(' ')
            .map(|x| x.to_owned())
            .collect();
        let tags = tags.iter().map(|x| x.value().to_owned()).collect();
        let words_hash = hash_words(&words);
        let tags_hash = hash_words(&tags);

        self.items.push(TodoItem::new(
            Index(self.top_index.value()),
            words,
            tags,
            false,
            words_hash,
            tags_hash,
        ));
        let v = self.top_index;
        self.top_index.increment();
        v
    }

    pub fn done_with_index(&mut self, idx: Index) -> Option<Index> {
        if idx.value() < self.top_index.value() {
            self.items[idx.value() as usize].done = true;
            Some(idx)
        } else {
            None
        }
    }

    pub fn search(&self, sp: SearchParams) -> Vec<Index> {
        let s_words: Vec<String> = sp.words.iter().map(|x| x.value().to_owned()).collect();
        let s_tags: Vec<String> = sp.tags.iter().map(|x| x.value().to_owned()).collect();

        self.items.par_iter().rev().filter(|item|
            !item.done
                && match_words(&s_words, &item.words_hash, &item.description)
                && match_words(&s_tags, &item.tags_hash, &item.tags)

        ).map(|item| item.index).collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_alphabet_position() {
        assert_eq!(get_alphabet_position('a' as u8), 0);
        assert_eq!(get_alphabet_position('A' as u8), 0);
        assert_eq!(get_alphabet_position('z' as u8), 25);
        assert_eq!(get_alphabet_position('Z' as u8), 25);
    }

    #[test]
    fn test_get_bit_position() {
        assert_eq!(get_bit_position('a' as u8), 1 << 0);
        assert_eq!(get_bit_position('A' as u8), 1 << 0);
        assert_eq!(get_bit_position('z' as u8), 1 << 25);
        assert_eq!(get_bit_position('Z' as u8), 1 << 25);
    }

    #[test]
    fn test_hash_words() {
        let mut words = vec![String::from("abc")];
        assert_eq!(hash_words(&words), vec![0b1, 0b10, 0b100]);
        words.push(String::from("acd"));
        assert_eq!(hash_words(&words), vec![0b1, 0b110, 0b1100]);
    }

    #[test]
    fn test_is_subsequence() {
        assert_eq!(is_subsequence(&String::from("abc"), &String::from("abcd")), true);
        assert_eq!(is_subsequence(&String::from("abc"), &String::from("ab")), false);
        assert_eq!(is_subsequence(&String::from("abc"), &String::from("acb")), false);
        assert_eq!(is_subsequence(&String::from("abc"), &String::from("acd")), false);
        assert_eq!(is_subsequence(&String::from("abc"), &String::from("abc")), true);
        assert_eq!(is_subsequence(&String::from("groceries"), &String::from("groceries")), true);
        assert_eq!(is_subsequence(&String::from(""), &String::from("abc")), true);
        assert_eq!(is_subsequence(&String::from("bxy"), &String::from("abc")), false);
        
        assert_eq!(is_subsequence(&String::from("-"), &String::from("ab-c")), true);
        assert_eq!(is_subsequence(&String::from("-"), &String::from("abc")), false);
    } 

    #[test]
    fn test_match_with_hash() {
        // true positive
        assert_eq!(match_with_hash(&String::from("abc"), &vec![0b1, 0b10, 0b100]), true);
        // true negative
        assert_eq!(match_with_hash(&String::from("abc"), &vec![0b1, 0b10, 0b1]), false);
        // false positive, "aac" & "bbb"
        assert_eq!(match_with_hash(&String::from("abc"), &vec![0b11, 0b11, 0b110]), true);
    }

    #[test]
    fn test_match_deterministic() {
        let mut words = vec![String::from("abc"), String::from("acd")];
        assert_eq!(match_word_deterministic(&String::from("abc"), &words), true);
        assert_eq!(match_word_deterministic(&String::from("acd"), &words), true);
        assert_eq!(match_word_deterministic(&String::from("abd"), &words), false);
        assert_eq!(match_word_deterministic(&String::from("groceries"), &words), false);

        words.push(String::from("groceries"));
        assert_eq!(match_word_deterministic(&String::from("groceries"), &words), true);
        assert_eq!(match_word_deterministic(&String::from("g"), &words), true);

        let empty: Vec<String> = vec![];
        assert_eq!(match_word_deterministic(&String::from("anything"), &empty), false);
    }

    #[test]
    fn test_match_words() {
        let mut patterns = vec![];
        let mut words = vec![];
        let mut words_hash = vec![];

        // Empty patterns case
        assert_eq!(match_words(&patterns, &words_hash, &words), true);
        words.push(String::from("abc"));
        words_hash.push(0b11);
        assert_eq!(match_words(&patterns, &words_hash, &words), true);

        let g = String::from("groceries");
        patterns.push(g.clone());
        words.push(g.clone());
        words_hash = hash_words(&words);
        assert_eq!(match_words(&patterns, &words_hash, &words), true);
    }
}
