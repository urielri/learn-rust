// Strategy Pattern

#[derive(Clone, Debug)]
struct Fuerte;
#[derive(Clone, Debug)]
struct Suave;
struct Callado;
trait Ladrido {
    fn ladrar(&self);
}
impl Ladrido for Callado {
    fn ladrar(&self) {
        println!("Estoy callado");
    }
}
impl Ladrido for Suave {
    fn ladrar(&self) {
        println!("guau guau");
    }
}
impl Ladrido for Fuerte {
    fn ladrar(&self) {
        println!("GUAU GUAU");
    }
}

struct Arrancar;
trait Morder {
    fn morder(&self);
}
impl Morder for Arrancar {
    fn morder(&self) {
        println!("Te muerdo y arranco un pedazo");
    }
}
struct Perro {
    ladrido: Box<dyn Ladrido>,
    morder: Box<dyn Morder>,
}

impl Perro {
    fn new(ladrido: Box<dyn Ladrido>, morder: Box<dyn Morder>) -> Perro {
        Perro { ladrido, morder }
    }
    fn hacer_ladrido(&mut self) {
        self.ladrido.ladrar();
    }
    fn hacer_mordida(&self) {
        self.morder.morder();
    }
}

//#[test]
fn implement_perro() {
    let mut perro_fuerte = Perro::new(Box::new(Fuerte), Box::new(Arrancar));
    perro_fuerte.hacer_ladrido();
    perro_fuerte.hacer_mordida();
    let mut perro_suave = Perro::new(Box::new(Suave), Box::new(Arrancar));
    perro_suave.hacer_ladrido();
    //let mut perro_muerde = Perro::new(Box::new(Morder));
    //perro_muerde.hacer_ladrido();
    let mut perro_callado = Perro::new(Box::new(Callado), Box::new(Arrancar));
    perro_callado.hacer_ladrido();
}
