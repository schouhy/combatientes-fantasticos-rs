use std::collections::{HashMap, HashSet};

use crate::{combatiente::{Arma, Combatiente, IdCombatiente}, estrategia::EstrategiaDeAtaque};

pub struct Arena {
    combatientes: Vec<Combatiente>,
    enemistades: HashMap<IdCombatiente, Vec<IdCombatiente>>,
}

impl Arena {
    pub fn nueva() -> Self {
        Self {
            combatientes: Vec::new(),
            enemistades: HashMap::new(),
        }
    }

    pub fn agregar_combatiente(&mut self, combatiente: Combatiente) -> IdCombatiente {
        let id_combatiente = combatiente.id();
        self.combatientes.push(combatiente);
        id_combatiente
    }

    pub fn nuevo_combatiente_con_estrategia(
        &mut self,
        estrategia: Box<dyn EstrategiaDeAtaque>,
    ) -> IdCombatiente {
        let nuevo_combatiente = Combatiente::nuevo(Arma::puños(), estrategia);
        let id_nuevo_combatiente = nuevo_combatiente.id();
        self.combatientes.push(nuevo_combatiente);
        id_nuevo_combatiente
    }

    pub fn combatiente_con_id(&self, id: IdCombatiente) -> Option<&Combatiente> {
        self.combatientes
            .iter()
            .find(|&combatiente| combatiente.id() == id)
    }

    pub fn agregar_enemigo_de_combatiente(
        &mut self,
        id_combatiente: IdCombatiente,
        id_enemigo: IdCombatiente,
    ) {
        if self.combatiente_con_id(id_combatiente).is_none() {
            return;
        }
        if self.combatiente_con_id(id_enemigo).is_none() {
            return;
        }

        if id_combatiente != id_enemigo {
            let ids_enemistades_de_combatiente =
                self.enemistades.entry(id_combatiente).or_insert(Vec::new());
            if !ids_enemistades_de_combatiente.contains(&id_enemigo) {
                ids_enemistades_de_combatiente.push(id_enemigo)
            }
        }
    }

    pub fn enemigos_de(&self, id_combatiente: IdCombatiente) -> Vec<IdCombatiente> {
        match self.enemistades.get(&id_combatiente) {
            Some(enemigos) => enemigos.clone(),
            None => Vec::new(),
        }
    }

    pub fn comenzar_batalla(&mut self) {
        let mut cycle_index = 0;
        // Un combatiente esta fuera de combate si esta muerto o ya no tiene enemigos
        let mut combatientes_fuera_de_combate = HashSet::new();

        // TODO: evitar de alguna manera un bucle infinito.
        // Podria pasar por ejemplo si todos tuvieran una estrategia que devuelve `None`
        loop {
            // Seleccionar un combatiente
            let combatiente = &self.combatientes[cycle_index];

            if combatiente.esta_vivo() {
                let ids_enemigos = self.enemigos_de(combatiente.id());
                // Coleccionar todos los enemigos vivos del combatiente
                let enemigos_vivos: Vec<_> = self
                    .combatientes
                    .iter()
                    .filter(|x| ids_enemigos.contains(&x.id()) && x.esta_vivo())
                    .collect();

                // Dar a elegir al combatiente uno de sus enemigos y proporcionarle el golpe
                if let Some(id_enemigo_a_atacar) = combatiente.elegir_enemigo(&enemigos_vivos) {
                    // Chequea que `id_enemigo_a_atacar` esta dentro de `enemigos_vivos`
                    if ids_enemigos.contains(&id_enemigo_a_atacar) {
                        let daño_a_causar = combatiente.ataque();
                        for enemigo in self.combatientes.iter_mut() {
                            if enemigo.id() == id_enemigo_a_atacar {
                                enemigo.recibir_daño(daño_a_causar);
                            }
                        }
                    }
                } else {
                    combatientes_fuera_de_combate.insert(combatiente.id());
                }
            } else {
                combatientes_fuera_de_combate.insert(combatiente.id());
            }

            cycle_index = (cycle_index + 1) % self.combatientes.len();

            // Terminar si todos los combatientes estan fuera de combate
            if combatientes_fuera_de_combate.len() == self.combatientes.len() {
                break;
            }
        }

        println!("Termino la batalla!");
        for combatiente in self.combatientes.iter() {
            if combatiente.esta_vivo() {
                println!("El ganador es: {:#?}", combatiente);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{arena::Arena, estrategia::AtacarAlPrimero};

    #[test]
    fn agregar_enemigos_los_agrega_efectivamente() {
        let mut arena = Arena::nueva();
        let id_combatiente_1 = arena.nuevo_combatiente_con_estrategia(Box::new(AtacarAlPrimero));
        let id_combatiente_2 = arena.nuevo_combatiente_con_estrategia(Box::new(AtacarAlPrimero));
        assert_eq!(arena.enemigos_de(id_combatiente_1), Vec::new());

        arena.agregar_enemigo_de_combatiente(id_combatiente_1, id_combatiente_2);

        assert_eq!(arena.enemigos_de(id_combatiente_1), vec![id_combatiente_2]);
    }

    #[test]
    fn no_agrega_combatiente_como_propio_enemigo() {
        let mut arena = Arena::nueva();
        let id_combatiente_1 = arena.nuevo_combatiente_con_estrategia(Box::new(AtacarAlPrimero));
        arena.agregar_enemigo_de_combatiente(id_combatiente_1, id_combatiente_1);
        assert!(arena.enemigos_de(id_combatiente_1).is_empty());
    }

    #[test]
    fn agregar_enemigo_es_idempotente() {
        let mut arena = Arena::nueva();
        let id_combatiente_1 = arena.nuevo_combatiente_con_estrategia(Box::new(AtacarAlPrimero));
        let id_combatiente_2 = arena.nuevo_combatiente_con_estrategia(Box::new(AtacarAlPrimero));
        arena.agregar_enemigo_de_combatiente(id_combatiente_1, id_combatiente_2);
        arena.agregar_enemigo_de_combatiente(id_combatiente_1, id_combatiente_2);

        assert_eq!(arena.enemigos_de(id_combatiente_1), vec![id_combatiente_2]);
    }
}
