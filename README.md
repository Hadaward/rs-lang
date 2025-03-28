# RS Lang
Este projeto foi inspirado em um desafio criado por uma comunidade brasileira, no qual é solicitado ao desafiado que implemente um interpretador de uma linguagem de programação simples em português, capaz de:
- Declarar variáveis
- Declarar laços de repetição
- Declarar funções
- Executar instruções simples de aritmética e lógica
- Interpretar caracteres Unicode em nomes de funções e variáveis  

## Futuro
A princípio, a ideia era seguir exatamente o que foi proposto no desafio. No entanto, decidi adotar uma abordagem diferente e manter o padrão seguido por outras linguagens de programação. Com isso em mente, este projeto será refeito com uma nova filosofia.  

Nada é definitivo ainda, mas as ideias principais são:  
- Implementar uma linguagem com suporte total a UTF-8  
- Implementar declarações de módulos que possibilitem a importação e exportação de tipos e objetos para outros módulos  
- Implementar declarações de estruturas e funções inspiradas em Rust, proporcionando a flexibilidade que o modelo clássico de herança não permite  
- Implementar uma linguagem fortemente tipada, mas que ofereça a mesma flexibilidade encontrada no TypeScript com seus tipos genéricos  

Inicialmente, a ideia da linguagem é que o escopo global seja um ambiente limpo e agradável, sem poluição desnecessária. Tudo o que for necessário estará disponível no módulo `std` (que significa Standard, ou, em tradução livre, Padrão), que será automaticamente registrado no interpretador.  

### Sintaxe
Tudo o que está nesta seção é apenas uma ideia e pode mudar a qualquer momento.  

```js
import { print, read, exit } from "std";
import { randint } from "std/random";

int randomNumber = randint(1, 10);
int userNumber = read({
  message: "Escolha um número aleatório entre 1 e 10."
})?;

if (userNumber < 1 || userNumber > 10) {
  print("O número escolhido não está no intervalo fornecido entre 1 e 10.");
  exit(1);
}

if (userNumber == randomNumber) {
  print("Você adivinhou o número! Parabéns!");
} else {
  print(f"Você escolheu o número errado! O número que escolhi foi {randomNumber}.");
}
```

Vamos começar explicando o sistema de módulos. Ele será inspirado no sistema de módulos do JavaScript. Como será uma linguagem interpretada, a importação cíclica de módulos será permitida. A única diferença é que aqui não haverá uma palavra-chave ``default``.
Todo módulo deve exportar os valores que deseja disponibilizar usando a palavra-chave export. Exemplo: ``export int MATH_PI = 3.14;``. Todo valor exportado deve ter um identificador para que possa ser importado através da palavra-chave ``import``.
O sistema de módulos usará o caminho dos arquivos como referência para encontrar o módulo. A diferença é que não será necessário o uso de extensões de arquivo ao importar um módulo. Apesar da inspiração vinda do JavaScript, o funcionamento desse sistema será bem parecido com o que temos em Rust.

Em seguida, temos a estrutura de declaração de variáveis, que na verdade é muito simples: ``tipo Identificador = expressão;``. Não quero me alongar muito aqui, mas vale destacar que, apesar da linguagem ser fortemente tipada, adotarei a palavra-chave ``auto``, emprestada do C++, para inferência de tipos. Isso torna o código mais fácil de lidar em certas situações onde o tipo inferido pela expressão é óbvio.
Uma curiosidade pode surgir ao ver o ``?`` logo após a chamada da função ``read``. Programadores de Rust vão reconhecê-lo como o símbolo especial para tratamento de resultados. No entanto, aqui ele terá uma funcionalidade especial e diferente. O ``?`` pode, sim, servir para tratar resultados, mas também terá um papel especial de auto-conversão de tipos.
Tipos declarados e tipos primitivos poderão implementar um método de conversão que tentará transformar o valor retornado no tipo esperado. Se a conversão for impossível, o programa falhará com um erro, a menos que o erro seja devidamente tratado.

O restante não tem muitos segredos: temos a declaração de estruturas condicionais, o uso de operadores lógicos e chamadas de funções. Em breve, estarei atualizando este documento para incluir exemplos de funções e outras sintaxes também.

Opiniões construtivas são bem-vindas!
