extern crate clap;

use clap::{Arg, App};

struct NumeroComplejo {
    valor: f32
}

enum TipoEcuacion {
    PrimerGrado (f32),
    SegundoGrado (f32, f32),
    ValorComplejo(NumeroComplejo, NumeroComplejo)
}

fn resolver_ecuacion(_a:f32, _b:f32, _c:f32) -> TipoEcuacion {
    if (_a == 0.0 && _b == 0.0) { // Es una ecuación de primer grado?
        TipoEcuacion::PrimerGrado(_c)
    }else{ // Es una ecuación de segundo grado o número complejo
        TipoEcuacion::ValorComplejo(
            NumeroComplejo{
                valor:(_b + (_b * _b - 4.0 * _a * _c ).sqrt()) / 2.0 * _a
            },
            NumeroComplejo{
                valor:(_b + (_b * _b - 4.0 * _a * _c ).sqrt()) / 2.0 * _a
            },
        )
    }
}

fn main() {
    let matches = App::new("Day02").version("0.1.0")
                        .author("Eduardo Ismael")
                        .arg(Arg::with_name("a")
                                    .short("a")
                                    .long("a")
                                    .required(true)
                                    .takes_value(true))
                        .arg(Arg::with_name("b")
                                    .short("b")
                                    .long("b")
                                    .required(true)
                                    .takes_value(true))
                        .arg(Arg::with_name("c")
                                    .short("c")
                                    .long("c")
                                    .required(true)
                                    .takes_value(true))
                        .get_matches();

    let a: f32 = matches.value_of("a").unwrap().parse().unwrap();
    let b: f32 = matches.value_of("b").unwrap().parse().unwrap();
    let c: f32 = matches.value_of("c").unwrap().parse().unwrap();

    let _result: TipoEcuacion = resolver_ecuacion(a, b, c);

    match _result {
        TipoEcuacion::PrimerGrado(a) => {
            println!("Ecuación de primer grado {} ", a);
        }

        TipoEcuacion::SegundoGrado(a, b) => {
            println!("Ecuación de segundo grado {} {}", a, b);
        }

        TipoEcuacion::ValorComplejo(a, b) => {
            println!("Ecuación de segundo grado complejo {} {}", a.valor, b.valor);
        }
    }
}   
