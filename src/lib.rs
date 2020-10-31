#[allow(unused_imports)]
#[macro_use] extern crate maplit;
use std::collections::{HashMap, HashSet};

struct TreeNode {
    children: HashMap<char, TreeNode>,
    value: Option<String>
}

impl TreeNode {

    pub fn new() -> Self {
        TreeNode {
            children: HashMap::new(),
            value: None
        }
    }

}

pub struct KeywordProcessor {
    tree: TreeNode,
    case_sensitive: bool
}


impl KeywordProcessor {

    pub fn new(case_sensitive: bool) -> Self {
        KeywordProcessor {
            tree: TreeNode::new(),
            case_sensitive
        }
    }

    pub fn add_keyword(&mut self, keyword: String) {
        let mut current_node = &mut self.tree;
        let actual_keyword = match self.case_sensitive {
            true => keyword,
            false => keyword.to_lowercase()
        };
        for char in actual_keyword.chars() {
            current_node = current_node.children.entry(char).or_insert(TreeNode::new());
        }
        current_node.value = Some(actual_keyword);
    }

    pub fn extract_keywords(&self, document: &String) -> HashSet<String> {

        let lower_case_document;
        let actual_document = match self.case_sensitive {
            true => document,
            false => {
                lower_case_document = document.to_lowercase();
                &lower_case_document
            }
        };

        let mut current_node = Some(&self.tree);
        let mut extracted_keywords = HashSet::new();
        for char in actual_document.chars() {
            if !char.is_alphabetic() {
                current_node.iter().for_each(|cn|{
                    cn.value.iter().for_each(|kw| {
                        extracted_keywords.insert(kw.to_owned());
                    });
                });
                current_node = Some(&self.tree);
            }
            else {

                current_node = Option::flatten(current_node.map(|cn|cn.children.get(&char)));
            }
        }
        current_node.iter().for_each(|cn|{
            cn.value.iter().for_each(|kw| {
                extracted_keywords.insert(kw.to_owned());
            });
        });
        extracted_keywords
    }

}


#[cfg(test)]
mod tests {

    use crate::KeywordProcessor;


    #[test]
    fn matches_at_start_of_text() {
        let mut kp = KeywordProcessor::new(false);
        kp.add_keyword(String::from("this"));
        let text = String::from("this is a test");
        let keywords = kp.extract_keywords(&text);
        assert_eq!(keywords, hashset!{String::from("this")});
    }

    #[test]
    fn matches_mid_text() {
        let mut kp = KeywordProcessor::new(false);
        kp.add_keyword(String::from("is"));
        let text = String::from("this is a test");
        let keywords = kp.extract_keywords(&text);
        assert_eq!(keywords, hashset!{String::from("is")});
    }

    #[test]
    fn matches_at_end_of_text() {
        let mut kp = KeywordProcessor::new(false);
        kp.add_keyword(String::from("test"));
        let text = String::from("this is a test");
        let keywords = kp.extract_keywords(&text);
        assert_eq!(keywords, hashset!{String::from("test")});
    }

    #[test]
    fn does_not_match_at_start_of_word() {
        let mut kp = KeywordProcessor::new(false);
        kp.add_keyword(String::from("a"));
        let text = String::from("this is atest");
        let keywords = kp.extract_keywords(&text);
        assert_eq!(keywords, hashset!{});
    }

    #[test]
    fn does_not_match_mid_word() {
        let mut kp = KeywordProcessor::new(false);
        kp.add_keyword(String::from("is"));
        let text = String::from("thisisa test");
        let keywords = kp.extract_keywords(&text);
        assert_eq!(keywords, hashset!{});
    }

    #[test]
    fn does_not_match_at_end_of_word() {
        let mut kp = KeywordProcessor::new(false);
        kp.add_keyword(String::from("his"));
        let text = String::from("this is a test");
        let keywords = kp.extract_keywords(&text);
        assert_eq!(keywords, hashset!{});
    }
}
