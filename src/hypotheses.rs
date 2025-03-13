
use crate::levenes::LevenesTest;
use colored::Colorize;

pub fn exercicio_01() {
    println!("{}", "01) Queremos verificar se 2 máquinas produzem peças com a
                    mesma variabilidade quanto ao diâmetro (em mm). Para isso,
                    sorteamos vinte amostras de peças de cada máquina e
                    obtivemos as seguintes medidas do diâmetro (em mm)".blue().bold());

    let maquina_a = vec![145.0,142.0,136.0,133.0,131.0,131.0,132.0,127.0,138.0,139.0,138.0,131.0,139.0,140.0,123.0,137.0,128.0,137.0,133.0,132.0];
    let maquina_b = vec![133.0,123.0,133.0,130.0,129.0,129.0,133.0,137.0,142.0,151.0,135.0,139.0,151.0,136.0,141.0,147.0,145.0,139.0,151.0,139.0];

    let grupos = vec![maquina_a, maquina_b];
    let result = LevenesTest::execute(grupos);

    println!("Estatística F: {}", result.estimate);
    println!("Valor p: {}", result.p_value);
    println!("DF entre: {}", result.df);
    println!("R: O valor p é {:.2}, que é maior que o nível de significância típico de 0,05. 
    Portanto, não há evidências suficientes para rejeitar a hipótese nula. As variâncias dos diâmetros das peças
     produzidas pelas duas máquinas podem ser consideradas estatisticamente iguais (homocedásticas)", result.p_value);
 }
