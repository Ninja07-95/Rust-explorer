use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Chemin du dossier à rechercher
    pub path: String,
    
    /// Motif de recherche (nom ou contenu)
    pub pattern: String,
    
    /// Rechercher dans le contenu des fichiers
    #[arg(short, long)]
    pub content: bool,
    
    /// Utiliser une expression régulière
    #[arg(short, long)]
    pub regex: bool,
    
    /// Nombre de threads à utiliser
    #[arg(short, long, default_value_t = 4)]
    pub threads: usize,
    
    /// Afficher les détails de performance
    #[arg(short, long)]
    pub benchmark: bool,
}
