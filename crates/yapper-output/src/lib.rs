use std::collections::BTreeSet;

use chrono::{DateTime, FixedOffset};
use yapper_domain::{SessionType, Utterance};

#[derive(Clone, Debug)]
pub struct TranscriptRenderInput {
    pub session_type: SessionType,
    pub created_at: DateTime<FixedOffset>,
    pub duration_seconds: u32,
    pub source_app: String,
    pub context: String,
    pub utterances: Vec<Utterance>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RenderedTranscript {
    pub filename: String,
    pub markdown: String,
}

pub fn render_transcript(input: &TranscriptRenderInput) -> RenderedTranscript {
    let is_voice_memo = matches!(input.session_type, SessionType::VoiceMemo);
    let file_label = if is_voice_memo {
        "Voice Memo"
    } else {
        "Call Recording"
    };
    let note_type = if is_voice_memo { "fleeting" } else { "meeting" };
    let log_tag = if is_voice_memo {
        "log/voice"
    } else {
        "log/meeting"
    };
    let source_tag = if is_voice_memo {
        "source/voice"
    } else {
        "source/meeting"
    };

    let filename = note_filename(&input.session_type, input.created_at, &input.context);
    let date_str = input.created_at.format("%Y-%m-%d").to_string();
    let time_str = input.created_at.format("%H:%M").to_string();
    let duration_str = format_duration(input.duration_seconds);
    let attendees = unique_speakers(&input.utterances);
    let attendees_yaml = if attendees.is_empty() {
        "[]".to_string()
    } else {
        format!("[\"{}\"]", attendees.join("\", \""))
    };
    let transcript_body = if input.utterances.is_empty() {
        String::new()
    } else {
        input
            .utterances
            .iter()
            .map(|utterance| {
                format!(
                    "**{}** ({})\n{}\n",
                    utterance.speaker.label, utterance.timestamp, utterance.text
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    let context = input.context.trim().to_string();
    let escaped_context = context.replace('"', "\\\"");

    let markdown = format!(
        "---\n\
type: {note_type}\n\
created: \"{date_str}\"\n\
time: \"{time_str}\"\n\
duration: \"{duration_str}\"\n\
source_app: \"{source_app}\"\n\
source_file: \"{filename}\"\n\
attendees: {attendees_yaml}\n\
context: \"{escaped_context}\"\n\
tags:\n\
  - {log_tag}\n\
  - status/inbox\n\
  - {source_tag}\n\
  - source/tome\n\
---\n\n\
# {file_label} - {date_str} {time_str}\n\n\
**Duration:** {duration_str} | **Speakers:** {speaker_count}\n\n\
---\n\n\
## Context\n\n\
{context_body}\n\n\
---\n\n\
## Transcript\n\n\
{transcript_body}",
        source_app = input.source_app,
        speaker_count = attendees.len(),
        context_body = context,
    );

    RenderedTranscript { filename, markdown }
}

pub fn note_filename(
    session_type: &SessionType,
    created_at: DateTime<FixedOffset>,
    context: &str,
) -> String {
    let date_prefix = created_at.format("%Y-%m-%d %H-%M-%S").to_string();
    let trimmed_context = context.trim();

    if !trimmed_context.is_empty() {
        let sanitized = trimmed_context
            .chars()
            .take(50)
            .collect::<String>()
            .replace('/', "-")
            .replace(':', "-")
            .trim()
            .to_string();

        if !sanitized.is_empty() {
            return format!("{date_prefix} {sanitized}.md");
        }
    }

    let label = match session_type {
        SessionType::VoiceMemo => "Voice Memo",
        SessionType::MeetingCapture => "Call Recording",
    };

    format!("{date_prefix} {label}.md")
}

fn format_duration(total_seconds: u32) -> String {
    format!("{:02}:{:02}", total_seconds / 60, total_seconds % 60)
}

fn unique_speakers(utterances: &[Utterance]) -> Vec<String> {
    utterances
        .iter()
        .fold(BTreeSet::new(), |mut speakers, utterance| {
            speakers.insert(utterance.speaker.label.clone());
            speakers
        })
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use yapper_domain::{SessionType, Speaker, Utterance};

    use super::{render_transcript, TranscriptRenderInput};

    #[test]
    fn renders_voice_memo_markdown_contract() {
        let created_at = DateTime::parse_from_rfc3339("2026-04-02T09:05:00+00:00").unwrap();
        let rendered = render_transcript(&TranscriptRenderInput {
            session_type: SessionType::VoiceMemo,
            created_at,
            duration_seconds: 65,
            source_app: "Microphone".to_string(),
            context: "Ideas for the launch memo".to_string(),
            utterances: vec![Utterance {
                speaker: Speaker::you(),
                text: "Voice memo scaffolding is wired end to end.".to_string(),
                timestamp: "09:05:03".to_string(),
            }],
        });

        assert_eq!(
            rendered.filename,
            "2026-04-02 09-05-00 Ideas for the launch memo.md"
        );
        assert_eq!(
            rendered.markdown,
            "---\n\
type: fleeting\n\
created: \"2026-04-02\"\n\
time: \"09:05\"\n\
duration: \"01:05\"\n\
source_app: \"Microphone\"\n\
source_file: \"2026-04-02 09-05-00 Ideas for the launch memo.md\"\n\
attendees: [\"You\"]\n\
context: \"Ideas for the launch memo\"\n\
tags:\n\
  - log/voice\n\
  - status/inbox\n\
  - source/voice\n\
  - source/tome\n\
---\n\n\
# Voice Memo - 2026-04-02 09:05\n\n\
**Duration:** 01:05 | **Speakers:** 1\n\n\
---\n\n\
## Context\n\n\
Ideas for the launch memo\n\n\
---\n\n\
## Transcript\n\n\
**You** (09:05:03)\n\
Voice memo scaffolding is wired end to end.\n"
        );
    }

    #[test]
    fn renders_meeting_markdown_contract() {
        let created_at = DateTime::parse_from_rfc3339("2026-04-02T14:30:00+00:00").unwrap();
        let rendered = render_transcript(&TranscriptRenderInput {
            session_type: SessionType::MeetingCapture,
            created_at,
            duration_seconds: 185,
            source_app: "Teams".to_string(),
            context: "Quarterly planning / hiring".to_string(),
            utterances: vec![
                Utterance {
                    speaker: Speaker::you(),
                    text: "We should keep the Windows scaffold small and testable.".to_string(),
                    timestamp: "14:30:08".to_string(),
                },
                Utterance {
                    speaker: Speaker::them(),
                    text: "Agreed, parity starts with the markdown contract.".to_string(),
                    timestamp: "14:30:26".to_string(),
                },
            ],
        });

        assert_eq!(
            rendered.filename,
            "2026-04-02 14-30-00 Quarterly planning - hiring.md"
        );
        assert_eq!(
            rendered.markdown,
            "---\n\
type: meeting\n\
created: \"2026-04-02\"\n\
time: \"14:30\"\n\
duration: \"03:05\"\n\
source_app: \"Teams\"\n\
source_file: \"2026-04-02 14-30-00 Quarterly planning - hiring.md\"\n\
attendees: [\"Them\", \"You\"]\n\
context: \"Quarterly planning / hiring\"\n\
tags:\n\
  - log/meeting\n\
  - status/inbox\n\
  - source/meeting\n\
  - source/tome\n\
---\n\n\
# Call Recording - 2026-04-02 14:30\n\n\
**Duration:** 03:05 | **Speakers:** 2\n\n\
---\n\n\
## Context\n\n\
Quarterly planning / hiring\n\n\
---\n\n\
## Transcript\n\n\
**You** (14:30:08)\n\
We should keep the Windows scaffold small and testable.\n\n\
**Them** (14:30:26)\n\
Agreed, parity starts with the markdown contract.\n"
        );
    }
}
