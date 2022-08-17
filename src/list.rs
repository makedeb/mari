use crate::{cache::{Cache, CachePackage, MprCache}, style};
use rust_apt::cache::{Cache as AptCache, PackageSort};

pub fn list(args: &clap::ArgMatches) {
    let pkglist: Vec<&String> = match args.get_many("pkg") {
        Some(pkglist) => pkglist.collect(),
        None => Vec::new(),
    };
    let apt_only = args.is_present("apt-only");
    let mpr_only = args.is_present("mpr-only");
    let installed_only = args.is_present("installed-only");
    let name_only = args.is_present("name-only");
    let mpr_url: &String = args.get_one("mpr-url").unwrap();

    let cache = Cache::new(AptCache::new(), MprCache::new(mpr_url));
    let mut candidates: Vec<&Vec<CachePackage>> = Vec::new();

    if !pkglist.is_empty() {
        for pkg in pkglist {
            if let Some(pkg_group) = cache.pkgmap().get(pkg) {
                candidates.push(pkg_group);
            }
        }
    } else {
        for pkg_group in cache.pkgmap().values() {
            candidates.push(pkg_group);
        }
    }

    let mut matches: Vec<&Vec<CachePackage>> = Vec::new();
    print!("{}", style::generate_pkginfo_entries(&candidates, &cache, apt_only, mpr_only, installed_only, name_only));
}
