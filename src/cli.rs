use clap::Parser;
use std::path::PathBuf;

// Définition de la structure CliArgs pour analyser les arguments de la ligne de commande
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    /// URL à télécharger
    pub url: Option<String>,

    /// Fichier d'entrée contenant des URLs (une par ligne)
    #[arg(short = 'i', long, value_name = "FILE")]
    pub input_file: Option<PathBuf>,

    /// Nom du fichier de sortie
    #[arg(short = 'O', long)]
    pub output: Option<PathBuf>,

    /// Préfixe du répertoire
    #[arg(short = 'P', long, value_name = "DIR")]
    pub directory_prefix: Option<PathBuf>,

    /// Limite de débit de téléchargement (par exemple, "300k", "2M")
    #[arg(long = "rate-limit")]
    pub rate_limit: Option<String>,

    /// Miroir du site web entier
    #[arg(long)]
    pub mirror: bool,

    /// Conversion des liens pour une consultation hors ligne
    #[arg(long = "convert-links")]
    pub convert_links: bool,

    /// Mode arrière-plan
    #[arg(short = 'B', long)]
    pub background: bool,

    /// Motifs de fichiers à rejeter
    #[arg(long = "reject")]
    pub reject: Option<String>,

    /// Répertoires à exclure
    #[arg(short = 'X', long = "exclude-directories")]
    pub exclude_directories: Option<String>,
}

// Fonction pour analyser les arguments de la ligne de commande
pub fn parse_args() -> CliArgs {
    CliArgs::parse()
}

impl CliArgs {
    // Méthode pour obtenir le chemin de sortie en fonction de l'URL
    pub fn get_output_path(&self, url: &str) -> PathBuf {
        let filename = if let Some(ref output) = self.output {
            output.clone()
        } else {
            crate::utils::get_filename_from_url(url)
        };

        if let Some(ref prefix) = self.directory_prefix {
            prefix.join(filename)
        } else {
            filename
        }
    }

    // Méthode pour analyser la limite de débit
    pub fn parse_rate_limit(&self) -> Option<u64> {
        self.rate_limit.as_ref().and_then(|limit| {
            let limit = limit.to_lowercase();
            if limit.ends_with('k') {
                limit[..limit.len()-1].parse::<u64>().ok().map(|n| n * 1024)
            } else if limit.ends_with('m') {
                limit[..limit.len()-1].parse::<u64>().ok().map(|n| n * 1024 * 1024)
            } else {
                limit.parse::<u64>().ok()
            }
        })
    }
}
