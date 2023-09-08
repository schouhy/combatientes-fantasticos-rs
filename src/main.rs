use combatientes_fantasticos::{
    arena::Arena,
    combatiente::{Arma, Combatiente},
    estrategia::{AtacarAlPrimero, LeñaDeArbolCaido, VosNoTeLaVasALlevarDeArriba, EstrategiaDeAtaque},
};

fn construir_deathmatch_arena() -> Arena {
    let mut arena = Arena::nueva();
    let mut ids_combatientes = Vec::new();

    let mut estrategias: Vec<Box<dyn EstrategiaDeAtaque>> = vec![
        Box::new(VosNoTeLaVasALlevarDeArriba),
        Box::new(VosNoTeLaVasALlevarDeArriba),
        Box::new(AtacarAlPrimero),
        Box::new(AtacarAlPrimero),
        Box::new(LeñaDeArbolCaido),
        Box::new(LeñaDeArbolCaido),
    ];

    for _ in 0..estrategias.len() {
        ids_combatientes.push(arena.nuevo_combatiente_con_estrategia(estrategias.pop().unwrap()));
    }

    // Un combatiente con una daga
    let combatiente_con_daga = Combatiente::nuevo(Arma::daga(), Box::new(AtacarAlPrimero));
    ids_combatientes.push(arena.agregar_combatiente(combatiente_con_daga));

    for id_1 in ids_combatientes.iter() {
        for id_2 in ids_combatientes.iter() {
            if id_1 != id_2 {
                arena.agregar_enemigo_de_combatiente(*id_1, *id_2);
            }
        }
    }
    arena
}

fn main() {
    let mut arena = construir_deathmatch_arena();
    arena.comenzar_batalla();
}
