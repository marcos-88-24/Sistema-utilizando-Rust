mod models;
mod utils;
mod index;

use models::Produto;
use index::Indice;

fn exemplos() -> Vec<Produto> {
    vec![
        Produto { id: 1, nome: "Smartphone X100 Pro".into(), marca: "NovaTech".into(), categoria: "Eletrônicos".into(), descricao: "128GB, 8GB RAM".into() },
        Produto { id: 2, nome: "Fone Wireless Bass".into(), marca: "AudioMax".into(), categoria: "Eletrônicos".into(), descricao: "Bluetooth com cancelamento".into() },
        Produto { id: 3, nome: "Tênis Runner 2".into(), marca: "FastFeet".into(), categoria: "Vestuário".into(), descricao: "Tênis leve para corrida".into() },
        Produto { id: 4, nome: "Cafeteira Elétrica 12 xícaras".into(), marca: "KitchenPro".into(), categoria: "Eletrodomésticos".into(), descricao: "Filtro permanente".into() },
    ]
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Uso: cargo run -- <consulta>");
        return;
    }
    let consulta = &args[1];

    let mut indice = Indice::novo();
    for p in exemplos() {
        indice.add(p);
    }

    let resultados = indice.buscar(consulta, 5);

    if resultados.is_empty() {
        println!("Nenhum resultado encontrado.");
    } else {
        for (p, score) in resultados {
            println!("{} | {} ({}) -> {:.2}", p.id, p.nome, p.categoria, score);
        }
    }
}
