use arbitrary::{Result, Unstructured};
use std::borrow::Cow;
use std::path::Path;
use wasm_encoder::{CustomSection, Encode, Section};
use wit_component::*;
use wit_parser::{Resolve, SourceMap};

pub fn run(u: &mut Unstructured<'_>) -> Result<()> {
    let wasm = u.arbitrary().and_then(|config| {
        log::debug!("config: {config:#?}");
        wit_smith::smith(&config, u)
    })?;
    write_file("doc1.wasm", &wasm);
    let (resolve, pkg) = match wit_component::decode(&wasm).unwrap() {
        DecodedWasm::WitPackage(resolve, pkg) => (resolve, pkg),
        DecodedWasm::Component(..) => unreachable!(),
    };

    // If we've decoded an empty package, make sure to only use the v1 decoder (v2 has no way to
    // represent an empty document).
    let encoding_version = {
        let package = &resolve.packages[pkg];
        let is_empty = package.worlds.is_empty() && package.interfaces.is_empty();
        if is_empty {
            Some(false)
        } else {
            u.arbitrary()?
        }
    };

    log::debug!("encoding_version: {encoding_version:?}");

    roundtrip_through_printing(encoding_version, "doc1", &resolve, &wasm);

    let (resolve2, pkg2) = match wit_component::decode(&wasm).unwrap() {
        DecodedWasm::WitPackage(resolve, pkg) => (resolve, pkg),
        DecodedWasm::Component(..) => unreachable!(),
    };

    let wasm2 = wit_component::encode(encoding_version, &resolve2, pkg2)
        .expect("failed to encode WIT document");
    write_file("doc2.wasm", &wasm2);
    roundtrip_through_printing(encoding_version, "doc2", &resolve2, &wasm2);

    if wasm != wasm2 {
        panic!("roundtrip wasm didn't match");
    }

    // If there's hundreds or thousands of worlds only work with the first few
    // to avoid timing out this fuzzer with asan enabled.
    for (id, _world) in resolve.worlds.iter().take(20) {
        let mut dummy = wit_component::dummy_module(&resolve, id);
        let metadata =
            wit_component::metadata::encode(&resolve, id, StringEncoding::UTF8, None).unwrap();
        let section = CustomSection {
            name: "component-type".into(),
            data: Cow::Borrowed(&metadata),
        };
        dummy.push(section.id());
        section.encode(&mut dummy);

        write_file("dummy.wasm", &dummy);
        let wasm = wit_component::ComponentEncoder::default()
            .module(&dummy)
            .unwrap()
            .encode()
            .unwrap();
        write_file("dummy.component.wasm", &wasm);
        wasmparser::Validator::new_with_features(wasmparser::WasmFeatures {
            component_model: true,
            ..Default::default()
        })
        .validate_all(&wasm)
        .unwrap();

        wit_component::decode(&wasm).unwrap();
    }
    Ok(())
}

fn roundtrip_through_printing(
    encoding_version: Option<bool>,
    file: &str,
    resolve: &Resolve,
    wasm: &[u8],
) {
    // For all packages in `resolve` print them all to a string, then re-parse
    // them and insert them into a `new_resolve`.
    let mut new_resolve = Resolve::default();
    let mut last = None;
    for (id, pkg) in resolve.packages.iter() {
        let mut map = SourceMap::new();
        let pkg_name = &pkg.name;
        let doc = WitPrinter::default().print(resolve, id).unwrap();
        write_file(&format!("{file}-{pkg_name}.wit"), &doc);
        map.push(format!("{pkg_name}.wit").as_ref(), doc);
        let unresolved = map.parse().unwrap();
        let id = new_resolve.push(unresolved).unwrap();
        last = Some(id);
    }

    // Finally encode the `new_resolve` which should be the exact same as
    // before.
    let wasm2 = wit_component::encode(encoding_version, &new_resolve, last.unwrap()).unwrap();
    write_file(&format!("{file}-reencoded.wasm"), &wasm2);
    if wasm != wasm2 {
        panic!("failed to roundtrip through text printing");
    }
}

fn write_file(path: &str, contents: impl AsRef<[u8]>) {
    if !log::log_enabled!(log::Level::Debug) {
        return;
    }
    log::debug!("writing file {path}");
    let contents = contents.as_ref();
    let path = Path::new(path);
    std::fs::write(path, contents).unwrap();
    if path.extension().and_then(|s| s.to_str()) == Some("wasm") {
        let path = path.with_extension("wat");
        log::debug!("writing file {}", path.display());
        std::fs::write(path, wasmprinter::print_bytes(&contents).unwrap()).unwrap();
    }
}
