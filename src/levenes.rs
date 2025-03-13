use statrs::distribution::{ContinuousCDF, FisherSnedecor};

pub struct LevenesTest {
    pub df: f64,
    pub estimate: f64,
    pub p_value: f64,
}


impl LevenesTest {
    pub fn execute(grupos: Vec<Vec<f64>>) -> LevenesTest {
        // Número de grupos e tamanho total da amostra
        let k = grupos.len();
        let n_total: usize = grupos.iter().map(|g| g.len()).sum();
    
        // Cálculo das medianas de cada grupo
        let medianas: Vec<f64> = grupos.iter().map(|g| {
            let mut g_clone = g.clone();
            g_clone.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let mid = g_clone.len() / 2;
            if g_clone.len() % 2 == 0 {
                (g_clone[mid - 1] + g_clone[mid]) / 2.0
            } else {
                g_clone[mid]
            }
        }).collect();
    
        // Cálculo das diferenças absolutas em relação às medianas
        let z: Vec<Vec<f64>> = grupos.iter().zip(medianas.iter()).map(|(g, &mediana)| {
            g.iter().map(|&x| (x - mediana).abs()).collect()
        }).collect();
    
        // Média geral das diferenças absolutas
        let z_barra_barra = z.iter().flatten().copied().sum::<f64>() / n_total as f64;
    
        // Soma dos quadrados entre grupos
        let ss_between = z.iter().map(|z_i| {
            let z_i_barra = z_i.iter().copied().sum::<f64>() / z_i.len() as f64;
            z_i.len() as f64 * (z_i_barra - z_barra_barra).powi(2)
        }).sum::<f64>();
    
        // Soma dos quadrados dentro dos grupos
        let ss_within = z.iter().map(|z_i| {
            let z_i_barra = z_i.iter().copied().sum::<f64>() / z_i.len() as f64;
            z_i.iter().map(|&z_ij| (z_ij - z_i_barra).powi(2)).sum::<f64>()
        }).sum::<f64>();
    
        // Graus de liberdade
        let df_between = (k - 1) as f64;
        let df_within = (n_total - k) as f64;
    
        // Estatística F
        let f_stat = (ss_between / df_between) / (ss_within / df_within);
    
        // Distribuição F para calcular o valor p
        let f_dist = FisherSnedecor::new(df_between, df_within).unwrap();
        let p_value = 1.0 - f_dist.cdf(f_stat);
    
        LevenesTest {
            df: df_within,
            estimate: f_stat,
            p_value,
        }
    }
}