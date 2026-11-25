# O texto do progresso nas eliminatorias asiaticas (wc_asia_league.cpp; leitor rx09, rodada 6, 25/11/2026). Item de
# TELAS, como o write_progress_text sul-americano do cx17_1: a virtual +0x38 so e lida pelas telas de competicoes de uma
# selecao. Caminho no jogo: numa carreira em 2005 (a campanha asiatica de 2006), o historico/competicoes de uma selecao
# asiatica, jogando comparando.
# - 0x912f80 WcAsiaLeague::write_progress_text thiscall(buf, tamanho, rodada, sub-rodada, situacao, competicao) ret 0x18:
#   tamanho < 30 (com sinal): a caixa (linha 0x3b6); a rodada (palavra com sinal) 0x3fd..0x406: situacao 6 ->
#   langlib_sprintf(buf, 0xad4824, genero +0x37 da competicao, 0xad9c64 ""), senao 0x9a7c04 "First Round"; 0x407..0x408:
#   7 -> 0xad4658, 1 -> 0xad47f8, senao 0x9a7b94 "Second Round"; 0x1e -> 0x9a7b10 "Third Round"; 0xa0 -> 0x9a751c
#   "Playoff"; outro nada. Prologo mov eax, [esp+8]; sub esp, 0x200 (10 bytes); igual no cm0102-1080.exe.
# Comparador: lado a lado (a original no buffer de quem chama, a copia num local do mesmo tamanho), so nas rodadas que
# escrevem. Usa o EMPTY_STRING do motor.
TELAS = True
SAVE = False
ROTINAS = [('wc_asia_league_write_progress_text', 0x912f80, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o texto do progresso nas eliminatórias asiáticas', 'the Asian World Cup qualifying progress text',
        [('wc_asia_league_write_progress_text', 'WcAsiaLeague::write_progress_text', '0x912f80', 'wc_asia_league.cpp', 'thiscall, ret 0x18',
          'regra: o texto da fase de uma seleção nas eliminatórias asiáticas: "Primeira Fase" ou "Classificada para a Segunda Fase"; "Segunda Fase", "Classificada para a Terceira Fase" ou "Classificada para a Copa do Mundo"; "Terceira Fase"; "Repescagem"',
          'rule: the stage text of a national team in the Asian qualifying: "First Round" or "Qualified For Second Round"; "Second Round", "Qualified For Third Round" or "Qualified For World Cup"; "Third Round"; "Playoff"')])
CAMPOS = []
INTERESSA = []
