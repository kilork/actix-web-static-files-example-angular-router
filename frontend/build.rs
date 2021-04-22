use static_files::NpmBuild;

fn main() {
    NpmBuild::new("./web")
        .target("./web/dist/angular-router-sample")
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .change_detection()
        .to_resource_dir()
        .build()
        .unwrap();
}
