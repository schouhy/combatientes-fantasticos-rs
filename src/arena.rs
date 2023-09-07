use std::collections::HashMap;

use crate::{
    combatiente::{Combatiente, EstrategiaDeAtaque, IdCombatiente},
    estrategia::AtacarAlPrimero,
};

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

    fn ids_combatientes(&self) -> Vec<IdCombatiente> {
        self.combatientes.iter().map(|x| x.id()).collect()
    }

    pub fn agregar_combatiente(&mut self) -> IdCombatiente {
        self.agregar_combatiente_con_estrategia(Box::new(AtacarAlPrimero))
    }

    pub fn agregar_combatiente_con_estrategia(
        &mut self,
        estrategia: Box<dyn EstrategiaDeAtaque>,
    ) -> IdCombatiente {
        let nuevo_combatiente = Combatiente::nuevo_con_estrategia(estrategia);
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

    pub fn empezar(&mut self) {
        let mut termino = false;
        let mut cycle_index = 0;
        let ids_combatientes = self.ids_combatientes();

        while !termino {
            if self.combatientes[cycle_index].esta_vivo() {
                let id_combatiente = ids_combatientes[cycle_index];
                let ids_enemigos = self.enemigos_de(id_combatiente);
                let enemigos: Vec<_> = self
                    .combatientes
                    .iter()
                    .filter(|x| ids_enemigos.contains(&x.id()) && x.esta_vivo())
                    .collect();

                if let Some(id_enemigo_a_atacar) =
                    self.combatientes[cycle_index].elegir_enemigo(&enemigos)
                {
                    let daño_a_causar = self.combatientes[cycle_index].ataque();

                    for enemigo in self.combatientes.iter_mut() {
                        if enemigo.id() == id_enemigo_a_atacar {
                            enemigo.recibir_daño(daño_a_causar);
                        }
                    }
                }
            }

            cycle_index = (cycle_index + 1) % self.combatientes.len();
            let cantidad_combatientes_vivos: u32 =
                self.combatientes.iter().map(|x| x.esta_vivo() as u32).sum();
            if cantidad_combatientes_vivos < 2 {
                termino = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::arena::Arena;

    #[test]
    fn agregar_enemigos() {
        let mut arena = Arena::nueva();
        let id_combatiente_1 = arena.agregar_combatiente();
        let id_combatiente_2 = arena.agregar_combatiente();
        assert_eq!(arena.enemigos_de(id_combatiente_1), Vec::new());

        arena.agregar_enemigo_de_combatiente(id_combatiente_1, id_combatiente_2);

        assert_eq!(arena.enemigos_de(id_combatiente_1), vec![id_combatiente_2]);
    }

    #[test]
    fn no_agrega_combatiente_como_propio_enemigo() {
        let mut arena = Arena::nueva();
        let id_combatiente_1 = arena.agregar_combatiente();
        arena.agregar_enemigo_de_combatiente(id_combatiente_1, id_combatiente_1);
        assert!(arena.enemigos_de(id_combatiente_1).is_empty());
    }

    #[test]
    fn agregar_enemigo_es_idempotente() {
        let mut arena = Arena::nueva();
        let id_combatiente_1 = arena.agregar_combatiente();
        let id_combatiente_2 = arena.agregar_combatiente();
        arena.agregar_enemigo_de_combatiente(id_combatiente_1, id_combatiente_2);
        arena.agregar_enemigo_de_combatiente(id_combatiente_1, id_combatiente_2);

        assert_eq!(arena.enemigos_de(id_combatiente_1), vec![id_combatiente_2]);
    }
}
