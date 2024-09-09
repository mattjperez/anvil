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


        let c = new_client();
        let r = c.get(incus_path("/instances")).send()
            .await.unwrap()
            .json::<Instances>()
            .await;

        println!("{:#?}", r);

        assert_ok!(r);
        Ok(())
    }
}
