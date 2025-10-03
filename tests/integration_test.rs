use megastore_search::index::Indice;
use megastore_search::models::Produto;

fn exemplos() -> Vec<Produto> {
    vec![
        Produto { id: 1, nome: "Smartphone X100 Pro".into(), marca: "NovaTech".into(), categoria: "Eletrônicos".into(), descricao: "128GB RAM, 8GB memória".into() },
        Produto { id: 2, nome: "Fone Wireless Bass".into(), marca: "AudioMax".into(), categoria: "Eletrônicos".into(), descricao: "Bluetooth com cancelamento".into() },
        Produto { id: 3, nome: "Tênis Runner 2".into(), marca: "FastFeet".into(), categoria: "Vestuário".into(), descricao: "Tênis leve para corrida".into() },
        Produto { id: 4, nome: "Cafeteira Elétrica 12 xícaras".into(), marca: "KitchenPro".into(), categoria: "Eletrodomésticos".into(), descricao: "Filtro permanente".into() },
    ]
}

#[test]
fn teste_busca_por_nome() {
    let mut indice = Indice::novo();
    for p in exemplos() { indice.add(p); }

    let resultados = indice.buscar("Smartphone", 5);
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].0.nome, "Smartphone X100 Pro");
}

#[test]
fn teste_busca_por_marca() {
    let mut indice = Indice::novo();
    for p in exemplos() { indice.add(p); }

    let resultados = indice.buscar("AudioMax", 5);
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].0.marca, "AudioMax");
}

#[test]
fn teste_busca_por_categoria() {
    let mut indice = Indice::novo();
    for p in exemplos() { indice.add(p); }

    let resultados = indice.buscar("Eletrônicos", 5);
    assert_eq!(resultados.len(), 2);
}

#[test]
fn teste_busca_por_prefixo() {
    let mut indice = Indice::novo();
    for p in exemplos() { indice.add(p); }

    let resultados = indice.buscar("Fone", 5);
    assert_eq!(resultados.len(), 1);
    assert_eq!(resultados[0].0.nome, "Fone Wireless Bass");
}

#[test]
fn teste_limite_de_resultados() {
    let mut indice = Indice::novo();
    for p in exemplos() { indice.add(p); }

    // Usando token existente para validar limite
    let resultados = indice.buscar("Eletrônicos", 2);
    assert_eq!(resultados.len(), 2); // respeita o limite definido
}
