use std::fs::{File, self};
//use std::hash::{SipHasher, self};
//use std::io::{Read, Write};
use std::io::Read;
use std::path::{Path, PathBuf};

use problem::Problem;

pub struct Solution<'p> {
  problem: &'p Problem,
  source: PathBuf,
}

impl<'p> Solution<'p> {
  pub fn new(problem: &'p Problem) -> Option<Solution<'p>> {
    let mut source = problem.directory().join(problem.id());
    source.set_extension("rs");
    let source = source;

    let pid = problem.id();

    if source.exists() {
      info!("{}: Found rs source file ", pid);

      let mut string = String::new();
      match File::open(&source).and_then(|mut f| {
        f.read_to_string(&mut string)
      }) {
        Err(_) => {
          info!("{}: Couldn't read rs source file", pid);

          None
        },
        Ok(_) => {
          let hash_dir = Path::new(".hashes");
          fs::create_dir_all(&hash_dir).ok().
            expect("Couldn't create .hashes directory");

//          let hash_file = hash_dir.join(pid).with_extension("rs");
//          let hash = hash::Hash::<_, SipHasher>(&string).to_string();

//          if hash_file.exists() {
//            let mut old_hash = String::new();
//            File::open(&hash_file).and_then(|mut f| {
//              f.read_to_string(&mut old_hash)
//            }).ok().
//              expect("Couldn't open hash file");
//
//            if old_hash == hash {
//              info!("{}: {} source hasn't changed, skipping", pid, name);
//              return None;
//            } else {
//              info!("{}: {} source has changed, benchmarking", pid, name);
//            }
//          }

//          File::create(&hash_file).and_then(|mut f| {
//            f.write_all(hash.as_bytes())
//          }).ok().
//            expect("Couldn't write hash file");

          Some(Solution {
            problem,
            source,
          })
        }
      }

    } else {
      None
    }
  }

  pub fn problem(&self) -> &Problem {
    self.problem
  }

  pub fn source(&self) -> &Path {
    &self.source
  }
}
