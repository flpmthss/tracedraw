# O painel Efeitos

Um objeto pode ter vários efeitos vivos ao mesmo tempo, aplicados um depois do
outro. O painel [[docker.effects]] lista os efeitos do objeto selecionado na
ordem em que são aplicados, de cima para baixo. Abra-o com
[[menu.window > menu.window.dockers > menu.window.dockers_effects]].

![O painel Efeitos](../../img/effects-effects-docker.png)

## Gerenciar efeitos {#stack}

Cada linha da lista tem:

- uma caixa de seleção que ativa ou desativa o efeito. Um efeito desativado
  mantém o lugar e as configurações, mas não é desenhado, exportado nem
  impresso;
- o nome do efeito, esmaecido enquanto está desativado;
- setas para cima e para baixo que movem o efeito para antes ou depois, o que
  muda o resultado (por exemplo, um contorno de um envelope, ou um envelope de
  um contorno);
- um X que remove o efeito.

Cada alteração é um passo de desfazer. Cada efeito continua sendo editado com a
própria ferramenta ou painel.

Abaixo da lista, os visuais prontos dão aos objetos selecionados um acabamento
com um clique: cromado, metal líquido, ouro, vidro líquido (que desfoca o que
está por baixo) e vidro transparente. Cada imagem mostra o objeto selecionado
com o visual; um desfazer o remove. As suas predefinições salvas vêm depois.

## Meio-tom

Na lista Efeitos de camada, **Meio-tom** transforma o objeto selecionado,
como ele fica com os outros efeitos (uma sombra interna para o visual de
tubo arredondado, um brilho, uma sobreposição), numa trama de pontos, e
continua ao vivo: mude o objeto ou os outros efeitos e os pontos acompanham.

- **Ponto**: redondo, quadrado, losango ou linha.
- **Modo de cor**: Luz RGB faz pontos de luz vermelha, verde e azul, do
  tamanho de cada canal de cor, para um visual pontilhado sobre fundo
  escuro; Tinta CMYK imprime pontos ciano, magenta, amarelo e preto sobre
  branco; Uma cor desenha pontos da cor ao lado, do tamanho da escuridão.
- **Espaço entre pontos**, **Escala do ponto** (em 100 % um tom cheio cobre
  a célula), **Suavidade** das bordas dos pontos e **Ângulo** da trama.
- **Centro X** e **Centro Y** posicionam um ponto na largura e na altura do
  objeto.
- **Cortar pela borda do objeto**: ligado, os pontos terminam na borda;
  desligado, pontos inteiros, menores onde o objeto os cobre menos.

Uma sombra interna na lista Efeitos de camada também tem **Espalhamento**:
mantém a sombra sólida nessa parte do desfoque antes de ela sumir, para
uma borda mais funda e arredondada.

## Outros comandos de efeito {#commands}

- [[menu.effects > menu.effects.flatten]] transforma os efeitos dos objetos
  selecionados em objetos simples.
- [[menu.effects.picker]] mostra uma lista de efeitos para aplicar à seleção.
- [[menu.object > menu.object.clear_effect]] remove o efeito da seleção.
- [[menu.object > menu.object.copy_effect]] copia um efeito de outro objeto, e
  [[menu.object.clone_effect]] faz um clone que o acompanha; veja
  [Copiar e clonar efeitos](copy-and-clone.md).
