use chrono::NaiveDate;

use crate::features::email::model::{
    EmailTemplate, LeaveRequestDecisionPayload, LeaveRequestSubmittedPayload,
};

const LAYOUT: &str = include_str!("templates/layout.html");
const SUBMITTED: &str = include_str!("templates/leave-request-submitted.html");
const DECISION: &str = include_str!("templates/leave-request-decision.html");

pub struct RenderedEmail {
    pub subject: String,
    pub html: String,
}

#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("unknown template {0:?}")]
    UnknownTemplate(String),
    #[error("payload does not match template: {0}")]
    BadPayload(#[from] serde_json::Error),
    #[error("template placeholder problem: {0}")]
    Placeholder(String),
}

/// `Text` is escaped. `Html` is inserted as-is and is only used for our own templates.
enum Value {
    Text(String),
    Html(String),
}

pub fn render(
    template: &str,
    payload: serde_json::Value,
    app_base_url: &str,
) -> Result<RenderedEmail, RenderError> {
    let template: EmailTemplate = template
        .parse()
        .map_err(|_| RenderError::UnknownTemplate(template.to_owned()))?;

    let (subject, content) = match template {
        EmailTemplate::LeaveRequestSubmitted => {
            let p: LeaveRequestSubmittedPayload = serde_json::from_value(payload)?;
            let subject = format!("New leave request from {}", p.employee_name);
            let content = fill(
                SUBMITTED,
                &[
                    ("employee_name", Value::Text(p.employee_name)),
                    ("leave_type", Value::Text(p.leave_type)),
                    ("date_from", Value::Text(format_date(p.date_from))),
                    ("date_to", Value::Text(format_date(p.date_to))),
                    ("duration", Value::Text(format_duration(p.total_duration))),
                    (
                        "notes",
                        Value::Text(
                            p.notes
                                .filter(|n| !n.trim().is_empty())
                                .unwrap_or_else(|| "None".to_owned()),
                        ),
                    ),
                    (
                        "link",
                        Value::Text(request_link(app_base_url, p.request_id)),
                    ),
                ],
            )?;
            (subject, content)
        }
        EmailTemplate::LeaveRequestDecision => {
            let p: LeaveRequestDecisionPayload = serde_json::from_value(payload)?;
            let decision = p.decision.to_string().to_lowercase();
            let subject = format!("Your leave request was {decision}");
            let content = fill(
                DECISION,
                &[
                    ("decision", Value::Text(decision)),
                    ("decided_by", Value::Text(p.decided_by)),
                    ("leave_type", Value::Text(p.leave_type)),
                    ("date_from", Value::Text(format_date(p.date_from))),
                    ("date_to", Value::Text(format_date(p.date_to))),
                    ("duration", Value::Text(format_duration(p.total_duration))),
                    (
                        "link",
                        Value::Text(request_link(app_base_url, p.request_id)),
                    ),
                ],
            )?;
            (subject, content)
        }
    };

    let html = fill(
        LAYOUT,
        &[
            ("subject", Value::Text(subject.clone())),
            ("content", Value::Html(content)),
        ],
    )?;

    Ok(RenderedEmail { subject, html })
}

/// Fills `{{key}}` placeholders in one pass. Substituted values are never scanned again.
fn fill(template: &str, values: &[(&str, Value)]) -> Result<String, RenderError> {
    let mut out = String::with_capacity(template.len() + 256);
    let mut rest = template;

    while let Some(start) = rest.find("{{") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let end = after
            .find("}}")
            .ok_or_else(|| RenderError::Placeholder("unclosed {{".to_owned()))?;
        let key = after[..end].trim();

        match values.iter().find(|(name, _)| *name == key) {
            Some((_, Value::Text(text))) => out.push_str(&escape(text)),
            Some((_, Value::Html(html))) => out.push_str(html),
            None => return Err(RenderError::Placeholder(format!("no value for {key:?}"))),
        }
        rest = &after[end + 2..];
    }

    out.push_str(rest);
    Ok(out)
}

fn format_date(date: NaiveDate) -> String {
    date.format("%a %-d %b %Y").to_string() // "Mon 23 Nov 2026"
}

fn format_duration(days: f64) -> String {
    if days == 1.0 {
        "1 day".to_owned()
    } else {
        format!("{days} days")
    }
}

fn request_link(app_base_url: &str, request_id: i32) -> String {
    format!(
        "{}/leave-request/{request_id}",
        app_base_url.trim_end_matches('/')
    )
}

/// Escapes text from users or the database before it goes into HTML.
fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn submitted(employee_name: &str, notes: Option<&str>) -> serde_json::Value {
        json!({
            "request_id": 5,
            "employee_name": employee_name,
            "leave_type": "Annual Leave",
            "date_from": "2026-11-23",
            "date_to": "2026-11-24",
            "total_duration": 2.0,
            "notes": notes,
        })
    }

    #[test]
    fn names_and_notes_are_escaped() {
        let email = render(
            "leave_request_submitted",
            submitted("<b>Avery</b>", Some("<script>alert(1)</script>")),
            "https://derbe.app",
        )
        .unwrap();

        assert!(email.html.contains("&lt;b&gt;Avery&lt;/b&gt;"));
        assert!(email.html.contains("&lt;script&gt;"));
        assert!(!email.html.contains("<script>"));
    }

    #[test]
    fn placeholders_inside_user_text_are_not_expanded() {
        let email = render(
            "leave_request_submitted",
            submitted("Avery", Some("{{link}}")),
            "https://derbe.app",
        )
        .unwrap();

        assert!(email.html.contains("{{link}}"));
    }

    #[test]
    fn every_placeholder_is_filled_and_link_is_correct() {
        let email = render(
            "leave_request_submitted",
            submitted("Avery", None),
            "https://derbe.app/",
        )
        .unwrap();

        assert!(!email.html.contains("{{"));
        assert!(email.html.contains("https://derbe.app/leave-request/5"));
        assert!(email.html.contains("Mon 23 Nov 2026"));
    }

    #[test]
    fn unknown_template_is_an_error() {
        assert!(matches!(
            render("nope", json!({}), "https://derbe.app"),
            Err(RenderError::UnknownTemplate(_))
        ));
    }
}
