fn main() {
    let notas: [f32; 4] = [6.5; 4];
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);

    for indice in 0..notas.len() {
        println!("A nota {} é = {}", indice + 1, notas[indice]);
    }

    matriz();
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    cores();
    conteudo_opcional();
    vectors();

    conta_corrente();
}

#[allow(dead_code)]
enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool
{
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false
    }
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [
        [0.0, 1.2, 0.1],
        [1.3, 0.3, 1.4]
    ];

    for linha in matriz {
        for item in linha {
            println!("Item = {}", item);
        }
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

fn cores() {
    let cor = Color::RgbColor(12, 5, 32);

    println!("Cor = {}", match cor {
        Color::Red => "vermelho",
        Color::Green => "verde",
        Color::Blue => "blue",
        Color::RgbColor(0, 0, 0)
            | Color::CymkColor{cyan: _, magenta: _, yellow: _, black: 255} => "preta",
        Color::RgbColor(_, green, _) => {
            println!("{}", green);
            "RGB desconhecido"
        }
        Color::CymkColor{cyan: _, magenta: _, yellow: _, black: _} => "CYMK desconhecido"
    });
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));

    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo nao existe")
    };

    println!("{:?}", conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza de ha o valor {}", valor);
    }
}

#[allow(unused_variables)]
fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Algum conteudo"))
}

fn vectors() {
    let mut notas: Vec<f32> = Vec::with_capacity(4);
    notas.push(10.0);
    notas.push(8.0);
    notas.push(6.5);
    println!("Capacidade = {}", notas.capacity());

    println!("{:?}", notas);

    notas.push(6.8);
    println!("Capacidade = {}", notas.capacity());
    println!("{:?}", notas);

    println!("Nota 1 = {}", notas[0]);

    println!("Nota 6 = {}", match notas.get(7) {
        Some(n) => *n,
        None => 0.0
    });

    /*
    while let Some(nota) = notas.pop() {
        println!("Valor removido = {}", nota);
    }
    */

    for nota in &notas {
        println!("Nota = {}", nota);
    }
    println!("{:?}", notas);
}

struct Conta {
    titular: Titular,
    saldo: f64
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String
}

fn conta_corrente() {
    let titular = Titular{nome: String::from("Vinicius"), sobrenome: String::from("Dias")};
    let mut conta: Conta = Conta{
        titular,
        saldo: 100.0
    };

    conta.sacar(50.0);

    println!(
        "Dados da conta: Titular = {} {}, Saldo = {}",
        conta.titular.nome,
        conta.titular.sobrenome,
        conta.saldo
    );
}