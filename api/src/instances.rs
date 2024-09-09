use serde::{Deserialize, Serialize};

use crate::Base;

#[derive(Serialize, Deserialize, Debug)]
pub struct Instances {
    #[serde(flatten)]
    base: Base,
    metadata: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::*;

    use std::io::Result;
    use tokio_test::assert_ok;

    #[tokio::test]
    async fn test_instances_root() -> Result<()> {
        let r = test_get("/instances")
            .await
            .json::<Instances>()
            .await;

        assert_ok!(r);
        Ok(())
    }
}
