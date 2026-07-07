use color_eyre::Result;
use tokio::time::{Duration, timeout};

use crate::types::SseMessage;

pub fn parse_sse(input: &str) -> Vec<SseMessage> {
    let mut messages = Vec::new();
    let mut current_id: Option<String> = None;
    let mut current_event: Option<String> = None;
    let mut current_data: Vec<String> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            if !current_data.is_empty() {
                messages.push(SseMessage {
                    id: current_id.take(),
                    event: current_event.take(),
                    data: current_data.join("\n"),
                });
                current_data.clear();
            }
            continue;
        }

        if let Some((field, value)) = line.split_once(':') {
            let value = value.strip_prefix(' ').unwrap_or(value);
            match field {
                "id" => current_id = Some(value.to_string()),
                "event" => current_event = Some(value.to_string()),
                "data" => current_data.push(value.to_string()),
                _ => {}
            }
        }
    }

    if !current_data.is_empty() {
        messages.push(SseMessage {
            id: current_id,
            event: current_event,
            data: current_data.join("\n"),
        });
    }

    messages
}

pub async fn read_sse_stream(
    response: reqwest::Response,
    limit: Option<u32>,
    timeout_secs: Option<u64>,
) -> Result<Vec<SseMessage>> {
    use tokio_stream::StreamExt;

    let mut messages = Vec::new();
    let mut buffer = String::new();
    let mut count = 0u32;

    let read_future = async {
        let mut stream = response.bytes_stream();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);
            buffer.push_str(&text);

            while let Some(pos) = buffer.find("\n\n") {
                let msg_text = buffer[..pos].to_string();
                buffer = buffer[pos + 2..].to_string();

                let parsed = parse_sse(&msg_text);
                for msg in parsed {
                    println!("{}", msg.data);
                    messages.push(msg);
                    count += 1;
                    if let Some(lim) = limit {
                        if count >= lim {
                            return Ok(messages);
                        }
                    }
                }
            }
        }

        if !buffer.is_empty() {
            let parsed = parse_sse(&buffer);
            for msg in parsed {
                println!("{}", msg.data);
                messages.push(msg);
            }
        }

        Ok(messages)
    };

    if let Some(secs) = timeout_secs {
        timeout(Duration::from_secs(secs), read_future)
            .await
            .map_err(|_| {
                color_eyre::eyre::eyre!("SSE stream timed out after {secs} seconds")
            })?
    } else {
        read_future.await
    }
}
