fn main() {
    println!("cargo:rerun-if-changed=migrations/gatekeeper");
    println!("cargo:rerun-if-changed=migrations/tenant");
}
