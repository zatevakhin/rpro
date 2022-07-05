
fn main() {

    tonic_build::configure()
         .build_server(true)
         .compile(
             &["proto/hello/hello.proto"],
             &["proto/hello"],
         ).unwrap();

    tonic_build::configure()
         .build_server(true)
         .compile(
             &["proto/account/account.proto"],
             &["proto/account"],
         ).unwrap();

 }
