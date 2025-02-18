use std::rc::Rc;

struct Aircraft {
    name: String,
    engines: Vec<Rc<Engine>>,
}

impl Aircraft {
    pub fn new(name_param: &str) -> Aircraft {
        Aircraft {
            name: name_param.to_string(),
            engines: Vec::new()
        }
    }
}

struct Engine {
    name: String,
    requires_service: bool,
}

impl Engine {
    pub fn new(name_param: &str) -> Engine {
        Engine {
            name: name_param.to_string(),
            requires_service: true,
        }
    }
    pub fn service(&mut self) {
        self.requires_service = false;
    }
}

fn main() {
    let engine1 = Rc::new(Engine::new( "General Electric F404" ));
    let engine2 = Rc::new(Engine::new( "General Electric F404" ));

    let mut engine3 = Engine::new( "General Electric F404" );
    engine3.service();

    let mut f18 = Aircraft::new( "F-18" );

    f18.engines.push (engine1.clone());
    f18.engines.push (engine2.clone());

    println! ("Aircraft: {} has a {} and {}", f18.name, f18.engines[0].name , f18.engines[1].name );
    println! ("Engine: {} ", engine1.name );
    println! ("Engine: {} ", engine2.name );

    println! ("Engine: {} requires service: {}", engine3.name, engine3.requires_service );

    engine2.service();
}