use serde_json;

use std::str::FromStr;
use git2::{ Oid, Time, Note };
use super::{ Error, Result };

#[derive(Clone, Deserialize)]
pub struct Data {
  timestamp: Option<String>,
  #[serde(rename="reviewRef")]
  review_ref: Option<String>,
  #[serde(rename="targetRef")]
  target_ref: Option<String>,
  requester: Option<String>,
  reviewers: Option<Vec<String>>,
  description: Option<String>,
  #[serde(rename="baseCommit")]
  base_commit: Option<String>,
}

#[derive(Clone)]
pub struct Request {
  commit_id: Oid,
  data: Data,
}

impl Request {
  pub fn all_from_note(commit_id: Oid, note: Note) -> Result<Vec<Request>> {
    note
      .message()
      .ok_or(Error::NotFound)
      .map(|message| Request::all_from_message(commit_id, message))
  }

  fn all_from_message(commit_id: Oid, message: &str) -> Vec<Request> {
    message
      .lines()
      .filter(|line| !line.is_empty())
      .filter_map(|line| Request::from_str(commit_id, line).map_err(|e| println!("{}", e)).ok())
      .collect()
  }

  fn from_str(commit_id: Oid, s: &str) -> Result<Request> {
    serde_json::de::from_str(s)
      .map_err(|err| From::from((err, s.to_string())))
      .map(|data| Request::from_data(commit_id, data))
  }

  fn from_data(commit_id: Oid, data: Data) -> Request {
    Request {
      commit_id: commit_id,
      data: data,
    }
  }

  pub fn commit_id(&self) -> Oid {
    self.commit_id
  }

  pub fn timestamp(&self) -> Option<Time> {
    self.data.timestamp.as_ref()
      .and_then(|timestamp|
        FromStr::from_str(timestamp)
          .ok()
          .map(|time| Time::new(time, 0)))
  }

  pub fn review_ref(&self) -> Option<&str> {
    self.data.review_ref.as_ref().map(|s| &**s)
  }

  pub fn target_ref(&self) -> Option<&str> {
    self.data.target_ref.as_ref().map(|s| &**s)
  }

  pub fn requester(&self) -> Option<&str> {
    self.data.requester.as_ref().map(|s| &**s)
  }

  pub fn reviewers(&self) -> Option<Vec<&str>> {
    self.data.reviewers.as_ref().map(|v| v.iter().map(|s| &**s).collect())
  }

  pub fn description(&self) -> Option<&str> {
    self.data.description.as_ref().map(|s| &**s)
  }

  pub fn base_commit(&self) -> Option<&str> {
    self.data.base_commit.as_ref().map(|s| &**s)
  }
}
