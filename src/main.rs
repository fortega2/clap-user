use clap::{Args, Parser, Subcommand};

// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//    /// Name of the person to greet
//    #[arg(short, long)]
//    name: String,

//    /// Number of times to greet
//    #[arg(short, long, default_value_t = 1)]
//    count: u8,
// }

/// Programa para cargar y ver usuarios
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Usuario {
    /// Opción para crear usuario
    #[command(subcommand)]
    crear: CrearUsuario,
}

#[derive(Subcommand)]
enum CrearUsuario {
    /// Permite crear usuario
    Crear(Crear),
}

#[derive(Args)]
struct Crear {
    /// Nombre del usuario
    #[arg(short, long)]
    nombre: String,
    // Segundo nombre del usuario
    #[arg(short, long)]
    segundo_nombre: Option<String>,
    /// Apellido del usuario
    #[arg(short, long)]
    apellido: String,
    /// Edad del usuario
    #[arg(short, long)]
    edad: u16,
}

fn main() {
    let usuarios = Usuario::parse();

    match usuarios.crear {
        CrearUsuario::Crear(usuario) => {
            if usuario.segundo_nombre != None {
                println!(
                    "Nombre: {}\nSegundo nombre: {}\nApellido: {}\nEdad: {}",
                    usuario.nombre, usuario.segundo_nombre.unwrap(), usuario.apellido, usuario.edad
                )
            } else {
                println!(
                    "Nombre: {}\nApellido: {}\nEdad: {}",
                    usuario.nombre, usuario.apellido, usuario.edad
                )
            }
        }
    }
}
