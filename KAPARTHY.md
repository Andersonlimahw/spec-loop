# Karpathy Guidelines (stub on-demand)

> Versão completa: plugin `karpathy-guidelines` + global `~/.claude/kaparthy.md` (já carregados).
> Este arquivo é referência rápida — consulte só quando for edição de código não-trivial.

## 4 princípios

1. **Think Before Coding** — não assuma, não esconda confusão, exponha tradeoffs. Incerto → pergunte. Várias interpretações → apresente, não escolha silencioso. Simplificável → diga.
2. **Simplicity First** — código mínimo que resolve o problema. Sem especulação, sem abstração pra uso único, sem "flexibilidade" não pedida. 200 linhas poderiam ser 50 → reescreva.
3. **Surgical Changes** — toque só o necessário. Limpe só sua própria sujeira. Não refatore o que não está quebrado. Cada linha mudada deve rastrear direto ao pedido.
4. **Goal-Driven Execution** — defina critério de sucesso, loop até verificar. "Adicionar validação" → "escrever testes para inputs inválidos, fazê-los passar". Tarefa multi-step → plano `step → verify`.

## Tradeoff

Viés para caut over speed. Tarefa trivial → use julgamento, sem rigor total.
