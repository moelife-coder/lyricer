mod lyric;
fn main() {
    loop {
        let player = find_player();
        loop {
            // Get metadata
            let metadata = player.get_metadata().unwrap();
            let audio_ending = metadata.length();
            let mut formatted_metadata = metadata.title().unwrap_or("[Unknown]").to_string();
            if let Some(i) = audio_ending {
                formatted_metadata.push_str(&format!(" ({:#?})", i));
            }
            if let Some(i) = metadata.artists() {
                formatted_metadata.push_str(&format!("\nArtist: {}", i.join(", ")));
            }
            // Get lyrics
            let audio_path =
                urlencoding::decode(metadata.url().unwrap_or("/").split("://").nth(1).unwrap())
                    .unwrap();
            let audio_path = std::path::Path::new(&audio_path);
            let lyrics = get_lyrics(audio_path);
            print_lyrics(
                &std::path::Path::new("/tmp/lyrics"),
                lyrics,
                &formatted_metadata,
                audio_ending,
                player.get_position().ok(),
                &player,
            );
        }
    }
}
fn get_lyrics(audio_path: &std::path::Path) -> Result<lyric::Lyric, ()> {
    if audio_path.is_file() {
        let mut lyric_name = std::path::PathBuf::from(&audio_path);
        if lyric_name.is_file() {
            lyric_name.set_extension("lrc");
            if let Ok(i) = std::fs::read(lyric_name) {
                return lyric::Lyric::parse(String::from_utf8_lossy(&i).to_string());
            }
        }
    }
    Err(())
}
fn print_lyrics(
    target_file: &std::path::Path,
    lyrics: Result<lyric::Lyric, ()>,
    formatted_metadata: &str,
    audio_ending: Option<std::time::Duration>,
    current_offset: Option<std::time::Duration>,
    player_handle: &mpris::Player,
) {
    let real_lyrics = match lyrics {
        Ok(i) => i.content,
        Err(_) => Box::new([lyric::LyricsType::Standard(
            std::time::Duration::default(),
            Box::from("No lyrics"),
        )]),
    };
    let mut current_duration = current_offset.unwrap_or_default();
    for i in real_lyrics.as_ref() {
        // We can't implement colored lyrics yet
        let (duration, lyric) = match i {
            lyric::LyricsType::Standard(i, j) => (i, j.as_ref().to_string()),
            lyric::LyricsType::Enhanced(i, j) => (
                i,
                j.as_ref()
                    .iter()
                    .map(|x| x.1.as_ref().to_string())
                    .collect::<Vec<String>>()
                    .join(" "),
            ),
        };
        duration
            .checked_sub(current_duration)
            .map(|x| std::thread::sleep(x));
        if !player_handle.is_running() {
            return;
        }
        //current_duration += duration.to_owned();
        if let Ok(i) = player_handle.get_position() {
            if i > (current_duration + std::time::Duration::from_millis(250)) {
                // Current playing position is 1 second faster than current display.
                println!("");
            } else if i + std::time::Duration::from_millis(250) < current_duration {
                // Current playing position is 1 seond slower than current display.
                // This code design disallow "return" to a point. Thus, we will simply request to recall the function.
                return;
            }
            current_duration = i
        } else {
            current_duration = duration.to_owned()
        }
        std::fs::write(
            target_file,
            &format!("{}", output(&lyric, formatted_metadata, "")),
        )
        .unwrap();
    }
    if let Some(audio_ending) = audio_ending {
        audio_ending
            .checked_sub(current_duration)
            .map(|x| std::thread::sleep(x));
    }
}
fn output(text: &str, tooltip: &str, class: &str) -> String {
    format!(
        "{{\"text\": \"{}\", \"tooltip\": \"{}\", \"class\": \"{}\"}}",
        text, tooltip, class
    )
}
fn find_player<'a>() -> mpris::Player<'a> {
    mpris::PlayerFinder::new().unwrap().find_active().unwrap()
}
