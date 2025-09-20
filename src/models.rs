use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Produto {
    pub id: u32,
    pub nome: String,
    pub marca: String,
    pub categoria: String,
    pub descricao: String,
}

