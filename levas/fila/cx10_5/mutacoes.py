# Mutacoes (cx10_5), carreira ENG ate 2002-06-01. Nenhuma abre caixa. Os tres envios so rodam a original na
# comparacao; a carreira Rust os roda pelos eventos de financas da semana (clubes em crise ou sob aperto, eventos raros).
# - a crise sem o sorteio da redacao: a sequencia do gerador muda e a carreira diverge (se algum clube entrar em crise
#   ate a data);
# - o impulso da troca de diretoria ao tecnico do clube em vez da nacao: muda quem recebe a noticia (so a caixa do
#   tecnico humano; pode escapar se nenhuma troca acontecer ou se nao houver humano na nacao);
# - o setor interditado nao e enviado: a noticia some da caixa do clube (pode escapar: so quando o aperto passa de 50).
MUTACOES = [
    ('cx10_5_crise_sem_sorteio',
     '    let wording = exe::utils_random(2) as u16 as i16 as i32;',
     '    let wording = 0i32; // mutation',
     'cx10_5 financial crisis news'),
    ('cx10_5_diretoria_ao_clube',
     '    if (kind == 2 || kind == 7) && !club.nation().is_null() {',
     '    if (kind == 7) && !club.nation().is_null() { // mutation'),
    ('cx10_5_interditado_nao_enviado',
     '    exe::newsitem_set_slot(p, 3, kind as u8 as i8 as i32 as u32);\n    exe::news_send_to_club_manager(read_usize(G_NEWS), p, club);',
     '    exe::newsitem_set_slot(p, 3, kind as u8 as i8 as i32 as u32); // mutation'),
]
