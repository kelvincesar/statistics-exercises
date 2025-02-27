use peroxide::fuga::*;
use colored::Colorize;
use statrs::distribution::{Binomial, Discrete};

fn main() { 
    println!("Running exercises!");
    exercise_01();
    exercise_02();
    exercise_03();
    exercise_04();
    exercise_05();
}

fn exercise_01() {
    println!("{}", "01) Uma companhia fabrica motores. As especificações requerem que o comprimento de uma certa haste deste motor esteja entre 7,48 cm e 7,52 cm. Os comprimentos destas hastes, fabricadas por um fornecedor, têm uma distribuição normal com média 7,505 cm e desvio padrão 0,01 cm. Qual a probabilidade de uma haste escolhida ao acaso estar dentro das especificações?".blue().bold());

    let mu = 7.505;
    let sigma = 0.01;

    let upper_bound = 7.52;
    let lower_bound = 7.48;

    // criando uma distribuição normal
    let normal_dist = Normal(mu, sigma);

    // cálculo da probabilidade acumulada;
    let p_lower = normal_dist.cdf(lower_bound);
    let p_upper = normal_dist.cdf(upper_bound);

    let p_final = p_upper - p_lower;

    println!("R: probabilidade de estar dentro das especificações: {:.4} \n", p_final * 100.0);
}

fn exercise_02() {
    println!("{}", "02) A distribuição da altura de 500 estudantes do sexo masculino de uma escola é aproximadamente Normal com média igual a 1,70 metro e desvio padrão igual a 2,5 centímetros. Aproximadamente quantos têm altura superior a 1,65m?".blue().bold());

    let mu = 1.70;
    let sigma = 2.5 / 100.0; // converter para m
    
    let total_students = 500.0 as f64;
    let height_threshold = 1.65;
    
    let normal = Normal(mu, sigma);
    // cálculo da probabilidade acumulada x <= 1.65
    let p_below = normal.cdf(height_threshold);

    let p = 1.0 - p_below;
    let expected_students = p * total_students;

    println!("R: probabilidade de alunos com maios de 1,65m: {:.0} ({:.4})\n", expected_students, p);
}

fn exercise_03() {
    println!("{}", "03) Uma clínica de emagrecimento recebe pacientes adultos com peso seguindo uma distribuição normal com média 130 kg e desvio padrão 20 kg. Para efeito de determinar o tratamento mais adequado, os 25% pacientes de menor peso são classificados de “magros”, enquanto dos 25% de maior peso de “obesos”. Determine os valores que delimitam cada uma dessas classificações.".blue().bold());

    let mu = 130.0;
    let sigma = 20.0;

    // percentis (primeiro e terceiro quartil)
    let p1 = 0.25;
    let p3 = 0.75;

    let q1 = normal_quantile(p1, mu, sigma);
    let q3 = normal_quantile(p3, mu, sigma);

    println!("R: limite para classificado como magro: {:.1}", q1);
    println!("   limite para classificado como obeso: {:.1}\n", q3);
}

fn exercise_04() {
    println!("{}", "04) O tempo de utilização de um caixa eletrônico por clientes de um certo banco, em minutos,  foi modelado por uma variável T com distribuição exponencial com parâmetro igual a 3. Determine a probabilidade de que um cliente demore menos de um minuto utilizando o caixa eletrônico.".blue().bold());

    let lambda = 3.0;
    let t = 1.0;

    let p_less_than_1 = 1.0 - (-lambda * t).exp();

    println!("R: Probabilidade de um cliente demorar menos de um minuto: {:.4}", p_less_than_1);

}

fn exercise_05() {
    println!("{}", "05) Um estudo mostra que em 60% dos casos de divórcio requeridos num certo município, a incompatibilidade é apontada como causa. Encontre a probabilidade de que entre 14 casos de divórcio requeridos naquele município mais de 12 apontem a incompatibilidade como causa.");

    let n = 14; // número de divórcios
    let p = 0.6;
    // Queremos encontrar a probabilidade de mais de 12 casos apontarem incompatibilidade.
    // Para isso é necessário usar a distribuição binomial que é ideal para modelar problemas
    // que envolvem reptições independentes de um evento com dois possíveis resultados (successo ou
    // fracasso). Neste caso, cada divórcio pode ou não apontar imcompatibilidade como causa, sendo
    // a probabilidade constante de 60%
    let binomial = Binomial::new(p, n as u64).unwrap();

    // calcula P(X = 13) e P(X = 14) usando PMF (probability mass function)
    // Calculamos para cada valor que seja maior que 12 até o limite do enunciado de 14.
    let p_13 = binomial.pmf(13);
    let p_14 = binomial.pmf(14);
    // calcula P(X > 12) = P(X = 13) + P(X = 14)
    let p_more_than_12 = p_13 + p_14;

    println!("Probabilidade de mais de 12 casos: {:.4}", p_more_than_12);

}

fn normal_quantile(p: f64, mu: f64, sigma: f64) -> f64 {
    mu + sigma * (2.0_f64.sqrt()) * inv_erf(2.0 * p - 1.0)
}


