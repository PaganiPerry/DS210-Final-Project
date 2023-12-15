// src/main.rs

mod OverallDegreeCentrality;
mod CategoryDegreeCentrality;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limit_exchanges = 10;
    OverallDegreeCentrality::analyze_exchanges(limit_exchanges).await?;

    let limit_categories = 5;
    CategoryDegreeCentrality::analyze_categories(limit_categories).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the analyze_exchanges function
    #[tokio::test]
    async fn test_analyze_exchanges() {
        // You can customize the test data or use mock data for testing
        let limit = 5;
        let result = OverallDegreeCentrality::analyze_exchanges(limit).await;

        // Add assertions based on the expected behavior of the function
        assert!(result.is_ok());
    }

    // Test the analyze_categories function
    #[tokio::test]
    async fn test_analyze_categories() {
        // You can customize the test data or use mock data for testing
        let limit = 5;
        let result = CategoryDegreeCentrality::analyze_categories(limit).await;

        // Add assertions based on the expected behavior of the function
        assert!(result.is_ok());
    }

    // Test the add_edges_to_graph function in OverallDegreeCentrality
    //Note: Consistently fails: 
    #[tokio::test]
    async fn test_add_edges_to_graph_overall() {
        let mut graph = petgraph::graph::DiGraph::<String, ()>::new();
        let platforms = vec!["platform1".to_string(), "platform2".to_string(), "platform3".to_string()];
    
        OverallDegreeCentrality::add_edges_to_graph(&mut graph, platforms.clone());
    
        // Print the content of the graph and the platforms vector
        println!("Graph: {:?}", graph);
        println!("Platforms: {:?}", platforms);
    
        // Update assertions based on the expected behavior of the function
        let expected_edge_count = platforms.len() * (platforms.len() - 1);
        assert_eq!(graph.node_count(), platforms.len());
        assert_eq!(graph.edge_count(), expected_edge_count);
    }
    
    //NOTE: Consistently passes (grpah is constructed slightly differetly from before) 
    // Test the add_edges_to_graph function in CategoryDegreeCentrality
    #[tokio::test]
    async fn test_add_edges_to_graph_category() {
        let mut graph = petgraph::graph::DiGraph::<String, ()>::new();
        let platforms = vec!["platform1".to_string(), "platform2".to_string(), "platform3".to_string()];

        CategoryDegreeCentrality::add_edges_to_graph(&mut graph, platforms.clone());

        // Print the content of the graph and the platforms vector
        println!("Graph: {:?}", graph);
        println!("Platforms: {:?}", platforms);

        // Add assertions based on the expected behavior of the function
        assert_eq!(graph.node_count(), platforms.len());
        assert_eq!(graph.edge_count(), platforms.len() * (platforms.len() - 1));
    }
}
