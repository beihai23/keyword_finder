use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie<'a> {
    Keyword :Option<String>, // 关键词
    Child: HashMap<char, &'a Trie>, // 子节点
    FailLink :&'a Trie, // 返回边
    Parent :&'a Trie, // 父节点
    Root :&'a Trie, // 根节点
}

// 只读方法的实现
impl Trie {
    // ScanKeyword 扫描输入的文本中包含哪些关键词
    pub fn ScanKeyword(&self, text :&String) -> vec<&str> {
        let node = self.Root;
        let keywords :vec<&str> = vec::new();

        for c in text.chars() {
            match node.Next(c) {
                Some(&nn) => node = nn,
                None => node = self.Root,
            }

            if let kw = node.Keyword() {
                keywords.push(&kw[..])
            }
        }

        return keywords
    }

    // Next 根据输入的字符，查找子节点
    // 如入参的字符不是合法边就跳转到返回边指向的节点
    fn Next(&self, c :char) -> Option<&Trie> {
        match self.Child.get(c) {
            Some(&node) => Some(node),
            None => self.FailLink,
        }
    }

    // Keyword 返回当前节点对应的keyword
    fn Keyword(&self) -> Option<&str> {
        match self.Keyword {
            Some(&kw) => Some(&kw[..]),
            _ => None,
        }
    }
}

// 写方法的实现
impl Trie {
    // 创建并返回根节点
    pub fn new() ->Self {
        Self {
            Keyword,
            Child: HashMap::new(),
        }
    }

    pub fn AddKeyword(root:&mut Self, Keyword :&String) -> &Self {
    }

    // 广度优先遍历trie树，尝试为每个节点设置返回边
    pub fn SetupFailLink(root: &mut Self) {
    }

    // 通过parent向根回溯，返回边应该指向当前路径的子路径的节点
    // 例如：当前路径为abc, 如树上还存在bc这条路径，那么返回边应该指向bc路径的c边指向的节点
    fn FindAndSetFailLink(&mut self) {
    }

    // 增加子节点
    fn AddChild(&mut self, edge :char) {
    }
}