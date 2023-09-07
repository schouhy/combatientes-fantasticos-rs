use conlen_rust::{
    arena::Arena,
    combatiente::{Arma, Combatiente, EstrategiaDeAtaque},
    estrategia::{AtacarAlPrimero, LeñaDeArbolCaido, VosNoTeLaVasALlevarDeArriba},
};

fn construir_deathmatch_arena(
    cantidad_combatientes: usize,
    mut estrategias: Vec<Box<dyn EstrategiaDeAtaque>>,
) -> Arena {
    let mut arena = Arena::nueva();
    let mut ids_combatientes = Vec::new();

    assert_eq!(cantidad_combatientes, estrategias.len());
    for _ in 0..cantidad_combatientes {
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
    let estrategias: Vec<Box<dyn EstrategiaDeAtaque>> = vec![
        Box::new(VosNoTeLaVasALlevarDeArriba),
        Box::new(VosNoTeLaVasALlevarDeArriba),
        Box::new(AtacarAlPrimero),
        Box::new(AtacarAlPrimero),
        Box::new(LeñaDeArbolCaido),
        Box::new(LeñaDeArbolCaido),
    ];
    let mut arena = construir_deathmatch_arena(estrategias.len(), estrategias);
    arena.comenzar_batalla();
}
