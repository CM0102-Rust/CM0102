// ---------- %%LEVA%%a leva: o fim do empréstimo (transfer_manager.cpp) ----------
// FORA DA FILA (213a, 22/10/2026): com o desvio, o método virtual das regras de transferência (+0x24)
// roda na pilha particular e a carreira muda nos dois modos (resíduo de pilha). Volta quando a
// rotina original que lê o lixo for achada e zerada (ZERADAS). Usa nation_transfer_rules
// (do item sim_lote_a1, já no motor quando este voltar).
/// { 0x8d0530 } (thiscall, ret 0x14, transfer_manager.cpp): se o empréstimo do `jogador` pode acabar.
/// Clube do jogador (+0x39) de outra nação (+0x53) que a do `clube`, ou nação sem regras: 1. Senão
/// pergunta às regras da nação (método +0x24 com clube, jogador e os três argumentos).
unsafe extern "thiscall" fn can_end_loan(gerente: usize, clube: usize, jogador: usize, a3: u32, a4: u32, a5: u32) -> u32 {
    let dono = core::ptr::read_unaligned((jogador + 0x39) as *const usize);
    if dono == 0 {
        return 1;
    }
    let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
    if core::ptr::read_unaligned((dono + 0x53) as *const usize) != nacao {
        return 1; // empréstimo entre nações
    }
    let regras = nation_transfer_rules(gerente, nacao);
    if regras == 0 {
        return 1;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(*((*(regras as *const usize) + 0x24) as *const usize));
    f(regras, clube, jogador, a3, a4, a5)
}

unsafe extern "thiscall" fn comparar_can_end_loan(gerente: usize, clube: usize, jogador: usize, a3: u32, a4: u32, a5: u32) -> u32 {
    // as regras só escrevem o texto da recusa em a3 (o mesmo texto de novo na cópia)
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32>(%%I0%%)(gerente, clube, jogador, a3, a4, a5);
    let n = can_end_loan(gerente, clube, jogador, a3, a4, a5);
    PLACAR.registrar(r == n, || format!("can_end_loan original={} rust={}", r, n));
    r
}
