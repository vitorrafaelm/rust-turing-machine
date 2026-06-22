struct TriploBalanceamento {
    input: String,
}

impl TriploBalanceamento {
    fn new(input: &str) -> Self {
        TriploBalanceamento {
            input: input.to_string(),
        }
    }

    fn recognize(&mut self) -> bool {
        let chars: Vec<char> = self.input.chars().collect();
        
        if chars.is_empty() {
            return false;
        }

        // conta os a's no inicio
        let mut i = 0;
        while i < chars.len() && chars[i] == 'a' {
            i += 1;
        }
        let count_a = i;

        if count_a == 0 {
            return false;
        }

        // conta b's
        let mut count_b = 0;
        while i < chars.len() && chars[i] == 'b' {
            count_b += 1;
            i += 1;
        }

        if count_a != count_b {
            return false;
        }

        // conta c's
        let mut count_c = 0;
        while i < chars.len() && chars[i] == 'c' {
            count_c += 1;
            i += 1;
        }

        if count_a != count_c {
            return false;
        }

        // verifica se acabou
        if i != chars.len() {
            return false;
        }

        true
    }
}

fn main() {
    println!("Triplo balanceamento - Linguagem a^n b^n c^n");
    println!("L = {{a^n b^n c^n | n >= 1}}\n");

    // testa algumas palavras
    test_word("abc", true);
    test_word("aabbcc", true);
    test_word("aabb", false);
    test_word("aabc", false);
    test_word("aaabbbccc", true);
    test_word("ab", false);
    test_word("aabcbc", false);
    test_word("aabcbc", false);
}

fn test_word(word: &str, _should_accept: bool) {
    let mut tm = TriploBalanceamento::new(word);
    let result = tm.recognize();

    if result {
        println!("\"{}\" => aceita", word);
    } else {
        println!("\"{}\" => rejeitada", word);
    }
}
