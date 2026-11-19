# Revolver e inflar 3D

Revolver e inflar 3D dá a um desenho plano um visual 3D sombreado feito de
formas vetoriais comuns: gire um perfil para fazer um vaso ou uma garrafa
com o seu rótulo em volta, ou infle uma forma como uma almofada.

## Fazer um objeto 3D {#make}

Abra o painel [[docker.shape3d]] em
[[menu.effects > menu.effects.shape3d]].

1. Selecione uma curva ou forma. A cor do preenchimento (ou do contorno) é a
   cor que recebe o sombreado.
2. Escolha o modo:
   - [[docker.shape3d_revolve]] gira a curva em volta de um eixo vertical na
     borda esquerda, como um torno de oleiro: desenhe metade do contorno de
     um vaso, garrafa ou copo. Ajuste [[docker.shape3d_segments]],
     [[docker.shape3d_tilt]] (quanto você olha de cima),
     [[docker.shape3d_turn]], [[docker.shape3d_sweep]] (menos de 360 o deixa
     aberto) e [[docker.shape3d_axis_right]].
   - [[docker.shape3d_inflate]] infla a forma; [[docker.shape3d_depth]]
     define a altura.
3. Ajuste a luz: [[docker.shape3d_light_angle]],
   [[docker.shape3d_light_height]] e [[docker.shape3d_shine]].
4. Para aplicar uma arte na superfície, selecione a arte (um logotipo, um
   rótulo, texto convertido em curvas) e clique em [[docker.shape3d_art]];
   depois selecione a curva de novo. Posicione com [[docker.shape3d_art_u]]
   (50 é a frente), [[docker.shape3d_art_v]] e [[docker.shape3d_art_size]].
   A arte virada para trás fica escondida.
5. Clique em [[docker.apply]]. [[docker.shape3d_keep]] mantém a curva
   original.

O resultado é um grupo de formas, selecionado, e um desfazer o remove.
