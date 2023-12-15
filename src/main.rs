// src/main.rs

#[cfg(test)]

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
mod test {
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
}