use crate::http::HttpResult;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DLSiteInfo {
    pub title: String,
    pub circle: String,
    pub scenario: Vec<String>,
    pub illustration: Vec<String>,
    pub category: Vec<String>,
    pub tags: Vec<String>,
    pub description: Vec<String>,
}

pub enum Language {
    EnUs,
    ZhCn,
    JaJp,
}

impl Language {
    pub fn as_lang_code(&self) -> &str {
        match self {
            Language::EnUs => "en_US",
            Language::ZhCn => "zh_CN",
            Language::JaJp => "ja_JP",
        }
    }

    pub fn tag_scenario(&self) -> &str {
        match self {
            Language::EnUs => "Scenario",
            Language::ZhCn => "剧情",
            Language::JaJp => "シナリオ",
        }
    }

    pub fn tag_illustration(&self) -> &str {
        match self {
            Language::EnUs => "Illustration",
            Language::ZhCn => "插画",
            Language::JaJp => "イラスト",
        }
    }
}

pub async fn fetch_dl_site_maniax(id: &str, language: Language) -> HttpResult<DLSiteInfo> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let url = format!(
        "https://www.dlsite.com/maniax/work/=/product_id/{id}.html/?locale={}",
        language.as_lang_code()
    );
    let response = client.get(&url).send().await?;
    let document = Html::parse_document(response.text().await?.as_str());

    let title_selector = Selector::parse("#work_name")?;
    let title = document
        .select(&title_selector)
        .next()
        .and_then(|el| el.text().next())
        .map(|text| text.trim().to_string())
        .unwrap_or_else(|| "Unknown Title".to_string());

    let circle_selector = Selector::parse(".maker_name a")?;
    let circle = document
        .select(&circle_selector)
        .next()
        .and_then(|el| el.text().next())
        .map(|text| text.trim().to_string())
        .unwrap_or_else(|| "Unknown Circle".to_string());

    let tr_selector = Selector::parse("tr")?;

    let mut scenario = Vec::new();
    let mut illustration = Vec::new();
    let mut flag = (false, false);

    for tr in document.select(&tr_selector) {
        let th = tr.select(&Selector::parse("th")?).next();
        if let Some(th) = th {
            if th.text().collect::<String>().trim() == language.tag_scenario() {
                scenario = tr
                    .select(&Selector::parse("td a")?)
                    .map(|a| a.text().collect::<String>().trim().to_string())
                    .collect();
                flag.0 = true;
            }
        }
        if let Some(th) = th {
            if th.text().collect::<String>().trim() == language.tag_illustration() {
                illustration = tr
                    .select(&Selector::parse("td a")?)
                    .map(|a| a.text().collect::<String>().trim().to_string())
                    .collect();
                flag.0 = true;
            }
        }
        if flag.0 && flag.1 {
            break; // Both scenario and illustration found, exit loop
        }
    }

    let category_selector = Selector::parse("#category_type a")?;
    let category = document
        .select(&category_selector)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .collect::<Vec<_>>();

    let tag_selector = Selector::parse(".main_genre a")?;
    let tags = document
        .select(&tag_selector)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .collect::<Vec<_>>();

    let desc_selector = Selector::parse(".work_parts_container[itemprop=description]")?;
    let description = document
        .select(&desc_selector)
        .flat_map(|el| el.text())
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect::<Vec<_>>();

    Ok(DLSiteInfo {
        title,
        circle,
        scenario,
        illustration,
        category,
        tags,
        description,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_fetch_maniax() {
        let id1 = "RJ01239331"; // クルセイダー・プリンセス～闇に堕ちるココロとカラダ～
        let result1 = fetch_dl_site_maniax(id1, Language::JaJp)
            .await
            .expect("fetch dl site maniax");
        dbg!(result1);

        let test2 = "RJ01397047"; // 救出のお嬢様
        let result2 = fetch_dl_site_maniax(test2, Language::ZhCn)
            .await
            .expect("fetch dl site maniax");
        dbg!(result2);

        let faulty_id = "RJ00000000"; // Invalid ID for testing error handling
        let result3 = fetch_dl_site_maniax(faulty_id, Language::EnUs)
            .await
            .expect("fetch dl site maniax");
        dbg!(result3);
    }
}
