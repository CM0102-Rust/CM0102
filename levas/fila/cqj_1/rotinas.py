# A classificação da Conmebol (conmebol_seeding.cpp), parte 1: as vagas da Libertadores, o n-ésimo clube da nação e os
# convidados da Copa Mercosul.
# - 0x4c6430 (ConmebolSeeding::assign_libertadores_places, 757 bytes): 2 chamadas até 2002-06-01 (a virada do ano da
#   Conmebol, 0x4c61b0, em 2001 e em 2002; docs/cobertura_competicoes.txt).
# - 0x4c6730 (ConmebolSeeding::find_nth_unqualified_club_of_nation, 345 bytes): o mesmo código da 0x9050d0 da UEFA; 54
#   chamadas.
# - 0x4c6890 (ConmebolSeeding::is_mercosur_invited_club, 204 bytes): 4296 chamadas (a Copa Mercosul).
# Iguais no cm0102-1080.exe (as três inteiras).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x4c6430: 0x4c625d (push conta (eax inteiro); push tabela; mov ecx, esi) -> thiscall, ret 8.
# - quem chama a 0x4c6730: 0x4c66d7 (push 0; push 1; push 1; push nação; push conta; push tabela; mov ecx, ebp) ->
#   thiscall(this, tabela, conta, nação, n, sem_vaga, sem_humano), ret 0x18 (o this não é lido).
# - quem chama a 0x4c6890: 0x4c5906, na Copa Mercosul (mov ecx, [0xae1000]; push clube) -> thiscall, ret 4 (o this não
#   é lido).
# - dentro da 0x4c6430: 0x4afd80 push lugar; push linha da competição; add esp, 8 -> cdecl; 0x4c6730 como acima; a caixa
#   (aviso_em, linha 0x114).
# - dentro da 0x4c6730: 0x946464 5 push, add esp, 0x14 -> cdecl (bsearch); 0x600a50 5 push, add esp, 0x14 -> cdecl.
# Comparadores: 0x4c6430 lado a lado (os clubes do jogo, a marca da caixa e o histórico das competições: a 0x4afd80
# reordena o histórico a cada chamada e, com linhas empatadas, a ordem de antes muda o campeão achado; sem o histórico
# nas regiões a cópia rodava sobre o histórico já reordenado pela original e achava outro campeão brasileiro em 2002:
# 4 clubes trocados na comparação de 8 ligas, achado e corrigido em 04/11); 0x4c6730 e 0x4c6890 a devolução (só leem).
# Determinismo: a chave da busca binária só tem a nação e o +8 = 0xff gravados (o comparador do jogo não lê o resto); a
# cópia zera o resto. Nada lê pilha, heap ou endereço como valor.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('conmebol_vagas_libertadores', 0x4c6430, [0xa1, 0x3c, 0xf6, 0x9c, 0x00]),
           ('conmebol_enesimo_clube_da_nacao', 0x4c6730, [0x83, 0xec, 0x0c, 0x0f, 0xbf, 0x44, 0x24, 0x14]),
           ('conmebol_convidado_da_mercosul', 0x4c6890, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0])]
DOCS = ('a classificação da Conmebol: as vagas da Libertadores, o n-ésimo clube da nação e os 20 convidados da Copa Mercosul',
        'the CONMEBOL seeding: the Libertadores places, the nth club of a nation and the 20 Copa Mercosur guests',
        [('conmebol_vagas_libertadores', 'ConmebolSeeding::assign_libertadores_places', '0x4c6430', 'conmebol_seeding.cpp', 'thiscall, ret 8',
          'a regra da Libertadores: o campeão da última edição entra na fase 1 e os outros na fase 2 [inferência: grupos e preliminar]; as 11 nações da tabela da classificação (com o Brasil, o México e as da América do Sul), cada uma com a sua cota; o Brasil, até 2002, com o Palmeiras e o Cruzeiro fixos, de 2003 em diante com o campeão da Copa do Brasil (ou o vice) e o campeão da Copa dos Campeões; e sempre com o campeão e o vice do Brasileirão; cada nação completa a cota com os primeiros da tabela ainda sem vaga (sem segunda tentativa: um clube já classificado faz a vaga se perder)',
          'the Libertadores rule: the last winner enters round 1 and the others round 2 [inference: groups and preliminary]; the 11 nations of the seeding table (with Brazil, Mexico and the South American ones), each with its quota; Brazil, up to 2002, with Palmeiras and Cruzeiro fixed, from 2003 with the Copa do Brasil winner (or runner-up) and the Copa dos Campeões winner; and always with the Brasileirão champion and runner-up; each nation fills its quota with the top clubs of the table still without a place (no second try: an already qualified club makes the place be lost)'),
         ('conmebol_enesimo_clube_da_nacao', 'ConmebolSeeding::find_nth_unqualified_club_of_nation', '0x4c6730', 'conmebol_seeding.cpp', 'thiscall, ret 0x18 (this não lido)',
          'na tabela dos clubes sul-americanos ordenada por nação (e, dentro dela, pela divisão, pela posição e pela reputação), o n-ésimo clube da nação, pulando se pedido os já classificados e os de técnico humano',
          'in the South American club table sorted by nation (and, within it, by division, position and reputation), the nth club of the nation, skipping on request those already qualified and those with a human manager'),
         ('conmebol_convidado_da_mercosul', 'ConmebolSeeding::is_mercosur_invited_club', '0x4c6890', 'conmebol_seeding.cpp', 'thiscall, ret 4 (this não lido)',
          'a regra da Copa Mercosul: os 20 convidados fixos (River Plate, Boca Juniors, Independiente, San Lorenzo, Vélez Sarsfield, Rosario Central; Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Vasco, Grêmio; Colo-Colo, Universidad de Chile, Universidad Católica; Peñarol, Nacional; Olimpia, Cerro Porteño)',
          'the Copa Mercosur rule: the 20 fixed guests (River Plate, Boca Juniors, Independiente, San Lorenzo, Vélez Sarsfield, Rosario Central; Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Vasco, Grêmio; Colo-Colo, Universidad de Chile, Universidad Católica; Peñarol, Nacional; Olimpia, Cerro Porteño)')])
CAMPOS = [('classificação da Conmebol +0 tabela de 11 nações (registros de 10 bytes: +0 id da nação, +8 a cota, char), +0x71 id do campeão da Copa dos Campeões (−1 sem); id e valor; lê (conmebol_vagas_libertadores)',
           'CONMEBOL seeding +0 table of 11 nations (10-byte records: +0 nation id, +8 quota, char), +0x71 Copa dos Campeões winner id (−1 none); id and value; reads (conmebol_vagas_libertadores)'),
          ('clube +0 id, +0x1db competição continental (−1 sem vaga), +0x1df fase de entrada; id e valor; lê e grava (conmebol_vagas_libertadores), lê (conmebol_enesimo_clube_da_nacao, conmebol_convidado_da_mercosul)',
           'club +0 id, +0x1db continental competition (−1 none), +0x1df entry round; id and value; reads and writes (conmebol_vagas_libertadores), reads (conmebol_enesimo_clube_da_nacao, conmebol_convidado_da_mercosul)'),
          ('[0xae2c92] ano do jogo; valor; lê (conmebol_vagas_libertadores)',
           '[0xae2c92] game year; value; reads (conmebol_vagas_libertadores)')]
INTERESSA = ['CMRTE']
