use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub modules: Vec<Module>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Hash, Eq, PartialEq)]
pub struct Module {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub readme: Option<String>,
    pub documentation: String,
    pub description: String,
    #[serde(default)]
    pub kind: ModuleKind,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub weak_dependencies: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ModuleKind {
    #[default]
    DataPack,
    ResourcePack,
}
