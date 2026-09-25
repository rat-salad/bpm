use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::process::Command;
use std::fs::remove_dir_all;

fn main() {
    let pkg_repo = "https://raw.githubusercontent.com/rat-salad/test-repo/refs/heads/main/";
    let args: Vec<String> = env::args().collect();
    let pkg_name: &str = &args[2];
    if args[1] == "rm" {
        find_local_installed_pkg();
    } else if args[1] == "get" {
        let buildfile = download_buildfile(pkg_name, pkg_repo);
        if validity_check(&buildfile) {
            buildfile_installer(buildfile.clone());
            installation_cleanup();
            package_db_manager(true, buildfile.clone());
        }
    } else {
        println!("━━> !!! Invalid command line option \"{}\", exiting", args[1]);
        exit(1);
    }
}

fn validity_check(buildfile: &str) -> bool {
    let check_categories = ["name", "description", "version", "files", "dependencies", "source", "build()"];
    println!("━┯> Starting buildfile validity checks");
    for i in check_categories {
        if buildfile.contains(i) {
            println!(" ├─> Valid package {:?} found", i);
        } else {
            println!(" └─> !!! No valid package {:?} found, terminating !!!", i);
            exit(1);
        }
    }
    println!(" └─> Package validity checks complete!\n");
    return true;
}

fn download_buildfile(pkg_name: &str, pkg_repo: &str) -> String {
    let pkg_url = pkg_repo.to_owned() + pkg_name;
    println!("━┯> Fetching buildfile from {pkg_repo}");
    println!(" ├─> Buildfile url: {}", pkg_url);

    let get_buildfile = Command::new("curl")
        .arg(pkg_url)
        .output();
    let response = String::from_utf8(get_buildfile.unwrap().stdout);
    if response.as_ref().expect("").len() <= 20 {
        println!(" └─> !!! Buildfile not found, either it is incomplete or does not exist");
        exit(1);
    }

    println!(" └─> Buildfile downloaded successfully!\n");
    return response
        .expect(" └─> !!! No buildfile found but it didnt exit at the right time i lowk dont understand why this happened if it did ????");
}

fn find_local_installed_pkg() {

}

fn find_pkg_name(buildfile: String) -> String {
    let mut name = String::new();
    for line in buildfile.lines() {
        if line.contains("name=") {
            name.push_str(&line.replace("name=", "").to_string());
        }
    }
    name
}
fn find_pkg_version(buildfile: String) -> String {
    let mut version = String::new();
    for line in buildfile.lines() {
        if line.contains("version=") {
            version.push_str(&line.replace("version=", "").to_string());
        }
    }
    version
}
fn buildfile_installer(buildfile: String) {
    println!("━┯> Beginning installation of package");

    let mut pkg_title = find_pkg_name(buildfile.clone());
    pkg_title.push_str("-");
    pkg_title.push_str(&find_pkg_version(buildfile.clone()));
    print!(" ├─> Package source directory: {}\n ├─> Downloading package source with ", pkg_title);
    for line in buildfile.lines() {
        if line.contains("source=") {
            let buildfile_proc = line.split_once("+");
            if let Some(i) = buildfile_proc {
                if i.0.replace("source=", "") == "git" {
                    print!("git\n");
                    let get_source = Command::new("git")
                        .arg("clone")
                        .arg(i.1.replace("source=", ""))
                        .arg(&pkg_title)
                        .output();
                }
            println!(" ├─> Package source downloaded\n ├─> Changing to source directory");

            let mut path = env::current_dir().unwrap();
            path.push(pkg_title.to_string());

            assert!(env::set_current_dir(&path).is_ok());

            println!(" ├─> Building and installing package from buildfile");

            let install_package = Command::new("/bin/sh")
                .args (&["-c", &buildfile])
                .output();
            println!("{:#?}", install_package);
            println!(" └─> Package successfully installed!\n");
            }
        }
    }
}

fn installation_cleanup() {
    println!("━┯> Removing up source directory");
    let mut src_directory = env::current_dir().unwrap();
    remove_dir_all(env::current_dir().unwrap());
    println!(" └─> Source directory removed!")
}

fn pkg_uninstaller() {

}

fn package_db_manager(add: bool, buildfile: String) {
    if add == true {
        println!("━┯> Updating package list with current package");

    } else {

    }
}
