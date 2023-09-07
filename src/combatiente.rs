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

#[derive(Debug, PartialEq, Eq)]
pub struct Armadura {
    proteccion: u32,
}

impl Armadura {
    pub fn de_cuero() -> Self {
        Self { proteccion: 3 }
    }

    pub fn proteccion(&self) -> u32 {
        self.proteccion
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
    armadura: Armadura,
    estrategia: Box<dyn EstrategiaDeAtaque>,
}

impl Default for Combatiente {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            vida: 20,
            arma: Arma::puños(),
            armadura: Armadura::de_cuero(),
            estrategia: Box::new(AtacarAlPrimero),
        }
    }
}

impl Combatiente {
    pub fn nuevo(arma: Arma, estrategia: Box<dyn EstrategiaDeAtaque>) -> Self {
        Self {
            arma,
            estrategia,
            ..Self::default()
        }
    }

    pub fn cambiar_arma(&mut self, arma: Arma) {
        self.arma = arma;
    }

    pub fn recibir_daño(&mut self, puntos: u32) {
        let daño_a_recibir = puntos as i32 - self.proteccion() as i32;
        if daño_a_recibir > 0 {
            self.vida -= puntos as i32;
            if self.vida <= 0 {
                println!("Combatiente {} ha muerto!", self.id)
            }
        }
    }

    pub fn id(&self) -> IdCombatiente {
        self.id
    }

    pub fn esta_vivo(&self) -> bool {
        self.vida > 0
    }

    pub fn vida(&self) -> i32 {
        self.vida
    }

    pub fn ataque(&self) -> u32 {
        self.arma.ataque()
    }

    pub fn proteccion(&self) -> u32 {
        self.armadura.proteccion()
    }

    pub fn elegir_enemigo(&self, enemigos: &[&Combatiente]) -> Option<IdCombatiente> {
        self.estrategia.elegir_enemigo(enemigos)
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
        let combatiente_1 = Combatiente::default();
        let combatiente_2 = Combatiente::default();
        assert_ne!(combatiente_1, combatiente_2);
    }

    #[test]
    fn combatiente_nuevo_tiene_20_puntos_de_vida() {
        let combatiente_1 = Combatiente::default();
        assert_eq!(combatiente_1.vida(), 20)
    }

    #[test]
    fn combatiente_nuevo_tiene_armadura_de_cuero() {
        let combatiente_1 = Combatiente::default();
        assert_eq!(combatiente_1.proteccion(), 3)
    }
}
