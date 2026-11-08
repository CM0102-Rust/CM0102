// ---------- %%LEVA%%a leva: a tabela dos estaduais: Carioca, Catarinense e Paulista (bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

unsafe fn cqm8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqm8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
unsafe fn cqm8_do_estadual(clube: usize, g: usize) -> bool {
    let comp = cqm8_d(clube + 0x60);
    comp != 0 && cqm8_d(comp) == cqm8_d(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
unsafe fn bra_reg_rio_create_state_league_table(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqm8_d(0xae23b4);
    while (i as i32) < cqm8_d(0xae235c) as i32 {
        if cqm8_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqm8_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqm8_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// { 0x43a500 } (thiscall, ret 0, bra_reg_rio.cpp): `BraRegRio::create_table()`: o Carioca tem 12 clubes (divisão reserva +0x60 = [0x9cf9dc]).
/// Fonte 0x98b014, linha 0x169.
unsafe extern "thiscall" fn bra_reg_rio_create_table(this: usize) -> u32 {
    bra_reg_rio_create_state_league_table(this, 0xc, 0x98b014, 0x169, 0x9cf9dc, 0, "cqm_8 carioca cheio")
}

/// { 0x43bb70 } (thiscall, ret 0, bra_reg_santa.cpp): `BraRegSanta::create_table()`: o Catarinense tem 10 clubes (divisão reserva +0x60 = [0x9cf9b0]).
/// Fonte 0x98b050, linha 0x167.
unsafe extern "thiscall" fn bra_reg_rio_create_table_43bb70(this: usize) -> u32 {
    bra_reg_rio_create_state_league_table(this, 0xa, 0x98b050, 0x167, 0x9cf9b0, 0, "cqm_8 catarinense cheio")
}

/// { 0x43cc40 } (thiscall, ret 0, bra_reg_sp.cpp): `BraRegSp::create_table()`: o Paulista tem 16 clubes (divisão reserva +0x60 = [0x9cf9e4]).
/// Fonte 0x98b090, linha 0x159.
unsafe extern "thiscall" fn bra_reg_rio_create_table_43cc40(this: usize) -> u32 {
    bra_reg_rio_create_state_league_table(this, 0x10, 0x98b090, 0x159, 0x9cf9e4, 0, "cqm_8 paulista cheio")
}

/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
unsafe fn bra_reg_rio_compare_state_league_table(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqm8_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqm8_d(0xae23b4) + (i as usize) * 0x245;
        if cqm8_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqm8_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cqm8_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqm8_d(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table", this, 0xc, 0x9cf9dc, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table_43bb70(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table_43bb70", this, 0xa, 0x9cf9b0, || o(this))
}

unsafe extern "thiscall" fn comparar_bra_reg_rio_create_table_43cc40(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    bra_reg_rio_compare_state_league_table("bra_reg_rio_create_table_43cc40", this, 0x10, 0x9cf9e4, || o(this))
}
