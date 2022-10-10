use crate::{
    cache::{Cache, MprCache},
    style,
};
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

    let cache = Cache::new(AptCache::new(), MprCache::new());
    let mut candidates = Vec::new();

    if !pkglist.is_empty() {
        for pkg in pkglist {
            if cache.apt_cache().get(pkg).is_some()
                || cache.mpr_cache().packages().get(pkg).is_some()
            {
                candidates.push(pkg.to_string());
            }
        }
    } else {
        for pkg in Cache::get_nonvirtual_packages(cache.apt_cache(), &PackageSort::default()) {
            let pkgname = pkg.name();
            if !candidates.contains(&pkgname) {
                candidates.push(pkgname);
            }
        }

        for pkg in cache.mpr_cache().packages().values() {
            if !candidates.contains(&pkg.pkgname) {
                candidates.push(pkg.pkgname.to_string());
            }
        }
    }

    if candidates.is_empty() {
        quit::with_code(exitcode::UNAVAILABLE);
    }

    print!(
        "{}",
        style::generate_pkginfo_entries(
            &candidates,
            &cache,
            apt_only,
            mpr_only,
            installed_only,
            name_only
        )
    );
}
