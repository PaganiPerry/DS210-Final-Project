
use reqwest;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use petgraph::{graph::DiGraph, visit::Dfs};
use rand::prelude::SliceRandom;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let limit = 5; // You can adjust the limit as needed
    analyze_categories(limit).await?;

    Ok(())
}

pub async fn analyze_categories(limit: usize) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = "7718c6d3-6ec6-4903-834e-1b1d27b6f072"; // Replace with your actual API key

    // Get total categories
    let categories_url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/categories";
    let categories_response = reqwest::Client::new()
        .get(categories_url)
        .query(&[("CMC_PRO_API_KEY", api_key)])
        .send()
        .await?;
    let categories_body = categories_response.text().await?;
    let categories_json: Value = serde_json::from_str(&categories_body)?;

    // Choose random categories
    let mut selected_categories = HashMap::new();
    if let Some(arr) = categories_json["data"].as_array() {
        for category in arr.choose_multiple(&mut rand::thread_rng(), limit) {
            if let Some(id) = category["id"].as_str() {
                if let Some(name) = category["name"].as_str() {
                    selected_categories.insert(id.to_string(), name.to_string());
                }
            }
        }
    }

    // Iterate through selected categories
    for (category_id, category_name) in &selected_categories {
        analyze_category(category_id, category_name, api_key).await?;
    }

    Ok(())
}

pub async fn analyze_category(
    category_id: &str,
    category_name: &str,
    api_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get assets for the category
    let assets_url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/category";
    let assets_response = reqwest::Client::new()
        .get(assets_url)
        .query(&[("CMC_PRO_API_KEY", api_key), ("id", category_id)])
        .send()
        .await?;
    let assets_body = assets_response.text().await?;
    let assets_json: Value = serde_json::from_str(&assets_body)?;

    // Create a graph to represent relationships between platforms
    let mut graph = DiGraph::<String, ()>::new();

    // Iterate through assets and get platform names
    if let Some(data) = assets_json["data"].as_object() {
        if let Some(coins) = data["coins"].as_array() {
            // Collect platform names
            let platforms: Vec<String> = coins
                .iter()
                .filter_map(|coin| {
                    coin["platform"]["name"]
                        .as_str()
                        .map(|name| name.trim().to_lowercase())
                })
                .collect();

            // Add edges to the graph based on platform relationships
            add_edges_to_graph(&mut graph, platforms);
        } else {
            println!("Error: Unable to extract coins array from data {:?}", &data);
        }
    } else {
        println!("Error: Unable to extract data object from assets_json {:?}", &assets_json);
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
        "The most popular platform for category {} ({}) is {} with degree centrality {}",
        category_name,
        category_id,
        most_popular_platform.0,
        most_popular_platform.1
    );

    Ok(())
}

// Helper function to add edges to the graph
pub fn add_edges_to_graph(graph: &mut DiGraph<String, ()>, platforms: Vec<String>) {
    let mut nodes = HashMap::new();

    for platform in &platforms {
        let node = *nodes.entry(platform.clone()).or_insert_with(|| graph.add_node(platform.clone()));

        for other_platform in &platforms {
            if platform != other_platform {
                let other_node = *nodes.entry(other_platform.clone()).or_insert_with(|| graph.add_node(other_platform.clone()));
                graph.add_edge(node, other_node, ());
            }
        }
    }
}
