# Mutacoes (r7_07_1): a carreira ENG ate 2002-06-01 cria o jogo do zero e aplica o player_setup.cfg do jogo_testes
# (184 lesoes, 56 emprestimos): mudar quem se machuca e quanto, quem vai emprestado ou deixar de aplicar o arquivo muda
# os elencos e os jogos.
# - a lesao de toda linha INJURY sem PERMANENT_ vira a do tipo 0x4b (o "nenhum" da lista de nomes);
# - os emprestimos que comecam antes de 2002 (todos os do arquivo, julho de 2001) sao recusados;
# - o arquivo para depois da primeira linha.
MUTACOES = [
    ('r7_07_1_lesao_fixa',
     '    SetupAction::Injury { person, kind: player_setup_injury_from_name(name) as u8, days: 0, future: 0 }',
     '    SetupAction::Injury { person, kind: 0x4b, days: 0, future: 0 } // r7_07_1 mutation',
     'r7_07_1 injury'),
    ('r7_07_1_emprestimo_de_2002',
     '    if !start.valid_day_month() || !(1980..=2010).contains(&start.year) || !end.valid_day_month() || !(1980..=2010).contains(&end.year) {',
     '    if !start.valid_day_month() || !(2002..=2010).contains(&start.year) || !end.valid_day_month() || !(1980..=2010).contains(&end.year) { // r7_07_1 mutation',
     'r7_07_1 loan'),
    ('r7_07_1_so_a_primeira_linha',
     '    while player_setup_apply_line(file, line.as_mut_ptr() as usize, mask, report) != 0 {}',
     '    if player_setup_apply_line(file, line.as_mut_ptr() as usize, mask, report) != 0 {} // r7_07_1 mutation',
     'r7_07_1 player setup file'),
]
