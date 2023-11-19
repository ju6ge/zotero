use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use zotero_derive::ItemCommon;

use crate::{shared_fields::{Tag, ItemCommon}, item::Creator};

#[derive(Debug, Serialize, Deserialize, Builder, Default, Clone, ItemCommon)]
#[serde(rename_all(deserialize = "camelCase", serialize = "camelCase"))]
#[builder(setter(into), default)]
pub struct PreprintData {
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub key: String,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub title: String,
    #[serde(default = "default_document_type")]
    pub item_type: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub creators: Vec<Creator>,
    #[serde(skip_serializing_if = "String::is_empty", default)]
    pub rights: String,

    pub tags: Vec<Tag>,
}

fn default_document_type() -> String {
    "preprint".to_string()
}

#[cfg(test)]
mod test {
    use assert_json_diff::assert_json_eq;
    use serde_json::Value;

    use crate::item::{Item, ItemType};

    #[test]
    fn deserialize_preprint() {
        let preprint_json = r#"{
  "data": {
    "DOI": "",
    "abstractNote": "Artiﬁcial intelligence (AI) researchers have been developing and reﬁning large language models (LLMs) that exhibit remarkable capabilities across a variety of domains and tasks, challenging our understanding of learning and cognition. The latest model developed by OpenAI, GPT-4 [Ope23], was trained using an unprecedented scale of compute and data. In this paper, we report on our investigation of an early version of GPT-4, when it was still in active development by OpenAI. We contend that (this early version of) GPT4 is part of a new cohort of LLMs (along with ChatGPT and Google’s PaLM for example) that exhibit more general intelligence than previous AI models. We discuss the rising capabilities and implications of these models. We demonstrate that, beyond its mastery of language, GPT-4 can solve novel and diﬃcult tasks that span mathematics, coding, vision, medicine, law, psychology and more, without needing any special prompting. Moreover, in all of these tasks, GPT-4’s performance is strikingly close to human-level performance, and often vastly surpasses prior models such as ChatGPT. Given the breadth and depth of GPT-4’s capabilities, we believe that it could reasonably be viewed as an early (yet still incomplete) version of an artiﬁcial general intelligence (AGI) system. In our exploration of GPT-4, we put special emphasis on discovering its limitations, and we discuss the challenges ahead for advancing towards deeper and more comprehensive versions of AGI, including the possible need for pursuing a new paradigm that moves beyond next-word prediction. We conclude with reﬂections on societal inﬂuences of the recent technological leap and future research directions.",
    "accessDate": "2023-03-25T16:54:59Z",
    "archive": "",
    "archiveID": "arXiv:2303.12712",
    "archiveLocation": "",
    "callNumber": "",
    "citationKey": "",
    "collections": [],
    "creators": [
      {
        "creatorType": "author",
        "firstName": "Sébastien",
        "lastName": "Bubeck"
      },
      {
        "creatorType": "author",
        "firstName": "Varun",
        "lastName": "Chandrasekaran"
      },
      {
        "creatorType": "author",
        "firstName": "Ronen",
        "lastName": "Eldan"
      },
      {
        "creatorType": "author",
        "firstName": "Johannes",
        "lastName": "Gehrke"
      },
      {
        "creatorType": "author",
        "firstName": "Eric",
        "lastName": "Horvitz"
      },
      {
        "creatorType": "author",
        "firstName": "Ece",
        "lastName": "Kamar"
      },
      {
        "creatorType": "author",
        "firstName": "Peter",
        "lastName": "Lee"
      },
      {
        "creatorType": "author",
        "firstName": "Yin Tat",
        "lastName": "Lee"
      },
      {
        "creatorType": "author",
        "firstName": "Yuanzhi",
        "lastName": "Li"
      },
      {
        "creatorType": "author",
        "firstName": "Scott",
        "lastName": "Lundberg"
      },
      {
        "creatorType": "author",
        "firstName": "Harsha",
        "lastName": "Nori"
      },
      {
        "creatorType": "author",
        "firstName": "Hamid",
        "lastName": "Palangi"
      },
      {
        "creatorType": "author",
        "firstName": "Marco Tulio",
        "lastName": "Ribeiro"
      },
      {
        "creatorType": "author",
        "firstName": "Yi",
        "lastName": "Zhang"
      }
    ],
    "date": "2023-03-22",
    "dateAdded": "2023-03-25T16:54:59Z",
    "dateModified": "2023-03-25T16:55:11Z",
    "deleted": 1,
    "extra": "arXiv:2303.12712 [cs]",
    "genre": "",
    "itemType": "preprint",
    "key": "HPXL75GC",
    "language": "en",
    "libraryCatalog": "arXiv.org",
    "place": "",
    "relations": {},
    "repository": "arXiv",
    "rights": "",
    "series": "",
    "seriesNumber": "",
    "shortTitle": "Sparks of Artificial General Intelligence",
    "tags": [
      {
        "tag": "Computer Science - Artificial Intelligence",
        "type": 1
      },
      {
        "tag": "Computer Science - Computation and Language",
        "type": 1
      },
      {
        "tag": "plato-read"
      }
    ],
    "title": "Sparks of Artificial General Intelligence: Early experiments with GPT-4",
    "url": "http://arxiv.org/abs/2303.12712",
    "version": 355
  },
  "key": "HPXL75GC",
  "library": {
    "id": 8071408,
    "links": {
      "alternate": {
        "href": "https://www.zotero.org/ju6ge",
        "type": "text/html"
      }
    },
    "name": "ju6ge",
    "type": "user"
  },
  "links": {
    "alternate": {
      "href": "https://www.zotero.org/ju6ge/items/HPXL75GC",
      "type": "text/html"
    },
    "self": {
      "href": "https://api.zotero.org/users/8071408/items/HPXL75GC",
      "type": "application/json"
    }
  },
  "meta": {
    "creatorSummary": "Bubeck et al.",
    "numChildren": 1,
    "parsedDate": "2023-03-22"
  },
  "version": 355
}
"#;
            let item: Item = serde_json::from_str(&preprint_json).unwrap();
            let all_fields: Value = serde_json::from_str(&preprint_json).unwrap();

            assert_json_eq!(&all_fields, &item);
    }
}
