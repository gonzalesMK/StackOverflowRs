use html2text::from_read;
use std::time::Instant;

use quick_cache::unsync::Cache;
use serde::Deserialize;

use crate::app::Question;

const STACK_OVERFLOW_URL: &str = "https://api.stackexchange.com/";
const UNANSWERED_QUESTIONS: &str =
    "2.3/questions/unanswered?order=desc&sort=activity&site=stackoverflow&filter=!6VCr095Ee9eW)AbNMHD5dNZ4Q";

#[derive(Deserialize, Debug, Clone)]
struct QuestionDTO {
    pub title: String,
    pub link: String,
    pub tags: Vec<String>,
    pub answer_count: u32,
    pub body: String,
}

pub fn from_html(html: &str) -> String {
    from_read(html.as_bytes(), 60)
}

impl From<QuestionDTO> for Question {
    fn from(dto: QuestionDTO) -> Question {
        let body = from_html(dto.body.as_str());
        Question {
            title: dto.title,
            link: dto.link,
            tags: dto.tags,
            answer_count: dto.answer_count,
            body: body.clone(),
            description: body.lines().take(3).collect::<Vec<_>>().join(". "),
            show_body: false,
        }
    }
}

// dto for this example
// {
//    "items": [
//        {
//            "tags": [
//                "visual-studio-code",
//                "conda",
//                "git-bash"
//            ],
//            "owner": {
//                "account_id": 22556525,
//                "reputation": 1,
//                "user_id": 16797965,
//                "user_type": "registered",
//                "profile_image": "https://www.gravatar.com/avatar/a8c3cfbb3ce63d250729e1a0727c02ca?s=256&d=identicon&r=PG&f=y&so-version=2",
//                "display_name": "Mar",
//                "link": "https://stackoverflow.com/users/16797965/mar"
//            },
//            "is_answered": false,
//            "view_count": 6,
//            "answer_count": 0,
//            "score": 0,
//            "last_activity_date": 1693399582,
//            "creation_date": 1693399468,
//            "last_edit_date": 1693399582,
//            "question_id": 77008172,
//            "content_license": "CC BY-SA 4.0",
//            "link": "https://stackoverflow.com/questions/77008172/vscode-could-not-find-conda-environment",
//            "title": "VSCode: Could not find conda environment"
//        },
//        ],
//        ,
//    "has_more": true,
//    "quota_max": 300,
//    "quota_remaining": 283
// }

#[derive(Deserialize, Debug, Clone)]
struct StackOverflowDto {
    items: Vec<QuestionDTO>,
    has_more: bool,
    quota_max: i32,
    quota_remaining: i32,
}

pub struct TemplateTTL<T> {
    pub ttl: T,
    pub created_at: Instant,
}

#[derive(Debug)]
pub struct StackOverflowClient {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
    pub cache: Cache<String, TemplateTTL<String>>,
}

impl StackOverflowClient {
    pub fn new(base_url: String) -> StackOverflowClient {
        StackOverflowClient {
            client: reqwest::blocking::Client::new(),
            base_url,
            cache: Cache::new(usize::MAX),
        }
    }

    pub fn default() -> StackOverflowClient {
        StackOverflowClient {
            client: reqwest::blocking::Client::new(),
            base_url: STACK_OVERFLOW_URL.to_string(),
            cache: Cache::new(usize::MAX),
        }
    }

    pub fn get_unanswered_questions(&mut self, page: u8) -> Result<Vec<Question>, reqwest::Error> {
        //check cache
        Ok(self
            .get_unanswered_questions_dto(page)?
            .items
            .iter()
            .map(|i| (*i).clone().into())
            .collect())
    }

    fn get_unanswered_questions_dto(
        &mut self,
        page: u8,
    ) -> Result<StackOverflowDto, reqwest::Error> {
        let url = format!("{}{}&page={}", self.base_url, UNANSWERED_QUESTIONS, page);

        let content = self.make_cached_request(&url)?;

        Ok(serde_json::from_str(&content).unwrap())
    }

    fn make_cached_request(&mut self, url: &str) -> Result<String, reqwest::Error> {
        if let Some(cached) = self.cache.get(url) {
            if cached.created_at.elapsed().as_secs() < 300 {
                return Ok(cached.ttl.clone());
            }
        }

        let content = self.make_request(url)?;

        self.cache.insert(
            url.to_string(),
            TemplateTTL {
                ttl: content.clone(),
                created_at: Instant::now(),
            },
        );

        Ok(content)
    }

    fn make_request(&mut self, url: &str) -> Result<String, reqwest::Error> {
        let response = self.client.get(url).send()?;
        Ok(response.text()?)
    }
}
