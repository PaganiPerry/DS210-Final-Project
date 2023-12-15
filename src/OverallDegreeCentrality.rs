use reqwest;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use petgraph::{graph::DiGraph, visit::Dfs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limit = 10; // You can adjust the limit as needed
    analyze_exchanges(limit).await?;

    Ok(())
}

pub async fn analyze_exchanges(limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "7718c6d3-6ec6-4903-834e-1b1d27b6f072"; // Replace with your actual API key

    // Get active exchanges
    let exchanges_url = "https://pro-api.coinmarketcap.com/v1/exchange/map";
    let exchanges_response = reqwest::Client::new()
        .get(exchanges_url)
        .query(&[
            ("CMC_PRO_API_KEY", api_key),
            ("listing_status", "active"),
            ("limit", &limit.to_string()),
            ("sort", "id"),
        ])
        .send()
        .await?;
    let exchanges_body = exchanges_response.text().await?;
    let exchanges_json: Value = serde_json::from_str(&exchanges_body)?;

    // Create a graph to represent relationships between platforms
    let mut graph = DiGraph::<String, ()>::new();

    // Iterate through exchanges and get assets
    // Iterate through exchanges and get assets
    for exchange in exchanges_json["data"].as_array().unwrap_or(&vec![]) {
      let exchange_id = exchange["id"].as_u64().unwrap_or(0).to_string();
      let assets_url = "https://pro-api.coinmarketcap.com/v1/exchange/assets";
      let assets_response = reqwest::Client::new()
          .get(assets_url)
          .query(&[("CMC_PRO_API_KEY", api_key), ("id", &exchange_id)])
          .send()
          .await?;
      let assets_body = assets_response.text().await?;
      let assets_json: Value = serde_json::from_str(&assets_body)?;

      // Debug print to check the structure of the JSON response
      dbg!(&assets_json);

      // Check if the data array is empty before collecting platform names
      if let Some(data_array) = assets_json["data"].as_array() {
          // Collect platform names
          let platforms: Vec<String> = data_array
              .iter()
              .filter_map(|asset| {
                  asset["platform"]["name"]
                      .as_str()
                      .map(|name| name.trim().to_lowercase())
              })
              .collect();

          // Add edges to the graph based on platform relationships
          add_edges_to_graph(&mut graph, platforms);
      } else {
          println!("Error: Unable to extract platform names from assets_json {:?}", &assets_json);
      }
  }

    // Calculate degree centrality
    let mut degree_centrality: HashMap<String, usize> = HashMap::new();
    let nodes: HashSet<_> = graph.node_indices().collect();

    for node in nodes {
        let mut dfs = Dfs::new(&graph, node);

        while let Some(visited) = dfs.next(&graph) {
            *degree_centrality.entry(graph[visited].clone()).or_insert(0) += 1;
        }
    }

    // Find the most popular platform
    let most_popular_platform = degree_centrality
        .iter()
        .max_by_key(|&(_, count)| count)
        .map(|(name, count)| (name.clone(), *count))
        .unwrap_or(("".to_string(), 0));

    // Print the most popular platform and its degree centrality
    println!(
        "The most popular platform is {} with degree centrality {}",
        most_popular_platform.0,
        most_popular_platform.1
    );
        // Normalize degree centrality values
    let max_degree = degree_centrality.values().cloned().max().unwrap_or(1) as f64;
    let normalized_degree_centrality: HashMap<String, f64> = degree_centrality
        .iter()
        .map(|(platform, count)| (platform.clone(), *count as f64 / max_degree))
        .collect();

    // Print the normalized degree centrality for all platforms
    for (platform, centrality) in &normalized_degree_centrality {
        println!("{}: {:.2}", platform, centrality);
    }

    Ok(())
}

// Helper function to add edges to the graph
pub fn add_edges_to_graph(graph: &mut DiGraph<String, ()>, platforms: Vec<String>) {
    for i in 0..platforms.len() {
        for j in (i + 1)..platforms.len() {
            let node_i = graph.add_node(platforms[i].clone());
            let node_j = graph.add_node(platforms[j].clone());
            graph.add_edge(node_i, node_j, ());
            graph.add_edge(node_j, node_i, ());
        }
    }
}

