# Wget

Ce projet est un outil en ligne de commande pour miroiter des sites web. Il permet de télécharger des pages web et de les enregistrer localement, en conservant la structure du site et en convertissant les liens pour une consultation hors ligne.

## Fonctionnalités

- **Téléchargement de pages web** : Télécharge des pages web à partir d'une URL ou d'un fichier contenant des URLs.
- **Miroir de site entier** : Télécharge récursivement toutes les pages d'un site web.
- **Conversion des liens** : Convertit les liens absolus en liens relatifs pour une consultation hors ligne.
- **Exclusion de fichiers et répertoires** : Permet d'exclure certains types de fichiers ou répertoires du téléchargement.
- **Limitation de débit** : Permet de limiter le débit de téléchargement.

## Installation

Pour utiliser cet outil, vous devez avoir Rust installé sur votre machine. Vous pouvez installer Rust en suivant les instructions sur [rust-lang.org](https://www.rust-lang.org/tools/install).

Une fois Rust installé, clonez ce dépôt et compilez le projet :

```bash
git clone https://github.com/votre-utilisateur/miroir-site-web.git
cd miroir-site-web
cargo build --release

Utilisation

Après avoir compilé le projet, vous pouvez utiliser l'outil en ligne de commande comme suit :

./target/release/miroir-site-web --url <URL> [options]

Options

    --url <URL> : URL à télécharger.
    -i, --input-file <FILE> : Fichier d'entrée contenant des URLs (une par ligne).
    -O, --output <FILE> : Nom du fichier de sortie.
    -P, --directory-prefix <DIR> : Préfixe du répertoire pour enregistrer les fichiers.
    --rate-limit <LIMIT> : Limite de débit de téléchargement (par exemple, "300k", "2M").
    --mirror : Miroir du site web entier.
    --convert-links : Convertir les liens pour une consultation hors ligne.
    -B, --background : Mode arrière-plan.
    --reject <PATTERN> : Motifs de fichiers à rejeter (séparés par des virgules).
    -X, --exclude-directories <DIRS> : Répertoires à exclure (séparés par des virgules).

Exemple

Pour miroiter un site web entier et convertir les liens pour une consultation hors ligne :

./target/release/miroir-site-web --url https://example.com --mirror --convert-links