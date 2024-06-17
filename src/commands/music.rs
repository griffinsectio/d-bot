use crate::*;

#[poise::command(slash_command, description_localized("en-US", "Download music from YouTube, YouTube Music, etc."))]
pub async fn music(
    ctx: Context<'_>,
    #[description = "The url of the music"]
    url: String
) -> Result<(), Error> {
    let fetching = ctx.say("Downloading the music for you...").await.unwrap();
    
    let _yt_dlp = process::Command::new("./src/yt-dlp")
    .arg("-S acodec:m4a")
    .arg("-f ba")
    .arg("-o %(title)s.%(ext)s")
    .arg("-P ./")
    .arg(url)
    .output()
    .unwrap();

    let re = Regex::new(r".*\.m4a").unwrap();

    let directory = fs::read_dir("./").unwrap();

    println!("{}", std::str::from_utf8(&_yt_dlp.stdout).unwrap());
    println!("{}", std::str::from_utf8(&_yt_dlp.stderr).unwrap());

    for dir_entry in directory {
        let entry_path = dir_entry.unwrap().path();
        let music_path = entry_path.to_str().unwrap();

        if re.is_match(music_path) {
            let music_attachment = CreateAttachment::path(music_path).await.unwrap();
            let music = CreateMessage::new()
            .add_file(music_attachment);

            ctx.channel_id().send_message(&ctx.http(), music).await.unwrap();
            std::fs::remove_file(music_path).unwrap();
        }
    }
    
    fetching.delete(ctx).await.unwrap();

    Ok(())
}