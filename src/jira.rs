
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::time::Instant;

use crate::models::{JiraConfig, Ticket};

#[derive(Debug, Deserialize)]
struct IssuesResponse {
    issues: Vec<Issue>,
}

#[derive(Debug,Deserialize)]
struct Issue {
    key: String,
    fields: Field,
}

#[derive(Debug,Deserialize)]
struct Field {
    summary: String,

    #[serde(default)]
    assignee: Option<Assignee>,

    status: Status,
}

#[derive(Debug, Deserialize)]
struct Assignee {
    #[serde(rename = "displayName")]
    display_name: String,
}

#[derive(Debug, Deserialize)]
struct Status {
    name: String,
}

pub fn query_jira(tickets: &String, config: &JiraConfig) -> HashMap<String, Ticket> {
    let client: Client = Client::new();

    let url = format!("{}/rest/api/3/search", &config.site);

    let request = client
        .get(url)
        .basic_auth(&config.username, Some(&config.api_token))
        .header("Accept", "application/json")
        .query(&[
            ("fields", "status,summary,assignee"),
            ("jql", format!("key in ({})", tickets).as_ref()),
        ]);
    
    log::debug!("built request: {:?}", request);

    let start_time = Instant::now();

    let response = request
        .send()
        .expect("Request failed");

    let duration = start_time.elapsed();

    log::debug!("Response: {}, took: {:?}", response.status(), duration);

    let json_response: IssuesResponse = response.json()
        .expect("JSON parsing failed");

    json_response
        .issues
        .into_iter()
        .map(|issue| {
            (
                issue.key.clone(),
                Ticket::new(
                    issue.key,
                    issue.fields.summary,
                    issue
                        .fields
                        .assignee
                        .map(|a| a.display_name)
                        .unwrap_or("Unassigned".to_string()),
                    issue.fields.status.name,
                ),
            )
        })
        .collect::<HashMap<String, Ticket>>()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_can_deserialize() {

        let issues_json = "
            {
                \"expand\": \"names,schema\",
                \"startAt\": 0,
                \"maxResults\": 50,
                \"total\": 1,
                \"issues\": [
                    {
                        \"expand\": \"operations,versionedRepresentations,editmeta,changelog,renderedFields\",
                        \"id\": \"000001\",
                        \"self\": \"https://portal.atlassian.net/rest/api/3/issue/000001\",
                        \"key\": \"ETC-0001\",
                        \"fields\": {
                            \"summary\": \"Case with incorrect values \",
                            \"assignee\": null,
                            \"status\": {
                                \"self\": \"https://portal.atlassian.net/rest/api/3/status/10004\",
                                \"description\": \"\",
                                \"iconUrl\": \"https://portal.atlassian.net/images/icons/statuses/open.png\",
                                \"name\": \"Open\",
                                \"id\": \"10004\",
                                \"statusCategory\": {
                                    \"self\": \"https://portal.atlassian.net/rest/api/3/statuscategory/2\",
                                    \"id\": 2,
                                    \"key\": \"new\",
                                    \"colorName\": \"blue-gray\",
                                    \"name\": \"To Do\"
                                }
                            }
                        }
                    }
                ]
            }
        ";

        let u  = serde_json::from_str::<IssuesResponse>(issues_json);

        assert!(u.is_ok());

        let issues = u.unwrap().issues;

        assert_eq!(issues.len(), 1);

        let first = issues.first().unwrap();

        assert_eq!(first.fields.status.name, "Open");

        assert!(first.fields.assignee.is_none());
    }

   
}