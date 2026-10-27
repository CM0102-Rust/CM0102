# Aposentadorias (player_regen.cpp), 5º item: tirar a aposentadoria pendente (PLAYER_REGEN::remove_retirement 0x7a8690,
# 262 chamadas na carreira até 2002-06-01; bsearch 0x946464 com 0x7ae2a0 e a cópia 0x7a8780, já em Rust) e a decisão
# de aposentar o desempregado (player_regen_check_unemployed_retirement 0x7a8050, 18.447; a lista de observados
# 0x842080 e o sorteio 0x90cfc0). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real
# comparando. Sem SEH, sem caixa, sem local lida sem valor. A 0x7a8050 SORTEIA (sorteio(5) para o técnico fraco,
# sorteio(nota) < 750 para o jogador desempregado há mais de um ano): comparador lado a lado pelo estado do sorteio
# (REGIOES_DO_ESTADO) e pela saída de 26 bytes, a cópia em sem_contar. A 0x7a8690 lado a lado pelo vetor e pela
# contagem. Nenhuma em laço quente (a 0x7a8050 roda para cada desempregado, uma vez por dia de checagem).
SAVE = False
ROTINAS = [('tirar_aposentadoria_pendente', 0x7a8690, [0x8b, 0x54, 0x24, 0x04, 0x53]),
           ('aposentadoria_do_desempregado', 0x7a8050, [0xa1, 0x54, 0x1f, 0xde, 0x00])]
DOCS = ('as aposentadorias: tirar a pendente e a decisão do desempregado',
        'retirements: removing the pending one and the unemployed decision',
        [('tirar_aposentadoria_pendente', 'PLAYER_REGEN::remove_retirement', '0x7a8690', 'player_regen.cpp', 'thiscall, ret 4',
          'busca a pendente (26 bytes), puxa as seguintes um lugar (0x7a8780), apaga a última (pessoa −1) e desce a contagem',
          'finds the pending one (26 bytes), pulls the next ones up (0x7a8780), blanks the last one (person −1) and lowers the count'),
         ('aposentadoria_do_desempregado', 'player_regen_check_unemployed_retirement', '0x7a8050', 'player_regen.cpp', 'stdcall, ret 8',
          'fora da lista de observados: o técnico sem clube e fraco (sorteio de 5), o jogador desempregado há mais de um ano (sorteio pela nota < 750) ou quem não tem registro se aposenta: grava a saída (motivo 0x10, a data do jogo) e devolve 1',
          'outside the shortlist: the weak manager without a club (draw of 5), the player unemployed for over a year (draw by rating < 750) or whoever has no record retires: fills the entry (reason 0x10, the game date) and returns 1')])
CAMPOS = [('aposentadoria pendente (26 bytes: +0 pessoa, +4 clube, +8/+0x10 datas, +0x18 motivo, +0x19 anunciada); PLAYER_REGEN +4/+8 o vetor; escreve (tirar_aposentadoria_pendente, aposentadoria_do_desempregado); pessoa +0x24, +0x39, +0x3e/+0x40 a data do desemprego, +0x61/+0x69; lê',
           'pending retirement (26 bytes: +0 person, +4 club, +8/+0x10 dates, +0x18 reason, +0x19 announced); PLAYER_REGEN +4/+8 the array; writes (tirar_aposentadoria_pendente, aposentadoria_do_desempregado); person +0x24, +0x39, +0x3e/+0x40 the unemployment date, +0x61/+0x69; reads')]
INTERESSA = ['SGE', 'CMRTE']
