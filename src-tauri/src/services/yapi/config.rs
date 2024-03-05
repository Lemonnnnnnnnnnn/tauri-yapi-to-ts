use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

use serde_json::{from_str, json};

use crate::{
    models::yapi::{
        category::{CategoryMenuItem, InterfaceDataItem},
        config::{YapiCategory, YapiConfig, YapiInterface, YapiProject},
        project::YapiProjectBaseInfo,
    },
    services::conversion::string_to_path_buf,
};

pub const PROJECT_CONFIG_NAME: &str = "yapi.json";

pub fn init_project_config(source_path: String) -> Result<(), io::Error> {
    let yapi_config = YapiConfig::default();
    let yapi_config_path = string_to_path_buf(source_path).join(PROJECT_CONFIG_NAME);

    if yapi_config_path.exists() {
        return Ok(());
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(yapi_config_path)?;
    file.write_all(json!(yapi_config).to_string().as_bytes())?;
    Ok(())
}

pub fn get_project_config(source_path: &str) -> Result<YapiConfig, io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(string_to_path_buf(source_path.to_string()).join(PROJECT_CONFIG_NAME))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(from_str(&contents)?)
}

pub fn write_project_config(source_path: &str, yapi_config: YapiConfig) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .open(string_to_path_buf(source_path.to_string()).join(PROJECT_CONFIG_NAME))?;
    file.write_all(json!(yapi_config).to_string().as_bytes())?;
    Ok(())
}

pub fn merge_yapi_project_to_project_config(
    source_path: &str,
    yapi_project_base_info: &YapiProjectBaseInfo,
    token: &str,
) -> Result<(), io::Error> {
    let mut yapi_config = get_project_config(source_path)?;
    if !yapi_config
        .project_list
        .iter()
        .any(|project| project.project_id == format!("{}", yapi_project_base_info._id))
    {
        yapi_config.project_list.push(YapiProject {
            token: token.to_string(),
            project_id: format!("{}", yapi_project_base_info._id),
            project_name: Some(yapi_project_base_info.name.clone()),
            categories: vec![],
        })
    };

    write_project_config(source_path, yapi_config)?;
    Ok(())
}

pub fn merge_category_to_project_config(
    category_menu_item: &CategoryMenuItem,
    source_path: &str,
    project_id: &str,
) -> Result<(), io::Error> {
    let mut yapi_config = get_project_config(source_path)?;
    // get the project match project_id in the yapi_config
    let project = yapi_config
        .project_list
        .iter_mut()
        .find(|project| project.project_id == project_id);

    if let Some(project) = project {
        if !project
            .categories
            .iter()
            .any(|category| category.id == category_menu_item._id.to_string())
        {
            project.categories.push(YapiCategory {
                id: category_menu_item._id.to_string(),
                name: category_menu_item.name.clone(),
                interfaces: vec![],
            })
        }
    }

    write_project_config(source_path, yapi_config)?;

    Ok(())
}

pub fn merge_interface_to_project_config(
    interface_data_item: &InterfaceDataItem,
    source_path: &str,
    cat_id: &str,
) -> Result<(), io::Error> {
    let mut yapi_config = get_project_config(source_path)?;

    'project: for project in &mut yapi_config.project_list {
        for category in &mut project.categories {
            if category.id == cat_id {
                if !category
                    .interfaces
                    .iter()
                    .any(|interface| interface.id == interface_data_item._id.to_string())
                {
                    category.interfaces.push(YapiInterface {
                        id: interface_data_item._id.to_string(),
                        name: interface_data_item.title.clone(),
                        path: Some(interface_data_item.path.clone()),
                        lock: Some(false),
                    })
                }
                break 'project;
            }
        }
    }

    write_project_config(source_path, yapi_config)?;

    Ok(())
}
