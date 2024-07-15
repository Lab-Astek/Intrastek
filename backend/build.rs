use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=prisma/schema.prisma");
    Command::new("cargo-prisma")
        .args(["generate"])
        .status()
        .unwrap();
    Command::new("cargo-prisma")
        .args(["db", "push"])
        .status()
        .unwrap();
    println!("cargo:warning=Prisma schema has been updated");
}
