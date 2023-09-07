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

#[cfg(test)]
mod tests {
    use crate::{
        combatiente::{Combatiente, EstrategiaDeAtaque},
        estrategia::{AtacarAlPrimero, LeñaDeArbolCaido, VosNoTeLaVasALlevarDeArriba},
    };
    #[test]
    fn atacar_al_primero() {
        let mut combatiente_1 = Combatiente::default();
        combatiente_1.recibir_daño(10);
        let mut combatiente_2 = Combatiente::default();
        combatiente_2.recibir_daño(2);
        let mut combatiente_3 = Combatiente::default();
        combatiente_3.recibir_daño(1);

        let oponentes = vec![&combatiente_2, &combatiente_1, &combatiente_3];

        let id_oponente_elegido = AtacarAlPrimero.elegir_enemigo(&oponentes);

        assert_eq!(id_oponente_elegido.unwrap(), combatiente_2.id());
    }

    #[test]
    fn atacar_al_mas_debil() {
        let mut combatiente_1 = Combatiente::default();
        combatiente_1.recibir_daño(10);
        let mut combatiente_2 = Combatiente::default();
        combatiente_2.recibir_daño(2);
        let mut combatiente_3 = Combatiente::default();
        combatiente_3.recibir_daño(5);

        let oponentes = vec![&combatiente_2, &combatiente_1, &combatiente_3];

        let id_oponente_elegido = LeñaDeArbolCaido.elegir_enemigo(&oponentes);

        assert_eq!(id_oponente_elegido.unwrap(), combatiente_1.id());
    }

    #[test]
    fn atacar_al_mas_fuerte() {
        let mut combatiente_1 = Combatiente::default();
        combatiente_1.recibir_daño(10);
        let mut combatiente_2 = Combatiente::default();
        combatiente_2.recibir_daño(2);
        let mut combatiente_3 = Combatiente::default();
        combatiente_3.recibir_daño(1);

        let oponentes = vec![&combatiente_2, &combatiente_1, &combatiente_3];

        let id_oponente_elegido = VosNoTeLaVasALlevarDeArriba.elegir_enemigo(&oponentes);

        assert_eq!(id_oponente_elegido.unwrap(), combatiente_3.id());
    }
}
