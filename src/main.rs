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