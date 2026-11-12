# Os órgãos disciplinares (Discipline::create_rule_sets 0x55d420, discipline.cpp; leitor sma, 12/11/2026). Convenções
# conferidas:
# - a rotina: thiscall(this, arg), ret 4;
# - 0x945501 (calloc 0x2b × 4) e 0x944cf1 (operator new 0x23) da CRT: cdecl;
# - os 43 construtores (0x7db100..0x7e43c0, originais): push arg (ou push 0); mov ecx, objeto -> thiscall, ret 4;
#   objeto nulo: a vaga fica 0;
# - a condição de cada um: o byte +0x11c da nação da chave (KEY_NATION_*, [0xae23a8] + id × 0x122); o da Oceania:
#   0x6699d0 (cdecl) com [0xae23a4] + [KEY_CONTINENT_OCEANIA] × 0xc6; os dois últimos: |[0xdd7110]| abaixo de 0x3e044 /
#   0x3e047 (cdq; xor; sub) passam 0 ao construtor;
# - a caixa de erro (discipline.cpp, linha 0x161e): show_error_at.
# Comparador: os órgãos são construídos uma vez por jogo; só a original roda e a tabela que ela deixa é conferida
# (um objeto exatamente nas vagas das ligas selecionadas; a da Oceania não é refeita).
SAVE = False
ROTINAS = [('discipline_create_rule_sets', 0x55d420, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('os órgãos disciplinares das ligas',
        'the discipline rule sets of the ruling bodies',
        [('discipline_create_rule_sets', 'Discipline::create_rule_sets', '0x55d420', 'discipline.cpp', 'thiscall, ret 4',
          'aloca a tabela de 43 órgãos disciplinares e constrói um objeto de 0x23 bytes por órgão ativo, na ordem do original: os três gerais sempre; os de cada país (Argentina, Bélgica, Brasil, Dinamarca, Inglaterra, Finlândia, França, Alemanha, Holanda, Irlanda, Itália, Japão, Noruega, Portugal, Escócia, Espanha, Suécia, EUA, Polônia, Grécia, Croácia, Austrália, Irlanda do Norte, Turquia, Rússia, País de Gales, Coreia do Sul; dois para alguns) quando a liga dele está selecionada; o da Oceania pelo continente; os dois últimos recebem 0 no construtor com dados mais antigos que a versão 0x3e044 / 0x3e047',
          'allocates the 43-entry ruling body table and constructs one 0x23-byte object per active ruling body, in the original\'s order: the three general ones always; each country\'s (Argentina, Belgium, Brazil, Denmark, England, Finland, France, Germany, Holland, Ireland, Italy, Japan, Norway, Portugal, Scotland, Spain, Sweden, USA, Poland, Greece, Croatia, Australia, Northern Ireland, Turkey, Russia, Wales, South Korea; two for some) when its league is selected; Oceania\'s by the continent; the last two get 0 in the constructor with data older than version 0x3e044 / 0x3e047')])
