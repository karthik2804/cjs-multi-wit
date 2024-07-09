use anyhow::{anyhow, Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use wit_component::{DecodedWasm, WitPrinter};
use wit_parser::{PackageId, Resolve};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long = "output-world", required = true)]
    output_world: String,
    #[clap(long = "wit-path")]
    wit_paths: Vec<String>,
    #[clap(long = "world")]
    worlds: Vec<String>,
    #[clap(long = "output-dir", default_value = "combined_wit")]
    output_dir: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut resolve = Resolve::default();
    let id = resolve.push_str("component.wit", &target_wit_source(&args.output_world))?;

    for path in &args.wit_paths {
        let (temp, _) = parse_wit(&PathBuf::from(path)).map_err(|e| anyhow!("{:?}", e))?;
        resolve.merge(temp).expect("could not merge wits");
    }

    let (world, _) = resolve
        .worlds
        .iter()
        .find(|(world, _)| resolve.worlds[*world].name == *args.output_world)
        .unwrap();

    for w in &args.worlds {
        let (temp, _) = resolve
            .worlds
            .iter()
            .find(|(world, _)| resolve.worlds[*world].name == *w)
            .unwrap();
        resolve.merge_worlds(temp, world).map_err(|e| {
            anyhow!(
                "unable to merge with world '{}' due to: {}",
                resolve.worlds[temp].name,
                e
            )
        })?;
    }

    let decoded = DecodedWasm::WitPackages(resolve, id);

    let resolve = decoded.resolve();
    let main = decoded.packages();

    let mut printer = WitPrinter::default();
    printer.emit_docs(false);

    let dir = PathBuf::from(args.output_dir);

    std::fs::create_dir_all(&dir).with_context(|| format!("failed to create directory"))?;

    let mut names = HashMap::new();
    for (_id, pkg) in resolve.packages.iter() {
        let cnt = names
            .entry(&pkg.name.name)
            .or_insert(HashMap::new())
            .entry(&pkg.name.namespace)
            .or_insert(0);
        *cnt += 1;
    }

    for (id, pkg) in resolve.packages.iter() {
        let output = printer.print(resolve, &[id])?;
        let out_dir = if main.contains(&id) {
            dir.clone()
        } else {
            let dir = dir.join("deps");
            let packages_with_same_name = &names[&pkg.name.name];
            let version = &pkg.name.version;
            let version_suffix = if let Some(version) = version {
                format!("-{}", version.to_string())
            } else {
                "".to_string()
            };
            if packages_with_same_name.len() == 1 {
                dir.join(format!("{}{}", &pkg.name.name, version_suffix))
            } else {
                let packages_with_same_namespace = packages_with_same_name[&pkg.name.namespace];
                if packages_with_same_namespace == 1 {
                    dir.join(format!("{}:{}", pkg.name.namespace, pkg.name.name))
                } else {
                    dir.join(pkg.name.to_string())
                }
            }
        };
        std::fs::create_dir_all(&out_dir)
            .with_context(|| format!("failed to create directory: {out_dir:?}"))?;
        let path = out_dir.join("main.wit");
        std::fs::write(&path, &output)
            .with_context(|| format!("failed to write file: {path:?}"))?;
    }

    Ok(())
}

fn parse_wit(path: &Path) -> Result<(Resolve, Vec<PackageId>)> {
    let mut resolve = Resolve::default();
    let id = if path.is_dir() {
        resolve.push_dir(&path)?.0
    } else {
        resolve.push_file(&path)?
    };
    Ok((resolve, id))
}

fn target_wit_source(world_name: &str) -> String {
    return format!(
        "package knitwit:combined;

world {world_name} {{
}}",
    );
}
