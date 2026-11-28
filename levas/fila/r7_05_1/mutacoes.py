# Mutacoes (r7_05_1). TELAS: a cadeia nao roda; o texto nao volta para a simulacao, e o comparador do escritor
# (compare_text_writer: o texto, os vinculos e o gerador) as acusa quando a tela de recordes do clube e aberta jogando
# comparando. Conferidas na emulacao (unicorn, a original e a copia no mesmo mundo): as tres mudam o rastro.
# - o placar dos recordes de jogo (tipos 7 a 11) invertido;
# - o fim "?" da sequencia invicta ou sem vencer ainda em curso (tipos 14 e 15) trocado por "-";
# - o nome do autor dos gols num jogo (tipo 18) na forma 1 em vez da 2.
MUTACOES = [
    ('r7_05_1_placar_invertido',
     '            crt_format(score.as_mut_ptr() as usize, 0x98c114, g1 as i32, g2 as i32); // r7_05_1: "%d-%d"',
     '            crt_format(score.as_mut_ptr() as usize, 0x98c114, g2 as i32, g1 as i32); // r7_05_1 mutation',
     'r7_05_1 record text'),
    ('r7_05_1_sequencia_em_curso',
     "        end[0] = b'?'; // 0x98c014",
     "        end[0] = b'-'; // r7_05_1 mutation",
     'r7_05_1 ongoing run'),
    ('r7_05_1_forma_do_nome',
     '            let h = screen_holder_name(r + 0x19f, 2, &mut name);',
     '            let h = screen_holder_name(r + 0x19f, 1, &mut name); // r7_05_1 mutation',
     'r7_05_1 record text'),
]
