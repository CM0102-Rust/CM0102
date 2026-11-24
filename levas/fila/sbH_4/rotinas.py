# A gravacao e a carga dos amistosos (friendly.cpp; leitor sbH, rodada 5, 24/11/2026; as sobras do sbG, ver
# levas/fila/sbG_5/FORA.md). SAVE com CARGA: o Friendly.tmp em CM3_TEMP, que o zadd_matching_files leva ao .qst.
# - 0x5c9650 Friendly::save_to_temp_file thiscall(nao lido) ret 4 (2,5 KB ate o ret 4 de 0x5c9f7f, mais a cauda das
#   caixas ate o ret 4 de 0x5ca01e; chamada so pela vtable +0x1c): fopen 0x945b00 (utils_build_file_path 0x90bde0
#   ("CM3_TEMP", "Friendly.tmp"), "wb"; sem arquivo a caixa 0x135b); fwrite 0x945ce6 de +0xa7 (8), +0xaf (2), +0x34dc,
#   +0x34e0, +0x34e4 (4) e, com contagem e vetor, o plano (+0x34d8, 0x14 bytes cada); os 127 torneios com a sede e os
#   convidados trocados pelo id (-1 sem; sede vazia -1 e os convidados como estao), gravados de uma vez e desfeitos,
#   cada competicao (+0x65) pela virtual +0x1c (1); as quatro listas com a contagem reservada (ftell 0x946aae, um dword
#   0; caixa), os registros (amistosos 0x19 com +0xc, +0x10, +0x14 por id; o byte +0x64 da vaga de cada torneio
#   pendente; a data (8) e o clube +0xc (4) de cada resposta; os nos de 0x107 com o clube +0, a nacao +4 (id; volta pela
#   G_NATION_TABLE de 0x122) e os tres clubes de cada vaga), a contagem regravada (fseek 0x946a22 (lugar, 0); caixa)
#   quando a lista existe e o fim (fseek (0, 2)); caixas 0x13b6/0x13ee, 0x1401/0x141c, 0x142f/0x1457, 0x146a/0x14c6;
#   +0x34f4 (2; caixa 0x14d2); por nacao ([0xae2350]) a contagem em palavra da lista de +0x34f6 (caixa 0x14de) e os
#   itens de 4 bytes (caixa 0x14ea); fclose 0x945a09. Cada caixa fecha o arquivo e para. Os locais pela pilha
#   (frame.py): a contagem em [quadro+0x10] (o push do ftell fica pendente ate o fwrite), this em [quadro+0x14], o lugar
#   da quarta lista em [quadro+0x18]. Prologo sub esp, 0x210 (6 bytes).
# - 0x5c8a00 Friendly::load_from_temp_file thiscall(nao lido) ret 4 -> 1/0 (3,1 KB, quadro SEH; da vtable +0x18 e do
#   construtor 0x5c1f80, 0x5c2217): sem arquivo 0 (sem caixa); fread 0x945bfe na mesma ordem; o plano com malloc
#   (capacidade x 0x14; caixa 0x121a) e a contagem +0x34e0; os torneios com os clubes de volta e, com o ponteiro salvo
#   da competicao nao nulo, a competicao refeita do registro [0xae23d0] + 0x6b x (+0xaf + vaga): kind 2 MiniLeague (new
#   0xf6, 0x766800 (registro)), kind 1 MiniCup (new 0xbc, 0x765e30), outro kind fica o ponteiro velho (defeito); cada
#   lista: a contagem (caixa), a velha liberada (LLIST::~LLIST 0x68d040 e operator delete 0x944c94), com contagem uma
#   nova (new 0xe, LLIST::LLIST 0x68d020) e os itens por LLIST::add_tail 0x68d0d0 (amistosos 0x19; a vaga do indice se
#   em uso, senao 0; o amistoso de Friendly::find_friendly_match 0x5c88f0 (data, clube); os nos de 0x107); caixas
#   0x1257, 0x1287, 0x12ad, 0x12db (sem fechar o arquivo: defeito); +0x34f4 (caixa 0x131b); por nacao uma lista nova em
#   +0x34f6 (as velhas nao sao liberadas), a contagem (caixa 0x1329) e os itens (caixa 0x1334); fclose e 1. O no local da
#   quarta lista nasce pelo vector constructor iterator 0x9463af (10 x 0x19, 0x645800, destrutor vazio 0x5a53c0): a
#   copia usa um buffer, ja que o fread grava os 0x107 bytes antes do uso. Prologo push -1; push 0x95a33b (7 bytes).
# Comparadores: so a original (gravam o arquivo, ou liberam e montam as listas e as competicoes); a prova e a do save
# (prova_save.py --leva N, com a carga do save de ferias). Conferir na prova se o save de ferias tem torneios, pedidos
# e vagas agendadas (as mutacoes tocam o que todo save tem: o cabecalho, as 127 vagas e as listas por nacao).
# Defeitos do original: na carga, depois das caixas 0x121a, 0x1257, 0x1287, 0x12ad e 0x12db o arquivo fica aberto; um
# torneio de kind fora de 1 e 2 com competicao fica com o ponteiro velho; as listas por nacao velhas nao sao liberadas.
# Depende do sbH_3 (SCHEDULED_SLOT_COUNT, SCHEDULED_SLOTS, SCHEDULED_SLOT_SIZE). Iguais no cm0102-1080.exe (inteiras).
SAVE = True
CARGA = True
ROTINAS = [('friendly_save_to_temp_file', 0x5c9650, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('friendly_load_from_temp_file', 0x5c8a00, [0x6a, 0xff, 0x68, 0x3b, 0xa3, 0x95, 0x00])]
DOCS = ('a gravação e a carga dos amistosos', 'saving and loading the friendlies',
        [('friendly_save_to_temp_file', 'Friendly::save_to_temp_file', '0x5c9650', 'friendly.cpp', 'thiscall, ret 4',
          'gravar o jogo grava os amistosos: a data base da pré-temporada e o primeiro id de torneio, o plano de amistosos da IA, as 127 vagas de torneio (e a competição de cada torneio), os amistosos marcados, os torneios pendentes, os pedidos à espera de resposta, as vagas agendadas, o último dia dos amistosos internacionais e, por nação, a lista dos amistosos internacionais; clubes e nações vão pelo id',
          "saving a game writes the friendlies: the pre-season base date and the first tournament id, the AI friendly plan, the 127 tournament slots (and each tournament's competition), the arranged friendlies, the pending tournaments, the requests awaiting a reply, the scheduled slots, the last day of the international friendlies and, per nation, its list of international friendlies; clubs and nations go by their ids"),
         ('friendly_load_from_temp_file', 'Friendly::load_from_temp_file', '0x5c8a00', 'friendly.cpp', 'thiscall, ret 4',
          'carregar o jogo lê os amistosos na ordem em que foram gravados, refaz a competição de cada torneio (miniliga ou minicopa) e as listas, trocando os ids de volta por clubes e nações',
          "loading a saved game reads the friendlies back in the order they were saved, rebuilds each tournament's competition (mini league or mini cup) and the lists, turning the ids back into clubs and nations")])
CAMPOS = []
INTERESSA = []
