use static_files::{NodeModulesStrategy, NpmBuild};

fn main() {
    unsafe {
        std::env::set_var("NODE_OPTIONS", "--openssl-legacy-provider");
    }
    NpmBuild::new("./web")
        .node_modules_strategy(NodeModulesStrategy::MoveToOutDir)
        .target("node_modules/dist/angular-router-sample")
        .install()
        .unwrap()
        .run("build")
        .unwrap()
        .change_detection()
        .to_resource_dir()
        .build()
        .unwrap();
}
