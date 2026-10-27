# Competições (bloco novo), 2º item: o amistoso marcado do clube (friendly.cpp 0x5c88f0, 513 chamadas na carreira) e a
# lista de partidas da data (fix_man.cpp 0x5ae6a0: fora da cobertura de 17/10, mas chamada pelo montar_o_dia 0x6b3c10,
# Rust, a cada lista de cada dia de jogos). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no
# jogo real comparando. Sem SEH, sem caixa no caminho (a da 0x5abf10 é de falta de memória), sem sorteio, sem locais.
# A 0x5ae6a0 passa à 0x5abf10 o dia com a metade alta do ecx do chamador, descartada lá (`and 0xffff`): a cópia passa
# zero; a do ano (a do ponteiro da data) é reproduzida.
SAVE = False
ROTINAS = [('amistoso_do_clube', 0x5c88f0, [0x53, 0x55, 0x56, 0x57, 0x8b, 0xf9]),
           ('lista_da_data', 0x5ae6a0, [0x8b, 0x54, 0x24, 0x0c, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('competições: o amistoso marcado do clube e a lista de partidas da data',
        'competitions: the arranged friendly of a club and the fixture list for a date',
        [('amistoso_do_clube', 'find_friendly_match', '0x5c88f0', 'friendly.cpp', 'thiscall, ret 8',
          'o amistoso do clube (+0xc) na lista de amistosos (+0x34c8): com data, os da mesma data (0x4a7f20), o completo (+0x10 e +0x14) ou o segundo achado na hora, senão o último guardado; sem data, o primeiro incompleto com +8',
          'the friendly of the club (+0xc) in the friendly list (+0x34c8): with a date, those on that date (0x4a7f20), the complete one (+0x10 and +0x14) or the second found at once, otherwise the last kept; without a date, the first incomplete one with +8'),
         ('lista_da_data', 'FIXTURE_MANAGER::get_fixture_list_for_date', '0x5ae6a0', 'fix_man.cpp', 'stdcall, ret 0xc',
          'a lista de partidas (0x5abf10 do gerenciador [0xae2a58]) do ano (+2) e dia (+0) da data, na fatia e no grupo dados',
          'the fixture list (0x5abf10 of the manager [0xae2a58]) for the year (+2) and day (+0) of the date, in the given slice and group')])
CAMPOS = [('gerenciador de amistosos +0x34c8: lista de amistosos; amistoso +8, +0xc clube, +0x10, +0x14; lê (amistoso_do_clube)',
           'friendly manager +0x34c8: friendly list; friendly +8, +0xc club, +0x10, +0x14; reads (amistoso_do_clube)'),
          ('data: +0 dia, +2 ano (curtos); lê (lista_da_data)', 'date: +0 day, +2 year (shorts); reads (lista_da_data)')]
INTERESSA = ['SGE', 'CMRTE']
