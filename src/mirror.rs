use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use reqwest::blocking::Client;
use url::Url;
use regex::Regex;

// Fonction principale pour miroiter un site web
pub fn mirror_site(args: crate::cli::CliArgs) -> Result<(), Box<dyn std::error::Error>> {
    let base_url = args.url.as_ref().expect("URL is required for mirroring");
    let base_url = Url::parse(base_url)?;

    let domain = base_url.domain().expect("Invalid URL");
    fs::create_dir_all(domain)?;

    let mut visited = HashSet::new();
    let client = Client::new();

    // Compiler les motifs de rejet
    let reject_patterns = args.reject.as_ref().map(|r| {
        r.split(',')
            .map(|s| format!(".*\\.{}$", s))
            .collect::<Vec<_>>()
    });

    let reject_regex = reject_patterns.as_ref().map(|patterns| {
        Regex::new(&patterns.join("|")).unwrap()
    });

    // Analyser les répertoires exclus
    let excluded_dirs: Vec<String> = args.exclude_directories
        .as_ref()
        .map(|dirs| dirs.split(',').map(String::from).collect())
        .unwrap_or_default();

    mirror_recursive(
        &base_url,
        Path::new(domain),
        &client,
        &mut visited,
        &reject_regex,
        &excluded_dirs,
        &args,
    )?;

    if args.convert_links {
        convert_links(Path::new(domain))?;
    }

    Ok(())
}

// Fonction récursive pour miroiter les pages du site
fn mirror_recursive(
    url: &Url,
    base_path: &Path,
    client: &Client,
    visited: &mut HashSet<String>,
    reject_regex: &Option<Regex>,
    excluded_dirs: &[String],
    args: &crate::cli::CliArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let url_str = url.as_str();

    // Vérifier si l'URL correspond à un motif de rejet
    if let Some(regex) = reject_regex {
        if regex.is_match(url_str) {
            return Ok(());
        }
    }

    // Ignorer les répertoires exclus
    if excluded_dirs.iter().any(|dir| url.path().starts_with(dir)) {
        return Ok(());
    }

    if visited.contains(url_str) {
        return Ok(());
    }

    visited.insert(url_str.to_string());

    // Télécharger la page actuelle
    let response = client.get(url_str).send()?;
    if !response.status().is_success() {
        return Ok(());
    }

    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let relative_path = url.path();
    let file_path = if relative_path.ends_with('/') || relative_path.is_empty() {
        base_path.join("index.html")
    } else {
        base_path.join(&relative_path[1..])
    };

    // Vérifier si le répertoire est exclu avant de le créer
    if let Some(parent) = file_path.parent() {
        if !excluded_dirs.iter().any(|dir| parent.to_string_lossy().contains(dir)) {
            fs::create_dir_all(parent)?;
        } else {
            return Ok(());
        }
    }

    // Sauvegarder le fichier
    let content = response.bytes()?;
    fs::write(&file_path, content)?;

    // Si c'est du HTML, analyser et suivre les liens récursivement
    if content_type.contains("text/html") {
        let file_content = fs::read(&file_path)?;
        let html = String::from_utf8_lossy(&file_content);
        let links = extract_links(&html);

        for link in links {
            if let Ok(absolute_url) = url.join(&link) {
                if absolute_url.domain() == url.domain() {
                    mirror_recursive(
                        &absolute_url,
                        base_path,
                        client,
                        visited,
                        reject_regex,
                        excluded_dirs,
                        args,
                    )?;
                }
            }
        }
    }

    Ok(())
}

// Extraire les liens d'une page HTML
fn extract_links(html: &str) -> Vec<String> {
    let mut links = Vec::new();

    // Liens href
    let href_regex = Regex::new(r#"href=["']([^"']+)["']"#).unwrap();
    for cap in href_regex.captures_iter(html) {
        links.push(cap[1].to_string());
    }

    // Liens src
    let src_regex = Regex::new(r#"src=["']([^"']+)["']"#).unwrap();
    for cap in src_regex.captures_iter(html) {
        links.push(cap[1].to_string());
    }

    links
}

// Convertir les liens absolus en relatifs dans les fichiers HTML
fn convert_links(base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let html_files: Vec<PathBuf> = walkdir::WalkDir::new(base_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "html"))
        .map(|e| e.path().to_path_buf())
        .collect();

    for file_path in html_files {
        let content = fs::read_to_string(&file_path)?;
        let base_url = file_path.parent().unwrap();

        // Convertir les URL absolues en relatives
        let modified_content = convert_absolute_to_relative(&content, base_url);

        fs::write(file_path, modified_content)?;
    }

    Ok(())
}

// Convertir les URL absolues en relatives
fn convert_absolute_to_relative(content: &str, base_path: &Path) -> String {
    let mut result = content.to_string();

    // Convertir les liens href
    let href_regex = Regex::new(r#"href=["'](https?://[^"']+)["']"#).unwrap();
    result = href_regex.replace_all(&result, |caps: &regex::Captures| {
        format!("href=\"{}\"", make_relative(&caps[1], base_path))
    }).to_string();

    // Convertir les liens src
    let src_regex = Regex::new(r#"src=["'](https?://[^"']+)["']"#).unwrap();
    result = src_regex.replace_all(&result, |caps: &regex::Captures| {
        format!("src=\"{}\"", make_relative(&caps[1], base_path))
    }).to_string();

    result
}

// Créer un chemin relatif à partir d'une URL absolue
fn make_relative(url: &str, _base_path: &Path) -> String {
    if let Ok(url) = Url::parse(url) {
        let path = url.path();
        if path.is_empty() || path == "/" {
            "index.html".to_string()
        } else {
            path[1..].to_string()
        }
    } else {
        url.to_string()
    }
}
