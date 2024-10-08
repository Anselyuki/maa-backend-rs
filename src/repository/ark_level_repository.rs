use bson::{doc, DateTime};
use futures::{stream::TryStreamExt, StreamExt};
use mongodb::{Collection, Database};
use serde::{Deserialize, Serialize};

use crate::MaaResult;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArkLevel {
    pub id: Option<String>,
    pub level_id: Option<String>,
    pub stage_id: Option<String>,
    // 文件版本, 用于判断是否需要更新
    pub sha: String,
    // 地图类型, 例: 主线、活动、危机合约
    pub cat_one: Option<String>,
    // 所属章节, 例: 怒号光明、寻昼行动
    pub cat_two: Option<String>,
    // 地图ID, 例: 7-18、FC-1
    pub cat_three: Option<String>,
    // 地图名, 例: 冬逝、爱国者之死
    pub name: Option<String>,
    pub width: i32,
    pub height: i32,
    // 只是服务器认为的当前版本地图是否开放
    pub is_open: Option<bool>,
    // 非实际意义上的活动地图关闭时间，只是服务器认为的关闭时间
    pub close_time: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ArkLevelMongo {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub level_id: Option<String>,
    pub stage_id: Option<String>,
    // 文件版本, 用于判断是否需要更新
    pub sha: String,
    // 地图类型, 例: 主线、活动、危机合约
    pub cat_one: Option<String>,
    // 所属章节, 例: 怒号光明、寻昼行动
    pub cat_two: Option<String>,
    // 地图ID, 例: 7-18、FC-1
    pub cat_three: Option<String>,
    // 地图名, 例: 冬逝、爱国者之死
    pub name: Option<String>,
    pub width: i32,
    pub height: i32,
    // 只是服务器认为的当前版本地图是否开放
    pub is_open: Option<bool>,
    // 非实际意义上的活动地图关闭时间，只是服务器认为的关闭时间
    pub close_time: Option<DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArkLevelInfo {
    pub level_id: String,
    pub stage_id: String,
    pub cat_one: String,
    pub cat_two: String,
    pub cat_three: String,
    pub name: String,
    pub width: i32,
    pub height: i32,
}

impl From<ArkLevel> for ArkLevelInfo {
    fn from(val: ArkLevel) -> Self {
        ArkLevelInfo {
            level_id: val.level_id.unwrap_or_default(),
            stage_id: val.stage_id.unwrap_or_default(),
            cat_one: val.cat_one.unwrap_or_default(),
            cat_two: val.cat_two.unwrap_or_default(),
            cat_three: val.cat_three.unwrap_or_default(),
            name: val.name.unwrap_or_default(),
            width: val.width,
            height: val.height,
        }
    }
}

impl Default for ArkLevel {
    fn default() -> Self {
        Self {
            id: None,
            level_id: None,
            stage_id: None,
            sha: "".to_string(),
            cat_one: None,
            cat_two: None,
            cat_three: None,
            name: None,
            width: 0,
            height: 0,
            is_open: None,
            close_time: None,
        }
    }
}

impl From<ArkLevel> for ArkLevelMongo {
    fn from(val: ArkLevel) -> Self {
        ArkLevelMongo {
            id: val.id,
            level_id: val.level_id,
            stage_id: val.stage_id,
            sha: val.sha,
            cat_one: val.cat_one,
            cat_two: val.cat_two,
            cat_three: val.cat_three,
            name: val.name,
            width: val.width,
            height: val.height,
            is_open: val.is_open,
            close_time: val.close_time,
        }
    }
}

impl From<ArkLevelMongo> for ArkLevel {
    fn from(val: ArkLevelMongo) -> Self {
        ArkLevel {
            id: val.id,
            level_id: val.level_id,
            stage_id: val.stage_id,
            sha: val.sha,
            cat_one: val.cat_one,
            cat_two: val.cat_two,
            cat_three: val.cat_three,
            name: val.name,
            width: val.width,
            height: val.height,
            is_open: val.is_open,
            close_time: val.close_time,
        }
    }
}

pub struct ArkLevelRepository {
    collection: Collection<ArkLevelMongo>,
}

impl ArkLevelRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("maa_level");
        Self { collection }
    }

    pub async fn query_all_levels(&self) -> MaaResult<Vec<ArkLevel>> {
        let cursor = self.collection.find(doc! {}).await?;
        let result: Vec<ArkLevel> =
            cursor.map(|x| x.map(Into::into)).try_collect().await?;
        Ok(result)
    }

    pub async fn query_level_by_keyword(
        &self,
        keyword: &str,
    ) -> MaaResult<Vec<ArkLevel>> {
        let filter = doc! {"$regex": format!(".*{}.*",keyword),"$options": "i"};
        let filter_doc = doc! {
            "or": [
                {"stageId": &filter},
                {"catThree": &filter},
                {"catTwo": &filter},
                {"catOne": &filter},
                {"name": &filter}
            ]
        };
        let cursor = self.collection.find(filter_doc).await?;
        let result: Vec<ArkLevel> =
            cursor.map(|x| x.map(Into::into)).try_collect().await?;
        Ok(result)
    }

    pub async fn insert_level(&self, level: ArkLevel) -> MaaResult<()> {
        let level = ArkLevelMongo::from(level);
        self.collection.insert_one(level).await?;
        Ok(())
    }
}
