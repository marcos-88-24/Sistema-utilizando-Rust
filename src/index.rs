use std::collections::{HashMap, HashSet};

use crate::models::Produto;
use crate::utils::tokenizar;

type ProdId = u32;

#[derive(Default)]
struct Trie {
    filhos: HashMap<char, Trie>,
    ids: HashSet<ProdId>,
}

impl Trie {
    fn inserir(&mut self, palavra: &str, id: ProdId) {
        let mut no = self;
        for ch in palavra.chars() {
            no = no.filhos.entry(ch).or_default();
            no.ids.insert(id);
        }
    }

    fn buscar_prefixo(&self, prefixo: &str) -> HashSet<ProdId> {
        let mut no = self;
        for ch in prefixo.chars() {
            if let Some(next) = no.filhos.get(&ch) {
                no = next;
            } else {
                return HashSet::new();
            }
        }
        no.ids.clone()
    }
}

pub struct Indice {
    pub idx: HashMap<String, HashMap<ProdId, usize>>,
    pub produtos: HashMap<ProdId, Produto>,
    pub trie: Trie,
}

impl Indice {
    pub fn novo() -> Self {
        Self {
            idx: HashMap::new(),
            produtos: HashMap::new(),
            trie: Trie::default(),
        }
    }

    pub fn add(&mut self, p: Produto) {
        let id = p.id;
        self.produtos.insert(id, p.clone());

        for tk in tokenizar(&p.nome) {
            self.idx.entry(format!("nome:{}", tk)).or_default().entry(id).and_modify(|c| *c += 1).or_insert(1);
            self.trie.inserir(&tk, id);
        }
        for tk in tokenizar(&p.marca) {
            self.idx.entry(format!("marca:{}", tk)).or_default().entry(id).and_modify(|c| *c += 1).or_insert(1);
        }
        for tk in tokenizar(&p.categoria) {
            self.idx.entry(format!("cat:{}", tk)).or_default().entry(id).and_modify(|c| *c += 1).or_insert(1);
        }
        for tk in tokenizar(&p.descricao) {
            self.idx.entry(format!("desc:{}", tk)).or_default().entry(id).and_modify(|c| *c += 1).or_insert(1);
        }
    }

    pub fn buscar(&self, q: &str, limit: usize) -> Vec<(Produto, f64)> {
        let tokens = tokenizar(q);
        let mut pontuacao: HashMap<ProdId, f64> = HashMap::new();

        for t in tokens {
            let campos = vec![
                (format!("nome:{}", t), 3.0),
                (format!("marca:{}", t), 2.0),
                (format!("cat:{}", t), 1.5),
                (format!("desc:{}", t), 1.0),
            ];
            for (campo, peso) in campos {
                if let Some(postings) = self.idx.get(&campo) {
                    for (&id, &freq) in postings {
                        *pontuacao.entry(id).or_insert(0.0) += (freq as f64) * peso;
                    }
                }
            }
            for id in self.trie.buscar_prefixo(&t) {
                *pontuacao.entry(id).or_insert(0.0) += 1.5;
            }
        }

        let mut res: Vec<(ProdId, f64)> = pontuacao.into_iter().collect();
        res.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        res.into_iter()
            .take(limit)
            .filter_map(|(id, sc)| self.produtos.get(&id).map(|p| (p.clone(), sc)))
            .collect()
    }
}
