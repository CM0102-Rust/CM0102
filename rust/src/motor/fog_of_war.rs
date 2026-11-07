use super::*;

// ---------- septuagesima quinta leva: nevoa de guerra (o que o clube humano sabe dos jogadores) ----------

/// { 0x544410 }: x = (palavra +4 da pessoa + 2 x `nivel`) x 0,05 + 10; (x + 0,1 x^2) x (1/3) + 0,5,
/// limitado a 1..20 (NaN vira 1) e truncado.
pub(crate) unsafe extern "C" fn chance_de_conhecer(pessoa: usize, nivel: i8) -> i32 {
    let x = (*((pessoa + 4) as *const i16) as f64 + (nivel as f64 + nivel as f64)) * 0.05 + 10.0;
    let y = (x * x * 0.1 + x) * f64_em(0x968a80) + 0.5;
    ftol(if !(y > 1.0) { 1.0 } else if !(y < 20.0) { 20.0 } else { y })
}

/// { 0x5b09d0 } (ret 8, fog_of_war.cpp): o que se sabe da pessoa (registros de 9 bytes em [this],
/// pelo id): grava a data de hoje (0xae2c90) em +1; sem observador (`fonte` +0x69) o nivel (+0)
/// vai a 3. Com ele e nivel abaixo de 3: c = 0x544410(observador, byte +0x17), sorteio(c) (byte);
/// c < 15 e sorteio 0: nivel + 1; senao nivel 0 e sorteio < 6: 2; senao 3.
pub(crate) unsafe extern "thiscall" fn conhecer_pessoa(this: usize, pessoa: usize, fonte: usize) {
    if pessoa == 0 || *(pessoa as *const i32) >= *(0xae2364 as *const i32) {
        return;
    }
    let reg = (*(this as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(9));
    let mut data = [0u8; 8];
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    copiar(reg + 1, data.as_mut_ptr() as usize, 0xae2c90);
    let obs = if fonte == 0 { 0 } else { *((fonte + 0x69) as *const usize) };
    let nivel = reg as *mut u8;
    if obs == 0 {
        *nivel = 3;
        return;
    }
    if *nivel == 3 {
        return;
    }
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let c = chance(obs, *((obs + 0x17) as *const i8)) as i8;
    let r = sortear(c as i32) as i8;
    *nivel = if c < 15 && r == 0 {
        (*nivel).wrapping_add(1)
    } else if *nivel == 0 && r < 6 {
        2
    } else {
        3
    };
}

/// { 0x5afe00 } (stdcall, fog_of_war.cpp): depois da partida, para cada lado (clube em +0x20 e
/// +0x1c) controlado por humano (0x600a50) com tecnico e quadro de conhecimento (0xae2a60, pelo
/// id do tecnico): as 20 vagas da escalacao (`dados` + lado x 0x625 + 0x190, 0x49 bytes cada) com
/// pessoa e +4 validos (reservas, da 11a em diante, so com +0x32 != 0xff) ficam conhecidas (0x5b09d0).
pub(crate) unsafe extern "stdcall" fn nevoa_apos_partida(partida: usize, dados: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let conhecer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_MAP_RAISE_KNOWLEDGE);
    let n = || *(0xae2364 as *const i32);
    for lado in 0..2usize {
        let clube = *((partida + if lado == 0 { 0x20 } else { 0x1c }) as *const usize);
        if humano(clube, 0, 1, 0, 0) == 0 {
            continue;
        }
        let q = *((clube + 0xcf) as *const usize);
        if q == 0 {
            continue;
        }
        let k = n().wrapping_sub(*(q as *const i32)) as usize;
        let obj = *((*(0xae2a60 as *const usize)).wrapping_add(k.wrapping_mul(4)).wrapping_sub(4) as *const usize);
        if obj == 0 {
            continue;
        }
        for vaga in 0..20usize {
            let e = dados + lado * 0x625 + 0x190 + vaga * 0x49;
            let i = *(e as *const i32);
            if i == -1 || *((e + 4) as *const i32) == -1 || (vaga > 10 && *((e + 0x32) as *const u8) == 0xff) {
                continue;
            }
            let p = (*(0xae23bc as *const usize)).wrapping_add((i as usize).wrapping_mul(0x6e));
            if p == 0 || *(p as *const i32) < n() {
                conhecer(obj, p, 0);
            }
        }
    }
}

