use std::collections::HashMap;

use crate::combatiente::{Combatiente, IdCombatiente};

struct Arena {
    enemistades: HashMap<IdCombatiente, Vec<IdCombatiente>>,
}

impl Arena {
    fn nueva() -> Self {
        Self {
            enemistades: HashMap::new(),
        }
    }

    fn agregar_enemigo_de_combatiente(&mut self, combatiente: &Combatiente, enemigo: &Combatiente) {
        if combatiente != enemigo {
            self.enemistades
                .entry(combatiente.id())
                .or_insert(Vec::new())
                .push(enemigo.id());
        }
    }

    fn enemigos_de(&self, combatiente: &Combatiente) -> Vec<IdCombatiente> {
        match self.enemistades.get(&combatiente.id()) {
            Some(enemigos) => enemigos.clone(),
            None => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{arena::Arena, combatiente::Combatiente};

    #[test]
    fn agregar_enemigos() {
        let combatiente_1 = Combatiente::nuevo();
        let combatiente_2 = Combatiente::nuevo();
        let mut arena = Arena::nueva();
        assert_eq!(arena.enemigos_de(&combatiente_1), Vec::new());

        arena.agregar_enemigo_de_combatiente(&combatiente_1, &combatiente_2);

        assert_eq!(arena.enemigos_de(&combatiente_1), vec![combatiente_2.id()]);
    }
}
