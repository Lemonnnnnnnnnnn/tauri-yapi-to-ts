use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

use serde_json::from_str;

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
    file.write_all(serde_json::to_string(&yapi_config)?.as_bytes())?;
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

pub fn get_specific_project_config(source_path: &str) -> Result<YapiConfig, io::Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(string_to_path_buf(source_path.to_string()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(from_str(&contents)?)
}

pub fn write_project_config(source_path: &str, yapi_config: YapiConfig) -> Result<(), io::Error> {
    // why use truncate?
    // write mean overwrite file, Overwriting doesn't mean you're overwriting the entire content of a file. Say your file has 8 As in it, then you write 3 Xs into to the buffer with the write(true) argument supplied, you have now overwritten the first 3 As, meaning your file now contains XXXAAAAA, see, your overwrote some data (example taken from here). Rust doesn't automatically truncate (remove file contents without removing the file) the file, for that you need to also call truncate(true)
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(string_to_path_buf(source_path.to_string()).join(PROJECT_CONFIG_NAME))?;
    file.write(serde_json::to_string(&yapi_config)?.as_bytes())?;

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

pub fn merge_config_projects(source_path: &str, other_path: &str) -> Result<(), io::Error> {
    let mut source_config = get_project_config(source_path)?;
    let other_config = get_specific_project_config(other_path)?;

    for o_project in other_config.project_list {
        let is_project_exist = source_config
            .project_list
            .iter()
            .any(|project| project.project_id == o_project.project_id);

        if !is_project_exist {
            source_config.project_list.push(o_project);
        } else {
            let source_project = source_config
                .project_list
                .iter_mut()
                .find(|p| p.project_id == o_project.project_id)
                .unwrap();

            for o_category in o_project.categories {
                let is_category_exist = source_project
                    .categories
                    .iter()
                    .any(|category| category.id == o_category.id);

                if !is_category_exist {
                    source_project.categories.push(o_category);
                } else {
                    let source_category = source_project
                        .categories
                        .iter_mut()
                        .find(|c| c.id == o_category.id)
                        .unwrap();

                    for o_interface in o_category.interfaces {
                        let is_interface_exist = source_category
                            .interfaces
                            .iter()
                            .any(|interface| interface.id == o_interface.id);
                        if !is_interface_exist {
                            source_category.interfaces.push(o_interface);
                        }
                    }
                }
            }
        }
    }

    write_project_config(source_path, source_config)?;
    Ok(())
}
