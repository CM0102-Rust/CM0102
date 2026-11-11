r"""Traduz os nomes locais e os parâmetros em português do motor, palavra por palavra (etapa 4 da reorganização,
08/11/2026; o leitor de fora, rodada 3: ~85% dos locais de mais de uma letra estavam em português).

Regras (para o código gerado não mudar):
- só troca um nome quando todas as palavras dele têm tradução no dicionário abaixo ou já são inglês/abreviação curta;
- a troca vale dentro de uma função de topo de cada vez, só no código (nunca em comentário nem em texto entre aspas);
- o nome novo não pode já existir na função, nem ser nome de item de topo (função, constante, módulo), nem palavra
  reservada;
- as listas de mutação recebem, em cada trecho, a troca da função onde ele está.

Uso: python traduzir_locais.py [--gravar]
"""
import os
import re
import runpy
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import mover_rotinas  # noqa: E402
import renomear_rotinas  # noqa: E402

D = dict(x.split(':') for x in '''clube:club clubes:clubs pessoa:person pessoas:persons fase:stage fases:stages lista:list ano:year anos:years
velho:old nacao:nation nacoes:nations nac:nation tipo:kind jogador:player jogadores:players linha:row linhas:rows vaga:slot vagas:slots
saida:out saidas:outs tabela:table tab:table nome:name liga:league jogo:game jogos:games lado:side humano:human rodadas:rounds
rodada:round antes:before construir:construct dia:day dias:days contrato:contract novo:new nova:new soltar:release
noticia:news noticias:news texto:text fim:end valor:value gravar:write registro:record registros:records partida:fixture
tecnico:manager tec:manager regioes:regions codigo:code selecao:national_team proximo:next prox:next oferta:offer ofertas:offers
ficha:sheet conta:count alvo:target casa:home marca:mark marcas:marks igual:same liberar:free depois:after copa:cup
ordenar:sort tirar:remove outro:other outra:other nota:rating motivo:reason hoje:today atual:current entrada:entry
sorteio:draw peso:weight formatar:format chamar:call plano:plan copiar:copy padrao:default gerente:manager chave:key
modo:mode melhor:best destruir:destroy fora:away arquivo:file tamanho:size tam:size time:team times:teams copia:copy
ler:read origem:source mes:month dono:owner soma:sum entregar:deliver vetor:array mascara:mask teto:ceiling total:total
menos:minus comprador:buyer esperado:expected idade:age idioma:language situacao:status vendedor:seller marcar:mark
inicio:start achou:found segunda:second segundo:second abrir:open grupo:group cargo:job caminho:path acrescentar:append
fator:factor parceiro:partner regra:rule regras:rules nivel:level cor:colour fechar:close destino:target quantos:how_many
elenco:squad campo:field ultimo:last ultima:last nao:not principal:main forca:strength versao:version busca:search
posicao:position categoria:category media:average estado:state amistoso:friendly financas:finances primeiro:first
primeira:first quem:who achar:find caixa:box competicao:competition indice:index lesao:injury alto:high livre:free
estadio:stadium gosta:likes realocar:realloc continente:continent fundo:bottom sortear:draw criar:create rotina:routine
narrar:narrate iniciar:init minuto:minute premio:prize lance:play emprestimo:loan troca:swap sair:exit montar:build
previstas:predicted limpar:clear tatica:tactic semana:week semanas:weeks completo:full zero:zero tem:has piso:floor
separar:split lidos:read celula:cell bloco:block fatia:slice forma:form atributo:attribute desfecho:outcome buscar:search
adversario:opponent divisao:division juntar:join pedido:request contar:count forte:strong registrar:register longe:far
estatisticas:stats estat:stats falta:foul fila:queue palavra:word banco:bench papel:role baixo:low minimo:minimum
historico:history pontos:points topo:top trocar:swap volta:return somar:add dois:two apagar:erase resultado:result
especial:special temporada:season resumo:summary escolhido:chosen agendar:schedule anterior:previous agora:now
resposta:reply reserva:reserve comum:common todos:all escrever:write carregar:load pasta:folder perto:near
guardado:stored mandante:home guardar:store dados:data erro:error pagar:pay lugar:place proprio:own pode:can meio:middle
vantagem:advantage seguinte:next dado:given alcance:reach ajustar:adjust narrador:commentator numero:number titulo:title
pior:worst preencher:fill avisar:warn gols:goals estatuto:status traduzir:translate goleiro:goalkeeper mesma:same
mesmo:same final:final gol:goal confronto:tie preparar:prepare visitante:away vencedor:winner perdedor:loser campeao:champion
vice:runner_up salario:salary verba:budget torneio:tournament cidade:city treinador:coach arbitro:referee olheiro:scout
diretor:director escolha:choice antigo:old achado:found feito:done meio:middle ida:first_leg maior:greater menor:smaller
quantas:how_many anterior:previous dias:days mes:month semana:week horario:time grupo:group grupos:groups jogada:play
chute:shot bola:ball dividida:tackle disputa:contest marcador:marker marcadores:markers acesso:promotion rebaixamento:relegation
rebaixados:relegated promovidos:promoted pontuacao:score empate:draw vitoria:win derrota:loss saldo:goal_difference
reputacao:reputation publico:attendance capacidade:capacity financeiro:financial receita:income despesa:expense
diretoria:board torcida:fans moral:morale felicidade:happiness habilidade:ability potencial:potential atual:current
pai:parent filho:child primeiros:first ultimos:last novos:new velhos:old vazio:empty cheio:full ativo:active ativa:active
dentro:inside chegada:arrival partida:fixture ponto:point unidade:unit unidades:units dezena:ten
alocar:alloc mais:more motor:engine ramo:branch mata:knockout forcar:force ajuste:adjust checa:check referencia:reference
janela:window janelas:windows colunas:columns coluna:column altura:height largura:width futura:future genero:gender
variante:variant posicoes:positions campos:fields fonte:font ataque:attack lesoes:injuries todas:all aceita:accepted
ordem:order razao:ratio desmarcar:unmark preco:price destinatarios:recipients conferir:check confere:checks resto:rest
metade:half qual:which arquivar:archive sempre:always trocas:swaps periodo:period interessados:interested termos:terms
listas:lists evento:event eventos:events cartao:card cartoes:cards formacao:formation refazer:redo direto:direct
casos:cases orgao:body disciplina:discipline ferias:holiday pares:pairs gravidade:severity presidente:chairman
manter:keep midia:media punir:punish expulso:sent_off tentativas:attempts tentativa:attempt cores:colours usado:used
bonus:bonus sorteados:drawn sorteado:drawn escolhidos:chosen candidatos:candidates candidato:candidate faltam:missing
falto:missing restante:remaining restantes:remaining quantidade:amount limiar:threshold saldo:balance ganho:gain
perda:loss chance:chance ofertante:bidder interessado:interested lista_negra:blacklist recusa:refusal aceite:accept'''.split())
# as palavras inglesas que o próprio dicionário produz contam como inglês (nomes já meio traduzidos)
# round 30 (11/11/2026, the outside reader's round 12: vaga, caiu, feliz, folego, grava, por, fala...)
D.update(dict(x.split(':') for x in '''lance:play mandante:home_side visitante:away_side estadio:stadium premio:prize
formacoes:formations bloco:block momento:moment ambicao:ambition cumprida:served velha:old parada:stop feito:done
caiu:fell feliz:happy folego:stamina grava:write fala:say quantas:how_many ganhou:won perdeu:lost empate:draw
vitoria:win derrota:defeat gols:goals gol:goal falta:foul cartao:card amarelo:yellow vermelho:red escolha:choice
escolhido:chosen proposta:proposal resposta:reply pedido:request dinheiro:money salario:wage verba:funds
torcida:supporters diretoria:board treino:training treinador:coach olheiro:scout juiz:referee arbitro:referee'''.split()))
INGLES_DO_DIC = set(w for v in D.values() for w in v.split('_'))
PREP = {'do': 'of', 'da': 'of', 'de': 'of', 'dos': 'of', 'das': 'of', 'no': 'in', 'na': 'in', 'nos': 'in', 'nas': 'in',
        'em': 'in', 'com': 'with', 'sem': 'without', 'por': 'by', 'pelo': 'by', 'pela': 'by', 'ao': 'to', 'para': 'for',
        'ate': 'until', 'e': 'and', 'o': '', 'a': '', 'os': '', 'as': '', 'um': 'one', 'uma': 'one'}
INGLES = set('''this id base extra ok item zip data local stream use of limit band contract rep dif pos fin lim mo mr cap est
cl idx buf rd ub nt lo national team div it st ev step bl towards dx dy rec is c2 s2 np sm al dd ax col nr pa arg dist cursor
tmp max nj tb par grav t2 pick pj lr off pd l2 0c nb cx cand f2 sai injury club min key to in out on at by up down new old
value count size index len ptr addr flag flags mask bit bits byte bytes word code mode type kind sum avg ratio rate result
next prev first last left right top bottom high low start end begin open close read write load save find get set put add
sub mul div mod copy move swap sort list table row rows col cols cell grid map key keys val vals tmp temp aux src dst buf
str msg err res ret arg args opt cfg ctx env obj ref ptr box vec hash lock time date day days year years month week'''.split())
# rodada 9 (08/11/2026): as palavras que mais bloqueavam a tradução do resto do nome
D.update(dict(x.split(':') for x in '''sorteia:draws topico:topic fazer:do inscritos:entrants redesenhar:redraw prazo:deadline
teste:test comunitaria:community decidir:decide guardada:stored faixa:band acertar:fix lados:sides chaves:keys pular:skip
corte:cut estrangeira:foreign gravados:written relogio:clock qualquer:any escolher:choose caso:case expectativas:expectations
aplicar:apply dar:give velha:old codigos:codes decide:decides notas:notes barra:bar itens:items painel:panel favorito:favourite
zerar:clear suspenso:suspended geral:general objeto:object voltar:go_back comparar:compare inicial:initial entra:enters
selecoes:national_teams mapa:map frente:front interesse:interest regiao:region seguir:follow antiga:old listada:listed
classe:class saiu:left laco:loop ignorar:ignore comissao:staff acao:action marcado:marked meu:my avancar:advance margem:margin
parte:part fichas:sheets opcao:option destaque:highlight modelo:template cota:quota sementes:seeds embaralhar:shuffle
som:sound vez:time usa:uses cod:code lin:row pres:presence ini:start cont:count'''.split()))
# rodada 19 (08/11/2026)
D.update(dict(x.split(':') for x in '''escritas:written escritos:written temporaria:temporary sobrenome:surname filtro:filter
metodo:method passa:passes canto:corner vence:wins mundo:world receptor:receiver datas:dates anfitriao:host
abaixo:below desempate:tiebreak previsto:predicted serie:series errado:wrong quieto:quiet limitar:limit remover:remove
curto:short estilo:style fixo:fixed datar:date pula:skips fatias:slices placar:scoreboard marcada:marked contagem:count
mudou:changed segue:follows passou:passed chutador:shooter agregado:aggregate resta:left ativos:active abas:tabs
atracao:attraction etapa:step custo:cost fixos:fixed nela:in_it onde:where gravado:written previstos:predicted pesos:weights
fica:stays voltas:laps ensaio:dry_run rascunho:draft desejado:wanted tres:three desligar:disable amanha:tomorrow
vinte:twenty meia:half marcados:marked adjetivo:adjective atualizar:update corpo:body minima:minimum hora:hour
sinal:sign expulsar:send_off narrou:narrated dominou:controlled rapida:fast arriscado:risky iguais:equal
espalhar:spread entradas:entries acima:above propria:own conduz:carries foto:photo subtipo:subtype assunto:subject
seis:six compensacao:compensation lugares:places grade:grid cauda:tail premios:prizes turnos:rounds pote:pot
reduz:reduces venceu:won funcao:function devolver:return tempo:time maiuscula:capital abreviado:abbreviated
longo:long terceiro:third sim:yes divisor:divisor terco:third contador:counter cinco:five porcento:percent'''.split()))
# rodada 20 (10/11/2026)
D.update(dict(x.split(':') for x in '''contratos:contracts argumento:argument deslocamento:offset esperada:expected obtido:got
como:how valida:valid dos:of_the reacao:reaction escolhida:chosen de:of no:in_the da:of_the designado:designated movido:moved
parado:stopped limite:limit mensagem:message avaliacao:rating estrangeiro:foreigner achada:found opcoes:options sem:without
escalacao:lineup contratado:signed zerado:cleared momento:moment acumulado:accumulated emprestado:loaned acertado:agreed
ponderados:weighted compartilhado:shared ofuscado:dimmed ocupado:occupied verdade:truth parada:stopped ocupada:occupied
chamada:call valido:valid inteiro:whole severidade:severity qualidade:quality ousado:bold menu:menu jogado:played
traduzido:translated forcado:forced anunciada:announced forcada:forced simulado:simulated aumento:raise gravada:written
listado:listed meses:months recusada:refused dinheiro:money negociacoes:negotiations vende:sells calado:silent novas:new
duracao:duration virada:turned ligada:on formacoes:formations real:real imagem:image reiniciado:restarted reputado:reputed
ambicao:ambition nasc:birth classificado:qualified copiado:copied cumprida:served atuacao:performance anulada:cancelled
convocado:called_up orcamento:budget rapido:fast assinado:signed punido:punished narrado:narrated titular:starter
contado:counted somado:summed atacada:attacked defendida:defended alocacao:allocation disposicao:disposition cansado:tired
machucado:injured ruido:noise dirigido:managed recuado:withdrawn decidido:decided angulo:angle fechado:closed fechada:closed
velocidade:speed narracao:commentary muito:very rolagem:scroll ruim:bad abandonada:abandoned valores:values
conectado:connected montagem:assembly vencido:expired montado:built acompanhado:followed conferido:checked pedida:requested
repartido:split lealdade:loyalty funcoes:functions reproduzido:replayed instrucao:instruction vencida:expired fracoes:fractions
repetido:repeated ponteiro:pointer preferido:preferred cancelado:cancelled reaproveitado:reused resolvido:resolved
paridade:parity conferida:checked tentar:try divisoes:divisions repetida:repeated liquido:net divida:debt
aquecimento:heating duvida:doubt batida:taken corrigido:fixed usada:used errada:wrong liberado:released
emprestada:loaned montada:built ordenado:sorted respondido:answered rebaixado:relegated ao:to_the'''.split()))
INGLES |= set('''l4c ops2 desl dest alt sel desloc sev l28 difference difs cat global obs fechar2 penalties adv item16 display
pts den esc pct transf for disp l3c desc masc ecx l24 tabela1 ult cmp cur pot nat signed sprintf esi kit ban txt'''.split())
INGLES |= set('''original qsort without with area bsearch delta fread until zread ops ids fwrite zwrite and calloc dir rust
malloc uefa mem one the regen conf bond respond penalty restart taker edi percent pair cid pref reg comp lado1'''.split())
INGLES |= set(w for v in D.values() for w in v.split('_'))
INGLES |= INGLES_DO_DIC
CURTO = re.compile(r'^[a-z]{1,3}\d*$|^[a-z]\d+$|^\d')
NOME = re.compile(r'\blet (?:mut )?([a-z_][a-z0-9_]*)\b|[(,|]\s*(?:mut )?([a-z_][a-z0-9_]*): ')


def traduzir(nome):
    partes = nome.split('_')
    saida = []
    mudou = False  # só com pelo menos uma palavra do dicionário (a variável `e` não vira `and`)
    for p in partes:
        if p in D:
            saida.append(D[p])
            mudou = True
        elif p in PREP and len(partes) > 1:  # preposição só dentro de nome composto
            if PREP[p]:
                saida.append(PREP[p])
        elif p in INGLES or CURTO.match(p) or p == '':
            saida.append(p)
        else:
            return None
    novo = '_'.join(x for x in saida if x != '')
    novo = re.sub(r'_+', '_', novo).strip('_')
    if not mudou or not novo or novo == nome or not re.fullmatch(r'[a-z_][a-z0-9_]*', novo):
        return None
    return novo


def topo_global():
    # só o espaço de nomes dos valores (fn, const, static; e os `use`): um local pode ter o nome de um módulo ou de um
    # tipo (`club + club::DIVISION` compila: o caminho `club::` é procurado entre os módulos)
    s = set()
    for p in renomear_rotinas.fontes():
        if os.path.basename(p) in ('records.rs', 'exe_calls.rs', 'fields.rs'):
            continue  # methods (`x.club()`) and path-qualified items (`exe::f`, `club::F`) do not clash with a local
        t = open(p, encoding='utf-8').read()
        s |= set(re.findall(r'^\s*(?:pub(?:\(crate\))? )?(?:unsafe )?(?:extern "\w+" )?(?:fn|const|static(?: mut)?|use) ([A-Za-z_]\w*)', t, re.M))
    return s


def so_codigo(t, f):
    """Aplica f só nas partes de código (fora de comentários e de textos entre aspas), linha a linha."""
    out = []
    for l in t.split('\n'):
        cod, sep, com = l.partition('//')
        partes = re.split(r'("(?:[^"\\]|\\.)*")', cod)
        out.append(''.join(p if p.startswith('"') else f(p) for p in partes) + sep + com)
    return '\n'.join(out)


ANTES = {}


def voltar_funcoes_com_erro(trocas_mut):
    """Compila; cada função com erro (ex.: um campo de struct com o mesmo nome de um local) volta inteira ao texto de
    antes, e os trechos de mutação dela não mudam. Repete até compilar."""
    import subprocess
    for volta in range(15):
        r = subprocess.run(['cargo', 'build', '--release', '--message-format=short'], cwd=os.path.join(R, 'rust'),
                           capture_output=True, text=True, encoding='utf-8', errors='replace')
        erros = re.findall(r'^(src[^:]+):(\d+):\d+: error', r.stderr, re.M)
        if not erros:
            return
        n = 0
        por = {}
        for f, l in erros:
            por.setdefault(os.path.normpath(os.path.join(R, 'rust', f)), set()).add(int(l))
        for p, linhas in por.items():
            if p not in ANTES:
                continue
            t = open(p, encoding='utf-8').read()
            velho = ANTES[p]
            bn, bv = mover_rotinas.blocos(t), mover_rotinas.blocos(velho)
            if len(bn) != len(bv):
                continue
            partes, pos = [], 0
            for (nn, a, b), (nv, av, bvv) in zip(bn, bv):
                l0, l1 = t.count('\n', 0, a) + 1, t.count('\n', 0, b) + 1
                if any(l0 <= l <= l1 for l in linhas) and t[a:b] != velho[av:bvv]:
                    partes.append(t[pos:a])
                    partes.append(velho[av:bvv])
                    pos = b
                    n += 1
                    for k in [k for k, (m, f_) in trocas_mut.items() if velho[av:bvv].count(m[1]) == 1]:
                        del trocas_mut[k]
            partes.append(t[pos:])
            open(p, 'w', encoding='utf-8').write(''.join(partes))
        print('volta %d: %d erros, %d funções voltaram ao texto de antes' % (volta, len(erros), n))
        if n == 0:
            sys.exit('erros que não vêm desta troca:\n' + r.stderr[-2000:])


def main():
    globais = topo_global() | renomear_rotinas.RESERVADAS
    mutacoes = []
    pasta = os.path.join(R, 'engenharia-reversa', 'mutacoes')
    for f in sorted(os.listdir(pasta)):
        if f.endswith('.py'):
            try:
                mutacoes += [(f, m) for m in runpy.run_path(os.path.join(pasta, f)).get('MUTACOES', [])]
            except Exception:
                pass
    trocas_mut = {}  # (lista, nome da mutação) -> dict de troca
    total = fns = 0
    for d, _, fs in os.walk(os.path.join(SRC, 'motor')):
        for f in fs:
            if not f.endswith('.rs'):
                continue
            p = os.path.join(d, f)
            t = open(p, encoding='utf-8').read()
            ANTES[p] = t
            partes, pos = [], 0
            for nome, a, b in mover_rotinas.blocos(t):
                bloco = t[a:b]
                cod = re.sub(r'//[^\n]*|"(?:[^"\\\n]|\\.)*"', ' ', bloco)
                ids = set(re.findall(r'\b[A-Za-z_]\w*\b(?!::)', cod))  # `club::X` does not take the name `club`
                locais = {m.group(1) or m.group(2) for m in NOME.finditer(cod)} - {None}
                troca, novos = {}, set()
                for n in sorted(locais):
                    novo = traduzir(n)
                    if novo and novo not in ids and novo not in globais and novo not in novos:
                        troca[n] = novo
                        novos.add(novo)
                if not troca:
                    continue
                rx = re.compile(r'\b(%s)\b' % '|'.join(map(re.escape, sorted(troca, key=len, reverse=True))))
                f_ = lambda s, rx=rx, troca=troca: rx.sub(lambda m: troca[m.group(1)], s)
                novo_bloco = so_codigo(bloco, f_)
                for lista, m in mutacoes:
                    if bloco.count(m[1]) == 1:
                        trocas_mut[(lista, m[0])] = (m, f_)
                partes.append(t[pos:a])
                partes.append(novo_bloco)
                pos = b
                total += len(troca)
                fns += 1
            partes.append(t[pos:])
            novo = ''.join(partes)
            if novo != t and '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8').write(novo)
    print('%d nomes trocados em %d funções; %d trechos de mutação acompanham' % (total, fns, len(trocas_mut)))
    if '--gravar' not in sys.argv:
        return
    voltar_funcoes_com_erro(trocas_mut)
    import json
    por_lista = {}
    for (lista, _), (m, f_) in trocas_mut.items():
        por_lista.setdefault(lista, []).append((m, f_))
    for lista, ms in por_lista.items():
        p = os.path.join(pasta, lista)
        s = open(p, encoding='utf-8').read()
        for m, f_ in ms:
            # a troca vale na tupla da própria mutação (vários trechos podem ter o mesmo texto): a partir de "('<nome>',"
            ini = s.find("('%s'," % m[0])
            if ini < 0:
                ini = s.find('("%s",' % m[0])
            if ini < 0:
                continue
            pos = ini
            for k in (1, 2):
                novo = so_codigo(m[k], f_)
                for velho_l, novo_l in ((repr(m[k]), repr(novo)), (json.dumps(m[k], ensure_ascii=False), json.dumps(novo, ensure_ascii=False))):
                    j = s.find(velho_l, pos)
                    if j >= 0:
                        s = s[:j] + novo_l + s[j + len(velho_l):]
                        pos = j + len(novo_l)
                        break
        open(p, 'w', encoding='utf-8').write(s)
    print('gravado')


if __name__ == '__main__':
    main()
