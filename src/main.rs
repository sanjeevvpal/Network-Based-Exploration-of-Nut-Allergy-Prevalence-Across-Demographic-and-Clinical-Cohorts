use std::error::Error;
use std::collections::HashMap;
use csv::{ReaderBuilder, Error as CsvError};
use petgraph::graph::{DiGraph, NodeIndex};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Record {
    subject_id: String,
    birth_year: i32,
    gender_factor: String,
    race_factor: String,
    ethnicity_factor: String,
    payer_factor: String,
    atopic_march_cohort: bool,
    age_start_years: f64,
    age_end_years: f64,
    peanut_alg_start: Option<f64>,
    peanut_alg_end: Option<f64>,
    treenut_alg_start: Option<f64>,
    treenut_alg_end: Option<f64>,
    walnut_alg_start: Option<f64>,
    walnut_alg_end: Option<f64>,
    pecan_alg_start: Option<f64>,
    pecan_alg_end: Option<f64>,
    pistach_alg_start: Option<f64>,
    pistach_alg_end: Option<f64>,
    almond_alg_start: Option<f64>,
    almond_alg_end: Option<f64>,
    brazil_alg_start: Option<f64>,
    brazil_alg_end: Option<f64>,
    hazelnut_alg_start: Option<f64>,
    hazelnut_alg_end: Option<f64>,
    cashew_alg_start: Option<f64>,
    cashew_alg_end: Option<f64>,
}

#[derive(Debug)]
struct Individual {
    id: String,
    gender: String,
    race: String,
    ethnicity: String,
    payer_factor: String,
    atopic_march_cohort: bool,
}

enum NodeType {
    Individual(Individual),
    NutAllergyStatus(String),
}

fn read_csv(file_path: &str) -> Result<Vec<Record>, CsvError> {
    let mut rdr = ReaderBuilder::new().from_path(file_path)?;
    rdr.deserialize().collect()
}

fn create_graph(records: Vec<Record>) -> DiGraph<NodeType, ()> {
    let mut graph = DiGraph::new();
    let mut individual_nodes = HashMap::new();
    let mut allergy_nodes = HashMap::new();

    let allergies = [
        "Peanut", "Treenut", "Walnut", "Pecan", "Pistachio", "Almond", "Brazil",
        "Hazelnut", "Cashew",
    ];

    for &allergy in allergies.iter() {
        let node = graph.add_node(NodeType::NutAllergyStatus(allergy.to_string()));
        allergy_nodes.insert(allergy, node);
    }

    for record in records {
        let age = (record.age_start_years + record.age_end_years) / 2.0;
        let individual_node = graph.add_node(NodeType::Individual(Individual {
            id: record.subject_id.clone(),
            gender: record.gender_factor,
            race: record.race_factor,
            ethnicity: record.ethnicity_factor,
            payer_factor: record.payer_factor,
            atopic_march_cohort: record.atopic_march_cohort,
        }));
        individual_nodes.insert(record.subject_id.clone(), individual_node);

        for &allergy in allergies.iter() {
            if let Some(_) = record.get_allergy_start(allergy) {
                if let Some(&allergy_node) = allergy_nodes.get(allergy) {
                    graph.add_edge(individual_node, allergy_node, ());
                }
            }
        }
    }
    graph
}

impl Record {
    fn get_allergy_start(&self, allergy: &str) -> Option<f64> {
        match allergy {
            "Peanut" => self.peanut_alg_start,
            "Treenut" => self.treenut_alg_start,
            "Walnut" => self.walnut_alg_start,
            "Pecan" => self.pecan_alg_start,
            "Pistachio" => self.pistach_alg_start,
            "Almond" => self.almond_alg_start,
            "Brazil" => self.brazil_alg_start,
            "Hazelnut" => self.hazelnut_alg_start,
            "Cashew" => self.cashew_alg_start,
            _ => None,
        }
    }
}

fn calculate_centrality(graph: &DiGraph<NodeType, ()>) {
    let mut gender_centrality = HashMap::new();
    let mut race_centrality = HashMap::new();
    let mut ethnicity_centrality = HashMap::new();
    let mut payer_centrality = HashMap::new();
    let mut cohort_centrality = HashMap::new();
    let mut allergy_centrality = HashMap::new();
    let mut gender_counts = HashMap::new();
    let mut race_counts = HashMap::new();
    let mut ethnicity_counts = HashMap::new();
    let mut payer_counts = HashMap::new();
    let mut cohort_counts = HashMap::new();
    // Allergies to consider
    let allergies = [
        "Peanut", "Treenut", "Walnut", "Pecan", "Pistachio", "Almond", "Cashew",
    ];
    
    for node in graph.node_indices() {
        match &graph[node] {
            NodeType::Individual(individual) => {
                let degree = graph.neighbors(node).count() as f64;
                println!("Degree centrality for node {} (ID: {}): {}", node.index(), individual.id, degree);
                *gender_centrality.entry(individual.gender.clone()).or_insert(0.0) += degree;
                *race_centrality.entry(individual.race.clone()).or_insert(0.0) += degree;
                *ethnicity_centrality.entry(individual.ethnicity.clone()).or_insert(0.0) += degree;
                *payer_centrality.entry(individual.payer_factor.clone()).or_insert(0.0) += degree;
                *cohort_centrality.entry(individual.atopic_march_cohort.to_string()).or_insert(0.0) += degree;
                
                *gender_counts.entry(individual.gender.clone()).or_insert(0) += 1;
                *race_counts.entry(individual.race.clone()).or_insert(0) += 1;
                *ethnicity_counts.entry(individual.ethnicity.clone()).or_insert(0) += 1;
                *payer_counts.entry(individual.payer_factor.clone()).or_insert(0) += 1;
                *cohort_counts.entry(individual.atopic_march_cohort.to_string()).or_insert(0) += 1;
            }
            NodeType::NutAllergyStatus(allergy_status) => {
                if allergies.contains(&allergy_status.as_str()) {
                    let degree = graph.neighbors(node).count() as f64;
                    allergy_centrality.insert(allergy_status.clone(), degree);
                }
            }
            _ => {}
        }
    }
// Calculate and print average centrality for each group
for (gender, total_degree) in gender_centrality.iter() {
    let count = *gender_counts.get(gender).unwrap_or(&1) as f64;
    println!("Average degree centrality for gender {}: {}", gender, total_degree / count);
}
for (race, total_degree) in race_centrality.iter() {
    let count = *race_counts.get(race).unwrap_or(&1) as f64;
    println!("Average degree centrality for race {}: {}", race, total_degree / count);
}
for (ethnicity, total_degree) in ethnicity_centrality.iter() {
    let count = *ethnicity_counts.get(ethnicity).unwrap_or(&1) as f64;
    println!("Average degree centrality for ethnicity {}: {}", ethnicity, total_degree / count);
}
for (payer, total_degree) in payer_centrality.iter() {
    let count = *payer_counts.get(payer).unwrap_or(&1) as f64;
    println!("Average degree centrality for payer factor {}: {}", payer, total_degree / count);
}
for (cohort, total_degree) in cohort_centrality.iter() {
    let count = *cohort_counts.get(cohort).unwrap_or(&1) as f64;
    println!("Average degree centrality for atopic march cohort {}: {}", cohort, total_degree / count);
}

}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "path_to_your_csv_file.csv";
    let records = read_csv(file_path)?;
    let graph = create_graph(records);
    calculate_centrality(&graph);
    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;

    // Mock data to simulate the CSV reading and graph creation
    fn get_mock_records() -> Vec<Record> {
        vec![
            Record {
                subject_id: "205650".to_string(),
                birth_year: 2000,
                gender_factor: "Male".to_string(),
                race_factor: "Race1".to_string(),
                ethnicity_factor: "Ethnicity1".to_string(),
                payer_factor: "Payer1".to_string(),
                atopic_march_cohort: true,
                age_start_years: 5.0,
                age_end_years: 10.0,
                peanut_alg_start: Some(1.0),
                peanut_alg_end: Some(2.0),
                treenut_alg_start: None,
                treenut_alg_end: None,
                walnut_alg_start: None,
                walnut_alg_end: None,
                pecan_alg_start: None,
                pecan_alg_end: None,
                pistach_alg_start: None,
                pistach_alg_end: None,
                almond_alg_start: None,
                almond_alg_end: None,
                brazil_alg_start: None,
                brazil_alg_end: None,
                hazelnut_alg_start: None,
                hazelnut_alg_end: None,
                cashew_alg_start: None,
                cashew_alg_end: None,
            },
           
        ]
    }
    

    #[test]
    fn test_csv_reading() {
        let file_path = "path_to_mock_csv_file.csv"; // Replace with a path to a mock CSV file
        let records = read_csv(file_path).unwrap();
        assert!(!records.is_empty()); // Check that records are read
    }

    #[test]
    fn test_graph_creation() {
        let records = get_mock_records();
        let graph = create_graph(records);
        assert!(!graph.node_indices().is_empty()); // Check that nodes are created
    }

    #[test]
    fn test_centrality_calculation() {
        let records = get_mock_records();
        let graph = create_graph(records);
        calculate_centrality(&graph); 
        
    }

    #[test]
    fn test_allergy_node_creation() {
        let records = get_mock_records();
        let graph = create_graph(records);
        let allergy_nodes = graph.node_indices()
            .filter(|&n| matches!(graph[n], NodeType::NutAllergyStatus(_)))
            .count();
        assert!(allergy_nodes > 0); // Check that allergy nodes are created
    }
}

