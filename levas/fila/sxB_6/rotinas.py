# A opiniao da diretoria sobre os jogadores preferidos e rejeitados do presidente e o tamanho do elenco
# (MANAGER_MANAGER::build_board_squad_opinion_text 0x693850, manager_manager.cpp; leitor sxB, rodada 5, 24/11/2026). A
# rotina inteira (0x693850..0x694403, ret 0xc em 0x694188 e mais oito; depois as tabelas de salto 0x694404, 0x694414 e
# 0x694424 do numero de frases), em pedacos com nome: SquadOpinion e squad_opinion_players (as tres listas),
# squad_opinion_sentence (as frases) e a rotina (os indicadores do elenco e a juncao).
# Chamada pelo texto da noticia da confianca da diretoria (0x6a4e4e, news_board_confidence_update_text 0x6a4d40, ja em
# Rust, pelo endereco do exe: roda na carreira automatica pelo modo CM_TEXTO_NOTICIAS quando sai essa noticia) e pela
# tela da confianca da diretoria (0x6b22cd). Item de SIMULACAO.
# Prologo: mov eax, 0x12f8; call _chkstk: o Prologue de 5 bytes (so o mov) dispensa RELATIVAS.
# Convencoes conferidas na desmontagem (cada push seguido ate a chamada):
# - a rotina: thiscall(manager_manager, buf, clube, ids), ret 0xc, devolve al (quantos nomeou); o ecx nao e lido;
# - 0x53b900 cdecl(clube); 0x53c590 database_count_club_squad cdecl(clube, 0, 0) -> bl (com sinal); 0x5a0590
#   thiscall([0xae2a38], clube); 0x5986b0 get_wealth_band thiscall(financa) -> al; 0x5480f0 e 0x548090
#   cdecl(presidente, lista, &n); 0x4cf5a0 get_contract thiscall([0xae1050], pessoa) ret 4 (a resposta gravada em
#   0xae1080, G_CONTRACT_LOOKUP_RESULT); 0x8cca10 thiscall([0xde1f54], pessoa, 0) ret 8; 0x544520 cdecl(pessoa, clube);
#   0x4cf5f0 get_contract_for_club thiscall([0xae1050], pessoa, clube) ret 8 (chamada duas vezes); 0x53dba0
#   cdecl(pessoa, buf, 200, 1); 0x54cdb0 date_days_since thiscall(0xae2c90, dia, ano) ret 8; 0x66f4e0 langlib_sprintf.
# Defeito do original: com tres frases o modelo de duas sobrescreve o de tres (o codigo cai de um caso no outro), e a
# terceira frase some.
SAVE = False
ROTINAS = [('build_board_squad_opinion_text', 0x693850, [0xb8, 0xf8, 0x12, 0x00, 0x00])]
DOCS = ('a opiniao da diretoria sobre os preferidos e rejeitados do presidente', "the board's opinion of the chairman's favoured and disliked players",
        [('build_board_squad_opinion_text', 'MANAGER_MANAGER::build_board_squad_opinion_text', '0x693850', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'escreve a opiniao da diretoria sobre os jogadores do presidente e devolve quantos nomeou (com os ids): nada sem clube, presidente, ficha dele ou numa selecao; o elenco e pequeno abaixo de 15, inchado acima de 30 com reputacao abaixo de 2000 ou acima de 45 abaixo de 3250 (rico: faixa 0xfe tambem acima de 40 abaixo de 3500; outras faixas nao incham); os rejeitados do clube abaixo da reputacao dele, nao vendidos nem listados e sem Bosman (a liberar), os preferidos do clube com menos de 36 anos e reputacao acima da do clube menos 750 (valiosos) e os preferidos de outros clubes com reputacao acima de 4500 e perto da do clube, com menos de 35 (reforcos); as frases de cada lista e a juncao pelo numero de frases (rico e inchado sem ninguem a liberar: o clube nao sustenta o elenco; pequeno sem ninguem a liberar: precisa de mais profundidade; senao a simples). Defeito do original: com tres frases o modelo de duas sobrescreve o de tres',
          "writes the board's opinion of the chairman's players and returns how many it named (with their ids): nothing without a club, a chairman, his sheet or for a national team; the squad is small under 15, oversized above 30 under reputation 2000 or above 45 under 3250 (wealthy: band 0xfe also above 40 under 3500; other bands are never oversized); the club's disliked players below its reputation, not sold on or listed and not leaving on a Bosman (to release), the club's favoured players under 36 with a reputation above the club's minus 750 (valuable) and the favoured players elsewhere with a reputation above 4500 close to the club's, under 35 (targets); each list's sentence and the join by the number of sentences (wealthy and oversized with nobody to release: the club cannot sustain the squad; small with nobody to release: it needs more depth; else the plain one). Defect of the original: with three sentences the two-sentence form overwrites the three-sentence one")])
