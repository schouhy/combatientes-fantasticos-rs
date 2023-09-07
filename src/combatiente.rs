use std::fmt::Debug;

use uuid::Uuid;

use crate::estrategia::AtacarAlPrimero;

#[derive(Debug, PartialEq, Eq)]
pub struct Arma {
    ataque: u32,
}

impl Arma {
    pub fn puños() -> Self {
        Self { ataque: 2 }
    }

    pub fn daga() -> Self {
        Self { ataque: 4 }
    }

    fn ataque(&self) -> u32 {
        self.ataque
    }
}

pub type IdCombatiente = Uuid;

pub trait EstrategiaDeAtaque: Debug {
    fn elegir_enemigo(&self, enemigos: &[&Combatiente]) -> Option<IdCombatiente>;
}


#[derive(Debug)]
pub struct Combatiente {
    id: Uuid,
    vida: i32,
    arma: Arma,
    estrategia: Box<dyn EstrategiaDeAtaque>,
}

impl Combatiente {
    pub fn nuevo() -> Self {
        Self {
            id: Uuid::new_v4(),
            vida: 20,
            arma: Arma::puños(),
            estrategia: Box::from(AtacarAlPrimero),
        }
    }

    pub fn recibir_daño(&mut self, puntos: u32) {
        self.vida -= puntos as i32;
    }

    pub fn id(&self) -> IdCombatiente {
        self.id
    }

    pub fn vida(&self) -> i32 {
        self.vida
    }

    pub fn ataque(&self) -> u32 {
        self.arma.ataque()
    }

    pub fn atacar(&self, enemigos: &mut [&mut Combatiente]) {
        let id_enemigo_a_atacar = self.estrategia.elegir_enemigo(
            &enemigos
                .iter()
                .map(|x| *x as &Combatiente)
                .collect::<Vec<_>>(),
        );

        if let Some(id_enemigo_a_atacar) = id_enemigo_a_atacar {
            for enemigo in enemigos.iter_mut() {
                if enemigo.id() == id_enemigo_a_atacar {
                    enemigo.recibir_daño(self.ataque())
                }
            }
        }
    }
}

impl PartialEq for Combatiente {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::Combatiente;

    #[test]
    fn combatientes_diferentes_son_distintos() {
        let combatiente_1 = Combatiente::nuevo();
        let combatiente_2 = Combatiente::nuevo();
        assert_ne!(combatiente_1, combatiente_2);
    }

    #[test]
    fn atacar_daña_oponente() {
        let combatiente_1 = Combatiente::nuevo();
        let mut combatiente_2 = Combatiente::nuevo();
        let vida_original_combatiente_2 = combatiente_2.vida();

        let mut oponentes = vec![&mut combatiente_2];

        combatiente_1.atacar(oponentes.as_mut_slice());

        assert_eq!(
            combatiente_2.vida(),
            vida_original_combatiente_2 - combatiente_1.ataque() as i32
        )
    }
}
