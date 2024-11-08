Funções em Rust

Uma função em Rust é uma forma de agrupar código que pode ser chamado várias vezes. As funções são definidas com a palavra-chave fn, seguida pelo nome da função, uma lista de parâmetros (se houver), e um tipo de retorno (se a função retornar algum valor).
Sintaxe básica de uma função

fn nome_da_funcao(parametro1: Tipo1, parametro2: Tipo2) -> TipoDeRetorno {
    // corpo da função
    // opcionalmente, pode retornar um valor
}

    Parâmetros: São os valores que a função pode receber quando é chamada. Eles devem ser explicitamente tipados.
    Tipo de retorno: O tipo do valor que a função retorna. Caso não retorne nada, usamos () (o tipo unitário).
    Corpo da função: Contém o código a ser executado. Rust inferirá o tipo de retorno automaticamente, mas também podemos explicitamente usar a palavra-chave return para devolver um valor.

Exemplo de função simples

fn saudacao(nome: &str) -> String {
    let mensagem = format!("Olá, {}!", nome);
    return mensagem;  // retorno explícito (opcional em Rust)
}

fn main() {
    let nome = "João";
    println!("{}", saudacao(nome)); // Chama a função
}

No exemplo acima, a função saudacao recebe uma referência a uma string (&str), cria uma saudação formatada e a retorna como uma string (String).
Função sem retorno

Se não há valor de retorno, usamos ():

fn imprime_mensagem() {
    println!("Mensagem sem retorno");
}

Structs em Rust

Em Rust, structs (estruturas) são usadas para definir tipos compostos, ou seja, para agrupar múltiplos valores que podem ser de tipos diferentes. As structs são a principal maneira de definir tipos personalizados.
Sintaxe básica de uma struct

struct NomeDaStruct {
    campo1: Tipo1,
    campo2: Tipo2,
}

    Campos: São as variáveis associadas a cada instância de uma struct.
    Tipo: Cada campo possui um tipo específico.

Exemplo de struct

struct Pessoa {
    nome: String,
    idade: u32,
}

fn main() {
    let pessoa = Pessoa {
        nome: String::from("Carlos"),
        idade: 30,
    };

    println!("Nome: {}, Idade: {}", pessoa.nome, pessoa.idade);
}

Neste exemplo, criamos uma struct Pessoa com dois campos: nome (do tipo String) e idade (do tipo u32). Depois, instanciamos a struct e acessamos seus campos.
Métodos em structs

Em Rust, é possível associar métodos a structs. Métodos são funções definidas dentro do contexto de uma struct e podem acessar ou modificar os campos da struct.

Para definir um método, usamos o impl (abreviação de "implementation"):

struct Retangulo {
    largura: u32,
    altura: u32,
}

impl Retangulo {
    // Método para calcular a área
    fn area(&self) -> u32 {
        self.largura * self.altura
    }
}

fn main() {
    let retangulo = Retangulo { largura: 10, altura: 5 };
    println!("Área do retângulo: {}", retangulo.area());  // Chama o método
}

Aqui, criamos uma struct Retangulo e associamos a ela um método area, que calcula a área com base nos campos largura e altura. O &self é uma referência à instância atual da struct, necessária para acessar seus campos dentro do método.
Resumo

    Funções: São definidas com fn, podem ter parâmetros e tipo de retorno. Podem ou não retornar valores, dependendo da necessidade.
    Structs: Permitem agrupar dados relacionados em um único tipo. Cada instância de uma struct pode ter valores diferentes para seus campos.
    Métodos: São funções associadas a structs, definidas dentro de um bloco impl. Usam &self (ou outras referências) para acessar dados da instância da struct.

Esses conceitos juntos formam a espinha dorsal do código em Rust, permitindo organizar e modularizar a lógica do programa de forma eficiente e segura. Se precisar de mais detalhes ou exemplos específicos, é só falar!