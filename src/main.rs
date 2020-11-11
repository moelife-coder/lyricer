mod lyric;
fn main() {
    pretty_env_logger::init();
    log::warn!("Lyricer started.");
    loop {
        let player = match find_player() {
            Ok(i) => i,
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        };
        loop {
            // Get metadata
            let metadata = match player.get_metadata() {
                Ok(i) => i,
                Err(i) => {
                    log::error!("Error when fetching metdata: {}", i);
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    continue;
                }
            };
            log::debug!("Metata found: {:?}", metadata);
            let audio_ending = metadata.length();
            log::debug!("Audio length: {:?}", audio_ending);
            let mut formatted_metadata = metadata.title().unwrap_or("[Unknown]").to_string();
            if let Some(i) = audio_ending {
                formatted_metadata.push_str(&format!(" ({:#?})", i));
            }
            if let Some(i) = metadata.artists() {
                formatted_metadata.push_str(&format!("Artist: {}", i.join(", ")));
            }
            log::info!("Formatted metadata: {}", formatted_metadata);
            // Get lyrics
            let audio_path =
                urlencoding::decode(metadata.url().unwrap_or("/").split("://").last().unwrap())
                    .unwrap();
            log::info!("Audio path: {}", audio_path);
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
                log::info!("Audio lyrics found. Parsing...");
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
    let mut current_audio = String::new();
    let mut is_current_audio = || {
        // If the audio has changed, then recall the function
        if let Ok(i) = player_handle.get_metadata() {
            let mut constructed = String::new();
            if let Some(i) = i.url() {
                constructed.push_str(i);
            }
            if let Some(i) = i.title() {
                constructed.push_str(i);
            }
            if constructed == current_audio {
                true
            } else {
                current_audio = constructed;
                false
            }
        } else {
            false
        }
    };
    is_current_audio();
    let real_lyrics = match lyrics {
        Ok(i) => i.content,
        Err(_) => Box::new([lyric::LyricsType::Standard(
            std::time::Duration::default(),
            Box::from("No lyrics"),
        )]),
    };
    let mut current_duration = current_offset.unwrap_or_default();
    for i in real_lyrics.as_ref() {
        if !is_current_audio() {
            log::warn!("Current audio has changed.");
            return;
        }
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
        if lyric.trim().is_empty() {
            continue;
        }
        duration
            .checked_sub(current_duration)
            .map(|x| std::thread::sleep(x));
        if !player_handle.is_running() {
            log::warn!("Player is not running. Stopping...");
            return;
        }
        //current_duration += duration.to_owned();
        if let Ok(i) = player_handle.get_position() {
            if i > (current_duration + std::time::Duration::from_millis(125)) {
                // Current playing position is 1 second faster than current display.
                log::warn!("Subtitle too slow. Trying to sync up...");
            } else if i + std::time::Duration::from_millis(125) < current_duration {
                // Current playing position is 1 seond slower than current display.
                // This code design disallow "return" to a point. Thus, we will simply request to recall the function.
                log::warn!("Subtitle too fast. Trying to sync up...");
                return;
            }
            current_duration = i
        } else {
            log::warn!("Player does not implement position command. Subtitle might not synced.");
            current_duration = duration.to_owned()
        }
        std::fs::write(
            target_file,
            &format!("{}", output(&lyric, formatted_metadata)),
        )
        .unwrap();
    }
    if let Some(audio_ending) = audio_ending {
        audio_ending
            .checked_sub(current_duration)
            .map(|x| std::thread::sleep(x));
    }
}
fn output(text: &str, tooltip: &str) -> String {
    format!("{{\"text\": \"{}\", \"tooltip\": \"{}\"}}", text, tooltip)
}
fn find_player<'a>() -> Result<mpris::Player<'a>, ()> {
    mpris::PlayerFinder::new()
        .map_err(|x| {
            log::error!("Error when finding mpris player: {}", x);
        })?
        .find_active()
        .map_err(|x| {
            log::error!("Error when finding active player: {}", x);
        })
}
