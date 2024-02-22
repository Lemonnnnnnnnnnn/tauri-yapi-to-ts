use std::path::PathBuf;

use super::{
    category::Category,
    context::Context,
    web_response::{CategoryMenuItem, CommonResponse},
};
type Response = CommonResponse<Vec<CategoryMenuItem>>;

#[derive(Debug)]
pub struct Project {
    pub id: String,
    pub token: String,
    context: Context,
}

impl Project {
    // 初始化project
    pub fn new(id: String, token: String, context: Context) -> Self {
        Self { id, token, context }
    }

    pub fn from_source_path(id: String, token: String, source_path: PathBuf) -> Self {
        let context = Context::from_source_path(&source_path);
        Self { id, token, context }
    }

    // 获取分类数据
    pub async fn fetch_categories(&self) -> Result<Vec<Category>, String> {
        match self.fetch_cat_menu().await {
            Ok(cat_menu) => {
                let cat_menu_data = cat_menu.data;

                let categories: Vec<Category> = cat_menu_data
                    .iter()
                    .map(|cat| {
                        let context = self.context.clone();
                        let id = cat._id.to_string();
                        let name = cat.name.clone();
                        Category::new(self.token.clone(), context, id, name)
                    })
                    .collect();

                Ok(categories)
            }
            Err(_) => Err("fetch cat menu error".to_owned()),
        }
    }

    // 获取分类目录
    async fn fetch_cat_menu(&self) -> Result<Response, Box<dyn std::error::Error>> {
        let get_cat_menu_url = format!(
            "{}/api/interface/getCatMenu?project_id={}&token={}",
            self.context.base_url.clone().unwrap(),
            self.id,
            self.token.clone()
        );

        let client = reqwest::Client::new();
        let res = client
            .get(get_cat_menu_url)
            .send()
            .await?
            .json::<Response>()
            .await?;

        Ok(res)
    }
}

// #[cfg(test)]
// mod test {
//     use super::super::context::get_test_context;
//     use super::*;

//     #[tokio::test]
//     #[ignore]
//     async fn test() -> Result<(), Box<dyn std::error::Error>> {
//         // let context = get_test_context();
//         // let id = "765";

//         // let mut project = Project::new(id.to_string(), TEMP_TOKEN.into(), context);
//         // project.fetch_categories().await.update_all().await;

//         Ok(())
//     }
// }
