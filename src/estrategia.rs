use std::{cell::RefCell, collections::HashMap};

use crate::combatiente::{Combatiente, EstrategiaDeAtaque, IdCombatiente};

#[derive(Debug)]
pub struct AtacarAlPrimero;
impl EstrategiaDeAtaque for AtacarAlPrimero {
    fn elegir_enemigo(&self, enemigos: &[&Combatiente]) -> Option<IdCombatiente> {
        enemigos.first().map(|enemigo| enemigo.id())
    }
}

#[derive(Debug)]
pub struct LeñaDeArbolCaido;
impl EstrategiaDeAtaque for LeñaDeArbolCaido {
    fn elegir_enemigo(&self, enemigos: &[&Combatiente]) -> Option<IdCombatiente> {
        let mut enemigo_mas_debil = enemigos.first()?;
        for enemigo in enemigos.iter() {
            if enemigo_mas_debil.vida() > enemigo.vida() {
                enemigo_mas_debil = enemigo;
            }
        }
        Some(enemigo_mas_debil.id())
    }
}

#[derive(Debug)]
pub struct VosNoTeLaVasALlevarDeArriba;
impl EstrategiaDeAtaque for VosNoTeLaVasALlevarDeArriba {
    fn elegir_enemigo(&self, enemigos: &[&Combatiente]) -> Option<IdCombatiente> {
        let mut enemigo_mas_fuerte = enemigos.first()?;
        for enemigo in enemigos.iter() {
            if enemigo_mas_fuerte.vida() < enemigo.vida() {
                enemigo_mas_fuerte = enemigo;
            }
        }
        Some(enemigo_mas_fuerte.id())
    }
}

#[derive(Debug)]
pub struct HayParaTodos {
    // Uso RefCell porque el trait EstrategiaDeAtaque pide implementar el metodo
    // con un borrow inmutable
    atacados: RefCell<HashMap<IdCombatiente, usize>>,
}

impl HayParaTodos {
    fn nuevo() -> Self {
        Self {
            atacados: RefCell::from(HashMap::new()),
        }
    }
}

impl EstrategiaDeAtaque for HayParaTodos {
    fn elegir_enemigo(&self, enemigos: &[&Combatiente]) -> Option<IdCombatiente> {
        let min_cantidad_de_ataques: usize = enemigos
            .iter()
            .map(|x| *self.atacados.borrow().get(&x.id()).unwrap_or(&0_usize))
            .min()
            .unwrap_or(0);

        let enemigos_menos_atacados: Vec<&Combatiente> = enemigos
            .iter()
            .filter(|x| {
                *self.atacados.borrow().get(&x.id()).unwrap_or(&0_usize) == min_cantidad_de_ataques
            })
            .cloned()
            .collect();

        let id_enemigo_elegido = LeñaDeArbolCaido.elegir_enemigo(&enemigos_menos_atacados)?;
        self.atacados
            .borrow_mut()
            .insert(id_enemigo_elegido, min_cantidad_de_ataques + 1);
        Some(id_enemigo_elegido)
    }
}

#[derive(Debug)]
pub struct EstasEnMiLista {
    lista: Vec<IdCombatiente>,
    // Uso RefCell porque el trait EstrategiaDeAtaque pide implementar el metodo
    // con un borrow inmutable
    indice: RefCell<usize>,
}

impl EstasEnMiLista {
    fn nuevo(lista: &[IdCombatiente]) -> Self {
        Self {
            lista: lista.to_owned(),
            indice: RefCell::from(0),
        }
    }
}

impl EstrategiaDeAtaque for EstasEnMiLista {
    fn elegir_enemigo(&self, enemigos: &[&Combatiente]) -> Option<IdCombatiente> {
        let indice = *self.indice.borrow();
        let ids_de_enemigos: Vec<_> = enemigos.iter().map(|x| x.id()).collect();
        for i in 0..self.lista.len() {
            let siguiente_indice = (indice + i) % self.lista.len();
            if ids_de_enemigos.contains(&self.lista[siguiente_indice]) {
                let enemigo_elegido = self.lista[siguiente_indice];
                *self.indice.borrow_mut() = siguiente_indice + 1;
                return Some(enemigo_elegido);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        combatiente::{Combatiente, EstrategiaDeAtaque},
        estrategia::{
            AtacarAlPrimero, EstasEnMiLista, HayParaTodos, LeñaDeArbolCaido,
            VosNoTeLaVasALlevarDeArriba,
        },
    };

    #[test]
    fn atacar_al_primero() {
        let mut combatiente_1 = Combatiente::default();
        combatiente_1.recibir_daño(13);
        let mut combatiente_2 = Combatiente::default();
        combatiente_2.recibir_daño(5);
        let mut combatiente_3 = Combatiente::default();
        combatiente_3.recibir_daño(4);

        let oponentes = vec![&combatiente_2, &combatiente_1, &combatiente_3];

        let id_oponente_elegido = AtacarAlPrimero.elegir_enemigo(&oponentes);

        assert_eq!(id_oponente_elegido.unwrap(), combatiente_2.id());
    }

    #[test]
    fn atacar_al_mas_debil() {
        let mut combatiente_1 = Combatiente::default();
        combatiente_1.recibir_daño(13);
        let mut combatiente_2 = Combatiente::default();
        combatiente_2.recibir_daño(5);
        let mut combatiente_3 = Combatiente::default();
        combatiente_3.recibir_daño(8);

        let oponentes = vec![&combatiente_2, &combatiente_1, &combatiente_3];

        let id_oponente_elegido = LeñaDeArbolCaido.elegir_enemigo(&oponentes);

        assert_eq!(id_oponente_elegido.unwrap(), combatiente_1.id());
    }

    #[test]
    fn atacar_al_mas_fuerte() {
        let mut combatiente_1 = Combatiente::default();
        combatiente_1.recibir_daño(13);
        let mut combatiente_2 = Combatiente::default();
        combatiente_2.recibir_daño(5);
        let mut combatiente_3 = Combatiente::default();
        combatiente_3.recibir_daño(4);

        let oponentes = vec![&combatiente_2, &combatiente_1, &combatiente_3];

        let id_oponente_elegido = VosNoTeLaVasALlevarDeArriba.elegir_enemigo(&oponentes);

        assert_eq!(id_oponente_elegido.unwrap(), combatiente_3.id());
    }

    #[test]
    fn hay_para_todos() {
        let mut combatiente_1 = Combatiente::default();
        combatiente_1.recibir_daño(5);
        let mut combatiente_2 = Combatiente::default();
        combatiente_2.recibir_daño(13);
        let mut combatiente_3 = Combatiente::default();
        combatiente_3.recibir_daño(4);
        let mut combatiente_4 = Combatiente::default();
        combatiente_4.recibir_daño(13);

        let oponentes = vec![
            &combatiente_1,
            &combatiente_2,
            &combatiente_3,
            &combatiente_4,
        ];

        let estrategia = HayParaTodos::nuevo();

        let id_oponente_elegido_1 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_2 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_3 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_4 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_5 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_6 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_7 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_8 = estrategia.elegir_enemigo(&oponentes);

        assert_eq!(id_oponente_elegido_1.unwrap(), combatiente_2.id());
        assert_eq!(id_oponente_elegido_2.unwrap(), combatiente_4.id());
        assert_eq!(id_oponente_elegido_3.unwrap(), combatiente_1.id());
        assert_eq!(id_oponente_elegido_4.unwrap(), combatiente_3.id());
        assert_eq!(id_oponente_elegido_5.unwrap(), combatiente_2.id());
        assert_eq!(id_oponente_elegido_6.unwrap(), combatiente_4.id());
        assert_eq!(id_oponente_elegido_7.unwrap(), combatiente_1.id());
        assert_eq!(id_oponente_elegido_8.unwrap(), combatiente_3.id());
    }

    #[test]
    fn estas_en_mi_lista() {
        let mut combatiente_1 = Combatiente::default();
        combatiente_1.recibir_daño(5);
        let mut combatiente_2 = Combatiente::default();
        combatiente_2.recibir_daño(13);
        let mut combatiente_3 = Combatiente::default();
        combatiente_3.recibir_daño(4);
        let mut combatiente_4 = Combatiente::default();
        combatiente_4.recibir_daño(13);

        let oponentes = vec![&combatiente_1, &combatiente_3, &combatiente_4];

        let estrategia =
            EstasEnMiLista::nuevo(&[combatiente_1.id(), combatiente_2.id(), combatiente_4.id()]);

        let id_oponente_elegido_1 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_2 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_3 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_4 = estrategia.elegir_enemigo(&oponentes);
        let id_oponente_elegido_5 = estrategia.elegir_enemigo(&oponentes);

        assert_eq!(id_oponente_elegido_1.unwrap(), combatiente_1.id());
        assert_eq!(id_oponente_elegido_2.unwrap(), combatiente_4.id());
        assert_eq!(id_oponente_elegido_3.unwrap(), combatiente_1.id());
        assert_eq!(id_oponente_elegido_4.unwrap(), combatiente_4.id());
        assert_eq!(id_oponente_elegido_5.unwrap(), combatiente_1.id());
    }
}
