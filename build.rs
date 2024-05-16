use std::process::Command;

fn main() {
    std::fs::remove_file("public/index.css").ok();

    Command::new("bunx")
        .args([
            "tailwindcss",
            "-c",
            "tailwind.config.js",
            "-i",
            "assets/styles/index.css",
            "-o",
            "public/index.css",
            "--minify",
        ])
        .status()
        .expect("failed to run tailwindcss");
}
