use rustube;

#[tokio::main]
async fn main() {
    println!(
        r"
    
░█████╗░░██████╗██╗░░██╗░█████╗░███╗░░██╗██████╗░███████╗██████╗░
██╔══██╗██╔════╝██║░██╔╝██╔══██╗████╗░██║██╔══██╗██╔════╝██╔══██╗
███████║╚█████╗░█████═╝░███████║██╔██╗██║██║░░██║█████╗░░██████╔╝
██╔══██║░╚═══██╗██╔═██╗░██╔══██║██║╚████║██║░░██║██╔══╝░░██╔══██╗
██║░░██║██████╔╝██║░╚██╗██║░░██║██║░╚███║██████╔╝███████╗██║░░██║
╚═╝░░╚═╝╚═════╝░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░╚══╝╚═════╝░╚══════╝╚═╝░░╚═╝
    "
    );
    println!("github =>     https://github.com/hamdymohamedak ");
    println!("Linkedin => https://www.linkedin.com/in/hamdy-askander-b67b32246/ ");
    println!("Facebook =>   https://www.facebook.com/hamdy.elgokar.5 ");


    println!("Enter Video Name:");

    let mut name = String::new();
    // Read user name input
    // get video Name
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter Video URL:");
    let mut get_url = String::new();
    // Read video URL input
    // get video URL
    std::io::stdin()
        .read_line(&mut get_url)
        .expect("Failed to read line");

    // Trim input URL
    let url = get_url.trim();

    // Download the best quality video
    match rustube::download_best_quality(&url).await {
        Ok(video_path) => {
            let video_file_name = format!("{}.mp4", name.trim());
            std::fs::rename(video_path, &video_file_name).expect("Failed to rename video file");
            println!("Downloaded video as {}", video_file_name);
        }
        Err(err) => eprintln!("Error downloading video: {:?}", err),
    };
}
